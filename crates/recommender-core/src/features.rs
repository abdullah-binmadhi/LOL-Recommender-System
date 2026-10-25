use crate::compat::truthy;
use serde_json::{json, Value};

/// Converts user questionnaire answers into normalized feature vectors.
/// Supports both modern 12-question format and legacy Q6–10 numeric answer IDs.
pub fn from_answers(answers: &Value) -> Value {
    let answer = |id: &str, default: &str| -> String {
        answers
            .get(id)
            .filter(|v| truthy(v))
            .and_then(Value::as_str)
            .unwrap_or(default)
            .to_owned()
    };
    let difficulty = match answer("1", "").as_str() {
        "Easy (1-3)" => 2.0,
        "Hard (7-8)" => 7.5,
        "Very Hard (9-10)" => 9.5,
        _ => 5.0,
    };
    let q4_raw = answer("4", "");
    let q4_parts: Vec<&str> = q4_raw
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    let (playstyle, damage, toughness, mobility, control, utility) = if q4_parts.is_empty() {
        ("balanced", 5.0, 5.0, 5.0, 5.0, 5.0)
    } else {
        let mut d_sum = 0.0;
        let mut t_sum = 0.0;
        let mut m_sum = 0.0;
        let mut c_sum = 0.0;
        let mut u_sum = 0.0;
        let mut p_style = "balanced";
        for part in &q4_parts {
            let (ps, d, t, m, c, u) = match *part {
                "High Damage Output" => ("aggressive", 8.0, 3.0, 7.0, 4.0, 2.0),
                "Tanky and Durable" => ("defensive", 4.0, 8.0, 3.0, 6.0, 5.0),
                "Support Team" => ("supportive", 3.0, 5.0, 4.0, 7.0, 8.0),
                _ => ("balanced", 5.0, 5.0, 5.0, 5.0, 5.0),
            };
            d_sum += d;
            t_sum += t;
            m_sum += m;
            c_sum += c;
            u_sum += u;
            if p_style == "balanced" {
                p_style = ps;
            }
        }
        let n = q4_parts.len() as f64;
        (p_style, d_sum / n, t_sum / n, m_sum / n, c_sum / n, u_sum / n)
    };
    let range = answers
        .get("5")
        .filter(|v| truthy(v))
        .and_then(Value::as_str)
        .map_or_else(|| "no_preference".to_owned(), str::to_lowercase);

    // If explicit semantic keys exist or quiz_version >= 2, use modern mapping; otherwise preserve legacy Q6-10 mapping
    let is_v2 = answers
        .get("quiz_version")
        .and_then(|v| {
            v.as_u64()
                .or_else(|| v.as_str().and_then(|s| s.parse::<u64>().ok()))
        })
        .unwrap_or(1)
        >= 2
        || answers.get("pressure_response").is_some();

    let is_25q = answers.get("25").is_some()
        || answers.get("20").is_some()
        || answers.get("power_curve").is_some();

    let (pressure, aesthetic, team_contrib, char_id, problem) = if is_25q {
        (
            answers
                .get("pressure_response")
                .or_else(|| answers.get("21"))
                .and_then(Value::as_str)
                .unwrap_or("Stay calm and strategic")
                .to_owned(),
            answers
                .get("aesthetic_preference")
                .or_else(|| answers.get("23"))
                .and_then(Value::as_str)
                .unwrap_or("Heroic")
                .to_owned(),
            answers
                .get("team_contribution")
                .or_else(|| answers.get("20"))
                .and_then(Value::as_str)
                .unwrap_or("Balance between both")
                .to_owned(),
            answers
                .get("thematic_fantasy")
                .or_else(|| answers.get("23"))
                .and_then(Value::as_str)
                .unwrap_or("No preference")
                .to_owned(),
            answers
                .get("problem_solving")
                .or_else(|| answers.get("25"))
                .and_then(Value::as_str)
                .unwrap_or("Analyze carefully before acting")
                .to_owned(),
        )
    } else if is_v2 {
        (
            answers
                .get("pressure_response")
                .or_else(|| answers.get("8"))
                .and_then(Value::as_str)
                .unwrap_or("Stay calm and strategic")
                .to_owned(),
            answers
                .get("aesthetic_preference")
                .or_else(|| answers.get("9"))
                .and_then(Value::as_str)
                .unwrap_or("Heroic")
                .to_owned(),
            answers
                .get("team_contribution")
                .or_else(|| answers.get("10"))
                .and_then(Value::as_str)
                .unwrap_or("Balance between both")
                .to_owned(),
            answers
                .get("character_identity")
                .or_else(|| answers.get("11"))
                .and_then(Value::as_str)
                .unwrap_or("No preference")
                .to_owned(),
            answers
                .get("problem_solving")
                .or_else(|| answers.get("12"))
                .and_then(Value::as_str)
                .unwrap_or("Analyze carefully before acting")
                .to_owned(),
        )
    } else {
        (
            answer("6", "Stay calm and strategic"),
            answer("7", "Heroic"),
            answer("8", "Balance between both"),
            answer("9", "No preference"),
            answer("10", "Analyze carefully before acting"),
        )
    };

    let raw_role = answer("2", "No Preference");
    let primary_role = raw_role
        .split(',')
        .next()
        .unwrap_or("No Preference")
        .trim()
        .to_string();
    let raw_pos = answer("3", "No Preference");
    let primary_pos = raw_pos
        .split(',')
        .next()
        .unwrap_or("No Preference")
        .trim()
        .to_string();

    let mut feat = json!({
        "role": primary_role,
        "position": primary_pos,
        "difficulty": difficulty,
        "damage": damage, "toughness": toughness, "mobility": mobility,
        "control": control, "utility": utility,
        "playstyle": playstyle, "range": range,
        "pressure_response": pressure,
        "aesthetic_preference": aesthetic,
        "team_contribution": team_contrib,
        "character_identity": char_id,
        "problem_solving": problem,
        "burst": ((damage / 3.33) * 0.7 + (mobility / 3.33) * 0.3) / 3.0 * 100.0,
        "sustain": ((toughness / 3.33) * 0.7 + (utility / 3.33) * 0.3) / 3.0 * 100.0,
        "cc": (control / 3.33) / 3.0 * 100.0
    });

    if is_v2 || is_25q {
        if let Some(map) = feat.as_object_mut() {
            let res = answers
                .get("resource")
                .or_else(|| answers.get("6"))
                .and_then(Value::as_str)
                .unwrap_or("No Preference");
            let raw_dmg = answers
                .get("damage_type")
                .or_else(|| answers.get("7"))
                .and_then(Value::as_str)
                .unwrap_or("No Preference");
            let dmg_type = if raw_dmg.to_lowercase().contains("hybrid")
                || (raw_dmg.contains("Physical") && raw_dmg.contains("Magic"))
            {
                "Hybrid".to_string()
            } else if raw_dmg.contains("Magic") {
                "Magic".to_string()
            } else if raw_dmg.contains("Physical") {
                "Physical".to_string()
            } else {
                "No Preference".to_string()
            };
            map.insert("resource".to_string(), json!(res));
            map.insert("damage_type".to_string(), json!(dmg_type));

            if is_25q {
                let add_dim = |m: &mut serde_json::Map<String, Value>, key: &str, id: &str| {
                    if let Some(v) = answers
                        .get(key)
                        .or_else(|| answers.get(id))
                        .and_then(Value::as_str)
                    {
                        m.insert(key.to_string(), json!(v));
                    }
                };
                add_dim(map, "power_curve", "8");
                add_dim(map, "targeting_style", "9");
                add_dim(map, "cooldown_cadence", "10");
                add_dim(map, "teamfight_stance", "11");
                add_dim(map, "defensive_style", "12");
                add_dim(map, "crowd_control", "13");
                add_dim(map, "mobility_type", "14");
                add_dim(map, "waveclear_speed", "15");
                add_dim(map, "split_vs_teamfight", "16");
                add_dim(map, "objective_affinity", "17");
                add_dim(map, "volatility_tolerance", "18");
                add_dim(map, "roaming_tendency", "19");
                add_dim(map, "duelist_orientation", "20");
                add_dim(map, "combat_sensation", "22");
                add_dim(map, "risk_profile", "24");
            }
        }
    }

    feat
}
