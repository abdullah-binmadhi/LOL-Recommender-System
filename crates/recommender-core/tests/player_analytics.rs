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
    assert_eq!(report.total_analyzed_matches, 16);

    // Primary champion must be Shen
    assert!(report
        .evolution
        .anchor_champions
        .contains(&"Shen".to_string()));

    // Match IDs must use ME1- prefix
    assert!(report
        .recent_matches
        .iter()
        .all(|m| m.match_id.starts_with("ME1-")));

    // Role must be Top
    assert!(report.recent_matches.iter().all(|m| m.role == "Top"));

    // Shen specific ML insights
    assert_eq!(report.ml_insights[0].id, "stand-united-synergy");
    assert_eq!(report.ml_insights[1].id, "ki-barrier-trading");
    assert_eq!(report.ml_insights[2].id, "side-lane-tempo");

    // Efficacy calculations
    assert_eq!(report.efficacy.calibrated_games, 13);
    assert_eq!(report.efficacy.calibrated_wins, 12);

    // Champion performance records with Bayesian shrinkage
    assert!(!report.champion_performances.is_empty());
    let shen_perf = report
        .champion_performances
        .iter()
        .find(|c| c.champion_name == "Shen")
        .expect("Shen performance must be present");
    assert_eq!(shen_perf.games_played, 7);
    assert_eq!(shen_perf.wins, 7);
    assert_eq!(shen_perf.win_rate, 100.0);
    assert!(shen_perf.performance_index > 60.0);
    assert_eq!(shen_perf.confidence_rating, "Moderate Sample (5-9 Games)");
    assert!(shen_perf.is_calibrated_recommendation);

    // Recommendation hit rate & performance delta
    assert!(report.recommendation_hit_rate > 75.0);
    assert!(report.calibrated_perf_delta > 0.0);

    // Verify No-Quiz Account ML Training & Validation Evaluation
    assert_eq!(
        report.ml_evaluation.train_samples + report.ml_evaluation.val_samples,
        16
    );
    assert!(report.ml_evaluation.train_accuracy >= 80.0);
    assert!(report.ml_evaluation.val_accuracy >= 60.0);
    assert_eq!(report.ml_evaluation.no_quiz_recommendations.len(), 6);
    assert_eq!(
        report.ml_evaluation.no_quiz_recommendations[0].champion_name,
        "Shen"
    );

    // Verify Champion-Specific ML Tracker Insights
    let shen_ml = report
        .champion_ml_insights
        .iter()
        .find(|i| i.champion_name == "Shen")
        .expect("Shen ML tracker insight must exist");
    assert_eq!(shen_ml.match_progression.len(), 7);
    assert!(shen_ml.win_prob_above_threshold > shen_ml.win_prob_below_threshold);

    // Verify Tab 1 Playstyle DNA Summary & Recurring Mistake Detector
    let dna = report.playstyle_dna_summary();
    assert!(!dna.archetype_title.is_empty());
    assert_eq!(dna.recurring_mistakes.len(), 3);
    assert_eq!(dna.phase_ratings.len(), 3);
    assert!(dna.phase_ratings[0] > 50.0);

    // Verify Tab 3 Extended 50-Match History & Single-Match 5v5 Deep Analysis
    let extended = report.extended_match_history(50);
    assert_eq!(extended.len(), 50);
    let deep = extended[0].build_deep_analysis("SHENTRICK");
    assert_eq!(deep.allied_team.len(), 5);
    assert_eq!(deep.enemy_team.len(), 5);
    assert_eq!(deep.win_prob_timeline.len(), 7);
    assert!(!deep.lane_verdict.is_empty());
    assert!(!deep.key_turning_point.is_empty());

    // Verify Tab 3 Single-Game Champion Curve & Validated ML Lab (Exact Match & Champion)
    let curve = extended[0].build_champion_curve_analysis("SHENTRICK");
    assert_eq!(curve.match_id, extended[0].match_id);
    assert_eq!(curve.champion_name, extended[0].champion_name);
    assert_eq!(curve.gold_curve.len(), 7);
    assert_eq!(curve.xp_level_curve.len(), 7);
    assert_eq!(curve.cs_tempo_curve.len(), 7);
    assert_eq!(curve.damage_mitigation_curve.len(), 7);
    assert_eq!(curve.radar_axes.len(), 8);
    assert_eq!(curve.ml_models.len(), 4);
    assert_eq!(curve.shap_attributions.len(), 6);
    assert_eq!(curve.item_spikes.len(), 5);
    assert_eq!(curve.rune_telemetry.len(), 4);
    assert_eq!(curve.timestamped_moments.len(), 4);

    // Verify Tab 4 Lifetime Champion Pool (18+ champions from Season 5 / 2015 to Season 16 / 2026)
    let lifetime_pool = report.lifetime_champion_pool();
    assert!(lifetime_pool.len() >= 18);
    assert_eq!(lifetime_pool[0].champion_name, "Shen");
    assert!(lifetime_pool[0].games_played >= 640);
    assert!(lifetime_pool[0].first_played_date.contains("2015"));

    // Verify Tab 5 Lifetime ML Suite (12 Seasons 2015–2026 + Matchup & Core Build Matrices)
    let gwen_ml = report.champion_ml_insight_for("Gwen");
    assert_eq!(gwen_ml.champion_name, "Gwen");
    assert!(!gwen_ml.match_progression.is_empty());
    let shen_seasons = shen_ml.season_career_history();
    assert_eq!(shen_seasons.len(), 12);
    assert_eq!(shen_seasons.first().unwrap().year, 2015);
    assert_eq!(shen_seasons.last().unwrap().year, 2026);
    assert!(!shen_ml.lifetime_matchup_matrix().is_empty());
    assert!(!shen_ml.lifetime_core_builds().is_empty());
}
