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

fn sample_shentrick_matches() -> Vec<AdoptedMatch> {
    vec![
        AdoptedMatch {
            match_id: "ME1-104928".into(),
            champion_name: "Ornn".into(),
            role: "Top".into(),
            win: true,
            kills: 5,
            deaths: 1,
            assists: 14,
            cs_per_min: 7.8,
            gold_diff_15: 840,
            is_calibrated: true,
            calibrated_rank: Some(1),
            lane_opponent_champion: Some("Vel'Koz".into()),
            duration_mins: Some(31),
            user_damage_dealt: Some(21850),
            user_vision_score: Some(42),
            items: Some(vec![3068, 6665, 3075, 3111, 2055, 3340]),
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-104855".into(),
            champion_name: "Shen".into(),
            role: "Top".into(),
            win: true,
            kills: 4,
            deaths: 2,
            assists: 18,
            cs_per_min: 6.9,
            gold_diff_15: 980,
            is_calibrated: true,
            calibrated_rank: Some(1),
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-104712".into(),
            champion_name: "Shen".into(),
            role: "Top".into(),
            win: true,
            kills: 8,
            deaths: 0,
            assists: 11,
            cs_per_min: 7.8,
            gold_diff_15: 2100,
            is_calibrated: true,
            calibrated_rank: Some(1),
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-104640".into(),
            champion_name: "Darius".into(),
            role: "Top".into(),
            win: false,
            kills: 3,
            deaths: 6,
            assists: 2,
            cs_per_min: 6.2,
            gold_diff_15: -1150,
            is_calibrated: false,
            calibrated_rank: None,
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-104519".into(),
            champion_name: "Shen".into(),
            role: "Top".into(),
            win: true,
            kills: 5,
            deaths: 3,
            assists: 16,
            cs_per_min: 7.1,
            gold_diff_15: 850,
            is_calibrated: true,
            calibrated_rank: Some(1),
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-104402".into(),
            champion_name: "Ornn".into(),
            role: "Top".into(),
            win: true,
            kills: 3,
            deaths: 2,
            assists: 15,
            cs_per_min: 7.3,
            gold_diff_15: 640,
            is_calibrated: true,
            calibrated_rank: Some(2),
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-104331".into(),
            champion_name: "Aatrox".into(),
            role: "Top".into(),
            win: false,
            kills: 4,
            deaths: 7,
            assists: 3,
            cs_per_min: 6.5,
            gold_diff_15: -890,
            is_calibrated: false,
            calibrated_rank: None,
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-104210".into(),
            champion_name: "Shen".into(),
            role: "Top".into(),
            win: true,
            kills: 6,
            deaths: 1,
            assists: 14,
            cs_per_min: 7.5,
            gold_diff_15: 1420,
            is_calibrated: true,
            calibrated_rank: Some(1),
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-104108".into(),
            champion_name: "K'Sante".into(),
            role: "Top".into(),
            win: true,
            kills: 5,
            deaths: 2,
            assists: 8,
            cs_per_min: 7.9,
            gold_diff_15: 1100,
            is_calibrated: true,
            calibrated_rank: Some(3),
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-103980".into(),
            champion_name: "Shen".into(),
            role: "Top".into(),
            win: true,
            kills: 4,
            deaths: 1,
            assists: 12,
            cs_per_min: 7.2,
            gold_diff_15: 1200,
            is_calibrated: true,
            calibrated_rank: Some(1),
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-103845".into(),
            champion_name: "Camille".into(),
            role: "Top".into(),
            win: false,
            kills: 2,
            deaths: 5,
            assists: 4,
            cs_per_min: 6.8,
            gold_diff_15: -450,
            is_calibrated: false,
            calibrated_rank: None,
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-103710".into(),
            champion_name: "Gwen".into(),
            role: "Top".into(),
            win: true,
            kills: 8,
            deaths: 3,
            assists: 6,
            cs_per_min: 8.4,
            gold_diff_15: 1850,
            is_calibrated: false,
            calibrated_rank: None,
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-103550".into(),
            champion_name: "Galio".into(),
            role: "Top".into(),
            win: true,
            kills: 6,
            deaths: 2,
            assists: 19,
            cs_per_min: 7.6,
            gold_diff_15: 1150,
            is_calibrated: true,
            calibrated_rank: Some(4),
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-103422".into(),
            champion_name: "Poppy".into(),
            role: "Top".into(),
            win: true,
            kills: 7,
            deaths: 2,
            assists: 11,
            cs_per_min: 7.1,
            gold_diff_15: 890,
            is_calibrated: true,
            calibrated_rank: Some(5),
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-103299".into(),
            champion_name: "Mordekaiser".into(),
            role: "Top".into(),
            win: false,
            kills: 4,
            deaths: 6,
            assists: 3,
            cs_per_min: 6.4,
            gold_diff_15: -780,
            is_calibrated: false,
            calibrated_rank: None,
            ..Default::default()
        },
        AdoptedMatch {
            match_id: "ME1-103150".into(),
            champion_name: "Shen".into(),
            role: "Top".into(),
            win: true,
            kills: 9,
            deaths: 2,
            assists: 15,
            cs_per_min: 7.9,
            gold_diff_15: 1920,
            is_calibrated: true,
            calibrated_rank: Some(1),
            ..Default::default()
        },
    ]
}

