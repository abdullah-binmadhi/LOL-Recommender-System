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
