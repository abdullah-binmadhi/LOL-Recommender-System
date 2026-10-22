use crate::compat::{num, text};
use serde_json::{json, Value};

/// Computes champion compatibility using Cosine Similarity over continuous feature embeddings.
pub fn score(features: &Value, champion: &Value) -> Value {
    let mut u_vec = Vec::with_capacity(9);
    let mut c_vec = Vec::with_capacity(9);

    let attrs = [
        "difficulty",
        "damage",
        "toughness",
        "mobility",
        "control",
        "utility",
    ];
    for attr in attrs {
        u_vec.push(num(features, attr));
        c_vec.push(num(champion, attr));
    }

    // Derived stats
    let derived = &champion["derived_stats"];
    u_vec.push(num(features, "burst") / 10.0);
    c_vec.push(num(derived, "burst") / 10.0);

    u_vec.push(num(features, "sustain") / 10.0);
    c_vec.push(num(derived, "sustain") / 10.0);

    u_vec.push(num(features, "cc") / 10.0);
    c_vec.push(num(derived, "cc") / 10.0);

    // Compute dot product and vector magnitudes
    let mut dot_product = 0.0;
    let mut u_norm_sq = 0.0;
    let mut c_norm_sq = 0.0;

    for (u, c) in u_vec.iter().zip(c_vec.iter()) {
        dot_product += u * c;
        u_norm_sq += u * u;
        c_norm_sq += c * c;
    }

    let u_norm = u_norm_sq.sqrt().max(0.001);
    let c_norm = c_norm_sq.sqrt().max(0.001);
    let cosine = (dot_product / (u_norm * c_norm)).clamp(-1.0, 1.0);

    let mut matched_criteria = Vec::new();
    let mut penalties = Vec::new();

    // Baseline angular score: maps [-1, 1] to [0, 100]
    let mut raw_score = ((cosine + 1.0) / 2.0) * 100.0;
    matched_criteria.push(format!("Vector cosine alignment: {:.1}%", raw_score));

    // Role weighting alignment
    let user_role = text(features, "role");
    let champ_role = text(champion, "role");
    if user_role == "No Preference" || user_role == "no_preference" || user_role.is_empty() {
        // Universal fit
    } else if user_role.eq_ignore_ascii_case(champ_role) {
        raw_score = (raw_score * 1.05).min(100.0);
        matched_criteria.push(format!("Role synergy boost: {champ_role}"));
    } else {
        raw_score *= 0.88;
        penalties.push(format!("Role divergence: {user_role} vs {champ_role}"));
    }

    let clamped_score = raw_score.clamp(10.0, 99.0);

    json!({
        "score": clamped_score,
        "rawScore": clamped_score,
        "details": {
            "contributingFactors": [{
                "factor": "Cosine Space Embedding",
                "weight": 100,
                "contribution": clamped_score
            }],
            "matchedCriteria": matched_criteria,
            "penalties": penalties
        }
    })
}
