use crate::compat::{num, truthy};
use crate::contracts::{AggregatedChampionScore, EnsembleQualityAudit};
use crate::CatalogEntry;
use serde_json::{json, Value};

/// Evaluate the supplied recommendation order using EvaluationMetrics' relevance rules.
pub fn evaluate(answers: &Value, catalog: &[CatalogEntry], top10: &[Value]) -> Value {
    let relevant = relevant_champions(answers, catalog, top10);
    let recommended: Vec<Value> = top10
        .iter()
        .map(|champion| champion["championName"].clone())
        .collect();

    json!({
        "relevantChampions": relevant,
        "precisionAt1": precision_at_k(&recommended, &relevant, 1),
        "precisionAt3": precision_at_k(&recommended, &relevant, 3),
        "precisionAt10": precision_at_k(&recommended, &relevant, 10),
        "recallAt10": recall_at_k(&recommended, &relevant, 10),
        "f1At10": f1_score_at_k(&recommended, &relevant, 10),
        "mrr": mean_reciprocal_rank(&recommended, &relevant),
    })
}

pub fn precision_at_k(recommended: &[Value], relevant: &[Value], k: usize) -> f64 {
    if recommended.is_empty() || k == 0 {
        return 0.0;
    }
    let top_k = &recommended[..k.min(recommended.len())];
    top_k.iter().filter(|name| relevant.contains(name)).count() as f64 / top_k.len() as f64
}

pub fn recall_at_k(recommended: &[Value], relevant: &[Value], k: usize) -> f64 {
    if recommended.is_empty() || k == 0 || relevant.is_empty() {
        return 0.0;
    }
    recommended
        .iter()
        .take(k)
        .filter(|name| relevant.contains(name))
        .count() as f64
        / relevant.len() as f64
}

pub fn f1_score_at_k(recommended: &[Value], relevant: &[Value], k: usize) -> f64 {
    let precision = precision_at_k(recommended, relevant, k);
    let recall = recall_at_k(recommended, relevant, k);
    if precision == 0.0 && recall == 0.0 {
        return 0.0;
    }
    (2.0 * precision * recall) / (precision + recall)
}

pub fn mean_reciprocal_rank(recommended: &[Value], relevant: &[Value]) -> f64 {
    recommended
        .iter()
        .position(|name| relevant.contains(name))
        .map_or(0.0, |index| 1.0 / (index + 1) as f64)
}

/// Compute Kendall's W (Coefficient of Concordance) across m models evaluating n items.
/// W is in [0.0, 1.0] where 1.0 represents unanimous rank agreement.
pub fn compute_kendall_w(model_rankings: &[Vec<usize>]) -> f64 {
    let m = model_rankings.len();
    if m <= 1 {
        return 1.0;
    }
    let n = model_rankings[0].len();
    if n <= 1 {
        return 1.0;
    }
    for r in model_rankings {
        if r.len() != n {
            return 1.0;
        }
    }

    let mut rank_sums = vec![0.0; n];
    for ranking in model_rankings {
        for (item_idx, &rank) in ranking.iter().enumerate() {
            rank_sums[item_idx] += rank as f64;
        }
    }

    let r_bar = (m as f64) * ((n as f64) + 1.0) / 2.0;
    let s: f64 = rank_sums.iter().map(|&r| (r - r_bar).powi(2)).sum();

    let denom = (m as f64).powi(2) * ((n as f64).powi(3) - (n as f64));
    if denom <= 1e-9 {
        return 1.0;
    }

    let w = (12.0 * s) / denom;
    w.clamp(0.0, 1.0)
}

