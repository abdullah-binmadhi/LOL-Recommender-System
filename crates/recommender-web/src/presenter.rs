//! Pure presentation logic for the recommender UI.
//!
//! Everything here is free of Leptos and browser types so it is unit-tested
//! natively. The `view!` macros in `views.rs` only render data prepared here.

use std::f64::consts::PI;

/// Role value meaning the user expressed no lane preference.
pub const NO_PREFERENCE: &str = "No Preference";

/// The five combat attributes shown for every champion (0 to 10).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Attributes {
    pub damage: f64,
    pub toughness: f64,
    pub mobility: f64,
    pub control: f64,
    pub utility: f64,
}

/// How well a champion fits the three user-facing preference dimensions (0 to 100).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatchBreakdown {
    pub role: f64,
    pub difficulty: f64,
    pub survivability: f64,
}

/// Computes the role, difficulty and survivability fit used in match cards.
pub fn match_breakdown(
    preferred_role: &str,
    champion_role: &str,
    preferred_difficulty: f64,
    champion_difficulty: f64,
    toughness: f64,
) -> MatchBreakdown {
    let role = if preferred_role == NO_PREFERENCE || preferred_role == champion_role {
        100.0
    } else {
        70.0
    };
    MatchBreakdown {
        role,
        difficulty: (100.0 - (champion_difficulty - preferred_difficulty).abs() * 10.0).max(0.0),
        survivability: (100.0 - (toughness - 5.0).abs() * 8.0).max(0.0),
    }
}

/// A highlighted strength chip: a stable CSS key and a display label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Strength {
    pub key: &'static str,
    pub label: &'static str,
}

/// Lists the strengths a champion earns from its attributes and difficulty.
pub fn strengths(attributes: &Attributes, difficulty: f64) -> Vec<Strength> {
    [
        (attributes.damage >= 8.0, "damage", "High Damage"),
        (attributes.toughness >= 7.0, "tank", "Tanky"),
        (attributes.mobility >= 7.0, "mobile", "Mobile"),
        (attributes.control >= 7.0, "control", "Crowd Control"),
        (attributes.utility >= 8.0, "utility", "High Utility"),
        (difficulty <= 3.0, "easy", "Beginner Friendly"),
    ]
    .into_iter()
    .filter(|(earned, _, _)| *earned)
    .map(|(_, key, label)| Strength { key, label })
    .collect()
}

/// Qualitative band for a compatibility score.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Perfect,
    Excellent,
    Strong,
    Good,
    Fair,
}

impl Tier {
    pub fn label(self) -> &'static str {
        match self {
            Tier::Perfect => "Perfect match",
            Tier::Excellent => "Excellent match",
            Tier::Strong => "Strong match",
            Tier::Good => "Good match",
            Tier::Fair => "Fair match",
        }
    }

    pub fn css(self) -> &'static str {
        match self {
            Tier::Perfect => "tier-perfect",
            Tier::Excellent => "tier-excellent",
            Tier::Strong => "tier-strong",
            Tier::Good => "tier-good",
            Tier::Fair => "tier-fair",
        }
    }
}

/// Maps a 0 to 100 score to a [`Tier`].
pub fn tier_for(score: f64) -> Tier {
    if score >= 85.0 {
        Tier::Perfect
    } else if score >= 75.0 {
        Tier::Excellent
    } else if score >= 65.0 {
        Tier::Strong
    } else if score >= 50.0 {
        Tier::Good
    } else {
        Tier::Fair
    }
}

/// Builds the three "why this champion" sentences for the spotlight card.
pub fn reasons(
    preferred_role: &str,
    champion_role: &str,
    breakdown: &MatchBreakdown,
    attributes: &Attributes,
) -> Vec<String> {
    let role = if preferred_role == NO_PREFERENCE {
        "Fits every lane, so your role stays flexible".to_owned()
    } else if breakdown.role >= 100.0 {
        format!("Plays your preferred {champion_role} role")
    } else {
        format!("A {champion_role} flex pick for your {preferred_role} preference")
    };

    let difficulty = if breakdown.difficulty >= 80.0 {
        "Mechanical difficulty matches your comfort level"
    } else if breakdown.difficulty >= 50.0 {
        "Slightly different learning curve than you asked for"
    } else {
        "Higher skill ceiling than your selected comfort level"
    }
    .to_owned();

    let (label, value) = [
        ("damage", attributes.damage),
        ("durability", attributes.toughness),
        ("mobility", attributes.mobility),
        ("crowd control", attributes.control),
        ("utility", attributes.utility),
    ]
    .into_iter()
    .fold(("damage", f64::MIN), |best, candidate| {
        if candidate.1 > best.1 {
            candidate
        } else {
            best
        }
    });

    vec![role, difficulty, format!("Standout {label}: {value:.0}/10")]
}

