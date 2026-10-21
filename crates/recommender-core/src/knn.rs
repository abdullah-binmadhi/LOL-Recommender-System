use crate::compat::{fixed, js_string, num, text, truthy};
use serde_json::{json, Value};

/// The per-champion result from SimpleKNN.calculateChampionScore.
pub fn score(features: &Value, champion: &Value) -> Value {
    let distance = calculate_distance(features, champion);
    let mut matched = Vec::new();
    let mut penalties = Vec::new();

    if role_matches(features, champion) {
        matched.push(format!(
            "Role/Herotype: {}/{}",
            property_string(champion, "role"),
            property_string(champion, "herotype")
        ));
    } else {
        penalties.push(format!(
            "Role mismatch: wanted {}, got {}",
            property_string(features, "role"),
            property_string(champion, "role")
        ));
    }

    for (feature, label) in [("burst", "Burst"), ("sustain", "Sustain"), ("cc", "CC")] {
        let derived = &champion["derived_stats"];
        let difference =
            (number_or(features, feature, 0.0) - number_or(derived, feature, 0.0)).abs();
        if difference < 20.0 {
            // Optional chaining short-circuits the entire toFixed call when stats are absent.
            let value = if derived.is_null() {
                "undefined".to_owned()
            } else {
                fixed(num(derived, feature), 0)
            };
            matched.push(format!("{label} alignment: {value}%"));
        }
    }

    if position_matches(features, champion) {
        let position = if truthy(&features["position"]) {
            property_string(features, "position")
        } else {
            "Any".to_owned()
        };
        matched.push(format!("Position: {position}"));
    } else if text(features, "position") != "No Preference" {
        penalties.push(format!(
            "Position mismatch: wanted {}",
            property_string(features, "position")
        ));
    }

    json!({
        "score": normalize_score(distance),
        "rawScore": distance,
        "details": {
            "contributingFactors": [{
                "factor": "Weighted Euclidean Distance",
                "weight": "N/A",
                "contribution": fixed(distance, 3)
            }],
            "matchedCriteria": matched,
            "penalties": penalties
        }
    })
}

fn number_or(value: &Value, key: &str, fallback: f64) -> f64 {
    if truthy(&value[key]) {
        num(value, key)
    } else {
        fallback
    }
}

fn property_string(value: &Value, key: &str) -> String {
    value
        .get(key)
        .map(js_string)
        .unwrap_or_else(|| "undefined".to_owned())
}

fn role_matches(features: &Value, champion: &Value) -> bool {
    text(features, "role") == "No Preference"
        || features.get("role") == champion.get("role")
        || features.get("role") == champion.get("herotype")
}

fn position_matches(features: &Value, champion: &Value) -> bool {
    text(features, "position") == "No Preference"
        || match champion.get("positions") {
            Some(Value::Array(positions)) => positions
                .iter()
                .any(|position| Some(position) == features.get("position")),
            // String.includes also works in the source when positions is a string.
            Some(Value::String(positions)) => {
                !positions.is_empty() && positions.contains(&property_string(features, "position"))
            }
            _ => false,
        }
}

fn calculate_distance(features: &Value, champion: &Value) -> f64 {
    let mut sum_squares = 0.0;
    for (feature, weight) in [
        ("difficulty", 1.0),
        ("damage", 1.2),
        ("toughness", 1.2),
        ("mobility", 1.0),
        ("control", 1.0),
        ("utility", 1.0),
    ] {
        let user = (number_or(features, feature, 5.0) - 1.0) / 9.0;
        let champion = (number_or(champion, feature, 5.0) - 1.0) / 9.0;
        let diff = user - champion;
        sum_squares += weight * diff * diff;
    }
    for feature in ["burst", "sustain", "cc"] {
        let user = number_or(features, feature, 0.0) / 100.0;
        let champion = number_or(&champion["derived_stats"], feature, 0.0) / 100.0;
        let diff = user - champion;
        sum_squares += 0.8 * diff * diff;
    }
    let role_match = if role_matches(features, champion) {
        0.0
    } else {
        1.0
    };
    sum_squares += 2.5 * role_match * role_match;
    let position_match = if position_matches(features, champion) {
        0.0
    } else {
        1.0
    };
    sum_squares += 1.5 * position_match * position_match;
    sum_squares.sqrt()
}