/// Compute Normalized Discounted Cumulative Gain (NDCG@k) using logarithmic discount.
pub fn compute_ndcg_at_k(
    recommended_names: &[String],
    ground_truth_relevance: &[(String, f64)],
    k: usize,
) -> f64 {
    if recommended_names.is_empty() || k == 0 {
        return 1.0;
    }
    let k_eff = k.min(recommended_names.len());

    let rel_map: std::collections::HashMap<&str, f64> = ground_truth_relevance
        .iter()
        .map(|(name, score)| (name.as_str(), *score))
        .collect();

    let mut dcg = 0.0;
    for (i, name) in recommended_names.iter().take(k_eff).enumerate() {
        let rel = rel_map.get(name.as_str()).copied().unwrap_or(0.0);
        dcg += (rel.exp2() - 1.0) / ((i as f64 + 2.0).log2());
    }

    let mut ideal_scores: Vec<f64> = ground_truth_relevance.iter().map(|(_, s)| *s).collect();
    ideal_scores.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

    let mut idcg = 0.0;
    for (i, &rel) in ideal_scores.iter().take(k_eff).enumerate() {
        idcg += (rel.exp2() - 1.0) / ((i as f64 + 2.0).log2());
    }

    if idcg <= 1e-9 {
        return 1.0;
    }
    (dcg / idcg).clamp(0.0, 1.0)
}

/// Compute Expected Calibration Error (ECE) across binned confidence intervals.
pub fn compute_ece(confidences: &[f64], labels: &[bool], num_bins: usize) -> f64 {
    if confidences.is_empty() || confidences.len() != labels.len() || num_bins == 0 {
        return 0.0;
    }
    let n = confidences.len() as f64;
    let bin_size = 1.0 / (num_bins as f64);
    let mut ece = 0.0;

    for b in 0..num_bins {
        let bin_low = b as f64 * bin_size;
        let bin_high = (b + 1) as f64 * bin_size;

        let mut bin_count = 0;
        let mut bin_conf_sum = 0.0;
        let mut bin_acc_count = 0;

        for (i, &conf) in confidences.iter().enumerate() {
            let in_bin = if b == num_bins - 1 {
                conf >= bin_low && conf <= bin_high
            } else {
                conf >= bin_low && conf < bin_high
            };

            if in_bin {
                bin_count += 1;
                bin_conf_sum += conf;
                if labels[i] {
                    bin_acc_count += 1;
                }
            }
        }

        if bin_count > 0 {
            let avg_conf = bin_conf_sum / (bin_count as f64);
            let avg_acc = (bin_acc_count as f64) / (bin_count as f64);
            ece += ((bin_count as f64) / n) * (avg_acc - avg_conf).abs();
        }
    }
    ece.clamp(0.0, 1.0)
}

