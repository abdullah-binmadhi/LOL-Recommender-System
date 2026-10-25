//! Pure Rust native SVG analytics dashboard for League of Legends champions.
//!
//! Replaces external Chart.js and CDN dependencies with deterministic, responsive
//! SVG charts for hero types, combat ranges, difficulty, release chronology,
//! and disclosed illustrative resource types.
//!
//! Features full faceted multi-chart cross-filtering, live candidate matching,
//! and heroic champion revelation when the matrix narrows to a unique pick.

use crate::state::AppState;
use leptos::prelude::*;
use recommender_core::analytics::{
    filter_champions, AnalyticsSummary, DifficultyMetric, MetricSlice,
};
use std::f64::consts::PI;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
fn champion_avatar(name: &str) -> String {
    let api_name = match name {
        "Aurelion Sol" => "AurelionSol",
        "Bel'Veth" => "Belveth",
        "Cho'Gath" => "Chogath",
        "Dr. Mundo" => "DrMundo",
        "Gnar" | "GnarBig" => "Gnar",
        "Jarvan IV" => "JarvanIV",
        "Kai'Sa" => "Kaisa",
        "Kha'Zix" => "Khazix",
        "Kog'Maw" => "KogMaw",
        "Lee Sin" => "LeeSin",
        "Master Yi" => "MasterYi",
        "Miss Fortune" => "MissFortune",
        "Nunu & Willump" => "Nunu",
        "Rek'Sai" => "RekSai",
        "Tahm Kench" => "TahmKench",
        "Twisted Fate" => "TwistedFate",
        "Vel'Koz" => "Velkoz",
        "Xin Zhao" => "XinZhao",
        _ => {
            return format!(
                "https://ddragon.leagueoflegends.com/cdn/15.16.1/img/champion/{}.png",
                name.chars()
                    .filter(|character| character.is_ascii_alphanumeric())
                    .collect::<String>()
            );
        }
    };
    format!("https://ddragon.leagueoflegends.com/cdn/15.16.1/img/champion/{api_name}.png")
}