fn normalize_score(distance: f64) -> f64 {
    let normalized = 100.0 - ((distance / 3.5) * 100.0);
    normalized.clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn herotype_and_derived_matches_preserve_detail_order_and_format() {
        let result = score(
            &json!({"role": "Mage", "position": "Mid", "burst": 60.5, "sustain": 35, "cc": 80}),
            &json!({"role": "Support", "herotype": "Mage", "positions": ["Mid"],
                "derived_stats": {"burst": 60.5, "sustain": 35, "cc": 80}}),
        );
        assert_eq!(
            result,
            json!({
                "score": 100.0, "rawScore": 0.0,
                "details": {
                    "contributingFactors": [{"factor": "Weighted Euclidean Distance", "weight": "N/A", "contribution": "0.000"}],
                    "matchedCriteria": ["Role/Herotype: Support/Mage", "Burst alignment: 61%", "Sustain alignment: 35%", "CC alignment: 80%", "Position: Mid"],
                    "penalties": []
                }
            })
        );
    }

    #[test]
    fn zero_and_other_falsy_numeric_values_default_to_five() {
        let features = json!({"role": "No Preference", "position": "No Preference",
            "difficulty": 0, "damage": false, "toughness": "", "mobility": null});
        let champion = json!({"difficulty": 5, "damage": 5, "toughness": 5,
            "mobility": 5, "control": 5, "utility": 5});
        assert_eq!(calculate_distance(&features, &champion), 0.0);
        assert_eq!(
            score(&features, &champion)["details"]["matchedCriteria"],
            json!([
                "Role/Herotype: undefined/undefined",
                "Burst alignment: undefined%",
                "Sustain alignment: undefined%",
                "CC alignment: undefined%",
                "Position: No Preference"
            ])
        );
    }

    #[test]
    fn mismatches_are_ordered_and_bot_is_not_an_adc_alias() {
        let result = score(
            &json!({"role": "Mage", "position": "Bot", "burst": 20, "sustain": 20, "cc": 20}),
            &json!({"role": "Marksman", "positions": ["ADC"], "derived_stats": {"burst": 0, "sustain": 0, "cc": 0}}),
        );
        assert_eq!(result["details"]["matchedCriteria"], json!([]));
        assert_eq!(
            result["details"]["penalties"],
            json!([
                "Role mismatch: wanted Mage, got Marksman",
                "Position mismatch: wanted Bot"
            ])
        );
        assert_eq!(
            score(&json!({}), &json!({"role": "Mage"}))["details"]["penalties"],
            json!(["Position mismatch: wanted undefined"])
        );
    }

    #[test]
    fn normalization_clamps_scores_but_not_distance_inputs() {
        assert_eq!(normalize_score(0.0), 100.0);
        assert_eq!(normalize_score(1.75), 50.0);
        assert_eq!(normalize_score(3.5), 0.0);
        assert_eq!(normalize_score(10.0), 0.0);
        assert_eq!(normalize_score(-1.0), 100.0);
        let result = score(
            &json!({"difficulty": 100, "role": "No Preference", "position": "No Preference"}),
            &json!({}),
        );
        assert!(result["rawScore"].as_f64().unwrap() > 3.5);
        assert_eq!(result["score"], json!(0.0));
    }

    #[test]
    fn migration_fixture_scores_and_details_match() {
        let catalog: Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/migration/catalog.json"
        ))
        .unwrap();
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
                let actual = score(&profile["features"], &entry["data"]);
                let expected = &profile["scores"]["knn"][name];
                for key in ["score", "rawScore"] {
                    assert!(
                        (actual[key].as_f64().unwrap() - expected[key].as_f64().unwrap()).abs()
                            < 1e-9,
                        "{} / {name} / {key}",
                        profile["id"]
                    );
                }
                assert_eq!(
                    actual["details"], expected["details"],
                    "{} / {name}",
                    profile["id"]
                );
            }
        }
    }
}