/// Stroke geometry for the radial compatibility gauge.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GaugeGeometry {
    pub circumference: f64,
    pub dash_offset: f64,
}

/// Computes the dash array and offset that fill `score` percent of a circle.
pub fn gauge_geometry(score: f64, radius: f64) -> GaugeGeometry {
    let circumference = 2.0 * PI * radius;
    let fraction = score.clamp(0.0, 100.0) / 100.0;
    GaugeGeometry {
        circumference,
        dash_offset: circumference * (1.0 - fraction),
    }
}

/// How closely the three ML models agree on a champion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Consensus {
    Unanimous,
    Aligned,
    Divergent,
}

impl Consensus {
    pub fn label(self) -> &'static str {
        match self {
            Consensus::Unanimous => "All three models agree",
            Consensus::Aligned => "Models mostly agree",
            Consensus::Divergent => "Models disagree, inspect the math",
        }
    }

    pub fn css(self) -> &'static str {
        match self {
            Consensus::Unanimous => "consensus-high",
            Consensus::Aligned => "consensus-mid",
            Consensus::Divergent => "consensus-low",
        }
    }
}

/// Classifies model agreement from the spread between the three scores.
pub fn consensus(random_forest: f64, decision_tree: f64, knn: f64) -> Consensus {
    let high = random_forest.max(decision_tree).max(knn);
    let low = random_forest.min(decision_tree).min(knn);
    let spread = high - low;
    if spread <= 12.0 {
        Consensus::Unanimous
    } else if spread <= 25.0 {
        Consensus::Aligned
    } else {
        Consensus::Divergent
    }
}

/// Overall quality verdict for the evaluation heuristic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Performance {
    pub level: &'static str,
    pub message: &'static str,
}