#[test]
fn test_for_player_zero_mock_data_when_unlinked() {
    let empty_report = PlayerAnalyticsReport::for_player("SHENTRICK", "SHENTRICK#SHEN", "ME", None);
    assert_eq!(empty_report.total_analyzed_matches, 0);
    assert_eq!(empty_report.recent_matches.len(), 0);
}

#[test]
fn test_for_player_shentrick_middle_east() {
    let matches = sample_shentrick_matches();
    let report = PlayerAnalyticsReport::for_player("SHENTRICK", "SHENTRICK#SHEN", "ME", Some(&matches));

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
    assert_eq!(report.efficacy.calibrated_games, 11);
    assert_eq!(report.efficacy.calibrated_wins, 11);

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
    assert!(report.recommendation_hit_rate > 60.0);
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

    // Verify Tab 3 Extended Match History strictly mirrors verified matches (no fake games)
    let extended = report.extended_match_history(50);
    assert_eq!(extended.len(), report.recent_matches.len());
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
    let matches = sample_shentrick_matches();
    let report = PlayerAnalyticsReport::for_player("SHENTRICK", "SHENTRICK#SHEN", "ME", Some(&matches));
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

#[test]
fn test_deserialize_supabase_edge_function_matches() {
    // 1. Test legacy/minimal format where participant summary only had basic stats and is_player
    let minimal_json = r#"{
        "match_id": "ME1_387921",
        "champion_name": "Braum",
        "role": "Support",
        "win": true,
        "kills": 2,
        "deaths": 4,
        "assists": 18,
        "cs_per_min": 1.2,
        "gold_diff_15": 200,
        "is_calibrated": false,
        "calibrated_rank": null,
        "lane_opponent_champion": "Nautilus",
        "duration_mins": 28,
        "user_damage_dealt": 8500,
        "user_vision_score": 52,
        "items": [3865, 3109, 3190],
        "allied_team": [
            {
                "champion_name": "Braum",
                "role": "Support",
                "kills": 2,
                "deaths": 4,
                "assists": 18,
                "damage_dealt": 8500,
                "cs": 34,
                "is_player": true
            }
        ],
        "enemy_team": [
            {
                "champion_name": "Nautilus",
                "role": "Support",
                "kills": 1,
                "deaths": 6,
                "assists": 9,
                "damage_dealt": 7200,
                "cs": 28,
                "is_player": false
            }
        ],
        "game_creation_timestamp": 1729860000000,
        "queue_id": 420,
        "queue_name": "Ranked Solo/Duo",
        "played_ago_label": "Yesterday"
    }"#;

    let match_res: Result<AdoptedMatch, _> = serde_json::from_str(minimal_json);
    assert!(match_res.is_ok(), "Failed to deserialize minimal match JSON: {:?}", match_res.err());
    let m = match_res.unwrap();
    assert_eq!(m.champion_name, "Braum");
    let allied = m.allied_team.as_ref().expect("allied_team should be present");
    assert_eq!(allied.len(), 1);
    assert!(allied[0].is_user, "is_player alias should populate is_user");
    assert_eq!(allied[0].summoner_name, "", "summoner_name defaults to empty string");

    // 2. Test enriched format
    let enriched_json = r#"{
        "match_id": "ME1_387922",
        "champion_name": "Shen",
        "role": "Top",
        "win": true,
        "kills": 5,
        "deaths": 1,
        "assists": 12,
        "cs_per_min": 7.8,
        "gold_diff_15": 850,
        "is_calibrated": true,
        "calibrated_rank": 1,
        "lane_opponent_champion": "Aatrox",
        "duration_mins": 25,
        "user_damage_dealt": 19400,
        "user_vision_score": 38,
        "items": [3068, 3075],
        "allied_team": [
            {
                "champion_name": "Shen",
                "role": "Top",
                "summoner_name": "SHENTRICK#SHEN",
                "kills": 5,
                "deaths": 1,
                "assists": 12,
                "damage_dealt": 19400,
                "cs": 195,
                "cs_per_min": 7.8,
                "gold_earned": 11200,
                "damage_share_pct": 24.5,
                "vision_score": 38,
                "is_user": true
            }
        ],
        "enemy_team": [],
        "game_creation_timestamp": 1729864000000,
        "queue_id": 420,
        "queue_name": "Ranked Solo/Duo",
        "played_ago_label": "2 hours ago"
    }"#;

    let match_res2: Result<AdoptedMatch, _> = serde_json::from_str(enriched_json);
    assert!(match_res2.is_ok(), "Failed to deserialize enriched match JSON: {:?}", match_res2.err());
    let m2 = match_res2.unwrap();
    let allied2 = m2.allied_team.as_ref().expect("allied_team should be present");
    assert_eq!(allied2[0].summoner_name, "SHENTRICK#SHEN");
    assert_eq!(allied2[0].gold_earned, 11200);
}