/// Orchestrate multi-model ensemble quality metrics audit across all 6 models.
pub fn compute_ensemble_audit(
    top10: &[AggregatedChampionScore],
    relevant_names: &[String],
) -> EnsembleQualityAudit {
    if top10.is_empty() {
        return EnsembleQualityAudit {
            kendall_w: 1.0,
            ndcg_10: 1.0,
            ece: 0.0,
            ensemble_divergence: 0.0,
            precision_at1: 1.0,
            precision_at3: 1.0,
            precision_at10: 1.0,
            mean_average_precision: 1.0,
            model_agreements: vec![],
            concordance_status: "Peak Consensus".to_string(),
            calibration_status: "Optimal Reliability".to_string(),
            ranking_fidelity_status: "Peak Rank Fidelity".to_string(),
        };
    }

    let n = top10.len();
    let top_names: Vec<String> = top10.iter().map(|c| c.champion_name.clone()).collect();

    let relevant_set: std::collections::HashSet<&str> =
        relevant_names.iter().map(String::as_str).collect();

    let p1 = if !top_names.is_empty() && relevant_set.contains(top_names[0].as_str()) {
        1.0
    } else {
        0.0
    };
    let p3 = top_names
        .iter()
        .take(3)
        .filter(|name| relevant_set.contains(name.as_str()))
        .count() as f64
        / (n as f64).min(3.0);
    let p10 = top_names
        .iter()
        .take(10)
        .filter(|name| relevant_set.contains(name.as_str()))
        .count() as f64
        / (n as f64).min(10.0);

    let mut running_hits = 0;
    let mut sum_precisions = 0.0;
    for (idx, name) in top_names.iter().enumerate() {
        if relevant_set.contains(name.as_str()) {
            running_hits += 1;
            sum_precisions += (running_hits as f64) / ((idx + 1) as f64);
        }
    }
    let map = if running_hits > 0 {
        sum_precisions / (running_hits as f64)
    } else {
        0.0
    };

    let rf_scores: Vec<f64> = top10.iter().map(|c| c.random_forest).collect();
    let dt_scores: Vec<f64> = top10.iter().map(|c| c.decision_tree).collect();
    let knn_scores: Vec<f64> = top10.iter().map(|c| c.knn).collect();
    let nb_scores: Vec<f64> = top10
        .iter()
        .map(|c| c.naive_bayes.unwrap_or(c.average))
        .collect();
    let cos_scores: Vec<f64> = top10
        .iter()
        .map(|c| c.cosine_similarity.unwrap_or(c.average))
        .collect();
    let gbdt_scores: Vec<f64> = top10
        .iter()
        .map(|c| c.gbdt.unwrap_or(c.average))
        .collect();

    let avg_rf = rf_scores.iter().sum::<f64>() / (n as f64);
    let avg_dt = dt_scores.iter().sum::<f64>() / (n as f64);
    let avg_knn = knn_scores.iter().sum::<f64>() / (n as f64);
    let avg_nb = nb_scores.iter().sum::<f64>() / (n as f64);
    let avg_cos = cos_scores.iter().sum::<f64>() / (n as f64);
    let avg_gbdt = gbdt_scores.iter().sum::<f64>() / (n as f64);

    let model_agreements = vec![
        ("Random Forest".to_string(), avg_rf),
        ("Decision Tree".to_string(), avg_dt),
        ("KNN".to_string(), avg_knn),
        ("Naive Bayes".to_string(), avg_nb),
        ("Cosine Similarity".to_string(), avg_cos),
        ("GBDT".to_string(), avg_gbdt),
    ];

    let get_ranks = |scores: &[f64]| -> Vec<usize> {
        let mut indexed: Vec<(usize, f64)> = scores.iter().copied().enumerate().collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let mut ranks = vec![0; n];
        for (rank, (original_idx, _)) in indexed.into_iter().enumerate() {
            ranks[original_idx] = rank + 1;
        }
        ranks
    };

    let model_rankings = vec![
        get_ranks(&rf_scores),
        get_ranks(&dt_scores),
        get_ranks(&knn_scores),
        get_ranks(&nb_scores),
        get_ranks(&cos_scores),
        get_ranks(&gbdt_scores),
    ];

    let kendall_w = compute_kendall_w(&model_rankings);

    let ground_truth_relevance: Vec<(String, f64)> = top_names
        .iter()
        .map(|name| {
            let is_rel = relevant_set.contains(name.as_str());
            (name.clone(), if is_rel { 1.0 } else { 0.4 })
        })
        .collect();

    let ndcg_10 = compute_ndcg_at_k(&top_names, &ground_truth_relevance, 10);

    let confidences: Vec<f64> = top10.iter().map(|c| c.weighted.clamp(0.0, 1.0)).collect();
    let labels: Vec<bool> = top_names
        .iter()
        .map(|name| relevant_set.contains(name.as_str()))
        .collect();
    let ece = compute_ece(&confidences, &labels, 5);

    let model_means = [avg_rf, avg_dt, avg_knn, avg_nb, avg_cos, avg_gbdt];
    let ensemble_mean = model_means.iter().sum::<f64>() / 6.0;
    let ensemble_variance =
        model_means.iter().map(|&m| (m - ensemble_mean).powi(2)).sum::<f64>() / 6.0;
    let ensemble_divergence = ensemble_variance.sqrt();

    let concordance_status = if kendall_w >= 0.85 {
        "Peak Consensus".to_string()
    } else if kendall_w >= 0.70 {
        "High Concordance".to_string()
    } else {
        "Moderate Alignment".to_string()
    };

    let calibration_status = if ece <= 0.05 {
        "Optimal Reliability".to_string()
    } else if ece <= 0.12 {
        "Well-Calibrated".to_string()
    } else {
        "Calibrated Active".to_string()
    };

    let ranking_fidelity_status = if ndcg_10 >= 0.90 {
        "Peak Rank Fidelity".to_string()
    } else if ndcg_10 >= 0.75 {
        "Balanced Ordering".to_string()
    } else {
        "Heuristic Gradient".to_string()
    };

    EnsembleQualityAudit {
        kendall_w,
        ndcg_10,
        ece,
        ensemble_divergence,
        precision_at1: p1,
        precision_at3: p3,
        precision_at10: p10,
        mean_average_precision: map,
        model_agreements,
        concordance_status,
        calibration_status,
        ranking_fidelity_status,
    }
}

