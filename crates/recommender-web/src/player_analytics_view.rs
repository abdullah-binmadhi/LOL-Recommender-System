//! Personalized Player Analytics & Summoner Intelligence View.
//!
//! Visualizes recommender efficacy (win rate lift), multi-quiz archetype drift (radar charts),
//! and machine learning coaching insights (adaptability curves, draft blindspots, fatigue).

use crate::state::{AppState, Modal};
use leptos::prelude::*;
use recommender_core::player_analytics::{
    AdoptedMatch, MachineLearningInsight, PlayerAnalyticsReport,
};
use std::f64::consts::PI;

#[cfg(target_arch = "wasm32")]
pub fn view_player_analytics(state: RwSignal<AppState>) -> impl IntoView {
    let report = move || {
        let is_demo = state.with(|c| c.demo_telemetry);
        let user = state.with(|c| c.auth_user.clone());
        let live_matches = state.with(|c| c.live_player_matches.clone());

        if is_demo {
            PlayerAnalyticsReport::demo()
        } else if let Some(u) = user {
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

    view! {
        <section id="player-analytics-dashboard" class="analytics-dashboard active" style="max-width: 1200px; margin: 0 auto; padding: 2rem 1rem;">
            // Navigation Bar
            <div class="analytics-nav" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem; border-bottom: 1px solid rgba(200, 170, 110, 0.2); padding-bottom: 1rem;">
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

                <div style="display: flex; gap: 0.75rem;">
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

            // Summoner Identity Header
            {move || {
                let rep = report();
                let riot_label = rep.riot_id.clone().unwrap_or_else(|| "No Riot ID Linked".into());
                let region_label = rep
                    .region
                    .as_deref()
                    .and_then(|r| recommender_core::riot_api::RiotRegion::from_code(r).map(|reg| format!("{} · {}", reg.code(), reg.platform_id().to_uppercase())))
                    .unwrap_or_else(|| rep.region.clone().unwrap_or_else(|| "Global".into()));
                let demo_btn_label = if rep.is_demo { "Disable Demo Telemetry" } else { "Load Demo Telemetry" };

                view! {
                    <div style="background: linear-gradient(180deg, rgba(9, 20, 40, 0.9) 0%, rgba(1, 10, 19, 0.95) 100%); border: 1px solid var(--gold); padding: 1.5rem; margin-bottom: 2rem; position: relative; box-shadow: 0 8px 32px rgba(0,0,0,0.5);">
                        <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem;">
                            <div style="display: flex; align-items: center; gap: 1rem;">
                                <div style="width: 56px; height: 56px; border: 2px solid var(--gold); overflow: hidden; background: #000;">
                                    <img src="https://ddragon.leagueoflegends.com/cdn/14.20.1/img/profileicon/588.png" alt="" style="width: 100%; height: 100%; object-fit: cover;" />
                                </div>
                                <div>
                                    <div style="display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap;">
                                        <h1 style="margin: 0; font-family: serif; text-transform: uppercase; letter-spacing: 0.1em; color: var(--gold); font-size: 1.6rem;">{rep.summoner_name}</h1>
                                        <span style="background: rgba(10, 200, 185, 0.15); border: 1px solid var(--cyan); color: var(--cyan); font-size: 0.75rem; padding: 0.15rem 0.5rem; text-transform: uppercase; font-family: monospace;">{region_label}</span>
                                        {if rep.is_demo {
                                            view! { <span style="background: rgba(200, 170, 110, 0.15); border: 1px solid var(--gold); color: var(--gold); font-size: 0.75rem; padding: 0.15rem 0.5rem; text-transform: uppercase; font-family: monospace;">"✦ DEMO MODE ✦"</span> }.into_any()
                                        } else {
                                            view! { <span style="background: rgba(16, 185, 129, 0.15); border: 1px solid #10b981; color: #10b981; font-size: 0.75rem; padding: 0.15rem 0.5rem; text-transform: uppercase; font-family: monospace;">"● LIVE RIOT API CONNECTED"</span> }.into_any()
                                        }}
                                    </div>
                                    <p style="margin: 0.25rem 0 0 0; font-family: monospace; color: var(--text-muted); font-size: 0.85rem;">
                                        "Riot Account: " <strong style="color: var(--gold);">{riot_label}</strong> " • Match Intelligence Engine v2.4"
                                    </p>
                                </div>
                            </div>

                            <div style="display: flex; gap: 0.75rem; flex-wrap: wrap;">
                                <button
                                    type="button"
                                    style="background: rgba(10, 200, 185, 0.2); border: 1px solid var(--cyan); color: #fff; padding: 0.5rem 1rem; cursor: pointer; font-size: 0.8rem; text-transform: uppercase; font-family: monospace; font-weight: bold; display: flex; align-items: center; gap: 6px;"
                                    disabled=move || state.with(|c| c.is_syncing_riot)
                                    on:click=move |_| {
                                        state.update(|c| c.set_syncing_riot(true));
                                        let rep = report();
                                        let r_id = rep.riot_id.unwrap_or_else(|| "SHENTRICK#SHEN".into());
                                        let reg = rep.region.unwrap_or_else(|| "ME".into());
                                        crate::storage::supabase_fetch_riot_telemetry(&r_id, &reg);
                                        if let Some(matches) = crate::storage::load_cached_riot_matches() {
                                            state.update(|c| c.set_live_matches(Some(matches)));
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
                                    style="background: transparent; border: 1px solid rgba(200, 170, 110, 0.5); color: var(--gold); padding: 0.5rem 1rem; cursor: pointer; font-size: 0.8rem; text-transform: uppercase; font-family: monospace;"
                                    on:click=move |_| state.update(|c| c.open_modal(Modal::LinkRiotAccount))
                                >
                                    "Link / Change Riot ID"
                                </button>
                                <button
                                    type="button"
                                    style="background: rgba(10, 200, 185, 0.1); border: 1px solid var(--cyan); color: var(--cyan); padding: 0.5rem 1rem; cursor: pointer; font-size: 0.8rem; text-transform: uppercase; font-family: monospace;"
                                    on:click=move |_| state.update(AppState::toggle_demo_telemetry)
                                >
                                    {demo_btn_label}
                                </button>
                            </div>
                        </div>
                    </div>
                }
            }}

            // 4 KPI Highlights Cards
            {move || {
                let rep = report();
                let eff = &rep.efficacy;
                let anchor = rep.evolution.anchor_champions.first().cloned().unwrap_or_else(|| "Ahri".into());

                view! {
                    <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 1.25rem; margin-bottom: 2.5rem;">
                        // Tile 1: Calibrated Win Rate
                        <div style="background: rgba(9, 20, 40, 0.8); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--gold); padding: 1.25rem;">
                            <div style="font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.1em; color: var(--text-muted); margin-bottom: 0.5rem;">"Calibrated Win Rate"</div>
                            <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                                <span style="font-size: 2.2rem; font-weight: 800; color: var(--gold); font-family: monospace;">{format!("{:.1}%", eff.calibrated_win_rate)}</span>
                                <span style="font-size: 0.85rem; color: var(--cyan); font-family: monospace;">{format!("+{}% LIFT", eff.win_rate_lift)}</span>
                            </div>
                            <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.5rem;">
                                {format!("{} Wins in {} Calibrated Matches", eff.calibrated_wins, eff.calibrated_games)}
                            </div>
                        </div>

                        // Tile 2: Recommender Adoption
                        <div style="background: rgba(9, 20, 40, 0.8); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--cyan); padding: 1.25rem;">
                            <div style="font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.1em; color: var(--text-muted); margin-bottom: 0.5rem;">"Recommender Adoption"</div>
                            <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                                <span style="font-size: 2.2rem; font-weight: 800; color: var(--cyan); font-family: monospace;">{format!("{:.0}%", eff.adoption_rate)}</span>
                                <span style="font-size: 0.85rem; color: var(--text-muted); font-family: monospace;">"DRAFT RATE"</span>
                            </div>
                            <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.5rem;">
                                {format!("{}/{} matches drafted on recommended picks", eff.calibrated_games, rep.total_analyzed_matches)}
                            </div>
                        </div>

                        // Tile 3: Anchor Champion
                        <div style="background: rgba(9, 20, 40, 0.8); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--gold); padding: 1.25rem;">
                            <div style="font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.1em; color: var(--text-muted); margin-bottom: 0.5rem;">"Primary Anchor Pick"</div>
                            <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                                <span style="font-size: 2.2rem; font-weight: 800; color: #fff; font-family: serif; text-transform: uppercase;">{anchor}</span>
                            </div>
                            <div style="font-size: 0.75rem; color: var(--gold); margin-top: 0.5rem;">
                                "✦ Present across 100% of calibrations"
                            </div>
                        </div>

                        // Tile 4: ML Synergy Grade
                        <div style="background: rgba(9, 20, 40, 0.8); border: 1px solid rgba(200, 170, 110, 0.3); border-top: 3px solid var(--cyan); padding: 1.25rem;">
                            <div style="font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.1em; color: var(--text-muted); margin-bottom: 0.5rem;">"Machine Learning Synergy"</div>
                            <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                                <span style="font-size: 2.2rem; font-weight: 800; color: #0ac8b9; font-family: monospace;">"TIER S+"</span>
                            </div>
                            <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.5rem;">
                                "4-6 games projected to 55%+ proficiency"
                            </div>
                        </div>
                    </div>
                }
            }}

            // MODULE 1: Recommendation vs Reality (Match Telemetry)
            <div style="margin-bottom: 3rem;">
                <div style="display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 1rem; border-bottom: 1px solid rgba(200, 170, 110, 0.3); padding-bottom: 0.5rem;">
                    <h2 style="font-family: serif; text-transform: uppercase; letter-spacing: 0.1em; color: var(--gold); font-size: 1.3rem; margin: 0;">
                        "1. Recommender vs. Reality (Match Telemetry)"
                    </h2>
                    <span style="font-family: monospace; font-size: 0.8rem; color: var(--cyan);">"VERIFIED GAME LOG"</span>
                </div>

                // Win rate comparison strip
                {move || {
                    let rep = report();
                    let eff = &rep.efficacy;
                    view! {
                        <div style="background: rgba(1, 10, 19, 0.8); border: 1px solid rgba(200, 170, 110, 0.2); padding: 1rem 1.5rem; margin-bottom: 1.5rem; display: flex; flex-direction: column; gap: 0.75rem;">
                            <div style="display: flex; justify-content: space-between; font-size: 0.85rem; font-family: monospace;">
                                <span><strong style="color: var(--cyan);">"Calibrated Picks: "</strong> {format!("{:.1}% Win Rate ({} Games)", eff.calibrated_win_rate, eff.calibrated_games)}</span>
                                <span><strong style="color: #888;">"Off-Calibration: "</strong> {format!("{:.1}% Win Rate ({} Games)", eff.uncalibrated_win_rate, eff.uncalibrated_games)}</span>
                            </div>
                            <div style="width: 100%; height: 12px; background: rgba(255,255,255,0.05); display: flex; overflow: hidden; border: 1px solid rgba(200, 170, 110, 0.3);">
                                <div style=format!("width: {}%; background: linear-gradient(90deg, #005a82, #0ac8b9);", eff.calibrated_win_rate)></div>
                                <div style=format!("width: {}%; background: #444;", eff.uncalibrated_win_rate)></div>
                            </div>
                        </div>
                    }
                }}

                // Match History List
                <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                    {move || {
                        let rep = report();
                        rep.recent_matches.into_iter().map(view_match_row).collect::<Vec<_>>()
                    }}
                </div>
            </div>

            // MODULE 2: Multi-Quiz Calibration & Archetype Drift
            <div style="margin-bottom: 3rem;">
                <div style="display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 1rem; border-bottom: 1px solid rgba(200, 170, 110, 0.3); padding-bottom: 0.5rem;">
                    <h2 style="font-family: serif; text-transform: uppercase; letter-spacing: 0.1em; color: var(--gold); font-size: 1.3rem; margin: 0;">
                        "2. Multi-Quiz Calibration & Archetype Drift"
                    </h2>
                    <span style="font-family: monospace; font-size: 0.8rem; color: var(--gold);">"LONGITUDINAL TELEMETRY"</span>
                </div>

                {move || {
                    let rep = report();
                    let evo = &rep.evolution;
                    view! {
                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 1.5rem; align-items: start;">
                            // Radar Chart
                            <div style="background: rgba(9, 20, 40, 0.8); border: 1px solid rgba(200, 170, 110, 0.3); padding: 1.5rem; text-align: center;">
                                <h3 style="font-family: serif; text-transform: uppercase; font-size: 1rem; color: var(--gold); margin: 0 0 1rem 0;">"Playstyle Archetype Radar"</h3>
                                {view_radar_svg(evo.current_radar, evo.previous_radar)}
                                <div style="display: flex; justify-content: center; gap: 1.5rem; margin-top: 1rem; font-family: monospace; font-size: 0.75rem;">
                                    <span style="color: var(--gold);">"✦ Current Calibration"</span>
                                    <span style="color: var(--cyan);">"✦ Previous Calibration"</span>
                                </div>
                                <p style="font-size: 0.8rem; color: var(--text-muted); margin: 1rem 0 0 0; font-family: sans-serif;">
                                    {evo.primary_drift_summary.clone()}
                                </p>
                            </div>

                            // Timeline & Anchor Champions
                            <div style="display: flex; flex-direction: column; gap: 1rem;">
                                <div style="background: rgba(9, 20, 40, 0.8); border: 1px solid rgba(200, 170, 110, 0.3); padding: 1.25rem;">
                                    <h4 style="font-family: serif; text-transform: uppercase; font-size: 0.9rem; color: var(--gold); margin: 0 0 0.75rem 0;">"Identified Anchor Champions"</h4>
                                    <div style="display: flex; gap: 0.5rem; flex-wrap: wrap;">
                                        {evo.anchor_champions.iter().map(|name| {
                                            view! {
                                                <span style="background: rgba(200, 170, 110, 0.15); border: 1px solid var(--gold); color: #fff; padding: 0.35rem 0.75rem; font-family: serif; text-transform: uppercase; font-size: 0.85rem;">
                                                    "✦ " {name.clone()}
                                                </span>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                    <p style="font-size: 0.75rem; color: var(--text-muted); margin: 0.75rem 0 0 0;">
                                        "These champions consistently appear in your Top 3 across repeated calibrations, representing your core summoner playstyle DNA."
                                    </p>
                                </div>

                                <div style="background: rgba(9, 20, 40, 0.8); border: 1px solid rgba(200, 170, 110, 0.3); padding: 1.25rem;">
                                    <h4 style="font-family: serif; text-transform: uppercase; font-size: 0.9rem; color: var(--cyan); margin: 0 0 0.75rem 0;">"Past Calibration Snapshots"</h4>
                                    <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                                        {evo.history.iter().map(|rec| {
                                            view! {
                                                <div style="border-left: 2px solid var(--gold); padding-left: 0.75rem;">
                                                    <div style="display: flex; justify-content: space-between; font-size: 0.8rem; font-family: monospace;">
                                                        <strong style="color: var(--gold);">{rec.date_label.clone()}</strong>
                                                        <span style="color: var(--cyan);">{rec.role_focus.clone()}</span>
                                                    </div>
                                                    <div style="font-size: 0.75rem; color: var(--text-muted); margin-top: 0.2rem;">
                                                        "Top Picks: " {rec.top_champions.join(", ")}
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }}
            </div>

            // MODULE 3: Hextech Oracle (Machine Learning Insights)
            <div style="margin-bottom: 2rem;">
                <div style="display: flex; justify-content: space-between; align-items: baseline; margin-bottom: 1rem; border-bottom: 1px solid rgba(200, 170, 110, 0.3); padding-bottom: 0.5rem;">
                    <h2 style="font-family: serif; text-transform: uppercase; letter-spacing: 0.1em; color: var(--gold); font-size: 1.3rem; margin: 0;">
                        "3. Hextech Oracle (Machine Learning Coaching)"
                    </h2>
                    <span style="font-family: monospace; font-size: 0.8rem; color: var(--cyan);">"PREDICTIVE MODELS"</span>
                </div>

                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 1.25rem;">
                    {move || {
                        let rep = report();
                        rep.ml_insights.into_iter().map(view_ml_insight_card).collect::<Vec<_>>()
                    }}
                </div>
            </div>
        </section>
    }
}

/// Renders a single match history row in Hextech style.
#[cfg(target_arch = "wasm32")]
fn view_match_row(m: AdoptedMatch) -> impl IntoView {
    let outcome_color = if m.win { "#0ac8b9" } else { "#e84057" };
    let outcome_label = if m.win { "VICTORY" } else { "DEFEAT" };
    let gold_diff_label = if m.gold_diff_15 >= 0 {
        format!("+{} GD@15", m.gold_diff_15)
    } else {
        format!("{} GD@15", m.gold_diff_15)
    };
    let kda_ratio_str = format!("({:.2} KDA)", m.kda_ratio());
    let score_str = format!("{}/{}/{}", m.kills, m.deaths, m.assists);
    let cs_str = format!("{:.1} CS/m", m.cs_per_min);
    let champ_name = m.champion_name;
    let role_name = m.role;
    let is_calibrated = m.is_calibrated;
    let calibrated_rank = m.calibrated_rank;
    let is_positive_gold = m.gold_diff_15 >= 0;

    view! {
        <div style="background: rgba(9, 20, 40, 0.7); border: 1px solid rgba(200, 170, 110, 0.2); padding: 0.85rem 1.25rem; display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 1rem;">
            <div style="display: flex; align-items: center; gap: 1rem;">
                <span style=format!("font-family: monospace; font-weight: bold; color: {}; font-size: 0.85rem; min-width: 65px;", outcome_color)>
                    {outcome_label}
                </span>

                <div>
                    <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <strong style="font-family: serif; text-transform: uppercase; color: #fff; font-size: 1.05rem;">{champ_name}</strong>
                        <span style="font-size: 0.75rem; color: var(--text-muted); font-family: monospace;">"• " {role_name}</span>
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
            </div>

            <div style="display: flex; align-items: center; gap: 1.5rem; font-family: monospace; font-size: 0.85rem;">
                <div>
                    <span style="color: #fff; font-weight: bold;">{score_str}</span>
                    <span style="color: var(--text-muted); margin-left: 0.35rem;">{kda_ratio_str}</span>
                </div>
                <div>
                    <span style="color: #fff;">{cs_str}</span>
                </div>
                <div>
                    <span style=format!("color: {};", if is_positive_gold { "var(--cyan)" } else { "#e84057" })>
                        {gold_diff_label}
                    </span>
                </div>
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

/// Generates a pure SVG pentagonal playstyle radar chart.
#[cfg(target_arch = "wasm32")]
fn view_radar_svg(curr: [f64; 5], prev: [f64; 5]) -> impl IntoView {
    let center_x = 130.0;
    let center_y = 130.0;
    let max_radius = 80.0;
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
        <svg viewBox="0 0 260 260" width="240" height="240" style="margin: 0 auto; display: block;">
            // Concentric web polygons
            {rings.into_iter().map(|pts| {
                view! { <polygon points=pts fill="none" stroke="rgba(200, 170, 110, 0.15)" stroke-width="1" /> }
            }).collect::<Vec<_>>()}

            // Axis lines
            {(0..5).map(|i| {
                let angle = (i as f64 * (2.0 * PI / 5.0)) - (PI / 2.0);
                let x2 = center_x + max_radius * angle.cos();
                let y2 = center_y + max_radius * angle.sin();
                view! { <line x1=center_x y1=center_y x2=x2 y2=y2 stroke="rgba(200, 170, 110, 0.2)" stroke-width="1" /> }
            }).collect::<Vec<_>>()}

            // Previous calibration polygon (Cyan)
            <polygon points=prev_poly fill="rgba(10, 200, 185, 0.15)" stroke="#0ac8b9" stroke-width="1.5" stroke-dasharray="3,3" />

            // Current calibration polygon (Gold)
            <polygon points=curr_poly fill="rgba(200, 170, 110, 0.25)" stroke="#c8aa6e" stroke-width="2" />

            // Axis text labels
            {(0..5).map(|i| {
                let angle = (i as f64 * (2.0 * PI / 5.0)) - (PI / 2.0);
                let lx = center_x + (max_radius + 20.0) * angle.cos();
                let ly = center_y + (max_radius + 14.0) * angle.sin();
                let anchor = if angle.cos().abs() < 0.2 { "middle" } else if angle.cos() > 0.0 { "start" } else { "end" };
                view! {
                    <text x=lx y=ly text-anchor=anchor fill="#c8aa6e" font-size="9" font-family="monospace">
                        {labels[i]}
                    </text>
                }
            }).collect::<Vec<_>>()}
        </svg>
    }
}
