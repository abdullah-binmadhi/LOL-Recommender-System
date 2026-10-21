//! Pure Rust native SVG analytics dashboard for League of Legends champions.
//!
//! Replaces external Chart.js and CDN dependencies with deterministic, responsive
//! SVG charts for hero types, combat ranges, difficulty, release chronology,
//! and disclosed illustrative resource types.

use crate::state::AppState;
use leptos::prelude::*;
use recommender_core::analytics::{AnalyticsSummary, DifficultyMetric, MetricSlice};
use std::f64::consts::PI;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
pub fn view_analytics(state: RwSignal<AppState>, summary: Arc<AnalyticsSummary>) -> impl IntoView {
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
                    style="border-color: var(--cyan); color: var(--cyan);"
                    on:click=move |_| {
                        if let Some(win) = web_sys::window() {
                            let _ = win.location().set_hash("#analytics");
                        }
                        state.update(AppState::open_player_analytics);
                    }
                >
                    "✦ Summoner Intelligence (Player Analytics) →"
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

            <div class="chart-grid">
                <div class="chart-container">
                    <div class="chart-header-block">
                        <h2 class="chart-title">"Champions per Hero Type"</h2>
                        <span class="chart-subtitle">"Distribution across primary character archetypes"</span>
                    </div>
                    <div class="chart-wrapper">
                        {render_hero_type_chart(&summary.hero_types)}
                    </div>
                </div>

                <div class="chart-container">
                    <div class="chart-header-block">
                        <h2 class="chart-title">"Melee vs. Ranged"</h2>
                        <span class="chart-subtitle">"Combat engagement distance ratio"</span>
                    </div>
                    <div class="chart-wrapper">
                        {render_range_type_chart(&summary.range_types, summary.total_champions)}
                    </div>
                </div>

                <div class="chart-container">
                    <div class="chart-header-block">
                        <h2 class="chart-title">"Resource Types"</h2>
                        <span class="chart-badge-synthetic">"⚠️ Illustrative Dataset"</span>
                    </div>
                    <div class="chart-wrapper">
                        {render_resource_type_chart(&summary.resource_types)}
                    </div>
                    <p class="chart-disclosure-note">
                        {summary.resource_disclosure.clone()}
                    </p>
                </div>

                <div class="chart-container">
                    <div class="chart-header-block">
                        <h2 class="chart-title">"Champion Difficulty Distribution"</h2>
                        <span class="chart-subtitle">"Riot mechanical learning curve ratings (1 to 3)"</span>
                    </div>
                    <div class="chart-wrapper">
                        {render_difficulty_chart(&summary.difficulties, summary.total_champions)}
                    </div>
                </div>

                <div class="chart-container chart-container-full">
                    <div class="chart-header-block">
                        <h2 class="chart-title">"Champions Released by Year"</h2>
                        <span class="chart-subtitle">"Historical champion debuts from 2009 launch to 2024"</span>
                    </div>
                    <div class="chart-wrapper chart-wrapper-wide">
                        {render_release_year_chart(&summary.release_years)}
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
fn render_hero_type_chart(slices: &[MetricSlice]) -> impl IntoView {
    let max_val = 50.0;
    let width = 600.0;
    let height = 300.0;
    let left_margin = 45.0;
    let right_margin = 25.0;
    let top_margin = 25.0;
    let bottom_margin = 40.0;
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
    let bar_width = (step * 0.58).clamp(32.0, 56.0);

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
            view! {
                <g class="chart-bar-group">
                    <defs>
                        <linearGradient id=grad_id.clone() x1="0" y1="0" x2="0" y2="1">
                            <stop offset="0%" stop-color=c_top />
                            <stop offset="100%" stop-color=c_bot />
                        </linearGradient>
                    </defs>
                    <rect
                        x=format!("{bar_x:.1}")
                        y=format!("{bar_y:.1}")
                        width=format!("{bar_width:.1}")
                        height=format!("{bar_h:.1}")
                        rx="6"
                        fill=format!("url(#{grad_id})")
                        stroke="rgba(255,255,255,0.25)"
                        stroke-width="1"
                    >
                        <title>{format!("{}: {} champions", slice.label, slice.count)}</title>
                    </rect>
                    <text
                        x=format!("{cx:.1}")
                        y=format!("{:.1}", bar_y - 6.0)
                        fill="#ffffff"
                        font-size="12"
                        font-weight="bold"
                        text-anchor="middle"
                    >
                        {slice.count}
                    </text>
                    <text
                        x=format!("{cx:.1}")
                        y=format!("{:.1}", height - 12.0)
                        fill="#cbd5e1"
                        font-size="12"
                        font-weight="600"
                        text-anchor="middle"
                    >
                        {slice.label.clone()}
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
fn render_range_type_chart(slices: &[MetricSlice], total: usize) -> impl IntoView {
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

        paths.push(view! {
            <path
                d=path_d
                fill=color
                stroke="#16213e"
                stroke-width="2"
            >
                <title>{format!("{}: {} champions ({:.1}%)", slice.label, slice.count, frac * 100.0)}</title>
            </path>
        });

        let pct = format!("{:.1}%", frac * 100.0);
        legend_items.push(view! {
            <div class="legend-row">
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
fn render_resource_type_chart(slices: &[MetricSlice]) -> impl IntoView {
    let max_val = 40.0;
    let width = 600.0;
    let height = 300.0;
    let left_margin = 45.0;
    let right_margin = 25.0;
    let top_margin = 25.0;
    let bottom_margin = 40.0;
    let plot_w = width - left_margin - right_margin;
    let plot_h = height - top_margin - bottom_margin;

    let colors = [
        ("#3b82f6", "#1d4ed8"), // Mana
        ("#ef4444", "#b91c1c"), // Fury
        ("#eab308", "#a16207"), // Energy
        ("#f97316", "#c2410c"), // Ferocity
        ("#8b5cf6", "#6d28d9"), // Rage
        ("#ec4899", "#be185d"), // Heat
        ("#64748b", "#334155"), // Other
    ];

    let n = slices.len().max(1);
    let step = plot_w / n as f64;
    let bar_width = (step * 0.58).clamp(24.0, 48.0);

    let bars = slices
        .iter()
        .enumerate()
        .map(|(i, slice)| {
            let cx = left_margin + i as f64 * step + step / 2.0;
            let bar_h = (slice.count as f64 / max_val) * plot_h;
            let bar_y = top_margin + plot_h - bar_h;
            let bar_x = cx - bar_width / 2.0;
            let (c_top, c_bot) = colors.get(i % colors.len()).copied().unwrap_or(("#6366f1", "#4338ca"));

            let grad_id = format!("resource_grad_{i}");
            view! {
                <g class="chart-bar-group">
                    <defs>
                        <linearGradient id=grad_id.clone() x1="0" y1="0" x2="0" y2="1">
                            <stop offset="0%" stop-color=c_top />
                            <stop offset="100%" stop-color=c_bot />
                        </linearGradient>
                    </defs>
                    <rect
                        x=format!("{bar_x:.1}")
                        y=format!("{bar_y:.1}")
                        width=format!("{bar_width:.1}")
                        height=format!("{bar_h:.1}")
                        rx="6"
                        fill=format!("url(#{grad_id})")
                        stroke="rgba(255,255,255,0.2)"
                        stroke-width="1"
                    >
                        <title>{format!("{}: {} champions (illustrative)", slice.label, slice.count)}</title>
                    </rect>
                    <text
                        x=format!("{cx:.1}")
                        y=format!("{:.1}", bar_y - 6.0)
                        fill="#ffffff"
                        font-size="12"
                        font-weight="bold"
                        text-anchor="middle"
                    >
                        {slice.count}
                    </text>
                    <text
                        x=format!("{cx:.1}")
                        y=format!("{:.1}", height - 12.0)
                        fill="#cbd5e1"
                        font-size="12"
                        font-weight="600"
                        text-anchor="middle"
                    >
                        {slice.label.clone()}
                    </text>
                </g>
            }
        })
        .collect_view();

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
fn render_difficulty_chart(diffs: &[DifficultyMetric], total: usize) -> impl IntoView {
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

        paths.push(view! {
            <path
                d=path_d
                fill=color
                stroke="#16213e"
                stroke-width="2"
            >
                <title>{format!("{}: {} champions ({:.1}%)", d.label, d.count, frac * 100.0)}</title>
            </path>
        });

        let pct = format!("{:.1}%", frac * 100.0);
        legend_items.push(view! {
            <div class="legend-row">
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
fn render_release_year_chart(slices: &[MetricSlice]) -> impl IntoView {
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

    // Build polyline points string
    let polyline_pts = points
        .iter()
        .map(|(x, y)| format!("{x:.1},{y:.1}"))
        .collect::<Vec<_>>()
        .join(" ");

    // Build area polygon string (down to baseline)
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
            view! {
                <g class="chart-point-group">
                    <circle
                        cx=format!("{x:.1}")
                        cy=format!("{y:.1}")
                        r="4.5"
                        fill="#8b5cf6"
                        stroke="#ffffff"
                        stroke-width="2"
                    >
                        <title>{format!("{}: {} champions released", slice.label, slice.count)}</title>
                    </circle>
                    <text
                        x=format!("{x:.1}")
                        y=format!("{:.1}", y - 9.0)
                        fill="#ffffff"
                        font-size="11"
                        font-weight="bold"
                        text-anchor="middle"
                    >
                        {slice.count}
                    </text>
                    <text
                        x=format!("{x:.1}")
                        y=format!("{:.1}", baseline_y + 18.0)
                        fill="#94a3b8"
                        font-size="11"
                        font-weight="500"
                        text-anchor="middle"
                    >
                        {slice.label.clone()}
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