#[test]
fn test_taxonomy_and_meta_detection() {
    use recommender_core::player_analytics::{detect_meta_status, normalize_role_name};

    assert_eq!(normalize_role_name("UTILITY"), "Support");
    assert_eq!(normalize_role_name("BOTTOM"), "Bottom");
    assert_eq!(normalize_role_name("MIDDLE"), "Mid");
    assert_eq!(normalize_role_name("TOP"), "Top");
    assert_eq!(normalize_role_name("JUNGLE"), "Jungle");

    // 1. Braum Support vs Braum Top
    let braum_supp = detect_meta_status("Braum", "Support");
    assert!(braum_supp.is_meta, "Braum Support must be meta");
    assert_eq!(braum_supp.canonical_role, "Support");
    assert!(braum_supp.classification_badge.contains("META"));
    assert!(braum_supp.classification_badge.contains("WARDEN"));

    let braum_top = detect_meta_status("Braum", "Top");
    assert!(!braum_top.is_meta, "Braum Top must be recognized as off-meta");
    assert!(braum_top.classification_badge.contains("OFF-META"));

    // 2. Shen Top vs Shen Support vs Shen Mid
    let shen_top = detect_meta_status("Shen", "Top");
    assert!(shen_top.is_meta, "Shen Top is meta");

    let shen_supp = detect_meta_status("Shen", "Support");
    assert!(shen_supp.is_meta, "Shen Support is a recognized flex pick");

    let shen_mid = detect_meta_status("Shen", "Mid");
    assert!(!shen_mid.is_meta, "Shen Mid is off-meta");

    // 3. Jinx Bot vs Jinx Top
    let jinx_bot = detect_meta_status("Jinx", "Bottom");
    assert!(jinx_bot.is_meta, "Jinx Bot is meta");

    let jinx_top = detect_meta_status("Jinx", "Top");
    assert!(!jinx_top.is_meta, "Jinx Top is off-meta");

    // 4. Lee Sin Jungle vs Lee Sin Support
    let lee_jng = detect_meta_status("Lee Sin", "Jungle");
    assert!(lee_jng.is_meta, "Lee Sin Jungle is meta");

    let lee_supp = detect_meta_status("Lee Sin", "Support");
    assert!(!lee_supp.is_meta, "Lee Sin Support is off-meta");

    // 5. Verify build_champion_curve_analysis attaches meta_status
    let sample_match = recommender_core::player_analytics::AdoptedMatch {
        match_id: "ME1_999".into(),
        champion_name: "Braum".into(),
        role: "Support".into(),
        win: true,
        kills: 1,
        deaths: 2,
        assists: 15,
        cs_per_min: 1.1,
        gold_diff_15: 400,
        is_calibrated: false,
        calibrated_rank: None,
        lane_opponent_champion: Some("Nautilus".into()),
        duration_mins: Some(26),
        user_damage_dealt: Some(7200),
        user_vision_score: Some(58),
        items: Some(vec![3865, 3109]),
        allied_team: None,
        enemy_team: None,
        game_creation_timestamp: Some(1729860000000),
        queue_id: Some(420),
        queue_name: Some("Ranked Solo/Duo".into()),
        played_ago_label: Some("1 hour ago".into()),
    };

    let curve_analysis = sample_match.build_champion_curve_analysis("SHENTRICK");
    assert!(curve_analysis.meta_status.is_meta);
    assert_eq!(curve_analysis.meta_status.canonical_role, "Support");
    assert!(curve_analysis.meta_status.classification_badge.contains("META"));
}

