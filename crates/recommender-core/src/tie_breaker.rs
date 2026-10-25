use crate::{
    aggregation::ModelWeights,
    contracts::AggregatedChampionScore,
    CatalogEntry,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DilemmaOption {
    pub label: String,
    pub description: String,
    pub favored_champion: String,
    pub attribute_focus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ScenarioDilemma {
    pub id: u32,
    pub phase: String,
    pub scenario: String,
    pub option_a: DilemmaOption,
    pub option_b: DilemmaOption,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TieBreakerSession {
    pub contender_a: String,
    pub contender_b: String,
    pub score_a: f64,
    pub score_b: f64,
    pub models_preferring_a: Vec<String>,
    pub models_preferring_b: Vec<String>,
    pub dilemmas: Vec<ScenarioDilemma>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TieBreakerResolution {
    pub winning_champion: String,
    pub consensus_confidence: f64,
    pub recommended_weights: ModelWeights,
}

impl TieBreakerSession {
    /// Generates an adaptive 3-dilemma scenario tie-breaker session comparing the two top contenders.
    pub fn generate(
        top_candidates: &[AggregatedChampionScore],
        catalog: &[CatalogEntry],
    ) -> Option<Self> {
        if top_candidates.len() < 2 {
            return None;
        }

        let a = &top_candidates[0];
        let b = &top_candidates[1];

        // Identify which models prefer champion A vs champion B
        let mut models_a = Vec::new();
        let mut models_b = Vec::new();

        let check_model = |name: &str, score_a: f64, score_b: f64, m_a: &mut Vec<String>, m_b: &mut Vec<String>| {
            if score_a >= score_b {
                m_a.push(name.to_string());
            } else {
                m_b.push(name.to_string());
            }
        };

        check_model("Random Forest", a.random_forest, b.random_forest, &mut models_a, &mut models_b);
        check_model("Decision Tree", a.decision_tree, b.decision_tree, &mut models_a, &mut models_b);
        check_model("KNN", a.knn, b.knn, &mut models_a, &mut models_b);
        if let (Some(s_a), Some(s_b)) = (a.naive_bayes, b.naive_bayes) {
            check_model("Naive Bayes", s_a, s_b, &mut models_a, &mut models_b);
        }
        if let (Some(s_a), Some(s_b)) = (a.cosine_similarity, b.cosine_similarity) {
            check_model("Cosine Similarity", s_a, s_b, &mut models_a, &mut models_b);
        }
        if let (Some(s_a), Some(s_b)) = (a.gbdt, b.gbdt) {
            check_model("GBDT", s_a, s_b, &mut models_a, &mut models_b);
        }

        // Extract attribute deltas between champion A and champion B from catalog
        let cat_a = catalog.iter().find(|c| c.name == a.champion_name);
        let cat_b = catalog.iter().find(|c| c.name == b.champion_name);

        let get_attr = |cat: Option<&CatalogEntry>, key: &str| -> f64 {
            cat.and_then(|c| {
                c.data
                    .get("attributes")
                    .and_then(|attr| attr.get(key))
                    .and_then(Value::as_f64)
                    .or_else(|| {
                        c.data
                            .get("derived_stats")
                            .and_then(|ds| ds.get(key))
                            .and_then(Value::as_f64)
                    })
                    .or_else(|| c.data.get(key).and_then(Value::as_f64))
            })
            .unwrap_or(2.0)
        };

        let attributes = [
            ("mobility", "High Mobility & Flanking", "Positional Stability & Anchoring"),
            ("damage", "High Burst Damage", "Sustained Control & Durability"),
            ("toughness", "Frontline Durability & Peeling", "Agile Skirmishing & Outplay"),
            ("control", "Area Crowd Control & Lockdown", "Solo Target Duelist Pressure"),
            ("utility", "Teamfight Utility & Buffs", "Raw Execution & Slaying"),
        ];

        let mut deltas: Vec<(&str, &str, &str, f64, bool)> = attributes
            .iter()
            .map(|(key, label_high, label_low)| {
                let val_a = get_attr(cat_a, key);
                let val_b = get_attr(cat_b, key);
                let diff = (val_a - val_b).abs();
                let a_is_higher = val_a >= val_b;
                (*key, *label_high, *label_low, diff, a_is_higher)
            })
            .collect();

        // Sort by largest attribute delta
        deltas.sort_by(|x, y| y.3.total_cmp(&x.3));

        let phases = [
            (
                "Early Laning (0-10m)",
                format!(
                    "Level 3 Wave Crash: The enemy laner has frozen the minion wave near their tower. Between {} and {}, how do you resolve this pressure point?",
                    a.champion_name, b.champion_name
                ),
            ),
            (
                "Mid-Game Objective (15-25m)",
                format!(
                    "Contested Dragon Fight: Enemy team has vision control in river. As {} and {} offer different playstyles, where is your focus?",
                    a.champion_name, b.champion_name
                ),
            ),
            (
                "Late-Game Deciding Standoff (30m+)",
                format!(
                    "Baron & Inhibitor Standoff: Both teams are at 4 items. The game rests on a single tactical execution. Which win condition fits your style: {}'s approach or {}'s approach?",
                    a.champion_name, b.champion_name
                ),
            ),
        ];

        let mut dilemmas = Vec::new();
        for (i, (phase, scenario)) in phases.into_iter().enumerate() {
            let delta = deltas.get(i).unwrap_or(&("tactical", "Aggressive Outplay", "Methodical Control", 1.0, true));
            let attr_key = delta.0;
            let a_higher = delta.4;

            let (label_a, desc_a, label_b, desc_b) = if a_higher {
                (
                    format!("{}: Aggressive Focus", a.champion_name),
                    format!("Leverage superior {} to dominate the interaction with high offensive pressure.", attr_key),
                    format!("{}: Calculated Focus", b.champion_name),
                    format!("Play with tactical composure, punishing enemy over-extensions with defensive consistency and controlled {}.", attr_key),
                )
            } else {
                (
                    format!("{}: Tactical Structure", a.champion_name),
                    format!("Maintain structure and capitalize on steady wave mechanics, zoning, and superior {}.", attr_key),
                    format!("{}: Explosive Outplay", b.champion_name),
                    format!("Exploit superior {} for high-velocity outplay and rapid picks.", attr_key),
                )
            };

            dilemmas.push(ScenarioDilemma {
                id: (i + 1) as u32,
                phase: phase.to_string(),
                scenario,
                option_a: DilemmaOption {
                    label: label_a,
                    description: desc_a,
                    favored_champion: a.champion_name.clone(),
                    attribute_focus: attr_key.to_string(),
                },
                option_b: DilemmaOption {
                    label: label_b,
                    description: desc_b,
                    favored_champion: b.champion_name.clone(),
                    attribute_focus: attr_key.to_string(),
                },
            });
        }

        Some(TieBreakerSession {
            contender_a: a.champion_name.clone(),
            contender_b: b.champion_name.clone(),
            score_a: a.average,
            score_b: b.average,
            models_preferring_a: models_a,
            models_preferring_b: models_b,
            dilemmas,
        })
    }

    /// Resolves user answers (vector of boolean: true for Option A, false for Option B) into auto-aligned ModelWeights.
    pub fn resolve(&self, user_choices: &[bool], current_weights: &ModelWeights) -> TieBreakerResolution {
        let votes_a = user_choices.iter().filter(|&&choice| choice).count();
        let votes_b = user_choices.len().saturating_sub(votes_a);

        let (winner, confidence, favored_models, disfavored_models) = if votes_a >= votes_b {
            let conf = votes_a as f64 / user_choices.len().max(1) as f64 * 100.0;
            (&self.contender_a, conf, &self.models_preferring_a, &self.models_preferring_b)
        } else {
            let conf = votes_b as f64 / user_choices.len().max(1) as f64 * 100.0;
            (&self.contender_b, conf, &self.models_preferring_b, &self.models_preferring_a)
        };

        // Adjust weights: boost models that favor the winner, trim models that disfavor
        let mut w = *current_weights;
        let boost = 0.08;
        let penalty = 0.04;

        let adjust_model = |name: &str, weight: &mut f64| {
            if favored_models.iter().any(|m| m.contains(name)) {
                *weight += boost;
            } else if disfavored_models.iter().any(|m| m.contains(name)) {
                *weight = (*weight - penalty).max(0.02);
            }
        };

        adjust_model("Random Forest", &mut w.random_forest);
        adjust_model("Decision Tree", &mut w.decision_tree);
        adjust_model("KNN", &mut w.knn);
        adjust_model("Naive Bayes", &mut w.naive_bayes);
        adjust_model("Cosine", &mut w.cosine_similarity);
        adjust_model("GBDT", &mut w.gbdt);

        TieBreakerResolution {
            winning_champion: winner.clone(),
            consensus_confidence: confidence,
            recommended_weights: w.normalized(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tie_breaker_session_generation_and_resolution() {
        let a = AggregatedChampionScore {
            champion_name: "Yasuo".into(),
            random_forest: 85.0,
            decision_tree: 60.0,
            knn: 80.0,
            naive_bayes: Some(60.0),
            cosine_similarity: Some(75.0),
            gbdt: Some(85.0),
            average: 74.0,
            weighted: 74.0,
        };
        let b = AggregatedChampionScore {
            champion_name: "Yone".into(),
            random_forest: 65.0,
            decision_tree: 88.0,
            knn: 70.0,
            naive_bayes: Some(85.0),
            cosine_similarity: Some(72.0),
            gbdt: Some(68.0),
            average: 74.5,
            weighted: 74.5,
        };

        let catalog = vec![
            CatalogEntry {
                name: "Yasuo".into(),
                data: json!({
                    "attributes": { "mobility": 3, "control": 1, "damage": 3, "toughness": 1 },
                }),
            },
            CatalogEntry {
                name: "Yone".into(),
                data: json!({
                    "attributes": { "mobility": 2, "control": 2, "damage": 3, "toughness": 2 },
                }),
            },
        ];

        let session = TieBreakerSession::generate(&[a, b], &catalog).expect("session generated");
        assert_eq!(session.dilemmas.len(), 3);
        assert!(session.models_preferring_a.contains(&"Random Forest".to_string()));
        assert!(session.models_preferring_b.contains(&"Decision Tree".to_string()));

        // Resolve choosing option A for all 3 dilemmas
        let resolution = session.resolve(&[true, true, true], &ModelWeights::default());
        assert_eq!(resolution.winning_champion, "Yasuo");
        assert_eq!(resolution.consensus_confidence, 100.0);
        // Yasuo favored Random Forest, so RF weight should be boosted
        assert!(resolution.recommended_weights.random_forest > ModelWeights::default().normalized().random_forest);
    }
}
