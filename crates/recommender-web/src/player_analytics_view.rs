//! Personalized Player Analytics & Riot Account Machine Learning Hub.
//!
//! Provides a 5-page Summoner Intelligence & ML Hub:
//! 1. `📊 Overview & Radar`: Executive KPIs, enlarged 5-axis Playstyle Archetype Radar, and Coaching Insights.
//! 2. `🤖 No-Quiz Riot ML Lab`: 80/20 Train/Validation ML pipeline trained on the player's match history + No-Quiz Champion Recommendations.
//! 3. `📜 Complete Game Logs`: Rich, filterable game logs for all matches played on the account with per-game ML grades.
//! 4. `🏆 Played Champions Pool`: Full roster breakdown of every champion played on the account with 1-click Pin & Track.
//! 5. `📌 Pinned Champion Tracker`: Longitudinal progress tracker & champion-specific ML win-condition thresholds.

use crate::state::{AppState, Modal};
use leptos::prelude::*;
use recommender_core::player_analytics::{
    AdoptedMatch, ChampionMatchPoint, MachineLearningInsight, PlayerAnalyticsReport,
};
use std::f64::consts::PI;

#[cfg(target_arch = "wasm32")]
const PINNED_CHAMPIONS_STORAGE_KEY: &str = "lol_rec_pinned_champions_v1";

#[cfg(target_arch = "wasm32")]
fn load_pinned_champions() -> Vec<String> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(PINNED_CHAMPIONS_STORAGE_KEY).ok().flatten())
        .and_then(|raw| serde_json::from_str::<Vec<String>>(&raw).ok())
        .unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