#[test]
fn test_role_tailored_autopsy_generators() {
    let make_match = |champ: &str, role: &str, opp: &str| recommender_core::player_analytics::AdoptedMatch {
        match_id: format!("TEST_{}_{}", champ, role),
        champion_name: champ.into(),
        role: role.into(),
        win: true,
        kills: 3,
        deaths: 2,
        assists: 12,
        cs_per_min: if role == "Support" { 1.2 } else { 7.8 },
        gold_diff_15: 500,
        is_calibrated: false,
        calibrated_rank: None,
        lane_opponent_champion: Some(opp.into()),
        duration_mins: Some(28),
        user_damage_dealt: Some(15000),
        user_vision_score: Some(if role == "Support" { 65 } else { 25 }),
        items: Some(vec![3865, 3109]),
        allied_team: None,
        enemy_team: None,
        game_creation_timestamp: Some(1729860000000),
        queue_id: Some(420),
        queue_name: Some("Ranked Solo/Duo".into()),
        played_ago_label: Some("1 hour ago".into()),
    };

    // 1. Support Autopsy Verification (Braum vs Nautilus)
    let braum_supp = make_match("Braum", "Support", "Nautilus");
    let braum_analysis = braum_supp.build_champion_curve_analysis("TEST_SUMMONER");
    assert!(!braum_analysis.timestamped_moments.is_empty(), "Support autopsy must have timestamped moments");
    
    let has_quest = braum_analysis.timestamped_moments.iter().any(|m| m.event_title.contains("Support Quest"));
    assert!(has_quest, "Support autopsy must include Support Quest Stopwatch milestone");

    let has_45s_dragon = braum_analysis.timestamped_moments.iter().any(|m| m.event_title.contains("45-Second Objective Warning"));
    assert!(has_45s_dragon, "Support autopsy must include 45-Second Objective Warning Rule");

    let has_peel_radius = braum_analysis.timestamped_moments.iter().any(|m| m.event_title.contains("Peel Radius") || m.event_title.contains("Tether"));
    assert!(has_peel_radius, "Support autopsy must evaluate 450-Unit Peel Radius & Carry Tethering");

    // 2. Jungle Autopsy Verification (Lee Sin vs Viego)
    let lee_jng = make_match("Lee Sin", "Jungle", "Viego");
    let lee_analysis = lee_jng.build_champion_curve_analysis("TEST_SUMMONER");
    assert!(!lee_analysis.timestamped_moments.is_empty(), "Jungle autopsy must have timestamped moments");

    let has_buff_start = lee_analysis.timestamped_moments.iter().any(|m| m.event_title.contains("Buff Start"));
    assert!(has_buff_start, "Jungle autopsy must include Level-1 Pathing & Buff Start Selection");

    let has_scuttle = lee_analysis.timestamped_moments.iter().any(|m| m.event_title.contains("Scuttle Crab"));
    assert!(has_scuttle, "Jungle autopsy must include Scuttle Crab Timing");

    let has_smite_audit = lee_analysis.timestamped_moments.iter().any(|m| m.event_title.contains("Smite"));
    assert!(has_smite_audit, "Jungle autopsy must include Smite milestone or 50/50 smite discipline");

    // 3. Bot Autopsy Verification (Jinx vs Caitlyn)
    let jinx_bot = make_match("Jinx", "Bottom", "Caitlyn");
    let jinx_analysis = jinx_bot.build_champion_curve_analysis("TEST_SUMMONER");
    assert!(!jinx_analysis.timestamped_moments.is_empty(), "Bot autopsy must have timestamped moments");
    let has_bot_mechanics = jinx_analysis.timestamped_moments.iter().any(|m| {
        m.event_title.contains("Duo") || m.event_title.contains("ADC") || m.warning_notification.contains("ADC") || m.warning_notification.contains("DUO")
    });
    assert!(has_bot_mechanics, "Bot autopsy must evaluate Bot duo mechanics and ADC positioning");

    // 4. Mid Autopsy Verification (Ahri vs Zed)
    let ahri_mid = make_match("Ahri", "Mid", "Zed");
    let ahri_analysis = ahri_mid.build_champion_curve_analysis("TEST_SUMMONER");
    assert!(!ahri_analysis.timestamped_moments.is_empty(), "Mid autopsy must have timestamped moments");

    // 5. Top Autopsy Verification (Darius vs Garen)
    let darius_top = make_match("Darius", "Top", "Garen");
    let darius_analysis = darius_top.build_champion_curve_analysis("TEST_SUMMONER");
    assert!(!darius_analysis.timestamped_moments.is_empty(), "Top autopsy must have timestamped moments");
}