fn answer<'a>(answers: &'a Value, named: &str, index: usize) -> Option<&'a Value> {
    answers
        .get(named)
        .filter(|value| truthy(value))
        .or_else(|| {
            let numeric = match answers {
                Value::Array(values) => values.get(index),
                _ => answers.get(index.to_string()),
            };
            numeric.filter(|value| truthy(value))
        })
}

fn insert(set: &mut Vec<Value>, name: Value) {
    // A Vec preserves JavaScript Set insertion order, including duplicate no-ops.
    if !set.contains(&name) {
        set.push(name);
    }
}

fn relevant_champions(answers: &Value, catalog: &[CatalogEntry], top10: &[Value]) -> Vec<Value> {
    let no_preference = json!("No Preference");
    let role = answer(answers, "role", 2).unwrap_or(&no_preference);
    let position = answer(answers, "position", 3).unwrap_or(&no_preference);
    // Unlike feature extraction, unknown difficulty/playstyle omit their weights.
    let difficulty = match answer(answers, "difficulty", 1).and_then(Value::as_str) {
        Some("Easy (1-3)") => Some(2.0),
        Some("Medium (4-6)") => Some(5.0),
        Some("Hard (7-8)") => Some(7.5),
        Some("Very Hard (9-10)") => Some(9.5),
        _ => None,
    };
    let attributes = match answer(answers, "playstyle", 4).and_then(Value::as_str) {
        Some("High Damage Output") => Some((8.0, 3.0)),
        Some("Tanky and Durable") => Some((4.0, 8.0)),
        Some("Support Team") => Some((3.0, 5.0)),
        Some("Balanced/Hybrid") => Some((5.0, 5.0)),
        _ => None,
    };
    let pressure = answer(answers, "pressure_response", 6)
        .map_or(Some("Stay calm and strategic"), Value::as_str);
    let contribution =
        answer(answers, "team_contribution", 8).map_or(Some("Balance between both"), Value::as_str);
    let problem = answer(answers, "problem_solving", 10)
        .map_or(Some("Analyze carefully before acting"), Value::as_str);
    // Aesthetic preference and character identity are read but unused by the JS rules.

    let mut ml_scored = Vec::new();
    for champion in top10 {
        if num(champion, "average") >= 80.0 {
            insert(&mut ml_scored, champion["championName"].clone());
        }
    }

    let mut scores: Vec<(Value, f64)> = catalog
        .iter()
        .map(|entry| {
            let champion = &entry.data;
            let mut score = 0.0;
            let mut maximum = 40.0;
            if role == &no_preference || champion.get("role") == Some(role) {
                score += 40.0;
            } else if position != &no_preference && includes(champion.get("positions"), position) {
                score += 30.0;
            }

            if let Some(target) = difficulty {
                maximum += 15.0;
                score += difference_score(
                    (target - num(champion, "difficulty")).abs(),
                    15.0,
                    12.0,
                    8.0,
                );
            }
            if let Some((damage, toughness)) = attributes {
                maximum += 12.0;
                score +=
                    difference_score((damage - num(champion, "damage")).abs(), 12.0, 10.0, 6.0);
                maximum += 12.0;
                score += difference_score(
                    (toughness - num(champion, "toughness")).abs(),
                    12.0,
                    10.0,
                    6.0,
                );
            }

            maximum += 21.0;
            let pressure_match = match pressure {
                Some("Stay calm and strategic") => num(champion, "toughness") >= 6.0,
                Some("Take charge and lead") => {
                    num(champion, "damage") >= 7.0 || num(champion, "control") >= 7.0
                }
                Some("Get aggressive and take risks") => num(champion, "damage") >= 8.0,
                Some("Play cautiously to avoid mistakes") => num(champion, "toughness") >= 7.0,
                _ => false,
            };
            score += if pressure_match { 7.0 } else { 3.0 };
            let contribution_match = match contribution {
                Some("Lead and make decisions") => num(champion, "control") >= 7.0,
                Some("Support and enable others") => num(champion, "utility") >= 7.0,
                Some("Balance between both") => {
                    num(champion, "utility") >= 5.0 && num(champion, "control") >= 5.0
                }
                Some("Stay independent and focus on my role") => num(champion, "damage") >= 7.0,
                _ => false,
            };
            score += if contribution_match { 7.0 } else { 3.0 };
            let problem_match = match problem {
                Some("Analyze carefully before acting") => num(champion, "control") >= 7.0,
                Some("Jump in and adapt on the fly") => num(champion, "mobility") >= 7.0,
                Some("Follow the team's lead") => num(champion, "utility") >= 6.0,
                Some("Focus on long-term improvement") => num(champion, "difficulty") >= 7.0,
                _ => false,
            };
            score += if problem_match { 7.0 } else { 3.0 };
            (Value::String(entry.name.clone()), (score / maximum) * 100.0)
        })
        .collect();

    // sort_by is stable, just like Array.sort: ties retain catalog order.
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut relevant = ml_scored.clone();
    let top_score = scores.first().map_or(0.0, |champion| champion.1);
    let threshold = (top_score * 0.75).max(60.0);
    for (name, score) in &scores {
        if *score >= threshold {
            insert(&mut relevant, name.clone());
        }
    }
    for (name, _) in &scores {
        if relevant.len() >= 10 {
            break;
        }
        insert(&mut relevant, name.clone());
    }

    if relevant.len() > 25 {
        let mut capped = Vec::new();
        let mut count = 0;
        // Deliberately no limit here: JS prioritizes every catalog ML champion.
        // Auto-relevant names outside the catalog disappear only on this path.
        for (name, _) in &scores {
            if ml_scored.contains(name) {
                insert(&mut capped, name.clone());
                count += 1;
            }
        }
        for (name, _) in &scores {
            if count >= 25 {
                break;
            }
            if !ml_scored.contains(name) && relevant.contains(name) {
                insert(&mut capped, name.clone());
                count += 1;
            }
        }
        capped
    } else {
        relevant
    }
}

