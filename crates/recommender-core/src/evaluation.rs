use crate::compat::{num, truthy};
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
}