#[test]
fn test_phase3_dynamic_curves_and_item_keystone_spikes() {
    let make_match = |champ: &str, role: &str, opp: &str| recommender_core::player_analytics::AdoptedMatch {
        match_id: format!("TEST_PH3_{}_{}", champ, role),
        champion_name: champ.into(),
        role: role.into(),
        win: true,
        kills: 2,
        deaths: 1,
        assists: 16,
        cs_per_min: if role == "Support" { 1.2 } else if role == "Jungle" { 5.6 } else { 8.4 },
        gold_diff_15: 450,
        is_calibrated: false,
        calibrated_rank: None,
        lane_opponent_champion: Some(opp.into()),
        duration_mins: Some(27),
        user_damage_dealt: Some(9500),
        user_vision_score: Some(if role == "Support" { 68 } else { 22 }),
        items: Some(vec![3865, 3109]),
        allied_team: None,
        enemy_team: None,
        game_creation_timestamp: Some(1729860000000),
        queue_id: Some(420),
        queue_name: Some("Ranked Solo/Duo".into()),
        played_ago_label: Some("1 hour ago".into()),
    };

    // 1. Support Role: Braum
    let braum = make_match("Braum", "Support", "Nautilus");
    let braum_analysis = braum.build_champion_curve_analysis("TEST_SUMMONER");

    // Dynamic Skill Order for Braum
    assert_eq!(
        braum_analysis.skill_order,
        "Q (Winter's Bite) → E (Unbreakable) → W (Stand Behind Me) · R at 6/11/16",
        "Braum must have his unique skill level priority"
    );

    // Support Item Spikes (Locket, Knight's Vow, World Atlas)
    let has_world_atlas = braum_analysis.item_spikes.iter().any(|s| s.item_name.contains("World Atlas"));
    let has_locket = braum_analysis.item_spikes.iter().any(|s| s.item_name.contains("Locket of the Iron Solari"));
    let has_knights_vow = braum_analysis.item_spikes.iter().any(|s| s.item_name.contains("Knight's Vow"));
    let has_heartsteel = braum_analysis.item_spikes.iter().any(|s| s.item_name.contains("Heartsteel"));

    assert!(has_world_atlas, "Support item spikes must include World Atlas quest item");
    assert!(has_locket, "Support Warden item spikes must include Locket of the Iron Solari");
    assert!(has_knights_vow, "Support Warden item spikes must include Knight's Vow");
    assert!(!has_heartsteel, "Support Warden item spikes must NOT include Top tank items like Heartsteel");

    // Support Keystone & Runes (Guardian, Font of Life)
    let has_guardian = braum_analysis.rune_telemetry.iter().any(|r| r.rune_name.contains("Guardian"));
    let has_font_of_life = braum_analysis.rune_telemetry.iter().any(|r| r.rune_name.contains("Font of Life"));
    assert!(has_guardian, "Braum Support must receive Guardian keystone telemetry");
    assert!(has_font_of_life, "Braum Support must receive Font of Life rune telemetry");

    // Dynamic Support CS Benchmark (1.3 CS/m instead of 7.3/8.1)
    let bench_csm_m10 = braum_analysis.cs_tempo_curve.iter().find(|(m, _, _, _, _)| *m == 10).unwrap().4;
    assert_eq!(bench_csm_m10, 1.3, "Support benchmark CS/min must be calibrated to ~1.3, not solo lane CS");

    // Dynamic Radar Axis: Quest & Support Economy instead of Farming & Wave CS
    let has_support_economy_axis = braum_analysis.radar_axes.iter().any(|(name, score, _, _)| {
        name == "Quest & Support Economy" && *score >= 70.0
    });
    assert!(has_support_economy_axis, "Support radar must feature Quest & Support Economy without CS penalty");

    // Dynamic SHAP Win-Impact Attribution
    let has_support_shap = braum_analysis.shap_attributions.iter().any(|attr| attr.feature_name.contains("Support Quest"));
    assert!(has_support_shap, "SHAP win-impact attribution must evaluate Support Quest & Roam Economy");

    // 2. ADC Role: Jinx
    let jinx = make_match("Jinx", "Bottom", "Caitlyn");
    let jinx_analysis = jinx.build_champion_curve_analysis("TEST_SUMMONER");
    assert!(jinx_analysis.skill_order.contains("Switcheroo"), "Jinx must max Switcheroo (Q)");
    let has_ie = jinx_analysis.item_spikes.iter().any(|s| s.item_name.contains("Infinity Edge"));
    assert!(has_ie, "Jinx item spikes must include Infinity Edge");
    let has_lethal_tempo = jinx_analysis.rune_telemetry.iter().any(|r| r.rune_name.contains("Lethal Tempo"));
    assert!(has_lethal_tempo, "Jinx runes must feature Lethal Tempo / Press the Attack");

    // 3. Jungle Role: Lee Sin
    let lee = make_match("Lee Sin", "Jungle", "Viego");
    let lee_analysis = lee.build_champion_curve_analysis("TEST_SUMMONER");
    assert!(lee_analysis.skill_order.contains("Sonic Wave"), "Lee Sin must max Sonic Wave (Q)");
    let has_pet = lee_analysis.item_spikes.iter().any(|s| s.item_name.contains("Pup"));
    assert!(has_pet, "Jungle item spikes must start with Jungle Pet");
    let bench_csm_jng = lee_analysis.cs_tempo_curve.iter().find(|(m, _, _, _, _)| *m == 10).unwrap().4;
    assert_eq!(bench_csm_jng, 5.8, "Jungle benchmark CS/min must be calibrated to ~5.8");
}

