use crate::compat::{num, text};
use serde_json::{json, Value};

/// Computes champion compatibility using Gradient Boosted Decision Stumps.
pub fn score(features: &Value, champion: &Value) -> Value {
    let mut total_score: f64 = 45.0; // Base prior prediction
    let learning_rate: f64 = 0.8;
    let mut matched_criteria = Vec::new();
    let mut penalties = Vec::new();
    let mut contributing_factors = Vec::new();

    // Stump 1: Role & Class Alignment (Max delta: +25.0 / -15.0)
    let user_role = text(features, "role");
    let champ_role = text(champion, "role");
    if user_role == "No Preference" || user_role == "no_preference" || user_role.is_empty() {
        total_score += 10.0 * learning_rate;
    } else if user_role.eq_ignore_ascii_case(champ_role) {
        let delta = 25.0 * learning_rate;
        total_score += delta;
        matched_criteria.push(format!("Stump 1: Role match +{:.1}", delta));
        contributing_factors.push(json!({
            "factor": "Boosted Role Stump",
            "weight": 85,
            "contribution": delta
        }));
    } else {
        let penalty = 15.0 * learning_rate;
        total_score -= penalty;
        penalties.push(format!("Stump 1: Role mismatch -{:.1}", penalty));
    }

    // Stump 2: Mechanical Difficulty Comfort (Max delta: +15.0 / -10.0)
    let u_diff = num(features, "difficulty");
    let c_diff = num(champion, "difficulty");
    let diff_gap = (u_diff - c_diff).abs();
    if diff_gap <= 1.5 {
        let delta = 15.0 * learning_rate;
        total_score += delta;
        matched_criteria.push("Stump 2: Exact difficulty tier match".to_string());
    } else if diff_gap <= 3.0 {
        total_score += 5.0 * learning_rate;
    } else {
        let penalty = 12.0 * learning_rate;
        total_score -= penalty;
        penalties.push(format!(
            "Stump 2: Mechanical ceiling divergence Δ={diff_gap:.1}"
        ));
    }

    // Stump 3: Damage & Combat Survivability Trade-off (Max delta: +15.0)
    let u_dmg = num(features, "damage");
    let c_dmg = num(champion, "damage");
    let u_tank = num(features, "toughness");
    let c_tank = num(champion, "toughness");
    if (u_dmg - c_dmg).abs() <= 2.0 && (u_tank - c_tank).abs() <= 2.0 {
        let delta = 15.0 * learning_rate;
        total_score += delta;
        matched_criteria.push("Stump 3: Damage/Durability ratio aligned".to_string());
    }

    // Stump 4: Mobility & Spacing (Max delta: +10.0)
    let u_mob = num(features, "mobility");
    let c_mob = num(champion, "mobility");
    if (u_mob - c_mob).abs() <= 2.0 {
        total_score += 10.0 * learning_rate;
        matched_criteria.push("Stump 4: Mobility & pacing aligned".to_string());
    }

    // Stump 5: Resource & Damage Type Cohesion (Max delta: +10.0)
    let user_res = text(features, "resource");
    let champ_res = text(champion, "resource_type");
    if !user_res.is_empty()
        && user_res != "No Preference"
        && user_res.eq_ignore_ascii_case(champ_res)
    {
        total_score += 10.0 * learning_rate;
        matched_criteria.push(format!("Stump 5: Resource verified: {champ_res}"));
    }

    let final_score = total_score.clamp(10.0, 99.0);

    json!({
        "score": final_score,
        "rawScore": final_score,
        "details": {
            "contributingFactors": contributing_factors,
            "matchedCriteria": matched_criteria,
            "penalties": penalties
        }
    })
}
