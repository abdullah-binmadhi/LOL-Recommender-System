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
        
            ..Default::default()
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
        
            ..Default::default()
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

    // Verify latest match is accurately Ornn vs Vel'Koz Top
    let latest_match = &report.recent_matches[0];
    assert_eq!(latest_match.champion_name, "Ornn");
    assert_eq!(latest_match.role, "Top");
    assert_eq!(latest_match.lane_opponent_champion.as_deref(), Some("Vel'Koz"));
    let deep = latest_match.build_deep_analysis(&report.summoner_name);
    assert_eq!(deep.lane_opponent_champion, "Vel'Koz");
    assert!(deep.personal_mistake_diagnosis.contains("Vel'Koz"));

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
    assert!(report.recommendation_hit_rate > 75.0);
    assert!(report.calibrated_perf_delta > 0.0);

    // Verify No-Quiz Account ML Training & Validation Evaluation
    assert_eq!(
        report.ml_evaluation.train_samples + report.ml_evaluation.val_samples,
        16
    );
    assert!(report.ml_evaluation.train_accuracy >= 80.0);
    assert!(report.ml_evaluation.val_accuracy >= 60.0);
    assert!(report.ml_evaluation.no_quiz_recommendations.len() >= 6);
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
    assert_eq!(shen_ml.match_progression.len(), 6);
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
    assert_eq!(extended[0].champion_name, "Ornn");
    assert_eq!(extended[0].lane_opponent_champion.as_deref(), Some("Vel'Koz"));
    let deep = extended[0].build_deep_analysis("SHENTRICK");
    assert_eq!(deep.lane_opponent_champion, "Vel'Koz");
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
    assert_eq!(curve.ml_models.len(), 6);
    assert!(!curve.ml_models[0].plain_english_meaning.is_empty());
    assert!(!curve.ml_models[0].key_alert_notification.is_empty());
    assert!(!curve.ml_models[0].actionable_improvement.is_empty());
    assert_eq!(curve.shap_attributions.len(), 6);
    assert_eq!(curve.item_spikes.len(), 5);
    assert_eq!(curve.rune_telemetry.len(), 4);
    assert_eq!(curve.timestamped_moments.len(), 18);
    assert!(!curve.timestamped_moments[0].warning_notification.is_empty());
    assert!(!curve.timestamped_moments[0].gold_xp_swing_label.is_empty());

    // Verify Tab 4 Verified Champion Pool
    let lifetime_pool = report.lifetime_champion_pool();
    assert!(lifetime_pool.len() >= 8);
    assert_eq!(lifetime_pool[0].champion_name, "Shen");
    assert!(lifetime_pool[0].games_played >= 1);
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

#[test]
fn test_role_authentic_matchups_and_builds() {
    use recommender_core::player_analytics::canonical_role_and_archetype;

    let (braum_role, braum_arch) = canonical_role_and_archetype("Braum");
    assert_eq!(braum_role, "Support");
    assert!(braum_arch.contains("Vanguard"));

    let (ornn_role, _) = canonical_role_and_archetype("Ornn");
    assert_eq!(ornn_role, "Top");

    let report = PlayerAnalyticsReport::for_player("SHENTRICK", "SHENTRICK#SHEN", "ME", None);
    let braum_ml = report.champion_ml_insight_for("Braum");
    assert_eq!(braum_ml.champion_name, "Braum");

    // Match point opponent for Braum must be a Support, NOT a Top laner
    let sample_pt = &braum_ml.match_progression[0];
    let opp = sample_pt.opponent_champion("Braum");
    let (opp_role, _) = canonical_role_and_archetype(&opp);
    assert_eq!(opp_role, "Support", "Expected Braum opponent to be a Support champion, got {}", opp);

    // Braum lifetime matchups must be Support champions (Nautilus, Leona, Blitzcrank, etc.)
    let braum_matchups = braum_ml.lifetime_matchup_matrix();
    assert!(!braum_matchups.is_empty());
    let opp_names: Vec<&str> = braum_matchups.iter().map(|m| m.opponent_champion.as_str()).collect();
    assert!(opp_names.contains(&"Nautilus"));
    assert!(opp_names.contains(&"Blitzcrank"));
    assert!(!opp_names.contains(&"Fiora"), "Braum should not face Fiora in bot lane matchups");

    // Braum builds must contain support tank items (Locket, Knight's Vow, etc.), never AP mage items
    let braum_builds = braum_ml.lifetime_core_builds();
    assert!(!braum_builds.is_empty());
    let all_items: Vec<&str> = braum_builds.iter().flat_map(|b| b.core_items.iter().map(|s| s.as_str())).collect();
    assert!(all_items.contains(&"Locket of the Iron Solari") || all_items.contains(&"Knight's Vow"));
    assert!(!all_items.contains(&"Rabadon's Deathcap"));
    assert!(!all_items.contains(&"Malignance / Luden's"));

    // Ornn builds must be tank/forge items, never bruiser items like Titanic Hydra as default
    let ornn_ml = report.champion_ml_insight_for("Ornn");
    let ornn_builds = ornn_ml.lifetime_core_builds();
    let ornn_items: Vec<&str> = ornn_builds.iter().flat_map(|b| b.core_items.iter().map(|s| s.as_str())).collect();
    assert!(ornn_items.contains(&"Sunfire Aegis") || ornn_items.contains(&"Jak'Sho, The Protean"));
}

