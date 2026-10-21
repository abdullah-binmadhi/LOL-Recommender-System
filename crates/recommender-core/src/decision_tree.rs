use crate::compat::{js_string, num, text, truthy};
use serde_json::{json, Value};

// Unlike `value || 0`, a missing operand in direct JS arithmetic becomes NaN.
fn numeric_operand(value: &Value, key: &str) -> f64 {
    if value.get(key).is_none() {
        f64::NAN
    } else {
        num(value, key)
    }
}

fn max_zero(value: f64) -> f64 {
    if value.is_nan() {
        value
    } else {
        value.max(0.0)
    }
}

fn property_string(value: &Value, key: &str) -> String {
    value
        .get(key)
        .map(js_string)
        .unwrap_or_else(|| "undefined".into())
}

fn defaulted_number(value: &Value, key: &str) -> f64 {
    if value.get(key).is_some_and(truthy) {
        num(value, key)
    } else {
        0.0
    }
}

/// Port of SimpleDecisionTree.calculateChampionScore, the per-champion operation
/// used by predictAll. Tree training does not affect this scoring path.
pub fn score(features: &Value, champion: &Value) -> Value {
    let mut raw_score = 0.0;
    let mut factors = Vec::new();
    let mut matched = Vec::new();
    let mut penalties = Vec::new();

    if features.get("role").and_then(Value::as_str) == Some("No Preference")
        || features.get("role") == champion.get("role")
    {
        raw_score += 40.0;
        factors.push(json!({"factor": "Role Match", "weight": 40, "contribution": 40}));
        matched.push(format!("Role: {}", property_string(champion, "role")));
    } else {
        raw_score -= 20.0;
        penalties.push(format!(
            "Role mismatch: wanted {}, got {}",
            property_string(features, "role"),
            property_string(champion, "role")
        ));
    }

    let position_matches = champion
        .get("positions")
        .and_then(Value::as_array)
        .is_some_and(|positions| {
            positions
                .iter()
                .any(|position| Some(position) == features.get("position"))
        });
    if text(features, "position") == "No Preference" || position_matches {
        raw_score += 30.0;
        factors.push(json!({"factor": "Position Match", "weight": 30, "contribution": 30}));
        let position = if features.get("position").is_some_and(truthy) {
            property_string(features, "position")
        } else {
            "Any".into()
        };
        matched.push(format!("Position: {position}"));
    } else if text(features, "position") != "No Preference" {
        raw_score -= 15.0;
        penalties.push(format!(
            "Position mismatch: wanted {}",
            property_string(features, "position")
        ));
    }

    for (key, factor, multiplier) in [
        ("difficulty", "Difficulty Similarity", 2.0),
        ("damage", "Damage Similarity", 1.5),
        ("toughness", "Toughness Similarity", 1.5),
    ] {
        let similarity = max_zero(
            15.0 - (numeric_operand(features, key) - numeric_operand(champion, key)).abs()
                * multiplier,
        );
        raw_score += similarity;
        factors.push(json!({"factor": factor, "weight": 15, "contribution": similarity}));
    }

    let derived_stats = champion.get("derived_stats").unwrap_or(&Value::Null);
    let burst_score = max_zero(
        10.0 - (defaulted_number(features, "burst") - defaulted_number(derived_stats, "burst"))
            .abs()
            / 5.0,
    );
    raw_score += burst_score;
    if burst_score > 7.0 {
        matched.push("Excellent Burst match".into());
    }

    let sustain_score = max_zero(
        10.0 - (defaulted_number(features, "sustain") - defaulted_number(derived_stats, "sustain"))
            .abs()
            / 5.0,
    );
    raw_score += sustain_score;
    if sustain_score > 7.0 {
        matched.push("Excellent Sustain match".into());
    }

    let toughness = numeric_operand(champion, "toughness");
    let damage = numeric_operand(champion, "damage");
    let control = numeric_operand(champion, "control");
    let utility = numeric_operand(champion, "utility");
    let mobility = numeric_operand(champion, "mobility");
    let role = text(champion, "role");
    let mut psych_score = 0;

    if text(features, "pressure_response") == "Stay calm and strategic" && toughness >= 6.0 {
        psych_score += 15;
    }
    if text(features, "pressure_response") == "Take charge and lead"
        && (damage >= 7.0 || control >= 7.0)
    {
        psych_score += 15;
    }
    if text(features, "pressure_response") == "Get aggressive and take risks" && damage >= 8.0 {
        psych_score += 15;
    }
    if text(features, "pressure_response") == "Play cautiously to avoid mistakes"
        && toughness >= 7.0
    {
        psych_score += 15;
    }

    if text(features, "aesthetic_preference") == "Heroic"
        && champion.get("title").is_some_and(truthy)
        && champion
            .get("title")
            .and_then(Value::as_str)
            .is_some_and(|title| title.contains("the"))
    {
        psych_score += 10;
    }
    if text(features, "aesthetic_preference") == "Mysterious"
        && (role == "Assassin" || role == "Mage")
    {
        psych_score += 10;
    }
    if text(features, "aesthetic_preference") == "Dark and edgy" && role == "Assassin" {
        psych_score += 10;
    }
    if text(features, "aesthetic_preference") == "Cute or playful" && role == "Support" {
        psych_score += 10;
    }
    if text(features, "aesthetic_preference") == "Monstrous or non-human"
        && (role == "Fighter" || role == "Tank")
    {
        psych_score += 10;
    }

    if text(features, "team_contribution") == "Lead and make decisions" && control >= 7.0 {
        psych_score += 15;
    }
    if text(features, "team_contribution") == "Support and enable others" && utility >= 7.0 {
        psych_score += 15;
    }
    if text(features, "team_contribution") == "Balance between both"
        && utility >= 5.0
        && control >= 5.0
    {
        psych_score += 15;
    }
    if text(features, "team_contribution") == "Stay independent and focus on my role"
        && damage >= 7.0
    {
        psych_score += 15;
    }

    if text(features, "problem_solving") == "Analyze carefully before acting" && control >= 7.0 {
        psych_score += 10;
    }
    if text(features, "problem_solving") == "Jump in and adapt on the fly" && mobility >= 7.0 {
        psych_score += 10;
    }
    if text(features, "problem_solving") == "Follow the team's lead" && utility >= 6.0 {
        psych_score += 10;
    }
    if text(features, "problem_solving") == "Focus on long-term improvement" && toughness >= 7.0 {
        psych_score += 10;
    }

    raw_score += f64::from(psych_score);
    factors
        .push(json!({"factor": "Psychological Match", "weight": 75, "contribution": psych_score}));
    if psych_score > 0 {
        matched.push(format!(
            "Psychological preferences matched ({psych_score} points)"
        ));
    }

    let normalized = ((raw_score - -35.0) / (210.0 - -35.0)) * 100.0;
    // Keep NaN for serde_json's null serialization, just like JSON.stringify.
    let normalized = if normalized.is_nan() {
        normalized
    } else {
        normalized.clamp(0.0, 100.0)
    };
    json!({
        "score": normalized,
        "rawScore": raw_score,
        "details": {
            "contributingFactors": factors,
            "matchedCriteria": matched,
            "penalties": penalties
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn champion() -> Value {
        json!({
            "role": "Mage", "positions": ["Mid"], "title": "the Hero",
            "difficulty": 5, "damage": 8, "toughness": 7,
            "control": 7, "utility": 7, "mobility": 7,
            "derived_stats": {"burst": 50, "sustain": 40}
        })
    }

    #[test]
    fn preserves_details_order_and_source_weights() {
        let result = score(
            &json!({
                "role": "Mage", "position": "Mid", "difficulty": 5,
                "damage": 8, "toughness": 7, "burst": 50, "sustain": 40,
                "pressure_response": "Stay calm and strategic",
                "aesthetic_preference": "Heroic",
                "team_contribution": "Lead and make decisions",
                "problem_solving": "Analyze carefully before acting"
            }),
            &champion(),
        );
        assert_eq!(result["rawScore"], json!(185.0));
        assert_eq!(result["score"], json!((220.0 / 245.0) * 100.0));
        assert_eq!(
            result["details"],
            json!({
                "contributingFactors": [
                    {"factor": "Role Match", "weight": 40, "contribution": 40},
                    {"factor": "Position Match", "weight": 30, "contribution": 30},
                    {"factor": "Difficulty Similarity", "weight": 15, "contribution": 15.0},
                    {"factor": "Damage Similarity", "weight": 15, "contribution": 15.0},
                    {"factor": "Toughness Similarity", "weight": 15, "contribution": 15.0},
                    {"factor": "Psychological Match", "weight": 75, "contribution": 50}
                ],
                "matchedCriteria": ["Role: Mage", "Position: Mid", "Excellent Burst match",
                    "Excellent Sustain match", "Psychological preferences matched (50 points)"],
                "penalties": []
            })
        );
    }

    #[test]
    fn mismatch_penalties_and_similarity_floors() {
        let result = score(
            &json!({
                "role": "Tank", "position": "Bot", "difficulty": 100,
                "damage": 100, "toughness": 100, "burst": 1000, "sustain": 1000
            }),
            &champion(),
        );
        assert_eq!(result["rawScore"], json!(-35.0));
        assert_eq!(result["score"], json!(0.0));
        assert_eq!(
            result["details"]["penalties"],
            json!([
                "Role mismatch: wanted Tank, got Mage",
                "Position mismatch: wanted Bot"
            ])
        );
        assert_eq!(result["details"]["matchedCriteria"], json!([]));
    }

    #[test]
    fn derived_matches_use_strict_threshold_and_missing_zero_defaults() {
        let mut features = json!({
            "role": "No Preference", "position": "No Preference",
            "difficulty": 5, "damage": 8, "toughness": 7, "burst": 35, "sustain": 25
        });
        let result = score(&features, &champion());
        assert_eq!(
            result["details"]["matchedCriteria"],
            json!(["Role: Mage", "Position: No Preference"])
        );
        features["burst"] = json!(35.5);
        features["sustain"] = json!(25.5);
        let result = score(&features, &champion());
        assert_eq!(
            result["details"]["matchedCriteria"],
            json!([
                "Role: Mage",
                "Position: No Preference",
                "Excellent Burst match",
                "Excellent Sustain match"
            ])
        );
        let mut data = champion();
        data.as_object_mut().unwrap().remove("derived_stats");
        features.as_object_mut().unwrap().remove("burst");
        features["sustain"] = Value::Null;
        assert_eq!(score(&features, &data)["rawScore"], json!(135.0));
    }

    #[test]
    fn all_psychological_options_keep_their_source_contributions() {
        for (key, option, role, expected) in [
            ("pressure_response", "Stay calm and strategic", "Mage", 15),
            ("pressure_response", "Take charge and lead", "Mage", 15),
            (
                "pressure_response",
                "Get aggressive and take risks",
                "Mage",
                15,
            ),
            (
                "pressure_response",
                "Play cautiously to avoid mistakes",
                "Mage",
                15,
            ),
            ("aesthetic_preference", "Heroic", "Mage", 10),
            ("aesthetic_preference", "Mysterious", "Mage", 10),
            ("aesthetic_preference", "Mysterious", "Assassin", 10),
            ("aesthetic_preference", "Dark and edgy", "Assassin", 10),
            ("aesthetic_preference", "Cute or playful", "Support", 10),
            (
                "aesthetic_preference",
                "Monstrous or non-human",
                "Fighter",
                10,
            ),
            ("aesthetic_preference", "Monstrous or non-human", "Tank", 10),
            ("team_contribution", "Lead and make decisions", "Mage", 15),
            ("team_contribution", "Support and enable others", "Mage", 15),
            ("team_contribution", "Balance between both", "Mage", 15),
            (
                "team_contribution",
                "Stay independent and focus on my role",
                "Mage",
                15,
            ),
            (
                "problem_solving",
                "Analyze carefully before acting",
                "Mage",
                10,
            ),
            (
                "problem_solving",
                "Jump in and adapt on the fly",
                "Mage",
                10,
            ),
            ("problem_solving", "Follow the team's lead", "Mage", 10),
            (
                "problem_solving",
                "Focus on long-term improvement",
                "Mage",
                10,
            ),
        ] {
            let mut data = champion();
            data["role"] = json!(role);
            let mut features = json!({"difficulty": 5, "damage": 8, "toughness": 7});
            features[key] = json!(option);
            let result = score(&features, &data);
            let factors = result["details"]["contributingFactors"].as_array().unwrap();
            assert_eq!(
                factors.last().unwrap()["contribution"],
                json!(expected),
                "{key}: {option}"
            );
        }
    }

    #[test]
    fn missing_direct_numeric_operands_serialize_as_null() {
        let result = score(&json!({}), &champion());
        assert!(result["rawScore"].is_null());
        assert!(result["score"].is_null());
        assert!(result["details"]["contributingFactors"][0]["contribution"].is_null());
        assert_eq!(
            result["details"]["penalties"],
            json!([
                "Role mismatch: wanted undefined, got Mage",
                "Position mismatch: wanted undefined"
            ])
        );
    }

    fn equivalent_json(actual: &Value, expected: &Value) -> bool {
        match (actual, expected) {
            (Value::Number(a), Value::Number(b)) => {
                (a.as_f64().unwrap() - b.as_f64().unwrap()).abs() < 1e-9
            }
            (Value::Array(a), Value::Array(b)) => {
                a.len() == b.len() && a.iter().zip(b).all(|(a, b)| equivalent_json(a, b))
            }
            (Value::Object(a), Value::Object(b)) => {
                a.len() == b.len()
                    && a.iter().all(|(key, value)| {
                        b.get(key)
                            .is_some_and(|other| equivalent_json(value, other))
                    })
            }
            _ => actual == expected,
        }
    }

    #[test]
    fn matches_all_migration_fixture_scores_and_details() {
        let fixture_dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/migration");
        let catalog: Value = serde_json::from_str(
            &std::fs::read_to_string(fixture_dir.join("catalog.json")).unwrap(),
        )
        .unwrap();
        let mut comparisons = 0;
        for entry in std::fs::read_dir(&fixture_dir).unwrap() {
            let path = entry.unwrap().path();
            if !path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("profile-")
            {
                continue;
            }
            let profile: Value =
                serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
            for entry in catalog.as_array().unwrap() {
                let name = entry["name"].as_str().unwrap();
                let expected = &profile["scores"]["decisionTree"][name];
                let actual = score(&profile["features"], &entry["data"]);
                assert!(
                    equivalent_json(&actual["details"], &expected["details"]),
                    "{}: {name}: {} != {}",
                    path.display(),
                    actual["details"],
                    expected["details"]
                );
                for key in ["rawScore", "score"] {
                    assert!(
                        (actual[key].as_f64().unwrap() - expected[key].as_f64().unwrap()).abs()
                            < 1e-9,
                        "{}: {name}.{key}: {} != {}",
                        path.display(),
                        actual[key],
                        expected[key]
                    );
                }
                comparisons += 1;
            }
        }
        assert_eq!(comparisons, 8 * 162);
    }
}