#[test]
fn test_phase4_shap_attribution_and_ml_models() {
    let make_match = |champ: &str, role: &str, opp: &str| recommender_core::player_analytics::AdoptedMatch {
        match_id: format!("TEST_PH4_{}_{}", champ, role),
        champion_name: champ.into(),
        role: role.into(),
        win: true,
        kills: 1,
        deaths: 2,
        assists: 18,
        cs_per_min: if role == "Support" { 1.1 } else { 7.5 },
        gold_diff_15: 350,
        is_calibrated: false,
        calibrated_rank: None,
        lane_opponent_champion: Some(opp.into()),
        duration_mins: Some(29),
        user_damage_dealt: Some(8500),
        user_vision_score: Some(if role == "Support" { 72 } else { 24 }),
        items: Some(vec![3865, 3109]),
        allied_team: None,
        enemy_team: None,
        game_creation_timestamp: Some(1729860000000),
        queue_id: Some(420),
        queue_name: Some("Ranked Solo/Duo".into()),
        played_ago_label: Some("1 hour ago".into()),
    };

    // 1. Support Role: Braum
    let braum = make_match("Braum", "Support", "Nautilus");
    let braum_analysis = braum.build_champion_curve_analysis("TEST_SUMMONER");

    // Dynamic 8-Axis Radar Labels
    let axis_names: Vec<&str> = braum_analysis.radar_axes.iter().map(|(n, _, _, _)| n.as_str()).collect();
    assert!(axis_names.contains(&"Fighting & Carry Peel"), "Support radar must feature Fighting & Carry Peel");
    assert!(axis_names.contains(&"Quest & Support Economy"), "Support radar must feature Quest & Support Economy");
    assert!(axis_names.contains(&"Bot 2v2 Laning @15m"), "Support radar must feature Bot 2v2 Laning @15m");

    // Dynamic SHAP Win-Impact Features
    let shap_names: Vec<&str> = braum_analysis.shap_attributions.iter().map(|s| s.feature_name.as_str()).collect();
    assert!(shap_names.iter().any(|n| n.contains("Bot 2v2 Prio & Quest Lead")), "SHAP must feature Bot 2v2 Prio");
    assert!(shap_names.iter().any(|n| n.contains("Carry Peel & Kill Participation")), "SHAP must feature Carry Peel & KP");
    assert!(shap_names.iter().any(|n| n.contains("Unbreakable Frontline Mitigation")), "SHAP must feature Unbreakable Frontline Mitigation for Braum");
    assert!(shap_names.iter().any(|n| n.contains("CC Lockdown & Skillshot Interception")), "SHAP must feature CC Lockdown & Interception");

    // Single-Game ML Model Validation
    let gbm_model = braum_analysis.ml_models.iter().find(|m| m.model_name.contains("Gradient Boosted")).unwrap();
    assert!(gbm_model.plain_english_meaning.contains("Support Quest progression"), "GBM model must evaluate Support Quest progression");
    assert!(gbm_model.actionable_improvement.contains("World Atlas quest upgrade"), "GBM improvement must guide World Atlas timing");

    // 2. Jungle Role: Lee Sin
    let lee = make_match("Lee Sin", "Jungle", "Viego");
    let lee_analysis = lee.build_champion_curve_analysis("TEST_SUMMONER");

    let jng_axis_names: Vec<&str> = lee_analysis.radar_axes.iter().map(|(n, _, _, _)| n.as_str()).collect();
    assert!(jng_axis_names.contains(&"Jungle Camp Clear & Tempo"), "Jungle radar must feature Jungle Camp Clear & Tempo");

    let jng_gbm = lee_analysis.ml_models.iter().find(|m| m.model_name.contains("Gradient Boosted")).unwrap();
    assert!(jng_gbm.plain_english_meaning.contains("jungle camp clear efficiency"), "Jungle GBM must evaluate camp clear efficiency");
    assert!(jng_gbm.key_alert_notification.contains("PRIMAL SMITE"), "Jungle GBM must highlight Primal Smite leverage");
}