fn save_pinned_champions(pins: &[String]) {
    if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
        if let Ok(json) = serde_json::to_string(pins) {
            let _ = storage.set_item(PINNED_CHAMPIONS_STORAGE_KEY, &json);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn view_player_analytics(state: RwSignal<AppState>) -> impl IntoView {
    let active_tab = RwSignal::new("overview".to_string());
    let pinned_champions = RwSignal::new(load_pinned_champions());
    let selected_tracked_champion = RwSignal::new(String::new());
    let log_role_filter = RwSignal::new("ALL".to_string());
    let log_outcome_filter = RwSignal::new("ALL".to_string());
    let log_champ_filter = RwSignal::new("ALL".to_string());

    let report = move || {
        let user = state.with(|c| c.auth_user.clone());
        let live_matches = state.with(|c| c.live_player_matches.clone());

        if let Some(u) = user {
            let s_name = if !u.full_name.is_empty() && u.full_name != "Mock Summoner" {
                u.full_name
            } else if let Some(ref r) = u.riot_id {
                r.split('#').next().unwrap_or("SHENTRICK").to_string()
            } else {
                "SHENTRICK".to_string()
            };
            let r_id = u.riot_id.unwrap_or_else(|| format!("{}#ME", s_name));
            let reg = u.riot_region.unwrap_or_else(|| "ME".into());
            PlayerAnalyticsReport::for_player(&s_name, &r_id, &reg, live_matches.as_deref())
        } else {
            PlayerAnalyticsReport::for_player(
                "SHENTRICK",
                "SHENTRICK#SHEN",
                "ME",
                live_matches.as_deref(),
            )
        }
    };

    let toggle_pin_champion = move |champ: String| {
        pinned_champions.update(|pins| {
            if let Some(pos) = pins.iter().position(|c| c.eq_ignore_ascii_case(&champ)) {
                pins.remove(pos);
            } else {
                pins.push(champ.clone());
            }
            save_pinned_champions(pins);
        });
        selected_tracked_champion.set(champ);
    };

    view! {
        <section id="player-analytics-dashboard" class="analytics-dashboard active" style="max-width: 1240px; margin: 0 auto; padding: 2rem 1rem;">
            // Top Global Navigation Bar
            <div class="analytics-nav" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem; border-bottom: 1px solid rgba(200, 170, 110, 0.2); padding-bottom: 1rem; flex-wrap: wrap; gap: 0.75rem;">
                <button
                    type="button"
                    class="back-button"
                    style="background: transparent; border: 1px solid rgba(200, 170, 110, 0.4); color: var(--gold); padding: 0.5rem 1rem; cursor: pointer; text-transform: uppercase; font-size: 0.85rem; letter-spacing: 0.05em;"
                    on:click=move |_| {
                        if let Some(win) = web_sys::window() {
                            let _ = win.location().set_hash("");
                        }
                        state.update(AppState::return_to_start);
                    }
                >
                    "← Back to Home"
                </button>

                <div style="display: flex; gap: 0.75rem; flex-wrap: wrap;">
                    <button
                        type="button"
                        class="back-button"
                        style="background: transparent; border: 1px solid rgba(10, 200, 185, 0.4); color: var(--cyan); padding: 0.5rem 1rem; cursor: pointer; text-transform: uppercase; font-size: 0.85rem; letter-spacing: 0.05em;"
                        on:click=move |_| {
                            if let Some(win) = web_sys::window() {
                                let _ = win.location().set_hash("#distribution");
                            }
                            state.update(AppState::open_champion_distribution);
                        }
                    >
                        "✦ Champion Distribution →"
                    </button>
                </div>
            </div>

            // Summoner Identity Header + Integrated 5-Page Navigation Bar
            {move || {
                let rep = report();
                let riot_label = rep.riot_id.clone().unwrap_or_else(|| "No Riot ID Linked".into());
                let region_label = rep
                    .region
                    .as_deref()
                    .and_then(|r| recommender_core::riot_api::RiotRegion::from_code(r).map(|reg| format!("{} · {}", reg.code(), reg.platform_id().to_uppercase())))
                    .unwrap_or_else(|| rep.region.clone().unwrap_or_else(|| "Global".into()));
                let cur_tab = active_tab.get();
                let pin_count = pinned_champions.get().len();

                view! {
                    <div style="background: linear-gradient(180deg, rgba(9, 20, 40, 0.94) 0%, rgba(1, 10, 19, 0.98) 100%); border: 1px solid var(--gold); padding: 1.5rem; margin-bottom: 2rem; position: relative; box-shadow: 0 8px 32px rgba(0,0,0,0.55);">
                        <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem; padding-bottom: 1.25rem; border-bottom: 1px solid rgba(200, 170, 110, 0.2);">
                            <div style="display: flex; align-items: center; gap: 1rem;">
                                <div style="width: 58px; height: 58px; border: 2px solid var(--gold); overflow: hidden; background: #000; flex-shrink: 0;">
                                    <img src="https://ddragon.leagueoflegends.com/cdn/14.20.1/img/profileicon/588.png" alt="" style="width: 100%; height: 100%; object-fit: cover;" />
                                </div>
                                <div>
                                    <div style="display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap;">
                                        <h1 style="margin: 0; font-family: serif; text-transform: uppercase; letter-spacing: 0.1em; color: var(--gold); font-size: 1.6rem;">{rep.summoner_name}</h1>
                                        <span style="background: rgba(10, 200, 185, 0.15); border: 1px solid var(--cyan); color: var(--cyan); font-size: 0.75rem; padding: 0.15rem 0.5rem; text-transform: uppercase; font-family: monospace;">{region_label}</span>
                                        {if state.with(|c| c.riot_data_source.as_deref() == Some("live")) {
                                            view! { <span style="background: rgba(16, 185, 129, 0.2); border: 1px solid #10b981; color: #10b981; font-size: 0.75rem; padding: 0.15rem 0.5rem; text-transform: uppercase; font-family: monospace; font-weight: bold;">"● OFFICIAL RIOT API (LIVE)"</span> }.into_any()
                                        } else {
                                            view! { <span style="background: rgba(10, 200, 185, 0.15); border: 1px solid var(--cyan); color: var(--cyan); font-size: 0.75rem; padding: 0.15rem 0.5rem; text-transform: uppercase; font-family: monospace;">"● RIOT TELEMETRY (LINKED)"</span> }.into_any()
                                        }}
                                    </div>
                                    <p style="margin: 0.3rem 0 0 0; font-family: monospace; color: var(--text-muted); font-size: 0.84rem;">
                                        "Riot Account: " <strong style="color: var(--gold);">{riot_label}</strong>
                                        " • " <span style="color: var(--cyan);">{format!("{} Matches & {} Champions Analyzed", rep.total_analyzed_matches, rep.champion_performances.len())}</span>
                                    </p>
                                </div>
                            </div>

                            // Riot Account Controls (No Demo Button)
                            <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center;">
                                <button
                                    type="button"
                                    style="background: rgba(10, 200, 185, 0.2); border: 1px solid var(--cyan); color: #fff; padding: 0.55rem 1.05rem; cursor: pointer; font-size: 0.8rem; text-transform: uppercase; font-family: monospace; font-weight: bold; display: flex; align-items: center; gap: 6px;"
                                    disabled=move || state.with(|c| c.is_syncing_riot)
                                    on:click=move |_| {
                                        state.update(|c| c.set_syncing_riot(true));
                                        let rep = report();
                                        let r_id = rep.riot_id.unwrap_or_else(|| "SHENTRICK#SHEN".into());
                                        let reg = rep.region.unwrap_or_else(|| "ME".into());
                                        crate::storage::supabase_fetch_riot_telemetry(&r_id, &reg);
                                        if let Some(matches) = crate::storage::load_cached_riot_matches() {
                                            let src = crate::storage::load_cached_riot_data_source();
                                            state.update(|c| c.set_live_matches(Some(matches), src));
                                        }
                                    }
                                >
                                    {move || if state.with(|c| c.is_syncing_riot) {
                                        "⚡ Syncing Riot API..."
                                    } else {
                                        "↻ Sync Live Games"
                                    }}
                                </button>
                                <button
                                    type="button"
                                    style="background: rgba(200, 170, 110, 0.12); border: 1px solid var(--gold); color: var(--gold); padding: 0.55rem 1.05rem; cursor: pointer; font-size: 0.8rem; text-transform: uppercase; font-family: monospace; font-weight: bold;"
                                    on:click=move |_| state.update(|c| c.open_modal(Modal::LinkRiotAccount))
                                >
                                    "🔗 Link / Change Riot ID"
                                </button>
                            </div>
                        </div>

                        // 5-Page Sub-Navigation Bar directly beneath/alongside Riot Controls
                        <div class="analytics-subnav-bar" style="display: flex; gap: 0.65rem; flex-wrap: wrap; padding-top: 1.1rem; align-items: center;">
                            <span style="font-family: monospace; font-size: 0.72rem; text-transform: uppercase; color: var(--text-muted); letter-spacing: 0.08em; margin-right: 0.25rem;">
                                "TELEMETRY PAGES:"
                            </span>
                            {[
                                ("overview", "📊 Overview & Radar"),
                                ("ml_lab", "🤖 No-Quiz Riot ML Lab"),
                                ("game_logs", "📜 Complete Game Logs"),
                                ("champions_pool", "🏆 Played Champions Pool"),
                                ("pinned_tracker", "📌 Pinned Champion Tracker"),
                            ].into_iter().map(|(tab_id, tab_label)| {
                                let is_active = cur_tab == tab_id;
                                let bg = if is_active {
                                    "linear-gradient(135deg, rgba(10, 200, 185, 0.26), rgba(200, 170, 110, 0.22))"
                                } else {
                                    "rgba(5, 14, 28, 0.75)"
                                };
                                let border = if is_active { "var(--cyan)" } else { "rgba(200, 170, 110, 0.3)" };
                                let color = if is_active { "#fff" } else { "var(--gold)" };
                                let id_str = tab_id.to_string();
                                let label_with_count = if tab_id == "pinned_tracker" && pin_count > 0 {
                                    format!("{} ({})", tab_label, pin_count)
                                } else {
                                    tab_label.to_string()
                                };
                                view! {
                                    <button
                                        type="button"
                                        class="analytics-subnav-btn"
                                        style=format!("background: {}; border: 1px solid {}; color: {}; padding: 0.5rem 0.95rem; cursor: pointer; font-family: monospace; font-size: 0.78rem; text-transform: uppercase; font-weight: bold; letter-spacing: 0.04em; transition: all 0.18s ease;", bg, border, color)
                                        on:click=move |_| active_tab.set(id_str.clone())
                                    >
                                        {label_with_count}
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                }
            }}

            // Multi-Page Content Router
            {move || {
                let rep = report();
                match active_tab.get().as_str() {
                    "ml_lab" => view_page_no_quiz_ml_lab(rep, active_tab, selected_tracked_champion, toggle_pin_champion).into_any(),
                    "game_logs" => view_page_complete_game_logs(rep, log_role_filter, log_outcome_filter, log_champ_filter, active_tab, selected_tracked_champion).into_any(),
                    "champions_pool" => view_page_champions_pool(rep, pinned_champions, active_tab, selected_tracked_champion, toggle_pin_champion).into_any(),
                    "pinned_tracker" => view_page_pinned_champion_tracker(rep, pinned_champions, selected_tracked_champion, toggle_pin_champion).into_any(),
                    _ => view_page_overview_and_radar(rep, active_tab, selected_tracked_champion).into_any(),
                }
            }}
        </section>
    }
}

// ============================================================================
// PAGE 1: OVERVIEW & PLAYSTYLE ARCHETYPE RADAR
// ============================================================================
#[cfg(target_arch = "wasm32")]
fn view_page_overview_and_radar(
    rep: PlayerAnalyticsReport,
    active_tab: RwSignal<String>,
    selected_tracked_champion: RwSignal<String>,
) -> impl IntoView {
    let eff = rep.efficacy.clone();
    let evo = rep.evolution.clone();
    let anchor = evo
        .anchor_champions
        .first()
        .cloned()
        .unwrap_or_else(|| "Shen".into());
    let val_acc = rep.ml_evaluation.val_accuracy;

    view! {
        <div>
            // 4 KPI Highlights Cards
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 1.25rem; margin-bottom: 2.25rem;">
                <div style="background: rgba(9, 20, 40, 0.85); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--gold); padding: 1.25rem;">
                    <div style="font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.1em; color: var(--text-muted); margin-bottom: 0.5rem;">"Calibrated Win Rate"</div>
                    <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                        <span style="font-size: 2.2rem; font-weight: 800; color: var(--gold); font-family: monospace;">{format!("{:.1}%", eff.calibrated_win_rate)}</span>
                        <span style="font-size: 0.85rem; color: var(--cyan); font-family: monospace;">{format!("+{:.1}% LIFT", eff.win_rate_lift)}</span>
                    </div>
                    <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.5rem;">
                        {format!("{} Wins in {} Calibrated Matches", eff.calibrated_wins, eff.calibrated_games)}
                    </div>
                </div>

                <div style="background: rgba(9, 20, 40, 0.85); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--cyan); padding: 1.25rem;">
                    <div style="font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.1em; color: var(--text-muted); margin-bottom: 0.5rem;">"Played Champions Pool"</div>
                    <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                        <span style="font-size: 2.2rem; font-weight: 800; color: var(--cyan); font-family: monospace;">{format!("{}", rep.champion_performances.len())}</span>
                        <span style="font-size: 0.85rem; color: var(--text-muted); font-family: monospace;">"ACTIVE CHAMPS"</span>
                    </div>
                    <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.5rem;">
                        {format!("Across {} verified account games", rep.total_analyzed_matches)}
                    </div>
                </div>

                <div style="background: rgba(9, 20, 40, 0.85); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--gold); padding: 1.25rem;">
                    <div style="font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.1em; color: var(--text-muted); margin-bottom: 0.5rem;">"Primary Anchor Pick"</div>
                    <div style="display: flex; align-items: baseline; justify-content: space-between; gap: 0.5rem;">
                        <span style="font-size: 2.0rem; font-weight: 800; color: #fff; font-family: serif; text-transform: uppercase;">{anchor.clone()}</span>
                        <button
                            type="button"
                            style="background: rgba(10, 200, 185, 0.15); border: 1px solid var(--cyan); color: var(--cyan); font-size: 0.7rem; padding: 0.25rem 0.55rem; cursor: pointer; font-family: monospace; text-transform: uppercase;"
                            on:click=move |_| {
                                selected_tracked_champion.set(anchor.clone());
                                active_tab.set("pinned_tracker".into());
                            }
                        >
                            "📌 Track ML →"
                        </button>
                    </div>
                    <div style="font-size: 0.75rem; color: var(--gold); margin-top: 0.5rem;">
                        "✦ Highest Empirical Mastery Anchor"
                    </div>
                </div>

                <div style="background: rgba(9, 20, 40, 0.85); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--cyan); padding: 1.25rem;">
                    <div style="font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.1em; color: var(--text-muted); margin-bottom: 0.5rem;">"ML Validation Accuracy"</div>
                    <div style="display: flex; align-items: baseline; justify-content: space-between; gap: 0.5rem;">
                        <span style="font-size: 2.2rem; font-weight: 800; color: #0ac8b9; font-family: monospace;">{format!("{:.1}%", val_acc)}</span>
                        <button
                            type="button"
                            style="background: rgba(200, 170, 110, 0.15); border: 1px solid var(--gold); color: var(--gold); font-size: 0.7rem; padding: 0.25rem 0.55rem; cursor: pointer; font-family: monospace; text-transform: uppercase;"
                            on:click=move |_| active_tab.set("ml_lab".into())
                        >
                            "🤖 Open ML Lab →"
                        </button>
                    </div>
                    <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.5rem;">
                        "80/20 Holdout Validation on Account Games"
                    </div>
                </div>
            </div>

            // Enlarged Playstyle Archetype Radar & Longitudinal Telemetry
            <div style="margin-bottom: 2.75rem;">
                <div style="display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 1rem; border-bottom: 1px solid rgba(200, 170, 110, 0.3); padding-bottom: 0.5rem;">
                    <h2 style="font-family: serif; text-transform: uppercase; letter-spacing: 0.1em; color: var(--gold); font-size: 1.3rem; margin: 0;">
                        "1. Playstyle Archetype Radar & Empirical Drift"
                    </h2>
                    <span style="font-family: monospace; font-size: 0.8rem; color: var(--gold);">"5-AXIS EMPIRICAL TELEMETRY"</span>
                </div>

                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(400px, 1fr)); gap: 1.5rem; align-items: stretch;">
                    // Fixed & Enlarged Radar Chart Card
                    <div style="background: rgba(9, 20, 40, 0.85); border: 1px solid rgba(200, 170, 110, 0.35); padding: 1.75rem; text-align: center; display: flex; flex-direction: column; justify-content: space-between;">
                        <h3 style="font-family: serif; text-transform: uppercase; font-size: 1.1rem; color: var(--gold); margin: 0 0 0.75rem 0; letter-spacing: 0.06em;">
                            "Playstyle Archetype Radar"
                        </h3>
                        {view_radar_svg(evo.current_radar, evo.previous_radar)}
                        <div style="display: flex; justify-content: center; gap: 1.75rem; margin-top: 1rem; font-family: monospace; font-size: 0.8rem;">
                            <span style="color: var(--gold); font-weight: bold;">"✦ Current Calibration"</span>
                            <span style="color: var(--cyan);">"✦ Previous Calibration"</span>
                        </div>
                        <p style="font-size: 0.82rem; color: var(--text-muted); margin: 0.85rem 0 0 0; line-height: 1.45;">
                            {evo.primary_drift_summary.clone()}
                        </p>
                    </div>

                    // Timeline & Anchor Champions
                    <div style="display: flex; flex-direction: column; gap: 1.15rem; justify-content: space-between;">
                        <div style="background: rgba(9, 20, 40, 0.85); border: 1px solid rgba(200, 170, 110, 0.3); padding: 1.35rem;">
                            <h4 style="font-family: serif; text-transform: uppercase; font-size: 0.95rem; color: var(--gold); margin: 0 0 0.75rem 0;">"Identified Anchor Champions"</h4>
                            <div style="display: flex; gap: 0.6rem; flex-wrap: wrap;">
                                {evo.anchor_champions.iter().map(|name| {
                                    let n_click = name.clone();
                                    view! {
                                        <button
                                            type="button"
                                            style="background: rgba(200, 170, 110, 0.15); border: 1px solid var(--gold); color: #fff; padding: 0.4rem 0.85rem; font-family: serif; text-transform: uppercase; font-size: 0.85rem; cursor: pointer;"
                                            on:click=move |_| {
                                                selected_tracked_champion.set(n_click.clone());
                                                active_tab.set("pinned_tracker".into());
                                            }
                                        >
                                            "✦ " {name.clone()} " (Inspect ML →)"
                                        </button>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                            <p style="font-size: 0.78rem; color: var(--text-muted); margin: 0.75rem 0 0 0; line-height: 1.45;">
                                "Click any anchor champion above to open its dedicated ML Win-Condition & Longitudinal Progress Tracker."
                            </p>
                        </div>

                        <div style="background: rgba(9, 20, 40, 0.85); border: 1px solid rgba(200, 170, 110, 0.3); padding: 1.35rem; flex: 1;">
                            <h4 style="font-family: serif; text-transform: uppercase; font-size: 0.95rem; color: var(--cyan); margin: 0 0 0.75rem 0;">"Past Calibration Snapshots"</h4>
                            <div style="display: flex; flex-direction: column; gap: 0.85rem;">
                                {evo.history.iter().map(|rec| {
                                    view! {
                                        <div style="border-left: 2px solid var(--gold); padding-left: 0.85rem;">
                                            <div style="display: flex; justify-content: space-between; font-size: 0.82rem; font-family: monospace;">
                                                <strong style="color: var(--gold);">{rec.date_label.clone()}</strong>
                                                <span style="color: var(--cyan);">{rec.role_focus.clone()}</span>
                                            </div>
                                            <div style="font-size: 0.78rem; color: var(--text-muted); margin-top: 0.25rem;">
                                                "Top Picks: " {rec.top_champions.join(", ")}
                                            </div>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            // Hextech Oracle (Machine Learning Coaching Cards)
            <div style="margin-bottom: 2rem;">
                <div style="display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 1rem; border-bottom: 1px solid rgba(200, 170, 110, 0.3); padding-bottom: 0.5rem;">
                    <h2 style="font-family: serif; text-transform: uppercase; letter-spacing: 0.1em; color: var(--gold); font-size: 1.3rem; margin: 0;">
                        "2. Hextech Oracle (Machine Learning Coaching)"
                    </h2>
                    <span style="font-family: monospace; font-size: 0.8rem; color: var(--cyan);">"PREDICTIVE TELEMETRY MODELS"</span>
                </div>

                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 1.25rem;">
                    {rep.ml_insights.into_iter().map(view_ml_insight_card).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// PAGE 2: NO-QUIZ RIOT ML LAB (TRAINING & VALIDATION + RECOMMENDATIONS)
// ============================================================================
#[cfg(target_arch = "wasm32")]
fn view_page_no_quiz_ml_lab<F>(
    rep: PlayerAnalyticsReport,
    active_tab: RwSignal<String>,
    selected_tracked_champion: RwSignal<String>,
    toggle_pin_champion: F,
) -> impl IntoView
where
    F: Fn(String) + Copy + Send + Sync + 'static,
{
    let eval = rep.ml_evaluation.clone();
    let [tp, fp, tn, fn_cnt] = eval.confusion_matrix;

    view! {
        <div>
            // Banner explaining No-Quiz Account ML Pipeline
            <div style="background: linear-gradient(90deg, rgba(10, 200, 185, 0.14), rgba(200, 170, 110, 0.12)); border: 1px solid var(--cyan); padding: 1.25rem 1.5rem; margin-bottom: 2rem; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem;">
                <div>
                    <div style="font-family: monospace; font-size: 0.75rem; color: var(--cyan); text-transform: uppercase; letter-spacing: 0.08em;">
                        "⚡ ZERO-QUIZ EMPIRICAL MACHINE LEARNING PIPELINE"
                    </div>
                    <h2 style="font-family: serif; text-transform: uppercase; color: var(--gold); font-size: 1.35rem; margin: 0.25rem 0;">
                        "Trained & Validated Directly on Your Riot Account Match History"
                    </h2>
                    <p style="margin: 0; font-size: 0.85rem; color: #ccc; max-width: 820px;">
                        "Skipped the quiz? Our in-browser Logistic Win-Probability & Feature Attribution Engine automatically splits your synced Riot games into an 80% Training Set and 20% Holdout Validation Set to learn what drives your victories and recommend high-synergy champions."
                    </p>
                </div>
                <div style="font-family: monospace; text-align: right;">
                    <span style="background: rgba(16, 185, 129, 0.2); border: 1px solid #10b981; color: #10b981; padding: 0.35rem 0.75rem; font-size: 0.8rem; font-weight: bold;">
                        {format!("✓ CONVERGED ({} TRAIN / {} VAL GAMES)", eval.train_samples, eval.val_samples)}
                    </span>
                </div>
            </div>

            // Train & Validation Metrics Grid
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 1.15rem; margin-bottom: 2rem;">
                <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--gold); padding: 1.15rem;">
                    <div style="font-size: 0.72rem; color: var(--text-muted); text-transform: uppercase; font-family: monospace;">"Training Split Accuracy (80%)"</div>
                    <div style="font-size: 2.0rem; font-weight: 800; color: var(--gold); font-family: monospace; margin-top: 0.35rem;">
                        {format!("{:.1}%", eval.train_accuracy)}
                    </div>
                    <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;">
                        {format!("Evaluated on {} training games", eval.train_samples)}
                    </div>
                </div>

                <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(10, 200, 185, 0.4); border-top: 3px solid var(--cyan); padding: 1.15rem;">
                    <div style="font-size: 0.72rem; color: var(--text-muted); text-transform: uppercase; font-family: monospace;">"Holdout Validation Accuracy (20%)"</div>
                    <div style="font-size: 2.0rem; font-weight: 800; color: var(--cyan); font-family: monospace; margin-top: 0.35rem;">
                        {format!("{:.1}%", eval.val_accuracy)}
                    </div>
                    <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;">
                        {format!("Out-of-sample generalization ({} val games)", eval.val_samples)}
                    </div>
                </div>

                <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid #10b981; padding: 1.15rem;">
                    <div style="font-size: 0.72rem; color: var(--text-muted); text-transform: uppercase; font-family: monospace;">"Binary Cross-Entropy (Log-Loss)"</div>
                    <div style="font-size: 2.0rem; font-weight: 800; color: #10b981; font-family: monospace; margin-top: 0.35rem;">
                        {format!("{:.3}", eval.log_loss)}
                    </div>
                    <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;">
                        "Baseline random guess = 0.693"
                    </div>
                </div>

                <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--gold); padding: 1.15rem;">
                    <div style="font-size: 0.72rem; color: var(--text-muted); text-transform: uppercase; font-family: monospace;">"Calibration Brier Score"</div>
                    <div style="font-size: 2.0rem; font-weight: 800; color: #fff; font-family: monospace; margin-top: 0.35rem;">
                        {format!("{:.3}", eval.brier_score)}
                    </div>
                    <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;">
                        "Mean squared probabilistic error (lower is better)"
                    </div>
                </div>
            </div>

            // Feature Importance Weights & Confusion Matrix / Loss Curve
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(380px, 1fr)); gap: 1.5rem; margin-bottom: 2.5rem;">
                // Left: Learned Feature Importances
                <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(200, 170, 110, 0.3); padding: 1.5rem;">
                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                        <h3 style="font-family: serif; text-transform: uppercase; color: var(--gold); font-size: 1.05rem; margin: 0;">
                            "Learned Win-Condition Feature Weights"
                        </h3>
                        <span style="font-family: monospace; font-size: 0.72rem; color: var(--cyan);">"SHAP / GRADIENT ATTRIBUTION"</span>
                    </div>
                    <div style="display: flex; flex-direction: column; gap: 1rem;">
                        {eval.feature_importances.into_iter().map(|(feat_name, weight_pct, note)| {
                            view! {
                                <div>
                                    <div style="display: flex; justify-content: space-between; font-family: monospace; font-size: 0.82rem; margin-bottom: 0.3rem;">
                                        <strong style="color: #fff;">{feat_name}</strong>
                                        <span style="color: var(--cyan); font-weight: bold;">{format!("{:.1}% Impact", weight_pct)}</span>
                                    </div>
                                    <div style="width: 100%; height: 10px; background: rgba(255,255,255,0.06); border: 1px solid rgba(200, 170, 110, 0.25); overflow: hidden; margin-bottom: 0.3rem;">
                                        <div style=format!("width: {}%; height: 100%; background: linear-gradient(90deg, var(--gold), var(--cyan));", (weight_pct * 2.2).clamp(10.0, 100.0))></div>
                                    </div>
                                    <div style="font-size: 0.75rem; color: var(--text-muted);">{note}</div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                // Right: Confusion Matrix & Gradient Descent Loss Convergence
                <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(200, 170, 110, 0.3); padding: 1.5rem; display: flex; flex-direction: column; justify-content: space-between; gap: 1.25rem;">
                    <div>
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.85rem;">
                            <h3 style="font-family: serif; text-transform: uppercase; color: var(--gold); font-size: 1.05rem; margin: 0;">
                                "Validation Confusion Matrix"
                            </h3>
                            <span style="font-family: monospace; font-size: 0.72rem; color: var(--gold);">"THRESHOLD P(WIN) ≥ 0.50"</span>
                        </div>
                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 0.75rem; font-family: monospace;">
                            <div style="background: rgba(16, 185, 129, 0.12); border: 1px solid #10b981; padding: 0.75rem; text-align: center;">
                                <div style="font-size: 0.7rem; color: #10b981;">"TRUE POSITIVES (Predicted Win ✓)"</div>
                                <div style="font-size: 1.5rem; font-weight: bold; color: #fff; margin-top: 0.2rem;">{tp}</div>
                            </div>
                            <div style="background: rgba(232, 64, 87, 0.12); border: 1px solid #e84057; padding: 0.75rem; text-align: center;">
                                <div style="font-size: 0.7rem; color: #e84057;">"FALSE POSITIVES (Throw Risk)"</div>
                                <div style="font-size: 1.5rem; font-weight: bold; color: #fff; margin-top: 0.2rem;">{fp}</div>
                            </div>
                            <div style="background: rgba(232, 64, 87, 0.12); border: 1px solid #e84057; padding: 0.75rem; text-align: center;">
                                <div style="font-size: 0.7rem; color: #e84057;">"FALSE NEGATIVES (Comeback Win)"</div>
                                <div style="font-size: 1.5rem; font-weight: bold; color: #fff; margin-top: 0.2rem;">{fn_cnt}</div>
                            </div>
                            <div style="background: rgba(10, 200, 185, 0.12); border: 1px solid var(--cyan); padding: 0.75rem; text-align: center;">
                                <div style="font-size: 0.7rem; color: var(--cyan);">"TRUE NEGATIVES (Predicted Loss ✓)"</div>
                                <div style="font-size: 1.5rem; font-weight: bold; color: #fff; margin-top: 0.2rem;">{tn}</div>
                            </div>
                        </div>
                    </div>

                    <div>
                        <div style="font-family: monospace; font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase; margin-bottom: 0.5rem;">
                            "8-Epoch Gradient Descent Log-Loss Convergence:"
                        </div>
                        <div style="display: flex; align-items: flex-end; gap: 0.45rem; height: 64px; padding: 0.5rem; background: rgba(1, 10, 19, 0.7); border: 1px solid rgba(200, 170, 110, 0.2);">
                            {eval.epoch_loss_curve.into_iter().enumerate().map(|(ep, loss)| {
                                let bar_h = ((loss / 0.75) * 100.0).clamp(18.0, 100.0);
                                view! {
                                    <div style="flex: 1; display: flex; flex-direction: column; align-items: center; gap: 3px;">
                                        <span style="font-family: monospace; font-size: 0.62rem; color: var(--cyan);">{format!("{:.2}", loss)}</span>
                                        <div style=format!("width: 100%; height: {:.0}px; background: linear-gradient(180deg, var(--cyan), rgba(200,170,110,0.4));", bar_h * 0.42)></div>
                                        <span style="font-family: monospace; font-size: 0.6rem; color: var(--text-muted);">{format!("E{}", ep + 1)}</span>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                </div>
            </div>

            // Account-Trained Champion Recommendations (No Quiz Required)
            <div>
                <div style="display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 1rem; border-bottom: 1px solid rgba(200, 170, 110, 0.3); padding-bottom: 0.5rem;">
                    <h2 style="font-family: serif; text-transform: uppercase; letter-spacing: 0.1em; color: var(--gold); font-size: 1.3rem; margin: 0;">
                        "Account-Trained Champion Recommendations (No Quiz Required)"
                    </h2>
                    <span style="font-family: monospace; font-size: 0.8rem; color: var(--cyan);">"LEARNED FROM RIOT MATCH TELEMETRY"</span>
                </div>

                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(330px, 1fr)); gap: 1.25rem;">
                    {eval.no_quiz_recommendations.into_iter().map(|rec| {
                        let c_name = rec.champion_name.clone();
                        let c_pin = rec.champion_name.clone();
                        view! {
                            <div style="background: rgba(9, 20, 40, 0.9); border: 1px solid rgba(200, 170, 110, 0.35); border-top: 3px solid var(--cyan); padding: 1.25rem; display: flex; flex-direction: column; justify-content: space-between; gap: 0.85rem;">
                                <div>
                                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
                                        <span style="font-family: monospace; font-size: 0.72rem; color: var(--cyan); background: rgba(10, 200, 185, 0.14); border: 1px solid var(--cyan); padding: 0.12rem 0.45rem;">
                                            {rec.status_tag}
                                        </span>
                                        <span style="font-family: monospace; font-size: 0.8rem; color: var(--gold); font-weight: bold;">
                                            {format!("{:.1}% Fit", rec.archetype_fit_pct)}
                                        </span>
                                    </div>
                                    <div style="display: flex; justify-content: space-between; align-items: baseline;">
                                        <h3 style="font-family: serif; text-transform: uppercase; color: #fff; font-size: 1.25rem; margin: 0;">
                                            {rec.champion_name}
                                        </h3>
                                        <span style="font-family: monospace; font-size: 0.8rem; color: var(--text-muted);">
                                            "Role: " <strong style="color: var(--gold);">{rec.role}</strong>
                                        </span>
                                    </div>
                                    <p style="font-size: 0.82rem; color: #ccc; margin: 0.65rem 0 0 0; line-height: 1.45;">
                                        {rec.rationale}
                                    </p>
                                </div>

                                <div style="display: flex; justify-content: space-between; align-items: center; border-top: 1px solid rgba(255,255,255,0.08); padding-top: 0.75rem;">
                                    <span style="font-family: monospace; font-size: 0.8rem; color: #10b981; font-weight: bold;">
                                        {format!("Projected WR: {:.1}%", rec.predicted_win_rate)}
                                    </span>
                                    <button
                                        type="button"
                                        style="background: rgba(200, 170, 110, 0.16); border: 1px solid var(--gold); color: var(--gold); padding: 0.35rem 0.7rem; font-family: monospace; font-size: 0.74rem; text-transform: uppercase; cursor: pointer;"
                                        on:click=move |_| {
                                            toggle_pin_champion(c_pin.clone());
                                            selected_tracked_champion.set(c_name.clone());
                                            active_tab.set("pinned_tracker".into());
                                        }
                                    >
                                        "📌 Pin & Track Progress →"
                                    </button>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

// ============================================================================
// PAGE 3: COMPLETE GAME LOGS (FILTERABLE RICH TELEMETRY)
// ============================================================================
#[cfg(target_arch = "wasm32")]
fn view_page_complete_game_logs(
    rep: PlayerAnalyticsReport,
    log_role_filter: RwSignal<String>,
    log_outcome_filter: RwSignal<String>,
    log_champ_filter: RwSignal<String>,
    active_tab: RwSignal<String>,
    selected_tracked_champion: RwSignal<String>,
) -> impl IntoView {
    let all_matches = rep.recent_matches.clone();
    let unique_champs: Vec<String> = rep
        .champion_performances
        .iter()
        .map(|c| c.champion_name.clone())
        .collect();

    view! {
        <div>
            <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem; margin-bottom: 1.25rem; border-bottom: 1px solid rgba(200, 170, 110, 0.3); padding-bottom: 0.75rem;">
                <div>
                    <h2 style="font-family: serif; text-transform: uppercase; letter-spacing: 0.1em; color: var(--gold); font-size: 1.3rem; margin: 0;">
                        "Complete Riot Account Game Logs"
                    </h2>
                    <p style="margin: 0.25rem 0 0 0; font-family: monospace; font-size: 0.8rem; color: var(--text-muted);">
                        "Full per-match telemetry with ML Win-Probability & Performance Grades"
                    </p>
                </div>

                // Filter Controls
                <div style="display: flex; gap: 0.65rem; flex-wrap: wrap; align-items: center; font-family: monospace; font-size: 0.78rem;">
                    // Outcome Filter
                    <select
                        style="background: rgba(9, 20, 40, 0.95); border: 1px solid var(--gold); color: var(--gold); padding: 0.4rem 0.65rem; font-family: monospace; font-size: 0.78rem;"
                        on:change=move |ev| log_outcome_filter.set(event_target_value(&ev))
                    >
                        <option value="ALL" selected=move || log_outcome_filter.get() == "ALL">"All Outcomes"</option>
                        <option value="WIN" selected=move || log_outcome_filter.get() == "WIN">"Victories Only"</option>
                        <option value="LOSS" selected=move || log_outcome_filter.get() == "LOSS">"Defeats Only"</option>
                    </select>

                    // Role Filter
                    <select
                        style="background: rgba(9, 20, 40, 0.95); border: 1px solid var(--cyan); color: var(--cyan); padding: 0.4rem 0.65rem; font-family: monospace; font-size: 0.78rem;"
                        on:change=move |ev| log_role_filter.set(event_target_value(&ev))
                    >
                        <option value="ALL" selected=move || log_role_filter.get() == "ALL">"All Roles"</option>
                        <option value="Top" selected=move || log_role_filter.get() == "Top">"Top Lane"</option>
                        <option value="Jungle" selected=move || log_role_filter.get() == "Jungle">"Jungle"</option>
                        <option value="Mid" selected=move || log_role_filter.get() == "Mid">"Mid Lane"</option>
                        <option value="Bot" selected=move || log_role_filter.get() == "Bot">"Bot Carry"</option>
                        <option value="Support" selected=move || log_role_filter.get() == "Support">"Support"</option>
                    </select>

                    // Champion Filter
                    <select
                        style="background: rgba(9, 20, 40, 0.95); border: 1px solid rgba(200, 170, 110, 0.5); color: #fff; padding: 0.4rem 0.65rem; font-family: monospace; font-size: 0.78rem;"
                        on:change=move |ev| log_champ_filter.set(event_target_value(&ev))
                    >
                        <option value="ALL" selected=move || log_champ_filter.get() == "ALL">"All Played Champions"</option>
                        {unique_champs.into_iter().map(|c| {
                            let c_val = c.clone();
                            let c_label = c.clone();
                            view! {
                                <option value=c_val selected=move || log_champ_filter.get() == c>{c_label}</option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
            </div>

            // Filtered Game Log Rows
            <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                {move || {
                    let role_f = log_role_filter.get();
                    let out_f = log_outcome_filter.get();
                    let champ_f = log_champ_filter.get();

                    let filtered: Vec<AdoptedMatch> = all_matches
                        .iter()
                        .filter(|m| {
                            let role_ok = role_f == "ALL" || m.role.eq_ignore_ascii_case(&role_f);
                            let out_ok = match out_f.as_str() {
                                "WIN" => m.win,
                                "LOSS" => !m.win,
                                _ => true,
                            };
                            let champ_ok = champ_f == "ALL" || m.champion_name.eq_ignore_ascii_case(&champ_f);
                            role_ok && out_ok && champ_ok
                        })
                        .cloned()
                        .collect();

                    if filtered.is_empty() {
                        view! {
                            <div style="background: rgba(9, 20, 40, 0.7); border: 1px solid rgba(200, 170, 110, 0.25); padding: 2rem; text-align: center; font-family: monospace; color: var(--text-muted);">
                                "No matches match the selected filters. Try resetting Role or Champion filters to 'All'."
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                                {filtered.into_iter().map(|m| view_rich_match_row(m, active_tab, selected_tracked_champion)).collect::<Vec<_>>()}
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </div>
    }
}

// ============================================================================
// PAGE 4: PLAYED CHAMPIONS POOL (ALL CHAMPIONS + 1-CLICK PINNING)
// ============================================================================
#[cfg(target_arch = "wasm32")]
fn view_page_champions_pool<F>(
    rep: PlayerAnalyticsReport,
    pinned_champions: RwSignal<Vec<String>>,
    active_tab: RwSignal<String>,
    selected_tracked_champion: RwSignal<String>,
    toggle_pin_champion: F,
) -> impl IntoView
where
    F: Fn(String) + Copy + Send + Sync + 'static,
{
    let perfs = rep.champion_performances.clone();
    let hit_rate = rep.recommendation_hit_rate;
    let perf_delta = rep.calibrated_perf_delta;

    view! {
        <div>
            <div style="display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 1rem; border-bottom: 1px solid rgba(200, 170, 110, 0.3); padding-bottom: 0.5rem; flex-wrap: wrap; gap: 0.5rem;">
                <div>
                    <h2 style="font-family: serif; text-transform: uppercase; letter-spacing: 0.1em; color: var(--gold); font-size: 1.3rem; margin: 0;">
                        "Played Champions Pool & Bayesian Mastery Index"
                    </h2>
                    <p style="margin: 0.25rem 0 0 0; font-family: monospace; font-size: 0.8rem; color: var(--text-muted);">
                        "Every champion collected from your Riot account games. Click 📌 Pin to track ongoing ML progress."
                    </p>
                </div>
                <div style="display: flex; gap: 1.25rem; font-family: monospace; font-size: 0.82rem;">
                    <span>
                        <span style="color: var(--text-muted);">"Hit Rate: "</span>
                        <strong style="color: var(--cyan);">{format!("{:.1}%", hit_rate)}</strong>
                    </span>
                    <span>
                        <span style="color: var(--text-muted);">"Calibrated Lift: "</span>
                        <strong style="color: var(--gold);">{format!("+{:.1} Index Pts", perf_delta)}</strong>
                    </span>
                </div>
            </div>

            <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(200, 170, 110, 0.3); padding: 1.25rem; overflow-x: auto;">
                <table style="width: 100%; border-collapse: collapse; text-align: left; font-family: monospace; font-size: 0.84rem;">
                    <thead>
                        <tr style="border-bottom: 1px solid rgba(200, 170, 110, 0.3); color: var(--gold); text-transform: uppercase;">
                            <th style="padding: 0.7rem;">"Champion"</th>
                            <th style="padding: 0.7rem;">"Games"</th>
                            <th style="padding: 0.7rem;">"Win Rate"</th>
                            <th style="padding: 0.7rem;">"Avg KDA"</th>
                            <th style="padding: 0.7rem;">"CS/Min"</th>
                            <th style="padding: 0.7rem;">"Bayesian Index"</th>
                            <th style="padding: 0.7rem;">"Confidence"</th>
                            <th style="padding: 0.7rem;">"ML Tracker & Pin"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {perfs.into_iter().map(|p| {
                            let c_name = p.champion_name.clone();
                            let c_pin = p.champion_name.clone();
                            let c_inspect = p.champion_name.clone();
                            let win_rate_style = if p.win_rate >= 50.0 {
                                "padding: 0.7rem; color: var(--cyan); font-weight: bold;"
                            } else {
                                "padding: 0.7rem; color: #e84057; font-weight: bold;"
                            };
                            view! {
                                <tr style="border-bottom: 1px solid rgba(255, 255, 255, 0.06);">
                                    <td style="padding: 0.7rem; color: #fff; font-weight: bold; font-family: serif; font-size: 0.98rem; text-transform: uppercase;">
                                        {c_name.clone()}
                                    </td>
                                    <td style="padding: 0.7rem; color: #ccc;">{format!("{}G ({}W)", p.games_played, p.wins)}</td>
                                    <td style=win_rate_style>{format!("{:.1}%", p.win_rate)}</td>
                                    <td style="padding: 0.7rem; color: #ccc;">{format!("{:.2}", p.avg_kda)}</td>
                                    <td style="padding: 0.7rem; color: #ccc;">{format!("{:.1}", p.avg_cs_per_min)}</td>
                                    <td style="padding: 0.7rem; color: var(--gold); font-weight: bold;">{format!("{:.1}", p.performance_index)}</td>
                                    <td style="padding: 0.7rem; color: var(--text-muted); font-size: 0.76rem;">{p.confidence_rating}</td>
                                    <td style="padding: 0.7rem;">
                                        <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
                                            <button
                                                type="button"
                                                style=move || {
                                                    let is_pinned = pinned_champions.get().iter().any(|x| x.eq_ignore_ascii_case(&c_pin));
                                                    if is_pinned {
                                                        "background: rgba(200, 170, 110, 0.28); border: 1px solid var(--gold); color: #fff; padding: 0.3rem 0.65rem; font-family: monospace; font-size: 0.74rem; cursor: pointer; font-weight: bold;"
                                                    } else {
                                                        "background: rgba(255, 255, 255, 0.05); border: 1px solid rgba(200, 170, 110, 0.4); color: var(--gold); padding: 0.3rem 0.65rem; font-family: monospace; font-size: 0.74rem; cursor: pointer;"
                                                    }
                                                }
                                                on:click={
                                                    let c_pin = p.champion_name.clone();
                                                    move |_| toggle_pin_champion(c_pin.clone())
                                                }
                                            >
                                                {
                                                    let c_check = p.champion_name.clone();
                                                    move || {
                                                        if pinned_champions.get().iter().any(|x| x.eq_ignore_ascii_case(&c_check)) {
                                                            "📌 Pinned"
                                                        } else {
                                                            "📌 Pin"
                                                        }
                                                    }
                                                }
                                            </button>
                                            <button
                                                type="button"
                                                style="background: rgba(10, 200, 185, 0.16); border: 1px solid var(--cyan); color: var(--cyan); padding: 0.3rem 0.65rem; font-family: monospace; font-size: 0.74rem; cursor: pointer;"
                                                on:click=move |_| {
                                                    selected_tracked_champion.set(c_inspect.clone());
                                                    active_tab.set("pinned_tracker".into());
                                                }
                                            >
                                                "🔬 ML Insights →"
                                            </button>
                                        </div>
                                    </td>
                                </tr>
                            }
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

// ============================================================================
// PAGE 5: PINNED CHAMPION TRACKER & LONGITUDINAL ML PROGRESSION
// ============================================================================
#[cfg(target_arch = "wasm32")]
fn view_page_pinned_champion_tracker<F>(
    rep: PlayerAnalyticsReport,
    pinned_champions: RwSignal<Vec<String>>,
    selected_tracked_champion: RwSignal<String>,
    toggle_pin_champion: F,
) -> impl IntoView
where
    F: Fn(String) + Copy + Send + Sync + 'static,
{
    let insights = rep.champion_ml_insights.clone();
    let default_champ = insights
        .first()
        .map(|i| i.champion_name.clone())
        .unwrap_or_else(|| "Shen".to_string());

    view! {
        <div>
            // Top Bar: Pinned Champions Switcher + Dropdown Selector
            <div style="background: rgba(9, 20, 40, 0.9); border: 1px solid rgba(200, 170, 110, 0.35); padding: 1.25rem 1.5rem; margin-bottom: 1.75rem; display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem;">
                <div>
                    <div style="font-family: monospace; font-size: 0.74rem; color: var(--cyan); text-transform: uppercase;">
                        "📌 DEDICATED CHAMPION ML TRACKER & LONGITUDINAL TELEMETRY"
                    </div>
                    <h2 style="font-family: serif; text-transform: uppercase; color: var(--gold); font-size: 1.3rem; margin: 0.2rem 0 0.5rem 0;">
                        "Select or Pin a Champion to Track Ongoing ML Progress"
                    </h2>

                    // Pinned Champion Quick-Pills
                    <div style="display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center;">
                        <span style="font-family: monospace; font-size: 0.75rem; color: var(--text-muted);">"Pinned Roster:"</span>
                        {move || {
                            let pins = pinned_champions.get();
                            if pins.is_empty() {
                                view! {
                                    <span style="font-family: monospace; font-size: 0.76rem; color: #888;">
                                        "No champions pinned yet — click '📌 Pin This Champion' on the right to save to your dashboard."
                                    </span>
                                }.into_any()
                            } else {
                                view! {
                                    <div style="display: flex; gap: 0.45rem; flex-wrap: wrap;">
                                        {pins.into_iter().map(|p_name| {
                                            let p_click = p_name.clone();
                                            let is_sel = selected_tracked_champion.get().eq_ignore_ascii_case(&p_name);
                                            let border = if is_sel { "var(--cyan)" } else { "var(--gold)" };
                                            let bg = if is_sel { "rgba(10, 200, 185, 0.22)" } else { "rgba(200, 170, 110, 0.14)" };
                                            view! {
                                                <button
                                                    type="button"
                                                    style=format!("background: {}; border: 1px solid {}; color: #fff; padding: 0.3rem 0.7rem; font-family: monospace; font-size: 0.76rem; cursor: pointer; font-weight: bold;", bg, border)
                                                    on:click=move |_| selected_tracked_champion.set(p_click.clone())
                                                >
                                                    "📌 " {p_name}
                                                </button>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>

                // Champion Selector Dropdown + Pin Toggle Button
                <div style="display: flex; gap: 0.65rem; align-items: center; flex-wrap: wrap;">
                    <select
                        style="background: rgba(1, 10, 19, 0.95); border: 1px solid var(--cyan); color: #fff; padding: 0.5rem 0.85rem; font-family: monospace; font-size: 0.82rem; font-weight: bold;"
                        on:change=move |ev| selected_tracked_champion.set(event_target_value(&ev))
                    >
                        {insights.iter().map(|ins| {
                            let c_val = ins.champion_name.clone();
                            let c_lbl = format!("{} ({} Games • {:.1}% WR)", ins.champion_name, ins.games_played, ins.win_rate);
                            let c_check = ins.champion_name.clone();
                            let def_c = default_champ.clone();
                            view! {
                                <option
                                    value=c_val
                                    selected=move || {
                                        let cur = selected_tracked_champion.get();
                                        let target = if cur.is_empty() { def_c.clone() } else { cur };
                                        target.eq_ignore_ascii_case(&c_check)
                                    }
                                >
                                    {c_lbl}
                                </option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>

                    {
                        let def_c = default_champ.clone();
                        move || {
                            let cur = selected_tracked_champion.get();
                            let active_champ = if cur.is_empty() {
                                pinned_champions.get().first().cloned().unwrap_or_else(|| def_c.clone())
                            } else {
                                cur
                            };
                            let is_pinned = pinned_champions.get().iter().any(|x| x.eq_ignore_ascii_case(&active_champ));
                            let btn_label = if is_pinned { "📌 Unpin Champion" } else { "📌 Pin This Champion" };
                            let c_toggle = active_champ.clone();
                            view! {
                                <button
                                    type="button"
                                    style="background: rgba(200, 170, 110, 0.22); border: 1px solid var(--gold); color: var(--gold); padding: 0.5rem 0.95rem; font-family: monospace; font-size: 0.8rem; font-weight: bold; text-transform: uppercase; cursor: pointer;"
                                    on:click=move |_| toggle_pin_champion(c_toggle.clone())
                                >
                                    {btn_label}
                                </button>
                            }
                        }
                    }
                </div>
            </div>

            // Selected Champion Deep ML Analysis Body
            {move || {
                let cur = selected_tracked_champion.get();
                let active_champ = if cur.is_empty() {
                    pinned_champions.get().first().cloned().unwrap_or_else(|| default_champ.clone())
                } else {
                    cur
                };

                let chosen = insights
                    .iter()
                    .find(|i| i.champion_name.eq_ignore_ascii_case(&active_champ))
                    .or_else(|| insights.first())
                    .cloned();

                match chosen {
                    None => view! {
                        <div style="padding: 2rem; text-align: center; color: var(--text-muted); font-family: monospace;">
                            "No champion telemetry available."
                        </div>
                    }.into_any(),
                    Some(ins) => {
                        let gd_sign = if ins.avg_gd_15 >= 0 { "+" } else { "" };
                        view! {
                            <div>
                                // 4 Champion-Specific ML Win-Condition Threshold Cards
                                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(230px, 1fr)); gap: 1.15rem; margin-bottom: 2rem;">
                                    <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--gold); padding: 1.15rem;">
                                        <div style="font-size: 0.72rem; color: var(--text-muted); font-family: monospace; text-transform: uppercase;">
                                            {format!("{} Empirical Win Rate", ins.champion_name)}
                                        </div>
                                        <div style="font-size: 2.0rem; font-weight: 800; color: var(--gold); font-family: monospace; margin-top: 0.3rem;">
                                            {format!("{:.1}%", ins.win_rate)}
                                        </div>
                                        <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;">
                                            {format!("{} games in {} role", ins.games_played, ins.primary_role)}
                                        </div>
                                    </div>

                                    <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(10, 200, 185, 0.4); border-top: 3px solid var(--cyan); padding: 1.15rem;">
                                        <div style="font-size: 0.72rem; color: var(--text-muted); font-family: monospace; text-transform: uppercase;">
                                            "ML Win Prob (Thresholds Met)"
                                        </div>
                                        <div style="font-size: 2.0rem; font-weight: 800; color: var(--cyan); font-family: monospace; margin-top: 0.3rem;">
                                            {format!("{:.1}%", ins.win_prob_above_threshold)}
                                        </div>
                                        <div style="font-size: 0.75rem; color: #e84057; margin-top: 0.35rem; font-family: monospace;">
                                            {format!("Drops to {:.1}% when missed", ins.win_prob_below_threshold)}
                                        </div>
                                    </div>

                                    <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid #10b981; padding: 1.15rem;">
                                        <div style="font-size: 0.72rem; color: var(--text-muted); font-family: monospace; text-transform: uppercase;">
                                            "Next-Game Target CS & GD@15"
                                        </div>
                                        <div style="font-size: 1.55rem; font-weight: 800; color: #10b981; font-family: monospace; margin-top: 0.4rem;">
                                            {format!("≥ {:.1} CS/m • +{} GD", ins.target_cs_per_min, ins.target_gd_at_15)}
                                        </div>
                                        <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.35rem;">
                                            {format!("Current Avg GD@15: {}{}", gd_sign, ins.avg_gd_15)}
                                        </div>
                                    </div>

                                    <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--gold); padding: 1.15rem;">
                                        <div style="font-size: 0.72rem; color: var(--text-muted); font-family: monospace; text-transform: uppercase;">
                                            "ML Death Discipline Cap"
                                        </div>
                                        <div style="font-size: 2.0rem; font-weight: 800; color: #fff; font-family: monospace; margin-top: 0.3rem;">
                                            {format!("≤ {} Deaths", ins.max_deaths_threshold)}
                                        </div>
                                        <div style="font-size: 0.75rem; color: var(--gold); margin-top: 0.35rem;">
                                            "Prevents mid-game shutdown loss"
                                        </div>
                                    </div>
                                </div>

                                // Actionable ML Directive & Bottleneck Banner
                                <div style="background: rgba(1, 10, 19, 0.85); border: 1px solid var(--gold); border-left: 4px solid var(--cyan); padding: 1.25rem 1.5rem; margin-bottom: 2rem;">
                                    <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 0.4rem;">
                                        <span style="font-family: monospace; font-size: 0.78rem; color: var(--cyan); text-transform: uppercase; font-weight: bold;">
                                            {format!("🔬 ML BOTTLENECK DIAGNOSIS FOR {}: {}", ins.champion_name.to_uppercase(), ins.primary_bottleneck)}
                                        </span>
                                        <span style="font-family: monospace; font-size: 0.72rem; color: var(--gold);">
                                            "AUTO-UPDATES ON EVERY RIOT SYNC"
                                        </span>
                                    </div>
                                    <p style="margin: 0; font-size: 0.92rem; color: #fff; line-height: 1.5;">
                                        <strong style="color: var(--gold);">"Next-Game ML Directive: "</strong>
                                        {ins.next_game_directive.clone()}
                                    </p>
                                </div>

                                // Longitudinal Game-by-Game Progression Chart + Match Breakdown
                                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(400px, 1fr)); gap: 1.5rem;">
                                    <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(200, 170, 110, 0.3); padding: 1.5rem;">
                                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                                            <h3 style="font-family: serif; text-transform: uppercase; color: var(--gold); font-size: 1.05rem; margin: 0;">
                                                {format!("{} Longitudinal ML Performance Curve", ins.champion_name)}
                                            </h3>
                                            <span style="font-family: monospace; font-size: 0.72rem; color: var(--cyan);">"CHRONOLOGICAL PROGRESSION"</span>
                                        </div>
                                        {view_champion_progression_svg(&ins.match_progression)}
                                    </div>

                                    <div style="background: rgba(9, 20, 40, 0.88); border: 1px solid rgba(200, 170, 110, 0.3); padding: 1.5rem;">
                                        <h3 style="font-family: serif; text-transform: uppercase; color: var(--gold); font-size: 1.05rem; margin: 0 0 1rem 0;">
                                            {format!("Recorded Games on {} ({} Matches)", ins.champion_name, ins.match_progression.len())}
                                        </h3>
                                        <div style="display: flex; flex-direction: column; gap: 0.65rem;">
                                            {ins.match_progression.iter().rev().map(|pt| {
                                                let badge_col = if pt.win { "#0ac8b9" } else { "#e84057" };
                                                let badge_txt = if pt.win { "WIN" } else { "LOSS" };
                                                let gd_lbl = if pt.gold_diff_15 >= 0 {
                                                    format!("+{} GD@15", pt.gold_diff_15)
                                                } else {
                                                    format!("{} GD@15", pt.gold_diff_15)
                                                };
                                                view! {
                                                    <div style="background: rgba(1, 10, 19, 0.75); border: 1px solid rgba(200, 170, 110, 0.2); padding: 0.65rem 0.95rem; display: flex; justify-content: space-between; align-items: center; font-family: monospace; font-size: 0.8rem; flex-wrap: wrap; gap: 0.5rem;">
                                                        <div style="display: flex; gap: 0.65rem; align-items: center;">
                                                            <span style="color: var(--text-muted);">{format!("G#{}", pt.game_number)}</span>
                                                            <strong style=format!("color: {};", badge_col)>{badge_txt}</strong>
                                                            <span style="color: #888; font-size: 0.74rem;">{pt.match_id.clone()}</span>
                                                        </div>
                                                        <div style="display: flex; gap: 1rem; align-items: center;">
                                                            <span style="color: #fff;">{format!("{:.1} KDA", pt.kda)}</span>
                                                            <span style="color: #ccc;">{format!("{:.1} CS/m", pt.cs_per_min)}</span>
                                                            <span style=format!("color: {};", if pt.gold_diff_15 >= 0 { "var(--cyan)" } else { "#e84057" })>{gd_lbl}</span>
                                                            <span style="background: rgba(200, 170, 110, 0.15); border: 1px solid var(--gold); color: var(--gold); padding: 0.1rem 0.4rem; font-size: 0.72rem;">
                                                                {format!("ML {:.1}", pt.perf_score)}
                                                            </span>
                                                        </div>
                                                    </div>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    }
                }
            }}
        </div>
    }
}

/// Renders an SVG longitudinal progression curve for a pinned/selected champion.
#[cfg(target_arch = "wasm32")]
fn view_champion_progression_svg(points: &[ChampionMatchPoint]) -> impl IntoView {
    let w = 420.0;
    let h = 210.0;
    let pad_x = 42.0;
    let pad_y = 28.0;
    let plot_w = w - 2.0 * pad_x;
    let plot_h = h - 2.0 * pad_y;

    let n = points.len();
    if n == 0 {
        return view! {
            <div style="padding: 2rem; text-align: center; color: var(--text-muted); font-family: monospace;">
                "Sync games to populate longitudinal progression."
            </div>
        }
        .into_any();
    }

    let coords: Vec<(f64, f64, ChampionMatchPoint)> = points
        .iter()
        .enumerate()
        .map(|(i, pt)| {
            let x = if n == 1 {
                w / 2.0
            } else {
                pad_x + (i as f64 / (n - 1) as f64) * plot_w
            };
            let norm_y = (pt.perf_score.clamp(20.0, 100.0) - 20.0) / 80.0;
            let y = (h - pad_y) - norm_y * plot_h;
            (x, y, pt.clone())
        })
        .collect();

    let polyline_pts = coords
        .iter()
        .map(|(x, y, _)| format!("{:.1},{:.1}", x, y))
        .collect::<Vec<_>>()
        .join(" ");

    view! {
        <svg viewBox="0 0 420 210" width="100%" style="max-width: 460px; height: auto; display: block; margin: 0 auto; overflow: visible;">
            // Horizontal reference lines (40, 60, 80, 100)
            {[40.0, 60.0, 80.0, 100.0].into_iter().map(|val| {
                let norm = (val - 20.0) / 80.0;
                let y = (h - pad_y) - norm * plot_h;
                view! {
                    <g>
                        <line x1=pad_x y1=y x2={w - pad_x} y2=y stroke="rgba(200, 170, 110, 0.16)" stroke-dasharray="3,3" stroke-width="1" />
                        <text x={pad_x - 8.0} y={y + 3.0} text-anchor="end" fill="#888" font-size="9" font-family="monospace">{format!("{:.0}", val)}</text>
                    </g>
                }
            }).collect::<Vec<_>>()}

            // Trajectory line
            <polyline points=polyline_pts fill="none" stroke="#0ac8b9" stroke-width="2.5" />

            // Game nodes
            {coords.into_iter().map(|(x, y, pt)| {
                let dot_col = if pt.win { "#0ac8b9" } else { "#e84057" };
                view! {
                    <g>
                        <circle cx=x cy=y r="5" fill=dot_col stroke="#091428" stroke-width="1.5" />
                        <text x=x y={y - 9.0} text-anchor="middle" fill="#c8aa6e" font-size="9.5" font-family="monospace" font-weight="bold">
                            {format!("{:.0}", pt.perf_score)}
                        </text>
                        <text x=x y={h - 8.0} text-anchor="middle" fill="#aaa" font-size="9" font-family="monospace">
                            {format!("G{}", pt.game_number)}
                        </text>
                    </g>
                }
            }).collect::<Vec<_>>()}
        </svg>
    }
    .into_any()
}

/// Renders a rich match history row with Match ID, KDA, CS/m, GD@15, ML Grade, and Pin shortcut.
#[cfg(target_arch = "wasm32")]
fn view_rich_match_row(
    m: AdoptedMatch,
    active_tab: RwSignal<String>,
    selected_tracked_champion: RwSignal<String>,
) -> impl IntoView {
    let outcome_color = if m.win { "#0ac8b9" } else { "#e84057" };
    let outcome_label = if m.win { "VICTORY" } else { "DEFEAT" };
    let gold_diff_label = if m.gold_diff_15 >= 0 {
        format!("+{} GD@15", m.gold_diff_15)
    } else {
        format!("{} GD@15", m.gold_diff_15)
    };
    let kda = m.kda_ratio();
    let kda_ratio_str = format!("({:.2} KDA)", kda);
    let score_str = format!("{}/{}/{}", m.kills, m.deaths, m.assists);
    let cs_str = format!("{:.1} CS/m", m.cs_per_min);
    let ml_grade = if m.win && kda >= 5.0 && m.gold_diff_15 >= 1000 {
        ("S+", "#c8aa6e")
    } else if m.win && kda >= 3.2 {
        ("S", "#0ac8b9")
    } else if m.win || kda >= 2.5 {
        ("A", "#10b981")
    } else {
        ("B", "#888")
    };

    let champ_name = m.champion_name.clone();
    let champ_click = m.champion_name.clone();
    let role_name = m.role;
    let match_id = m.match_id;
    let is_calibrated = m.is_calibrated;
    let calibrated_rank = m.calibrated_rank;
    let is_positive_gold = m.gold_diff_15 >= 0;

    view! {
        <div style="background: rgba(9, 20, 40, 0.78); border: 1px solid rgba(200, 170, 110, 0.25); padding: 0.9rem 1.25rem; display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 1rem;">
            <div style="display: flex; align-items: center; gap: 1rem; flex-wrap: wrap;">
                <span style=format!("font-family: monospace; font-weight: bold; color: {}; font-size: 0.85rem; min-width: 68px;", outcome_color)>
                    {outcome_label}
                </span>

                <span style="font-family: monospace; font-size: 0.74rem; color: #888; background: rgba(0,0,0,0.35); padding: 0.15rem 0.45rem; border: 1px solid rgba(255,255,255,0.08);">
                    {match_id}
                </span>

                <div style="display: flex; align-items: center; gap: 0.55rem; flex-wrap: wrap;">
                    <strong style="font-family: serif; text-transform: uppercase; color: #fff; font-size: 1.08rem;">{champ_name}</strong>
                    <span style="font-size: 0.76rem; color: var(--text-muted); font-family: monospace;">"• " {role_name}</span>
                    {if is_calibrated {
                        let rank_str = calibrated_rank.map(|r| format!("Rank #{}", r)).unwrap_or_else(|| "Top Pick".into());
                        view! {
                            <span style="background: rgba(200, 170, 110, 0.2); border: 1px solid var(--gold); color: var(--gold); font-size: 0.7rem; padding: 0.1rem 0.4rem; text-transform: uppercase; font-family: monospace;">
                                "✦ CALIBRATED (" {rank_str} ") ✦"
                            </span>
                        }.into_any()
                    } else {
                        view! {
                            <span style="background: rgba(255, 255, 255, 0.05); border: 1px solid #444; color: #888; font-size: 0.7rem; padding: 0.1rem 0.4rem; text-transform: uppercase; font-family: monospace;">
                                "OFF-CALIBRATION"
                            </span>
                        }.into_any()
                    }}
                </div>
            </div>

            <div style="display: flex; align-items: center; gap: 1.25rem; font-family: monospace; font-size: 0.84rem; flex-wrap: wrap;">
                <div>
                    <span style="color: #fff; font-weight: bold;">{score_str}</span>
                    <span style="color: var(--text-muted); margin-left: 0.35rem;">{kda_ratio_str}</span>
                </div>
                <div>
                    <span style="color: #fff;">{cs_str}</span>
                </div>
                <div>
                    <span style=format!("color: {}; font-weight: bold;", if is_positive_gold { "var(--cyan)" } else { "#e84057" })>
                        {gold_diff_label}
                    </span>
                </div>
                <span style=format!("background: rgba(0,0,0,0.4); border: 1px solid {}; color: {}; padding: 0.15rem 0.45rem; font-size: 0.75rem; font-weight: bold;", ml_grade.1, ml_grade.1)>
                    {format!("ML {}", ml_grade.0)}
                </span>
                <button
                    type="button"
                    style="background: transparent; border: 1px solid rgba(10, 200, 185, 0.45); color: var(--cyan); padding: 0.22rem 0.55rem; font-family: monospace; font-size: 0.72rem; cursor: pointer;"
                    on:click=move |_| {
                        selected_tracked_champion.set(champ_click.clone());
                        active_tab.set("pinned_tracker".into());
                    }
                >
                    "📌 Track →"
                </button>
            </div>
        </div>
    }
}

/// Renders a Machine Learning coaching card.
#[cfg(target_arch = "wasm32")]
fn view_ml_insight_card(card: MachineLearningInsight) -> impl IntoView {
    view! {
        <div style="background: rgba(9, 20, 40, 0.9); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--gold); padding: 1.25rem; display: flex; flex-direction: column; justify-content: space-between;">
            <div>
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;">
                    <span style="font-size: 0.7rem; color: var(--cyan); text-transform: uppercase; font-family: monospace;">{card.category}</span>
                    <span style="background: rgba(10, 200, 185, 0.15); border: 1px solid var(--cyan); color: var(--cyan); font-size: 0.65rem; padding: 0.1rem 0.4rem; font-family: monospace;">{card.badge}</span>
                </div>
                <h3 style="font-family: serif; text-transform: uppercase; color: var(--gold); font-size: 1.05rem; margin: 0 0 0.5rem 0;">{card.title}</h3>
                <p style="font-size: 0.8rem; color: #ccc; margin: 0 0 0.75rem 0; line-height: 1.4;">{card.description}</p>
                <div style="background: rgba(1, 10, 19, 0.6); border-left: 2px solid var(--gold); padding: 0.5rem 0.75rem; margin-bottom: 0.75rem;">
                    <p style="margin: 0; font-size: 0.78rem; color: #fff; font-family: sans-serif;">
                        <strong style="color: var(--gold);">"ML Action: "</strong> {card.recommendation}
                    </p>
                </div>
            </div>

            <div style="display: flex; justify-content: space-between; align-items: center; border-top: 1px solid rgba(255,255,255,0.08); padding-top: 0.5rem; font-family: monospace; font-size: 0.75rem;">
                <span style="color: var(--cyan);">{card.projected_impact}</span>
                <span style="color: var(--text-muted);">{format!("{}% Confidence", card.confidence_score)}</span>
            </div>
        </div>
    }
}

/// Generates an enlarged, zero-clipping SVG pentagonal playstyle radar chart with vertex badges.
#[cfg(target_arch = "wasm32")]
fn view_radar_svg(curr: [f64; 5], prev: [f64; 5]) -> impl IntoView {
    let center_x = 210.0;
    let center_y = 172.0;
    let max_radius = 108.0;
    let labels = [
        "Aggression",
        "Mobility",
        "Durability",
        "Utility",
        "Waveclear",
    ];

    let compute_points = |scores: [f64; 5]| -> String {
        let mut pts = Vec::new();
        for (i, score) in scores.iter().enumerate() {
            let angle = (i as f64 * (2.0 * PI / 5.0)) - (PI / 2.0);
            let r = (score.clamp(0.0, 100.0) / 100.0) * max_radius;
            let x = center_x + r * angle.cos();
            let y = center_y + r * angle.sin();
            pts.push(format!("{:.1},{:.1}", x, y));
        }
        pts.join(" ")
    };

    let curr_poly = compute_points(curr);
    let prev_poly = compute_points(prev);

    // Web background rings (25%, 50%, 75%, 100%)
    let rings: Vec<String> = vec![0.25, 0.5, 0.75, 1.0]
        .into_iter()
        .map(|frac| {
            let mut ring_pts = Vec::new();
            for i in 0..5 {
                let angle = (i as f64 * (2.0 * PI / 5.0)) - (PI / 2.0);
                let r = frac * max_radius;
                let x = center_x + r * angle.cos();
                let y = center_y + r * angle.sin();
                ring_pts.push(format!("{:.1},{:.1}", x, y));
            }
            ring_pts.join(" ")
        })
        .collect();

    view! {
        <svg viewBox="0 0 420 340" width="100%" style="max-width: 440px; height: auto; overflow: visible; margin: 0 auto; display: block;">
            // Concentric web polygons
            {rings.into_iter().map(|pts| {
                view! { <polygon points=pts fill="none" stroke="rgba(200, 170, 110, 0.18)" stroke-width="1" /> }
            }).collect::<Vec<_>>()}

            // Axis lines
            {(0..5).map(|i| {
                let angle = (i as f64 * (2.0 * PI / 5.0)) - (PI / 2.0);
                let x2 = center_x + max_radius * angle.cos();
                let y2 = center_y + max_radius * angle.sin();
                view! { <line x1=center_x y1=center_y x2=x2 y2=y2 stroke="rgba(200, 170, 110, 0.25)" stroke-width="1" /> }
            }).collect::<Vec<_>>()}

            // Previous calibration polygon (Cyan)
            <polygon points=prev_poly fill="rgba(10, 200, 185, 0.16)" stroke="#0ac8b9" stroke-width="1.75" stroke-dasharray="4,4" />

            // Current calibration polygon (Gold)
            <polygon points=curr_poly fill="rgba(200, 170, 110, 0.28)" stroke="#c8aa6e" stroke-width="2.25" />

            // Vertex dots on Current Calibration
            {(0..5).map(|i| {
                let angle = (i as f64 * (2.0 * PI / 5.0)) - (PI / 2.0);
                let r = (curr[i].clamp(0.0, 100.0) / 100.0) * max_radius;
                let vx = center_x + r * angle.cos();
                let vy = center_y + r * angle.sin();
                view! {
                    <circle cx=vx cy=vy r="4" fill="#c8aa6e" stroke="#091428" stroke-width="1.5" />
                }
            }).collect::<Vec<_>>()}

            // Axis text labels + live percentage scores with generous horizontal clearance
            {(0..5).map(|i| {
                let angle = (i as f64 * (2.0 * PI / 5.0)) - (PI / 2.0);
                let lx = center_x + (max_radius + 24.0) * angle.cos();
                let ly = center_y + (max_radius + 20.0) * angle.sin();
                let anchor = if angle.cos().abs() < 0.2 {
                    "middle"
                } else if angle.cos() > 0.0 {
                    "start"
                } else {
                    "end"
                };
                view! {
                    <g>
                        <text x=lx y=ly text-anchor=anchor fill="#c8aa6e" font-size="11.5" font-weight="bold" font-family="monospace">
                            {labels[i]}
                        </text>
                        <text x=lx y={ly + 14.0} text-anchor=anchor fill="#0ac8b9" font-size="10.5" font-family="monospace">
                            {format!("{:.0}%", curr[i])}
                        </text>
                    </g>
                }
            }).collect::<Vec<_>>()}
        </svg>
    }
}
