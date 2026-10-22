use recommender_core::player_analytics::{AdoptedMatch, CalibrationRecord, PlayerAnalyticsReport};

#[test]
fn test_demo_player_analytics_report() {
    let report = PlayerAnalyticsReport::demo();

    assert_eq!(report.summoner_name, "Faker");
    assert_eq!(report.riot_id.as_deref(), Some("Faker#T1"));
    assert_eq!(report.region.as_deref(), Some("KR"));
    assert!(report.is_demo);
    assert_eq!(report.total_analyzed_matches, 10);

    // Efficacy assertions
    assert_eq!(report.efficacy.calibrated_games, 7);
    assert_eq!(report.efficacy.calibrated_wins, 7);
    assert_eq!(report.efficacy.calibrated_win_rate, 100.0);
    assert_eq!(report.efficacy.uncalibrated_games, 3);
    assert_eq!(report.efficacy.uncalibrated_wins, 0);
    assert_eq!(report.efficacy.uncalibrated_win_rate, 0.0);
    assert_eq!(report.efficacy.win_rate_lift, 100.0);
    assert_eq!(report.efficacy.adoption_rate, 70.0);

    // Anchor champion: Ahri was in all 3 calibrations
    assert!(report
        .evolution
        .anchor_champions
        .contains(&"Ahri".to_string()));

    // ML insights generated
    assert_eq!(report.ml_insights.len(), 3);
    assert_eq!(report.ml_insights[0].id, "adaptability-curve");
    assert_eq!(report.ml_insights[1].id, "draft-blindspot");
    assert_eq!(report.ml_insights[2].id, "fatigue-advisory");
}

#[test]
fn test_custom_player_analytics_calculation() {
    let matches = vec![
        AdoptedMatch {
            match_id: "NA-1".into(),
            champion_name: "Jinx".into(),
            role: "Bottom".into(),
            win: true,
            kills: 10,
            deaths: 2,
            assists: 8,
            cs_per_min: 9.1,
            gold_diff_15: 1200,
            is_calibrated: true,
            calibrated_rank: Some(1),
        },
        AdoptedMatch {
            match_id: "NA-2".into(),
            champion_name: "Kaisa".into(),
            role: "Bottom".into(),
            win: false,
            kills: 3,
            deaths: 6,
            assists: 4,
            cs_per_min: 7.2,
            gold_diff_15: -800,
            is_calibrated: false,
            calibrated_rank: None,
        },
    ];

    let calibrations = vec![CalibrationRecord {
        id: "c-1".into(),
        date_label: "Today".into(),
        role_focus: "Marksman".into(),
        top_champions: vec!["Jinx".into(), "Caitlyn".into(), "Ashe".into()],
        aggression: 80.0,
        mobility: 60.0,
        durability: 40.0,
        utility: 50.0,
        waveclear: 90.0,
    }];

    let report = PlayerAnalyticsReport::new(
        "Doublelift".into(),
        Some("Doublelift#NA1".into()),
        Some("NA".into()),
        false,
        matches,
        calibrations,
    );

    assert_eq!(report.total_analyzed_matches, 2);
    assert_eq!(report.efficacy.calibrated_games, 1);
    assert_eq!(report.efficacy.calibrated_win_rate, 100.0);
    assert_eq!(report.efficacy.uncalibrated_games, 1);
    assert_eq!(report.efficacy.uncalibrated_win_rate, 0.0);
    assert_eq!(report.efficacy.win_rate_lift, 100.0);
    assert_eq!(report.efficacy.adoption_rate, 50.0);
}

#[test]
fn test_for_player_shentrick_middle_east() {
    let report = PlayerAnalyticsReport::for_player("SHENTRICK", "SHENTRICK#SHEN", "ME", None);

    assert_eq!(report.summoner_name, "SHENTRICK");
    assert_eq!(report.riot_id.as_deref(), Some("SHENTRICK#SHEN"));
    assert_eq!(report.region.as_deref(), Some("ME"));
    assert!(!report.is_demo);
    assert_eq!(report.total_analyzed_matches, 10);

    // Primary champion must be Shen
    assert!(report.evolution.anchor_champions.contains(&"Shen".to_string()));

    // Match IDs must use ME1- prefix
    assert!(report.recent_matches.iter().all(|m| m.match_id.starts_with("ME1-")));

    // Role must be Top
    assert!(report.recent_matches.iter().all(|m| m.role == "Top"));

    // Shen specific ML insights
    assert_eq!(report.ml_insights[0].id, "stand-united-synergy");
    assert_eq!(report.ml_insights[1].id, "ki-barrier-trading");
    assert_eq!(report.ml_insights[2].id, "side-lane-tempo");

    // Efficacy calculations
    assert_eq!(report.efficacy.calibrated_games, 8);
    assert_eq!(report.efficacy.calibrated_wins, 8);
    assert_eq!(report.efficacy.calibrated_win_rate, 100.0);

    // Champion performance records with Bayesian shrinkage
    assert!(!report.champion_performances.is_empty());
    let shen_perf = report
        .champion_performances
        .iter()
        .find(|c| c.champion_name == "Shen")
        .expect("Shen performance must be present");
    assert_eq!(shen_perf.games_played, 6);
    assert_eq!(shen_perf.wins, 6);
    assert_eq!(shen_perf.win_rate, 100.0);
    assert!(shen_perf.performance_index > 60.0);
    assert_eq!(shen_perf.confidence_rating, "Moderate Sample (5-9 Games)");
    assert!(shen_perf.is_calibrated_recommendation);

    // Recommendation hit rate & performance delta
    assert_eq!(report.recommendation_hit_rate, 80.0);
    assert!(report.calibrated_perf_delta > 0.0);
}
