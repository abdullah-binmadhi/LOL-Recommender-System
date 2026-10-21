use crate::compat::truthy;
use serde_json::{json, Value};

/// Preserve the production numeric answer IDs, including the legacy Q6–10 mismatch.
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
    let (playstyle, damage, toughness, mobility, control, utility) = match answer("4", "").as_str()
    {
        "High Damage Output" => ("aggressive", 8.0, 3.0, 7.0, 4.0, 2.0),
        "Tanky and Durable" => ("defensive", 4.0, 8.0, 3.0, 6.0, 5.0),
        "Support Team" => ("supportive", 3.0, 5.0, 4.0, 7.0, 8.0),
        _ => ("balanced", 5.0, 5.0, 5.0, 5.0, 5.0),
    };
    let range = answers
        .get("5")
        .filter(|v| truthy(v))
        .and_then(Value::as_str)
        .map_or_else(|| "no_preference".to_owned(), str::to_lowercase);
    json!({
        "role": answer("2", "No Preference"),
        "position": answer("3", "No Preference"),
        "difficulty": difficulty,
        "damage": damage, "toughness": toughness, "mobility": mobility,
        "control": control, "utility": utility,
        "playstyle": playstyle, "range": range,
        "pressure_response": answer("6", "Stay calm and strategic"),
        "aesthetic_preference": answer("7", "Heroic"),
        "team_contribution": answer("8", "Balance between both"),
        "character_identity": answer("9", "No preference"),
        "problem_solving": answer("10", "Analyze carefully before acting"),
        "burst": ((damage / 3.33) * 0.7 + (mobility / 3.33) * 0.3) / 3.0 * 100.0,
        "sustain": ((toughness / 3.33) * 0.7 + (utility / 3.33) * 0.3) / 3.0 * 100.0,
        "cc": (control / 3.33) / 3.0 * 100.0
    })
}
