use crate::compat::{num, text};
use serde_json::{json, Value};

/// The per-champion result from SimpleRandomForest.predictAll, using an existing forest.
pub fn score(features: &Value, champion: &Value, forest: &[crate::Tree]) -> Value {
    // predictAll evaluates every champion in every tree, not just bootstrap members.
    let votes: Vec<f64> = forest
        .iter()
        .map(|tree| calculate_tree_score(features, champion, tree))
        .collect();
    let mean = votes.iter().fold(0.0, |sum, vote| sum + vote) / votes.len() as f64;
    let variance = votes
        .iter()
        .fold(0.0, |sum, vote| sum + (vote - mean).powi(2))
        / votes.len() as f64;

    json!({
        "score": mean,
        "rawScore": mean,
        "uncertainty": variance.sqrt(),
        "details": {
            "contributingFactors": [{
                "factor": "Ensemble Average", "weight": 100, "contribution": mean
            }],
            "matchedCriteria": [format!("{} trees voted", forest.len())],
            "penalties": []
        }
    })
}

fn calculate_tree_score(features: &Value, champion: &Value, tree: &crate::Tree) -> f64 {
    let includes = |feature: &str| tree.feature_indices.iter().any(|name| name == feature);
    let mut score = 0.0;
    let mut max_possible_score = 0.0;

    // Preserve the JavaScript branch order, irrespective of subset ordering.
    if includes("role") {
        max_possible_score += 40.0;
        if text(features, "role") == "No Preference" || features.get("role") == champion.get("role")
        {
            score += 40.0;
        }
    }
    for (feature, maximum, multiplier) in [
        ("difficulty", 20.0, 2.0),
        ("damage", 15.0, 1.5),
        ("toughness", 15.0, 1.5),
    ] {
        if includes(feature) {
            max_possible_score += maximum;
            let distance = (num(features, feature) - num(champion, feature)).abs();
            let contribution = maximum - distance * multiplier;
            // Rust's f64::max discards NaN, unlike JavaScript's Math.max.
            score += if contribution.is_nan() {
                contribution
            } else {
                contribution.max(0.0)
            };
        }
    }
    for feature in ["mobility", "control", "utility"] {
        if includes(feature) {
            max_possible_score += 10.0;
            score += (num(champion, feature) / 10.0) * 10.0;
        }
    }

    if max_possible_score > 0.0 {
        (score / max_possible_score) * 100.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree(features: &[&str]) -> crate::Tree {
        crate::Tree {
            id: 42,
            feature_indices: features.iter().map(|name| (*name).to_owned()).collect(),
            bootstrap_champion_names: vec!["Not the scored champion".to_owned()],
        }
    }

    #[test]
    fn subsets_are_membership_tests_and_bootstrap_does_not_filter_scores() {
        let features = json!({"role": "Mage", "difficulty": 5, "damage": 8, "toughness": 3});
        let champion = json!({"role": "Mage", "herotype": "Tank", "difficulty": 7,
            "damage": 10, "toughness": 1, "mobility": 12, "control": 3, "utility": 5});
        let subset = tree(&[
            "utility",
            "mobility",
            "role",
            "difficulty",
            "damage",
            "toughness",
            "control",
        ]);
        let expected = (40.0 + 16.0 + 12.0 + 12.0 + 12.0 + 3.0 + 5.0) / 120.0 * 100.0;
        assert_eq!(
            calculate_tree_score(&features, &champion, &subset),
            expected
        );
        assert_eq!(
            calculate_tree_score(&features, &champion, &tree(&["role", "role"])),
            100.0
        );
        assert_eq!(
            calculate_tree_score(&features, &champion, &tree(&["unknown"])),
            0.0
        );
        assert_eq!(
            calculate_tree_score(&json!({"role": "Tank"}), &champion, &tree(&["role"])),
            0.0
        );
        assert_eq!(
            calculate_tree_score(
                &json!({"role": "No Preference"}),
                &champion,
                &tree(&["role"])
            ),
            100.0
        );
    }

    #[test]
    fn ensemble_uses_population_standard_deviation_and_predict_all_details() {
        let result = score(
            &json!({"role": "Mage"}),
            &json!({"role": "Mage"}),
            &[tree(&["role"]), tree(&[])],
        );
        assert_eq!(
            result,
            json!({
                "score": 50.0, "rawScore": 50.0, "uncertainty": 50.0,
                "details": {
                    "contributingFactors": [{"factor": "Ensemble Average", "weight": 100, "contribution": 50.0}],
                    "matchedCriteria": ["2 trees voted"], "penalties": []
                }
            })
        );
    }

    #[test]
    fn similarity_floors_at_zero_without_clamping_champion_attributes() {
        let result = calculate_tree_score(
            &json!({"difficulty": 100, "damage": 100, "toughness": 100}),
            &json!({"difficulty": 1, "damage": 1, "toughness": 1}),
            &tree(&["difficulty", "damage", "toughness"]),
        );
        assert_eq!(result, 0.0);
        assert_eq!(
            calculate_tree_score(&json!({}), &json!({"mobility": 12}), &tree(&["mobility"])),
            120.0
        );
    }

    #[test]
    fn migration_fixture_scores_and_details_match() {
        let catalog: Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/migration/catalog.json"
        ))
        .unwrap();
        let forest: Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/migration/forest.json"
        ))
        .unwrap();
        let forest: Vec<crate::Tree> = forest
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| crate::Tree {
                id: entry["id"].as_u64().unwrap() as usize,
                feature_indices: entry["featureIndices"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap().to_owned())
                    .collect(),
                bootstrap_champion_names: entry["bootstrapChampionNames"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap().to_owned())
                    .collect(),
            })
            .collect();
        for source in [
            include_str!("../../../tests/fixtures/migration/profile-mage-mid.json"),
            include_str!("../../../tests/fixtures/migration/profile-marksman-bot.json"),
            include_str!("../../../tests/fixtures/migration/profile-tank-top.json"),
            include_str!("../../../tests/fixtures/migration/profile-support-team.json"),
            include_str!("../../../tests/fixtures/migration/profile-empty-defaults.json"),
            include_str!("../../../tests/fixtures/migration/profile-unknown-values.json"),
            include_str!("../../../tests/fixtures/migration/profile-no-preference-adc.json"),
            include_str!("../../../tests/fixtures/migration/profile-legacy-psychological-10.json"),
        ] {
            let profile: Value = serde_json::from_str(source).unwrap();
            for entry in catalog.as_array().unwrap() {
                let name = entry["name"].as_str().unwrap();
                let actual = score(&profile["features"], &entry["data"], &forest);
                let expected = &profile["scores"]["randomForest"][name];
                for key in ["score", "rawScore", "uncertainty"] {
                    assert!(
                        (actual[key].as_f64().unwrap() - expected[key].as_f64().unwrap()).abs()
                            < 1e-9,
                        "{} / {name} / {key}",
                        profile["id"]
                    );
                }
                let mut details = actual["details"].clone();
                details["contributingFactors"][0]["contribution"] = expected["score"].clone();
                assert_eq!(details, expected["details"], "{} / {name}", profile["id"]);
            }
        }
    }
}