/// Summarises precision@10 and MRR into a verdict.
pub fn performance(precision_at10: f64, mrr: f64) -> Performance {
    if precision_at10 >= 0.6 && mrr >= 0.5 {
        Performance {
            level: "Excellent",
            message: "The recommendations are highly aligned with your preferences! The top suggestions should be great matches for your playstyle.",
        }
    } else if precision_at10 >= 0.4 || mrr >= 0.3 {
        Performance {
            level: "Good",
            message: "The recommendations show good alignment with your preferences. You should find several suitable champions in the top suggestions.",
        }
    } else {
        Performance {
            level: "Needs Improvement",
            message: "The recommendations show room for improvement. Consider adjusting your preferences for better matches.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ATTRS: Attributes = Attributes {
        damage: 8.0,
        toughness: 4.0,
        mobility: 9.0,
        control: 5.0,
        utility: 3.0,
    };

    #[test]
    fn breakdown_matches_the_legacy_formulas() {
        let exact = match_breakdown("Mid", "Mid", 5.0, 5.0, 5.0);
        assert_eq!(exact.role, 100.0);
        assert_eq!(exact.difficulty, 100.0);
        assert_eq!(exact.survivability, 100.0);

        let off = match_breakdown("Top", "Mid", 2.0, 9.0, 1.0);
        assert_eq!(off.role, 70.0);
        assert_eq!(off.difficulty, 30.0);
        assert_eq!(off.survivability, 68.0);

        assert_eq!(
            match_breakdown(NO_PREFERENCE, "Mid", 5.0, 5.0, 5.0).role,
            100.0
        );
        assert_eq!(
            match_breakdown("Mid", "Mid", 1.0, 10.0, 5.0).difficulty,
            10.0
        );
        assert_eq!(
            match_breakdown("Mid", "Mid", 0.0, 20.0, 5.0).difficulty,
            0.0
        );
    }

    #[test]
    fn strengths_use_the_legacy_thresholds() {
        let labels: Vec<_> = strengths(&ATTRS, 3.0).iter().map(|s| s.label).collect();
        assert_eq!(labels, ["High Damage", "Mobile", "Beginner Friendly"]);
        assert!(strengths(&ATTRS, 4.0).iter().all(|s| s.key != "easy"));
    }

    #[test]
    fn tiers_change_at_the_documented_boundaries() {
        assert_eq!(tier_for(85.0), Tier::Perfect);
        assert_eq!(tier_for(84.9), Tier::Excellent);
        assert_eq!(tier_for(75.0), Tier::Excellent);
        assert_eq!(tier_for(65.0), Tier::Strong);
        assert_eq!(tier_for(50.0), Tier::Good);
        assert_eq!(tier_for(49.9), Tier::Fair);
        assert_eq!(Tier::Perfect.css(), "tier-perfect");
        assert_eq!(Tier::Fair.label(), "Fair match");
    }

    #[test]
    fn reasons_cover_role_difficulty_and_standout_attribute() {
        let breakdown = match_breakdown("Mid", "Mid", 5.0, 5.0, 5.0);
        let lines = reasons("Mid", "Mid", &breakdown, &ATTRS);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "Plays your preferred Mid role");
        assert_eq!(lines[1], "Mechanical difficulty matches your comfort level");
        assert_eq!(lines[2], "Standout mobility: 9/10");

        let flex = match_breakdown("Top", "Mid", 5.0, 5.0, 5.0);
        assert_eq!(
            reasons("Top", "Mid", &flex, &ATTRS)[0],
            "A Mid flex pick for your Top preference"
        );
        assert_eq!(
            reasons(NO_PREFERENCE, "Mid", &breakdown, &ATTRS)[0],
            "Fits every lane, so your role stays flexible"
        );

        let hard = match_breakdown("Mid", "Mid", 1.0, 9.0, 5.0);
        assert_eq!(
            reasons("Mid", "Mid", &hard, &ATTRS)[1],
            "Higher skill ceiling than your selected comfort level"
        );
    }

    #[test]
    fn reasons_break_attribute_ties_by_listed_order() {
        let tied = Attributes {
            damage: 6.0,
            toughness: 6.0,
            mobility: 6.0,
            control: 6.0,
            utility: 6.0,
        };
        let breakdown = match_breakdown("Mid", "Mid", 5.0, 5.0, 5.0);
        assert_eq!(
            reasons("Mid", "Mid", &breakdown, &tied)[2],
            "Standout damage: 6/10"
        );
    }

    #[test]
    fn gauge_fills_the_requested_fraction_and_clamps() {
        let half = gauge_geometry(50.0, 52.0);
        assert!((half.dash_offset - half.circumference / 2.0).abs() < 1e-9);
        assert!((gauge_geometry(100.0, 52.0).dash_offset).abs() < 1e-9);
        let empty = gauge_geometry(-20.0, 52.0);
        assert!((empty.dash_offset - empty.circumference).abs() < 1e-9);
        assert!((gauge_geometry(250.0, 52.0).dash_offset).abs() < 1e-9);
    }

    #[test]
    fn consensus_is_based_on_the_score_spread() {
        assert_eq!(consensus(80.0, 75.0, 72.0), Consensus::Unanimous);
        assert_eq!(consensus(84.0, 59.0, 72.0), Consensus::Aligned);
        assert_eq!(consensus(90.0, 40.0, 70.0), Consensus::Divergent);
        assert_eq!(Consensus::Unanimous.css(), "consensus-high");
        assert_eq!(Consensus::Aligned.label(), "Models mostly agree");
    }

    #[test]
    fn performance_verdict_thresholds_are_preserved() {
        assert_eq!(performance(0.6, 0.5).level, "Excellent");
        assert_eq!(performance(0.59, 0.5).level, "Good");
        assert_eq!(performance(0.1, 0.3).level, "Good");
        assert_eq!(performance(0.1, 0.1).level, "Needs Improvement");
    }
}