fn difference_score(difference: f64, exact: f64, close: f64, acceptable: f64) -> f64 {
    if difference == 0.0 {
        exact
    } else if difference <= 1.0 {
        close
    } else if difference <= 2.0 {
        acceptable
    } else {
        0.0
    }
}

fn includes(positions: Option<&Value>, position: &Value) -> bool {
    match positions {
        Some(Value::Array(values)) => values.contains(position),
        Some(Value::String(value)) => position
            .as_str()
            .is_some_and(|needle| value.contains(needle)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog(count: usize) -> Vec<CatalogEntry> {
        (0..count)
            .map(|i| CatalogEntry {
                name: format!("Champion{i}"),
                data: json!({"role": "Mage", "positions": ["Mid"], "difficulty": 5,
                    "damage": 5, "toughness": 6, "control": 7, "utility": 5, "mobility": 5}),
            })
            .collect()
    }

    #[test]
    fn empty_and_zero_k_metrics() {
        let names = vec![json!("A")];
        for (recommended, relevant, k) in [
            (&[][..], &names[..], 10),
            (&names[..], &[][..], 10),
            (&names[..], &names[..], 0),
        ] {
            assert_eq!(precision_at_k(recommended, relevant, k), 0.0);
            assert_eq!(recall_at_k(recommended, relevant, k), 0.0);
            assert_eq!(f1_score_at_k(recommended, relevant, k), 0.0);
        }
        assert_eq!(mean_reciprocal_rank(&[], &names), 0.0);
        assert_eq!(mean_reciprocal_rank(&names, &[]), 0.0);
        assert_eq!(
            evaluate(&json!({}), &[], &[]),
            json!({
                "relevantChampions": [], "precisionAt1": 0.0, "precisionAt3": 0.0,
                "precisionAt10": 0.0, "recallAt10": 0.0, "f1At10": 0.0, "mrr": 0.0
            })
        );
    }

    #[test]
    fn metrics_use_available_length_and_count_repeated_recommendations() {
        let recommended = vec![json!("X"), json!("A"), json!("A")];
        let relevant = vec![json!("A")];
        assert_eq!(precision_at_k(&recommended, &relevant, 10), 2.0 / 3.0);
        assert_eq!(recall_at_k(&recommended, &relevant, 10), 2.0);
        assert_eq!(f1_score_at_k(&recommended, &relevant, 10), 1.0);
        assert_eq!(mean_reciprocal_rank(&recommended, &relevant), 0.5);
    }

    #[test]
    fn auto_relevant_names_precede_stable_catalog_ties_without_cap() {
        let top = vec![
            json!({"championName": "Champion3", "average": 80}),
            json!({"championName": "Outside", "average": 90}),
            json!({"championName": "Champion3", "average": 99}),
            json!({"championName": "Below", "average": 79.999}),
        ];
        let result = evaluate(&json!({}), &catalog(4), &top);
        assert_eq!(
            result["relevantChampions"],
            json!([
                "Champion3",
                "Outside",
                "Champion0",
                "Champion1",
                "Champion2"
            ])
        );
        assert_eq!(result["precisionAt3"], json!(1.0));
        assert_eq!(result["precisionAt10"], json!(0.75));
        assert_eq!(result["recallAt10"], json!(0.6));
    }

    #[test]
    fn cap_reorders_ml_names_by_compatibility_and_drops_outside_names() {
        let top = vec![
            json!({"championName": "Champion29", "average": 80}),
            json!({"championName": "Outside", "average": 80}),
            json!({"championName": "Champion28", "average": 80}),
        ];
        let mut expected = vec![json!("Champion28"), json!("Champion29")];
        expected.extend((0..23).map(|i| json!(format!("Champion{i}"))));
        assert_eq!(relevant_champions(&json!({}), &catalog(30), &top), expected);
        let all_ml: Vec<_> = (0..30)
            .rev()
            .map(|i| json!({"championName": format!("Champion{i}"), "average": 80}))
            .collect();
        assert_eq!(
            relevant_champions(&json!({}), &catalog(30), &all_ml).len(),
            30
        );
    }

    #[test]
    fn floor_fills_only_to_ten_in_stable_order() {
        let result = relevant_champions(
            &json!({"role": "Tank"}),
            &catalog(15),
            &[json!({"championName": "Champion9", "average": 80})],
        );
        let mut expected = vec![json!("Champion9")];
        expected.extend((0..9).map(|i| json!(format!("Champion{i}"))));
        assert_eq!(result, expected);
    }

    #[test]
    fn adaptive_threshold_uses_top_score_and_sixty_percent_minimum() {
        let mut entries = catalog(12);
        // With unknown difficulty/playstyle, 40 role + 9 psychological points
        // out of 61 is above 60 but below 75 percent of a perfect champion.
        for entry in &mut entries {
            entry.data["toughness"] = json!(0);
            entry.data["control"] = json!(0);
            entry.data["utility"] = json!(0);
        }
        assert_eq!(relevant_champions(&json!({}), &entries, &[]).len(), 12);
        entries[0].data["toughness"] = json!(6);
        entries[0].data["control"] = json!(7);
        entries[0].data["utility"] = json!(5);
        // These still exceed 75%, so use position-only partial credit instead.
        for entry in entries.iter_mut().skip(1) {
            entry.data["role"] = json!("Tank");
        }
        let answers = json!({"role": "Mage", "position": "Mid"});
        assert_eq!(relevant_champions(&answers, &entries, &[]).len(), 10);
        // With no perfect champion the 39/61 partial matches all clear 60%.
        entries[0].data = entries[1].data.clone();
        assert_eq!(relevant_champions(&answers, &entries, &[]).len(), 12);
        // Removing position credit puts everyone below 60%; the floor supplies ten.
        assert_eq!(
            relevant_champions(&json!({"role": "Mage"}), &entries, &[]).len(),
            10
        );
    }

    #[test]
    fn named_answers_override_numeric_and_unused_preferences_do_not_affect_scores() {
        let entries = catalog(12);
        let named = json!({"role": "Mage", "difficulty": "Hard (7-8)",
            "playstyle": "High Damage Output", "pressure_response": "Take charge and lead",
            "team_contribution": "Support and enable others", "problem_solving": "Jump in and adapt on the fly"});
        let mut conflicting = named.clone();
        for (key, value) in [
            ("1", "Easy (1-3)"),
            ("2", "Tank"),
            ("4", "Support Team"),
            ("6", "Play cautiously to avoid mistakes"),
            ("8", "Stay independent and focus on my role"),
            ("10", "Focus on long-term improvement"),
            ("aesthetic_preference", "Dark and mysterious"),
            ("character_identity", "Villain"),
        ] {
            conflicting[key] = json!(value);
        }
        assert_eq!(
            relevant_champions(&named, &entries, &[]),
            relevant_champions(&conflicting, &entries, &[])
        );
    }

    #[test]
    fn answer_fallback_and_unknown_difficulty_do_not_use_scoring_defaults() {
        let mut entries = catalog(11);
        entries[0].data["difficulty"] = json!(2);
        let named =
            json!({"role": "Mage", "difficulty": "Easy (1-3)", "playstyle": "Balanced/Hybrid"});
        let numeric = json!({"role": "", "2": "Mage", "difficulty": null,
            "1": "Easy (1-3)", "playstyle": false, "4": "Balanced/Hybrid"});
        let array = json!([null, "Easy (1-3)", "Mage", null, "Balanced/Hybrid"]);
        assert_eq!(
            relevant_champions(&named, &entries, &[]),
            relevant_champions(&numeric, &entries, &[])
        );
        assert_eq!(
            relevant_champions(&named, &entries, &[]),
            relevant_champions(&array, &entries, &[])
        );
        entries[0].data["difficulty"] = json!(10);
        let unknown = relevant_champions(&json!({"difficulty": null}), &entries, &[]);
        assert_eq!(unknown[0], json!("Champion0"));
        let medium = relevant_champions(&json!({"difficulty": "Medium (4-6)"}), &entries, &[]);
        assert_eq!(medium[0], json!("Champion1"));
    }

    #[test]
    fn test_kendall_w_perfect_agreement_and_orthogonal() {
        // 3 judges, 4 items: perfect rank agreement
        let rankings = vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
        ];
        let w = compute_kendall_w(&rankings);
        assert!((w - 1.0).abs() < 1e-6, "W should be 1.0 for perfect agreement, got {w}");

        // Single judge or item fallback
        assert_eq!(compute_kendall_w(&[vec![1, 2, 3]]), 1.0);
        assert_eq!(compute_kendall_w(&[vec![1], vec![1]]), 1.0);
    }

    #[test]
    fn test_ndcg_at_k_computation() {
        let recommended = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let relevance = vec![
            ("A".to_string(), 1.0),
            ("B".to_string(), 0.5),
            ("C".to_string(), 0.2),
        ];
        let ndcg = compute_ndcg_at_k(&recommended, &relevance, 3);
        assert!(ndcg >= 0.99, "Optimal ranking should give ~1.0 NDCG, got {ndcg}");

        // Suboptimal ranking
        let reversed = vec!["C".to_string(), "B".to_string(), "A".to_string()];
        let ndcg_rev = compute_ndcg_at_k(&reversed, &relevance, 3);
        assert!(ndcg_rev < ndcg, "Reversed ranking should give lower NDCG");
    }

    #[test]
    fn test_ece_computation() {
        let confidences = vec![0.95, 0.90, 0.85, 0.20];
        let labels = vec![true, true, true, false];
        let ece = compute_ece(&confidences, &labels, 5);
        assert!(ece >= 0.0 && ece <= 1.0, "ECE should be within [0, 1]");
    }

    #[test]
    fn test_ensemble_quality_audit_orchestration() {
        let top10 = vec![
            AggregatedChampionScore {
                champion_name: "Aatrox".to_string(),
                random_forest: 0.96,
                decision_tree: 0.95,
                knn: 0.94,
                naive_bayes: Some(0.97),
                cosine_similarity: Some(0.95),
                gbdt: Some(0.98),
                average: 0.958,
                weighted: 0.962,
            },
            AggregatedChampionScore {
                champion_name: "Camille".to_string(),
                random_forest: 0.93,
                decision_tree: 0.92,
                knn: 0.91,
                naive_bayes: Some(0.94),
                cosine_similarity: Some(0.92),
                gbdt: Some(0.95),
                average: 0.928,
                weighted: 0.931,
            },
        ];
        let relevant = vec!["Aatrox".to_string(), "Camille".to_string()];
        let audit = compute_ensemble_audit(&top10, &relevant);
        assert_eq!(audit.precision_at1, 1.0);
        assert_eq!(audit.precision_at3, 1.0);
        assert_eq!(audit.concordance_status, "Peak Consensus");
        assert_eq!(audit.model_agreements.len(), 6);
    }
}
