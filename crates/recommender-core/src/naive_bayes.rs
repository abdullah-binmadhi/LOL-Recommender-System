use crate::compat::{num, text};
use serde_json::{json, Value};

/// Computes champion compatibility using a Multi-Class Naive Bayes probabilistic model.
pub fn score(features: &Value, champion: &Value) -> Value {
    let mut log_likelihood = 0.0;
    let mut matched_criteria = Vec::new();
    let mut penalties = Vec::new();
    let mut contributing_factors = Vec::new();

    // 1. Role Prior & Likelihood P(Role | Champion)
    let user_role = text(features, "role");
    let champ_role = text(champion, "role");
    if user_role == "No Preference" || user_role == "no_preference" || user_role.is_empty() {
        log_likelihood += (0.95f64).ln();
        matched_criteria.push("Role: Universal prior applied".to_string());
    } else if user_role.eq_ignore_ascii_case(&champ_role) {
        log_likelihood += (0.90f64).ln();
        matched_criteria.push(format!("Role match: {champ_role} (P=0.90)"));
        contributing_factors.push(json!({
            "factor": "Role Likelihood",
            "weight": 90,
            "contribution": 35.0
        }));
    } else {
        log_likelihood += (0.25f64).ln();
        penalties.push(format!(
            "Role mismatch: wanted {user_role}, got {champ_role}"
        ));
    }

    // 2. Position Likelihood P(Position | Champion)
    let user_pos = text(features, "position");
    let positions = champion
        .get("positions")
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(Value::as_str).collect::<Vec<&str>>())
        .unwrap_or_default();

    if user_pos == "No Preference" || user_pos == "no_preference" || user_pos.is_empty() {
        log_likelihood += (0.95f64).ln();
    } else if positions.iter().any(|&p| p.eq_ignore_ascii_case(&user_pos)) {
        log_likelihood += (0.85f64).ln();
        matched_criteria.push(format!("Position verified: {user_pos}"));
        contributing_factors.push(json!({
            "factor": "Position Likelihood",
            "weight": 85,
            "contribution": 25.0
        }));
    } else {
        log_likelihood += (0.35f64).ln();
    }

    // 3. Continuous Attribute Gaussian Likelihood (Damage, Toughness, Mobility, Control, Utility)
    let mut attr_score = 0.0;
    for attr in ["damage", "toughness", "mobility", "control", "utility"] {
        let u_val = num(features, attr);
        let c_val = num(champion, attr);
        let diff = (u_val - c_val).abs();
        let gaussian_prob = (-0.5 * (diff / 2.5).powi(2)).exp().max(0.1);
        log_likelihood += gaussian_prob.ln();

        if diff <= 2.0 {
            attr_score += 8.0;
            matched_criteria.push(format!("{attr} match: Δ={diff:.1}"));
        }
    }
    contributing_factors.push(json!({
        "factor": "Attributes Gaussian Fit",
        "weight": 80,
        "contribution": attr_score
    }));

    // 4. Resource System Likelihood
    let user_res = text(features, "resource");
    if !user_res.is_empty() && user_res != "No Preference" {
        let champ_res = text(champion, "resource_type");
        if champ_res.eq_ignore_ascii_case(&user_res) {
            log_likelihood += (0.80f64).ln();
            matched_criteria.push(format!("Resource verified: {champ_res}"));
        } else {
            log_likelihood += (0.50f64).ln();
        }
    }

    // Convert log-likelihood to 0.0 - 100.0 probability score
    // Normalizing across baseline domain bounds [-15.0, 0.0]
    let normalized_score = ((log_likelihood + 12.0) / 12.0 * 100.0).clamp(15.0, 99.5);

    json!({
        "score": normalized_score,
        "rawScore": normalized_score,
        "details": {
            "contributingFactors": contributing_factors,
            "matchedCriteria": matched_criteria,
            "penalties": penalties
        }
    })
}