#[cfg(target_arch = "wasm32")]
pub fn view_analytics(state: RwSignal<AppState>, summary: Arc<AnalyticsSummary>) -> impl IntoView {
    let summary_champions = Arc::clone(&summary);
    let total_champions = summary.total_champions;

    let filtered_champions = Memo::new(move |_| {
        let filter = state.with(|s| s.roster_filter.clone());
        filter_champions(
            &summary_champions.champions,
            filter.hero_type.as_deref(),
            filter.range_type.as_deref(),
            filter.resource_type.as_deref(),
            filter.difficulty,
            filter.release_year.as_deref(),
        )
        .into_iter()
        .cloned()
        .collect::<Vec<_>>()
    });

    let is_filtered = move || state.with(|s| s.roster_filter.is_active());

    view! {
        <section id="analytics-dashboard" class="analytics-dashboard active">
            <div class="analytics-nav" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;">
                <button
                    type="button"
                    class="back-button"
                    on:click=move |_| {
                        if let Some(win) = web_sys::window() {
                            let _ = win.location().set_hash("");
                        }
                        state.update(AppState::return_to_start);
                    }
                >
                    "← Back to Home"
                </button>

                <button
                    type="button"
                    class="back-button"
                    style="border-color: var(--cyan); color: var(--cyan); display: inline-flex; align-items: center; gap: 6px;"
                    on:click=move |_| {
                        if let Some(win) = web_sys::window() {
                            let _ = win.location().set_hash("#analytics");
                        }
                        state.update(AppState::open_player_analytics);
                    }
                >
                    {crate::player_analytics_view::view_lol_icon("hex-compass")}
                    <span>"Summoner Intelligence (Player Analytics) →"</span>
                </button>
            </div>

            <div class="analytics-header">
                <h1>"Champion Distribution & Roster Matrix"</h1>
                <p class="analytics-subtitle">
                    "Global demographic breakdown and combat taxonomy across all 171 League of Legends champions"
                </p>
            </div>

            <div class="analytics-stats-ribbon">
                <div class="stat-pill">
                    <span class="stat-pill-label">"Total Champions"</span>
                    <span class="stat-pill-value">{summary.total_champions}</span>
                </div>
                <div class="stat-pill">
                    <span class="stat-pill-label">"Hero Classes"</span>
                    <span class="stat-pill-value">{summary.hero_types.len()}</span>
                </div>
                <div class="stat-pill">
                    <span class="stat-pill-label">"Combat Types"</span>
                    <span class="stat-pill-value">{summary.range_types.len()}</span>
                </div>
                <div class="stat-pill">
                    <span class="stat-pill-label">"Difficulty Tiers"</span>
                    <span class="stat-pill-value">{summary.difficulties.len()}</span>
                </div>
                <div class="stat-pill">
                    <span class="stat-pill-label">"Historical Years"</span>
                    <span class="stat-pill-value">{summary.release_years.len()}</span>
                </div>
            </div>

            // Interactive Filter Ribbon & Roster Matcher
            <div class="matrix-filter-ribbon">
                <div class="matrix-filter-header">
                    <span class="matrix-sparkle">{crate::player_analytics_view::view_lol_icon("hex-sparkle")}</span>
                    <span class="matrix-title">"Interactive Roster Matrix"</span>
                    {move || {
                        let count = filtered_champions.with(|c| c.len());
                        let total = total_champions;
                        if is_filtered() {
                            view! {
                                <span class="matrix-counter-badge active">
                                    {format!("{count} of {total} Champions Match Query")}
                                </span>
                            }.into_any()
                        } else {
                            view! {
                                <span class="matrix-counter-badge">
                                    "Click any chart segment below to cross-filter champions"
                                </span>
                            }.into_any()
                        }
                    }}
                    {move || {
                        if is_filtered() {
                            view! {
                                <button
                                    type="button"
                                    class="matrix-reset-btn"
                                    style="display: inline-flex; align-items: center; gap: 5px;"
                                    on:click=move |_| state.update(|s| s.clear_roster_filter())
                                >
                                    {crate::player_analytics_view::view_lol_icon("nexus-pulse")}
                                    <span>"Reset All Filters"</span>
                                </button>
                            }.into_any()
                        } else {
                            view! { <span class="matrix-reset-placeholder"></span> }.into_any()
                        }
                    }}
                </div>

                // Filter chips
                {move || {
                    let filter = state.with(|s| s.roster_filter.clone());
                    let mut chips: Vec<AnyView> = Vec::new();

                    if let Some(ref ht) = filter.hero_type {
                        let ht_val = ht.clone();
                        chips.push(view! {
                            <span class="matrix-chip">
                                <span class="chip-text">"Class: " <strong>{ht.clone()}</strong></span>
                                <button type="button" class="chip-x" title="Remove filter" on:click=move |_| state.update(|s| s.toggle_roster_hero_type(ht_val.clone()))>"✕"</button>
                            </span>
                        }.into_any());
                    }
                    if let Some(ref rt) = filter.range_type {
                        let rt_val = rt.clone();
                        chips.push(view! {
                            <span class="matrix-chip">
                                <span class="chip-text">"Combat: " <strong>{rt.clone()}</strong></span>
                                <button type="button" class="chip-x" title="Remove filter" on:click=move |_| state.update(|s| s.toggle_roster_range_type(rt_val.clone()))>"✕"</button>
                            </span>
                        }.into_any());
                    }
                    if let Some(ref res) = filter.resource_type {
                        let res_val = res.clone();
                        chips.push(view! {
                            <span class="matrix-chip">
                                <span class="chip-text">"Resource: " <strong>{res.clone()}</strong></span>
                                <button type="button" class="chip-x" title="Remove filter" on:click=move |_| state.update(|s| s.toggle_roster_resource_type(res_val.clone()))>"✕"</button>
                            </span>
                        }.into_any());
                    }
                    if let Some(diff) = filter.difficulty {
                        let diff_label = match diff {
                            1 => "Low (1)",
                            2 => "Medium (2)",
                            3 => "High (3)",
                            _ => "Custom",
                        };
                        chips.push(view! {
                            <span class="matrix-chip">
                                <span class="chip-text">"Difficulty: " <strong>{diff_label}</strong></span>
                                <button type="button" class="chip-x" title="Remove filter" on:click=move |_| state.update(|s| s.toggle_roster_difficulty(diff))>"✕"</button>
                            </span>
                        }.into_any());
                    }
                    if let Some(ref yr) = filter.release_year {
                        let yr_val = yr.clone();
                        chips.push(view! {
                            <span class="matrix-chip">
                                <span class="chip-text">"Debuted: " <strong>{yr.clone()}</strong></span>
                                <button type="button" class="chip-x" title="Remove filter" on:click=move |_| state.update(|s| s.toggle_roster_release_year(yr_val.clone()))>"✕"</button>
                            </span>
                        }.into_any());
                    }

                    if chips.is_empty() {
                        view! { <span class="matrix-no-chips"></span> }.into_any()
                    } else {
                        view! {
                            <div class="matrix-chips-tray">
                                {chips}
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Dynamic Matched Roster Section or Champion Revealed Showcase
            {move || {
                let list = filtered_champions.get();
                let count = list.len();
                let has_filter = is_filtered();

                if !has_filter {
                    return view! { <div style="display: none;"></div> }.into_any();
                }

                if count == 1 {
                    let champ = list[0].clone();
                    let champ_name = champ.name.clone();
                    let champ_name_btn = champ.name.clone();
                    let avatar = champion_avatar(&champ.name);
                    view! {
                        <div class="champion-reveal-heroic">
                            <div class="reveal-badge">"CHAMPION REVEALED"</div>
                            <div class="reveal-content">
                                <img class="reveal-avatar" src=avatar alt=champ.name.clone() />
                                <div class="reveal-info">
                                    <h2 class="reveal-name">{champ.name.clone()}</h2>
                                    <span class="reveal-title">{champ.title.clone()}</span>
                                    <div class="reveal-tags">
                                        <span class="reveal-tag">"Class: " {champ.herotype.clone()}</span>
                                        <span class="reveal-tag">"Combat: " {champ.range_type.clone()}</span>
                                        <span class="reveal-tag">"Resource: " {champ.resource_type.clone()}</span>
                                        <span class="reveal-tag">{format!("Difficulty {}/3", champ.difficulty)}</span>
                                        <span class="reveal-tag">{format!("Debuted in {}", champ.release_year)}</span>
                                    </div>
                                    <p class="reveal-trivia">
                                        {format!("Demographic Match: {} is the single champion across the entire 171-character roster uniquely satisfying all selected matrix parameters.", champ_name)}
                                    </p>
                                    <div class="reveal-actions">
                                        <button
                                            type="button"
                                            class="btn btn-secondary btn-md"
                                            style="display: inline-flex; align-items: center; gap: 6px;"
                                            on:click=move |_| state.update(|s| s.open_modal(crate::state::Modal::Champion(champ_name_btn.clone())))
                                        >
                                            {crate::player_analytics_view::view_lol_icon("hex-compass")}
                                            <span>"Inspect Details & Counters"</span>
                                        </button>
                                        <button
                                            type="button"
                                            class="btn btn-primary btn-md"
                                            on:click=move |_| state.update(crate::state::AppState::start_quiz)
                                        >
                                            "Calibrate Quiz for This Archetype →"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else if count > 1 {
                    let cards = list.into_iter().map(|c| {
                        let name_btn = c.name.clone();
                        let avatar = champion_avatar(&c.name);
                        view! {
                            <div
                                class="roster-micro-card"
                                role="button"
                                tabindex="0"
                                title=format!("Click to inspect {}", c.name)
                                on:click=move |_| state.update(|s| s.open_modal(crate::state::Modal::Champion(name_btn.clone())))
                            >
                                <img class="micro-avatar" src=avatar alt=c.name.clone() />
                                <div class="micro-meta">
                                    <strong class="micro-name">{c.name.clone()}</strong>
                                    <span class="micro-role">{format!("{} · {}", c.herotype, c.range_type)}</span>
                                </div>
                            </div>
                        }
                    }).collect_view();

                    view! {
                        <div class="matched-roster-section">
                            <div class="roster-section-header">
                                <h3 style="display: inline-flex; align-items: center; gap: 6px;">
                                    {crate::player_analytics_view::view_lol_icon("arcane-matrix")}
                                    <span>{format!("Matched Champions ({})", count)}</span>
                                </h3>
                                <span class="roster-section-subtitle">"Click any champion card below to inspect full stats, matchups, and counters."</span>
                            </div>
                            <div class="matched-roster-grid">
                                {cards}
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="zero-match-banner">
                            <div class="zero-match-title">
                                <span style="display:inline-flex;width:15px;height:15px;margin-right:6px;vertical-align:middle;">
                                    {crate::player_analytics_view::view_lol_icon("arcane-matrix")}
                                </span>
                                "No Champions Match This Exact Combination"
                            </div>
                            <p>"No champions currently satisfy all of the active filters simultaneously. Try removing a filter to broaden your query."</p>
                            <button
                                type="button"
                                class="btn btn-secondary btn-sm"
                                style="display: inline-flex; align-items: center; gap: 5px;"
                                on:click=move |_| state.update(|s| s.clear_roster_filter())
                            >
                                {crate::player_analytics_view::view_lol_icon("nexus-pulse")}
                                <span>"Reset All Filters"</span>
                            </button>
                        </div>
                    }.into_any()
                }
            }}

            <div class="chart-grid">
                // Tier 1: Champions per Hero Type (Full Width)
                <div class="chart-container chart-container-full">
                    <div class="chart-header-block">
                        <h2 class="chart-title">"Champions per Hero Type"</h2>
                        <span class="chart-subtitle">"Click a bar to filter by class · 6 Core Archetypes"</span>
                    </div>
                    <div class="chart-wrapper">
                        {render_hero_type_chart(&summary.hero_types, state)}
                    </div>
                </div>

                // Tier 2: Combat Engagement (Left) & Mechanical Curve (Right)
                <div class="chart-container">
                    <div class="chart-header-block">
                        <h2 class="chart-title">"Melee vs. Ranged"</h2>
                        <span class="chart-subtitle">"Click a slice to filter combat engagement"</span>
                    </div>
                    <div class="chart-wrapper">
                        {render_range_type_chart(&summary.range_types, summary.total_champions, state)}
                    </div>
                </div>

                <div class="chart-container">
                    <div class="chart-header-block">
                        <h2 class="chart-title">"Champion Difficulty Distribution"</h2>
                        <span class="chart-subtitle">"Click a tier to filter by mechanical curve"</span>
                    </div>
                    <div class="chart-wrapper">
                        {render_difficulty_chart(&summary.difficulties, summary.total_champions, state)}
                    </div>
                </div>

                // Tier 3: Resource Systems (Full Width)
                <div class="chart-container chart-container-full">
                    <div class="chart-header-block">
                        <h2 class="chart-title">"Resource Systems"</h2>
                        <span class="chart-badge-synthetic">"Click a bar to filter by resource · Official Riot Games Data Dragon"</span>
                    </div>
                    <div class="chart-wrapper">
                        {render_resource_type_chart(&summary.resource_types, state)}
                    </div>
                    <p class="chart-disclosure-note">
                        {summary.resource_disclosure.clone()}
                    </p>
                </div>

                // Tier 4: Historical Debut Timeline (Full Width)
                <div class="chart-container chart-container-full">
                    <div class="chart-header-block">
                        <h2 class="chart-title">"Champions Released by Year"</h2>
                        <span class="chart-subtitle">"Historical champion debuts from 2009 launch to 2026 · Click a node to filter by era"</span>
                    </div>
                    <div class="chart-wrapper chart-wrapper-wide">
                        {render_release_year_chart(&summary.release_years, state)}
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Helper for SVG arc generation in donut slices
fn donut_slice_path(
    cx: f64,
    cy: f64,
    r_in: f64,
    r_out: f64,
    start_angle: f64,
    end_angle: f64,
) -> String {
    let x1 = cx + r_out * start_angle.cos();
    let y1 = cy + r_out * start_angle.sin();
    let x2 = cx + r_out * end_angle.cos();
    let y2 = cy + r_out * end_angle.sin();
    let x3 = cx + r_in * end_angle.cos();
    let y3 = cy + r_in * end_angle.sin();
    let x4 = cx + r_in * start_angle.cos();
    let y4 = cy + r_in * start_angle.sin();
    let angle_diff = (end_angle - start_angle).abs();
    let large_arc = if angle_diff > PI { 1 } else { 0 };
    format!(
        "M {x1:.2},{y1:.2} A {r_out:.2},{r_out:.2} 0 {large_arc} 1 {x2:.2},{y2:.2} L {x3:.2},{y3:.2} A {r_in:.2},{r_in:.2} 0 {large_arc} 0 {x4:.2},{y4:.2} Z"
    )
}

#[cfg(target_arch = "wasm32")]
fn render_hero_type_chart(slices: &[MetricSlice], state: RwSignal<AppState>) -> impl IntoView {
    let max_val = 50.0;
    let width = 900.0;
    let height = 340.0;
    let left_margin = 55.0;
    let right_margin = 35.0;
    let top_margin = 30.0;
    let bottom_margin = 45.0;
    let plot_w = width - left_margin - right_margin;
    let plot_h = height - top_margin - bottom_margin;

    let colors = [
        ("#f43f5e", "#be123c"), // Fighter (Rose)
        ("#3b82f6", "#1d4ed8"), // Mage (Blue)
        ("#eab308", "#a16207"), // Marksman (Gold)
        ("#10b981", "#047857"), // Tank (Emerald)
        ("#a855f7", "#7e22ce"), // Assassin (Purple)
        ("#06b6d4", "#0e7490"), // Support (Cyan)
    ];

    let n = slices.len().max(1);
    let step = plot_w / n as f64;
    let bar_width = (step * 0.52).clamp(48.0, 72.0);

    let bars = slices
        .iter()
        .enumerate()
        .map(|(i, slice)| {
            let cx = left_margin + i as f64 * step + step / 2.0;
            let bar_h = (slice.count as f64 / max_val) * plot_h;
            let bar_y = top_margin + plot_h - bar_h;
            let bar_x = cx - bar_width / 2.0;
            let (c_top, c_bot) = colors
                .get(i % colors.len())
                .copied()
                .unwrap_or(("#6366f1", "#4338ca"));

            let grad_id = format!("hero_grad_{i}");
            let label = slice.label.clone();
            let label_click = slice.label.clone();
            let label_click_txt = slice.label.clone();
            let count = slice.count;

            view! {
                <g class="chart-bar-group" style="cursor: pointer;">
                    <defs>
                        <linearGradient id=grad_id.clone() x1="0" y1="0" x2="0" y2="1">
                            <stop offset="0%" stop-color=c_top />
                            <stop offset="100%" stop-color=c_bot />
                        </linearGradient>
                    </defs>
                    {
                        let l = label.clone();
                        move || {
                            let is_active = state.with(|s| s.roster_filter.hero_type.as_deref() == Some(&l));
                            let stroke = if is_active { "#ffd700" } else { "rgba(255,255,255,0.25)" };
                            let stroke_w = if is_active { "2.5" } else { "1" };
                            let l_inner = l.clone();
                            view! {
                                <rect
                                    x=format!("{bar_x:.1}")
                                    y=format!("{bar_y:.1}")
                                    width=format!("{bar_width:.1}")
                                    height=format!("{bar_h:.1}")
                                    rx="6"
                                    fill=format!("url(#{grad_id})")
                                    stroke=stroke
                                    stroke-width=stroke_w
                                    on:click=move |_| state.update(|s| s.toggle_roster_hero_type(l_inner.clone()))
                                >
                                    <title>{format!("{l_inner}: {count} champions (Click to filter)")}</title>
                                </rect>
                            }
                        }
                    }
                    <text
                        x=format!("{cx:.1}")
                        y=format!("{:.1}", bar_y - 6.0)
                        fill="#ffffff"
                        font-size="12"
                        font-weight="bold"
                        text-anchor="middle"
                        on:click=move |_| state.update(|s| s.toggle_roster_hero_type(label_click.clone()))
                    >
                        {count}
                    </text>
                    <text
                        x=format!("{cx:.1}")
                        y=format!("{:.1}", height - 12.0)
                        fill="#cbd5e1"
                        font-size="12"
                        font-weight="600"
                        text-anchor="middle"
                        on:click=move |_| state.update(|s| s.toggle_roster_hero_type(label_click_txt.clone()))
                    >
                        {label}
                    </text>
                </g>
            }
        })
        .collect_view();

    let grid_lines = [0, 10, 20, 30, 40, 50]
        .into_iter()
        .map(|tick| {
            let y = top_margin + plot_h - (tick as f64 / max_val) * plot_h;
            view! {
                <g>
                    <line
                        x1=format!("{left_margin:.1}")
                        y1=format!("{y:.1}")
                        x2=format!("{:.1}", width - right_margin)
                        y2=format!("{y:.1}")
                        stroke="rgba(255,255,255,0.08)"
                        stroke-dasharray="3 3"
                    />
                    <text
                        x=format!("{:.1}", left_margin - 8.0)
                        y=format!("{:.1}", y + 4.0)
                        fill="#94a3b8"
                        font-size="11"
                        text-anchor="end"
                    >
                        {tick}
                    </text>
                </g>
            }
        })
        .collect_view();

    view! {
        <svg
            id="heroTypeSvg"
            class="analytics-svg"
            viewBox=format!("0 0 {width} {height}")
            preserveAspectRatio="xMidYMid meet"
            role="img"
            aria-label="Champions per Hero Type chart"
        >
            {grid_lines}
            {bars}
        </svg>
    }
}

#[cfg(target_arch = "wasm32")]
fn render_range_type_chart(
    slices: &[MetricSlice],
    total: usize,
    state: RwSignal<AppState>,
) -> impl IntoView {
    let width = 460.0;
    let height = 300.0;
    let cx = 150.0;
    let cy = 150.0;
    let r_out = 100.0;
    let r_in = 60.0;

    let colors = [("#3b82f6", "#2563eb"), ("#f43f5e", "#e11d48")];
    let mut current_angle = -PI / 2.0;

    let mut paths = Vec::new();
    let mut legend_items = Vec::new();

    for (i, slice) in slices.iter().enumerate() {
        let frac = if total > 0 {
            slice.count as f64 / total as f64
        } else {
            0.0
        };
        let slice_angle = frac * 2.0 * PI;
        let end_angle = current_angle + slice_angle;

        let path_d = donut_slice_path(cx, cy, r_in, r_out, current_angle, end_angle);
        let (color, _) = colors
            .get(i % colors.len())
            .copied()
            .unwrap_or(("#6366f1", "#4338ca"));

        let label_for_path = slice.label.clone();
        let label_for_legend = slice.label.clone();
        let count = slice.count;
        let pct = format!("{:.1}%", frac * 100.0);
        let pct_tooltip = pct.clone();

        let lp = label_for_path.clone();
        paths.push(view! {
            {
                let lp_inner = lp.clone();
                let path_d_c = path_d.clone();
                let pct_tt = pct_tooltip.clone();
                move || {
                    let is_active = state.with(|s| s.roster_filter.range_type.as_deref() == Some(&lp_inner));
                    let stroke = if is_active { "#ffd700" } else { "#16213e" };
                    let stroke_w = if is_active { "3.5" } else { "2" };
                    let lp_click = lp_inner.clone();
                    view! {
                        <path
                            d=path_d_c.clone()
                            fill=color
                            stroke=stroke
                            stroke-width=stroke_w
                            style="cursor: pointer;"
                            on:click=move |_| state.update(|s| s.toggle_roster_range_type(lp_click.clone()))
                        >
                            <title>{format!("{lp_inner}: {count} champions ({pct_tt}) · Click to filter")}</title>
                        </path>
                    }
                }
            }
        });

        legend_items.push(view! {
            <div
                class="legend-row"
                style="cursor: pointer;"
                on:click=move |_| state.update(|s| s.toggle_roster_range_type(label_for_legend.clone()))
            >
                <span class="legend-swatch" style=format!("background-color: {color};")></span>
                <span class="legend-label">{slice.label.clone()}</span>
                <span class="legend-value">{format!("{} ({})", slice.count, pct)}</span>
            </div>
        });

        current_angle = end_angle;
    }

    view! {
        <div class="donut-chart-layout">
            <svg
                id="rangeTypeSvg"
                class="analytics-svg donut-svg"
                viewBox=format!("0 0 {width} {height}")
                preserveAspectRatio="xMidYMid meet"
                role="img"
                aria-label="Melee vs Ranged chart"
            >
                {paths}
                <text
                    x=format!("{cx:.1}")
                    y=format!("{:.1}", cy - 5.0)
                    fill="#ffffff"
                    font-size="22"
                    font-weight="bold"
                    text-anchor="middle"
                >
                    {total}
                </text>
                <text
                    x=format!("{cx:.1}")
                    y=format!("{:.1}", cy + 16.0)
                    fill="#94a3b8"
                    font-size="12"
                    text-anchor="middle"
                >
                    "Champions"
                </text>
            </svg>
            <div class="donut-legend">
                {legend_items}
            </div>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn render_resource_type_chart(slices: &[MetricSlice], state: RwSignal<AppState>) -> impl IntoView {
    let max_val = 160.0;
    let width = 1000.0;
    let height = 370.0;
    let left_margin = 55.0;
    let right_margin = 35.0;
    let top_margin = 35.0;
    let bottom_margin = 85.0;
    let plot_w = width - left_margin - right_margin;
    let plot_h = height - top_margin - bottom_margin;

    let colors = [
        ("#3b82f6", "#1d4ed8"), // Mana (Arcane Blue)
        ("#94a3b8", "#475569"), // None / Manaless (Slate Steel)
        ("#eab308", "#a16207"), // Energy (Ionian Amber)
        ("#f97316", "#c2410c"), // Fury (Burning Orange)
        ("#06b6d4", "#0891b2"), // Flow (Wind Cyan)
        ("#a855f7", "#7e22ce"), // Rage (Primal Violet)
        ("#b91c1c", "#7f1d1d"), // Blood Well (Darkin Crimson)
        ("#f59e0b", "#b45309"), // Courage (Noxian Gold)
        ("#64748b", "#334155"), // Shield (Iron Armor)
        ("#10b981", "#047857"), // Ferocity (Hunter Emerald)
        ("#ec4899", "#be185d"), // Heat (Yordle Pink)
        ("#d97706", "#92400e"), // Grit (Pit Fighter Bronze)
        ("#ef4444", "#991b1b"), // Crimson Rush (Hemomancer Red)
    ];

    let n = slices.len().max(1);
    let step = plot_w / n as f64;
    let bar_width = (step * 0.55).clamp(24.0, 44.0);

    let bars = slices
        .iter()
        .enumerate()
        .map(|(i, slice)| {
            let cx = left_margin + i as f64 * step + step / 2.0;
            let bar_h = if slice.count > 0 {
                ((slice.count as f64 / max_val) * plot_h).max(6.0)
            } else {
                0.0
            };
            let bar_y = top_margin + plot_h - bar_h;
            let bar_x = cx - bar_width / 2.0;
            let (c_top, c_bot) = colors
                .get(i % colors.len())
                .copied()
                .unwrap_or(("#6366f1", "#4338ca"));

            let grad_id = format!("res_grad_{i}");
            let label = slice.label.clone();
            let label_click = slice.label.clone();
            let label_click_txt = slice.label.clone();
            let count = slice.count;
            let label_y = top_margin + plot_h + 16.0;

            view! {
                <g class="chart-bar-group" style="cursor: pointer;">
                    <defs>
                        <linearGradient id=grad_id.clone() x1="0" y1="0" x2="0" y2="1">
                            <stop offset="0%" stop-color=c_top />
                            <stop offset="100%" stop-color=c_bot />
                        </linearGradient>
                    </defs>
                    {
                        let l = label.clone();
                        move || {
                            let is_active = state.with(|s| s.roster_filter.resource_type.as_deref() == Some(&l));
                            let stroke = if is_active { "#ffd700" } else { "rgba(255,255,255,0.25)" };
                            let stroke_w = if is_active { "2.5" } else { "1" };
                            let l_inner = l.clone();
                            view! {
                                <rect
                                    x=format!("{bar_x:.1}")
                                    y=format!("{bar_y:.1}")
                                    width=format!("{bar_width:.1}")
                                    height=format!("{bar_h:.1}")
                                    rx="5"
                                    fill=format!("url(#{grad_id})")
                                    stroke=stroke
                                    stroke-width=stroke_w
                                    on:click=move |_| state.update(|s| s.toggle_roster_resource_type(l_inner.clone()))
                                >
                                    <title>{format!("{l_inner}: {count} champions · Click to filter")}</title>
                                </rect>
                            }
                        }
                    }
                    <text
                        x=format!("{cx:.1}")
                        y=format!("{:.1}", bar_y - 6.0)
                        fill="#ffffff"
                        font-size="12"
                        font-weight="bold"
                        text-anchor="middle"
                        on:click=move |_| state.update(|s| s.toggle_roster_resource_type(label_click.clone()))
                    >
                        {count}
                    </text>
                    <text
                        x=format!("{cx:.1}")
                        y=format!("{label_y:.1}")
                        transform=format!("rotate(-32, {cx:.1}, {label_y:.1})")
                        fill="#cbd5e1"
                        font-size="11.5"
                        font-weight="600"
                        text-anchor="end"
                        style="cursor: pointer;"
                        on:click=move |_| state.update(|s| s.toggle_roster_resource_type(label_click_txt.clone()))
                    >
                        {label}
                    </text>
                </g>
            }
        })
        .collect_view();

    let grid_lines = [0, 40, 80, 120, 160]
        .into_iter()
        .map(|tick| {
            let y = top_margin + plot_h - (tick as f64 / max_val) * plot_h;
            view! {
                <g>
                    <line
                        x1=format!("{left_margin:.1}")
                        y1=format!("{y:.1}")
                        x2=format!("{:.1}", width - right_margin)
                        y2=format!("{y:.1}")
                        stroke="rgba(255,255,255,0.08)"
                        stroke-dasharray="3 3"
                    />
                    <text
                        x=format!("{:.1}", left_margin - 8.0)
                        y=format!("{:.1}", y + 4.0)
                        fill="#94a3b8"
                        font-size="11"
                        text-anchor="end"
                    >
                        {tick}
                    </text>
                </g>
            }
        })
        .collect_view();

    view! {
        <svg
            id="resourceTypeSvg"
            class="analytics-svg"
            viewBox=format!("0 0 {width} {height}")
            preserveAspectRatio="xMidYMid meet"
            role="img"
            aria-label="Resource Types chart"
        >
            {grid_lines}
            {bars}
        </svg>
    }
}

#[cfg(target_arch = "wasm32")]
fn render_difficulty_chart(
    diffs: &[DifficultyMetric],
    total: usize,
    state: RwSignal<AppState>,
) -> impl IntoView {
    let width = 460.0;
    let height = 300.0;
    let cx = 150.0;
    let cy = 150.0;
    let r_out = 100.0;
    let r_in = 60.0;

    let colors = [
        ("#10b981", "#059669"), // Low (Green)
        ("#3b82f6", "#2563eb"), // Medium (Blue)
        ("#ef4444", "#dc2626"), // High (Red)
    ];

    let mut current_angle = -PI / 2.0;
    let mut paths = Vec::new();
    let mut legend_items = Vec::new();

    for (i, d) in diffs.iter().enumerate() {
        let frac = if total > 0 {
            d.count as f64 / total as f64
        } else {
            0.0
        };
        let slice_angle = frac * 2.0 * PI;
        let end_angle = current_angle + slice_angle;

        let path_d = donut_slice_path(cx, cy, r_in, r_out, current_angle, end_angle);
        let (color, _) = colors
            .get(i % colors.len())
            .copied()
            .unwrap_or(("#6366f1", "#4338ca"));

        let level = d.level;
        let label = d.label.clone();
        let count = d.count;
        let pct = format!("{:.1}%", frac * 100.0);
        let pct_tooltip = pct.clone();

        paths.push(view! {
            {
                let path_d_c = path_d.clone();
                let pct_tt = pct_tooltip.clone();
                move || {
                    let is_active = state.with(|s| s.roster_filter.difficulty == Some(level));
                    let stroke = if is_active { "#ffd700" } else { "#16213e" };
                    let stroke_w = if is_active { "3.5" } else { "2" };
                    view! {
                        <path
                            d=path_d_c.clone()
                            fill=color
                            stroke=stroke
                            stroke-width=stroke_w
                            style="cursor: pointer;"
                            on:click=move |_| state.update(|s| s.toggle_roster_difficulty(level))
                        >
                            <title>{format!("{label}: {count} champions ({pct_tt}) · Click to filter")}</title>
                        </path>
                    }
                }
            }
        });

        legend_items.push(view! {
            <div
                class="legend-row"
                style="cursor: pointer;"
                on:click=move |_| state.update(|s| s.toggle_roster_difficulty(level))
            >
                <span class="legend-swatch" style=format!("background-color: {color};")></span>
                <span class="legend-label">{d.label.clone()}</span>
                <span class="legend-value">{format!("{} ({})", d.count, pct)}</span>
            </div>
        });

        current_angle = end_angle;
    }

    view! {
        <div class="donut-chart-layout">
            <svg
                id="difficultySvg"
                class="analytics-svg donut-svg"
                viewBox=format!("0 0 {width} {height}")
                preserveAspectRatio="xMidYMid meet"
                role="img"
                aria-label="Champion Difficulty Distribution chart"
            >
                {paths}
                <text
                    x=format!("{cx:.1}")
                    y=format!("{:.1}", cy - 5.0)
                    fill="#ffffff"
                    font-size="20"
                    font-weight="bold"
                    text-anchor="middle"
                >
                    "3 Tiers"
                </text>
                <text
                    x=format!("{cx:.1}")
                    y=format!("{:.1}", cy + 16.0)
                    fill="#94a3b8"
                    font-size="12"
                    text-anchor="middle"
                >
                    "Complexity"
                </text>
            </svg>
            <div class="donut-legend">
                {legend_items}
            </div>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
fn render_release_year_chart(slices: &[MetricSlice], state: RwSignal<AppState>) -> impl IntoView {
    let max_val = 45.0;
    let width = 960.0;
    let height = 320.0;
    let left_margin = 50.0;
    let right_margin = 40.0;
    let top_margin = 35.0;
    let bottom_margin = 45.0;
    let plot_w = width - left_margin - right_margin;
    let plot_h = height - top_margin - bottom_margin;

    let n = slices.len().max(2);
    let step = plot_w / (n - 1) as f64;

    let points: Vec<(f64, f64)> = slices
        .iter()
        .enumerate()
        .map(|(i, slice)| {
            let x = left_margin + i as f64 * step;
            let y = top_margin + plot_h - (slice.count as f64 / max_val) * plot_h;
            (x, y)
        })
        .collect();

    let polyline_pts = points
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ");

    let baseline_y = top_margin + plot_h;
    let mut area_pts = format!("{left_margin:.1},{baseline_y:.1} ");
    area_pts.push_str(&polyline_pts);
    let last_x = points.last().map(|p| p.0).unwrap_or(width - right_margin);
    area_pts.push_str(&format!(" {last_x:.1},{baseline_y:.1}"));

    let grid_lines = [0, 10, 20, 30, 40]
        .into_iter()
        .map(|tick| {
            let y = top_margin + plot_h - (tick as f64 / max_val) * plot_h;
            view! {
                <g>
                    <line
                        x1=format!("{left_margin:.1}")
                        y1=format!("{y:.1}")
                        x2=format!("{:.1}", width - right_margin)
                        y2=format!("{y:.1}")
                        stroke="rgba(255,255,255,0.08)"
                        stroke-dasharray="3 3"
                    />
                    <text
                        x=format!("{:.1}", left_margin - 8.0)
                        y=format!("{:.1}", y + 4.0)
                        fill="#94a3b8"
                        font-size="11"
                        text-anchor="end"
                    >
                        {tick}
                    </text>
                </g>
            }
        })
        .collect_view();

    let dots = slices
        .iter()
        .zip(points.iter())
        .map(|(slice, &(x, y))| {
            let year = slice.label.clone();
            let count = slice.count;
            let y_c = year.clone();
            let y_c2 = year.clone();

            view! {
                <g class="chart-point-group" style="cursor: pointer;">
                    {
                        let y_inner = y_c.clone();
                        move || {
                            let is_active = state.with(|s| s.roster_filter.release_year.as_deref() == Some(&y_inner));
                            let r = if is_active { "7" } else { "4.5" };
                            let fill = if is_active { "#ffd700" } else { "#8b5cf6" };
                            let stroke_w = if is_active { "2.5" } else { "2" };
                            let y_click = y_inner.clone();
                            view! {
                                <circle
                                    cx=format!("{x:.1}")
                                    cy=format!("{y:.1}")
                                    r=r
                                    fill=fill
                                    stroke="#ffffff"
                                    stroke-width=stroke_w
                                    on:click=move |_| state.update(|s| s.toggle_roster_release_year(y_click.clone()))
                                >
                                    <title>{format!("{y_inner}: {count} champions released · Click to filter")}</title>
                                </circle>
                            }
                        }
                    }
                    <text
                        x=format!("{x:.1}")
                        y=format!("{:.1}", y - 9.0)
                        fill="#ffffff"
                        font-size="11"
                        font-weight="bold"
                        text-anchor="middle"
                        on:click=move |_| state.update(|s| s.toggle_roster_release_year(y_c.clone()))
                    >
                        {count}
                    </text>
                    <text
                        x=format!("{x:.1}")
                        y=format!("{:.1}", baseline_y + 18.0)
                        fill="#94a3b8"
                        font-size="11"
                        font-weight="500"
                        text-anchor="middle"
                        on:click=move |_| state.update(|s| s.toggle_roster_release_year(y_c2.clone()))
                    >
                        {year}
                    </text>
                </g>
            }
        })
        .collect_view();

    view! {
        <svg
            id="releaseYearSvg"
            class="analytics-svg"
            viewBox=format!("0 0 {width} {height}")
            preserveAspectRatio="xMidYMid meet"
            role="img"
            aria-label="Champions Released by Year chart"
        >
            <defs>
                <linearGradient id="areaGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="0%" stop-color="#8b5cf6" stop-opacity="0.38" />
                    <stop offset="100%" stop-color="#8b5cf6" stop-opacity="0.02" />
                </linearGradient>
            </defs>
            {grid_lines}
            <polygon
                points=area_pts
                fill="url(#areaGradient)"
            />
            <polyline
                points=polyline_pts
                fill="none"
                stroke="#8b5cf6"
                stroke-width="3"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
            {dots}
        </svg>
    }
}