#[test]
fn test_no_quiz_multi_role_and_class_recommendations() {
    let report = PlayerAnalyticsReport::for_player("SHENTRICK", "SHENTRICK#SHEN", "ME", None);
    let recs = &report.ml_evaluation.no_quiz_recommendations;

    // Must have rich pool across multiple roles
    assert!(recs.len() >= 30, "Expected at least 30 recommendations across roles, got {}", recs.len());

    // Verify candidates exist in each of the 5 roles
    let has_top = recs.iter().any(|r| r.role == "Top");
    let has_jungle = recs.iter().any(|r| r.role == "Jungle");
    let has_mid = recs.iter().any(|r| r.role == "Mid");
    let has_adc = recs.iter().any(|r| r.role == "ADC");
    let has_support = recs.iter().any(|r| r.role == "Support");
    assert!(has_top, "Missing Top recommendations");
    assert!(has_jungle, "Missing Jungle recommendations");
    assert!(has_mid, "Missing Mid recommendations");
    assert!(has_adc, "Missing ADC recommendations");
    assert!(has_support, "Missing Support recommendations");

    // Verify candidates exist across major classes
    let has_mage = recs.iter().any(|r| r.class.contains("Mage"));
    let has_marksman = recs.iter().any(|r| r.class.contains("Marksman"));
    let has_tank = recs.iter().any(|r| r.class.contains("Tank"));
    let has_warden = recs.iter().any(|r| r.class.contains("Warden"));
    assert!(has_mage, "Missing Mage recommendations");
    assert!(has_marksman, "Missing Marksman recommendations");
    assert!(has_tank, "Missing Tank recommendations");
    assert!(has_warden, "Missing Warden recommendations");

    // Check that each recommendation has valid fit % and non-empty rationale
    for r in recs {
        assert!(r.archetype_fit_pct >= 50.0 && r.archetype_fit_pct <= 100.0);
        assert!(!r.rationale.is_empty());
        assert!(!r.combat_focus.is_empty());
    }
}

#[test]
fn test_all_100_cataloged_recurring_mistakes() {
    let mistakes = recommender_core::player_analytics::all_cataloged_recurring_mistakes();
    assert_eq!(mistakes.len(), 100, "Must contain exactly 100 cataloged mistakes");

    let mut seen_titles = std::collections::HashSet::new();
    for m in &mistakes {
        assert!(!m.mistake_title.is_empty(), "Mistake title must not be empty");
        assert!(!m.phase_window.is_empty(), "Phase window must not be empty");
        assert!(!m.root_cause.is_empty(), "Root cause must not be empty");
        assert!(!m.corrective_drill.is_empty(), "Corrective drill must not be empty");
        assert!(m.frequency_pct > 0.0 && m.frequency_pct <= 100.0, "Frequency must be in 0..100%");
        assert!(m.win_prob_penalty_pct > 0.0 && m.win_prob_penalty_pct <= 100.0, "Penalty must be positive");
        assert!(seen_titles.insert(m.mistake_title.clone()), "Duplicate mistake title: {}", m.mistake_title);
    }

    // Verify all 5 categories are well-represented
    let early_lane = mistakes.iter().filter(|m| m.phase_window.contains("EARLY LANE") || m.phase_window.contains("EARLY-MID")).count();
    let skirmish = mistakes.iter().filter(|m| m.phase_window.contains("SKIRMISH")).count();
    let mid_macro = mistakes.iter().filter(|m| m.phase_window.contains("MID GAME")).count();
    let late_fight = mistakes.iter().filter(|m| m.phase_window.contains("LATE TEAMFIGHT")).count();
    let economy = mistakes.iter().filter(|m| m.phase_window.contains("ECONOMY") || m.phase_window.contains("ITEMIZATION")).count();

    assert!(early_lane >= 15, "Early lane category under-represented: {}", early_lane);
    assert!(skirmish >= 15, "Skirmish category under-represented: {}", skirmish);
    assert!(mid_macro >= 15, "Mid macro category under-represented: {}", mid_macro);
    assert!(late_fight >= 15, "Late fight category under-represented: {}", late_fight);
    assert!(economy >= 15, "Economy category under-represented: {}", economy);
}

