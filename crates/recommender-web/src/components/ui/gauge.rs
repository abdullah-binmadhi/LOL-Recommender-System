use crate::presenter::gauge_geometry;
use leptos::prelude::*;

const RADIUS: f64 = 52.0;

/// Radial compatibility gauge rendered as pure SVG. `id` keeps the gradient unique.
#[component]
pub fn RadialGauge(
    score: f64,
    #[prop(into)] id: String,
    #[prop(into, optional)] caption: String,
) -> impl IntoView {
    let geometry = gauge_geometry(score, RADIUS);
    let gradient = format!("{id}-gradient");
    let stroke = format!("url(#{gradient})");
    let rounded = score.clamp(0.0, 100.0).round();

    view! {
        <div class="gauge" role="img" aria-label=format!("{rounded:.0} percent compatibility")>
            <svg viewBox="0 0 120 120" class="gauge-svg" aria-hidden="true">
                <defs>
                    <linearGradient id=gradient x1="0" y1="0" x2="1" y2="1">
                        <stop offset="0%" stop-color="#f0d99a"></stop>
                        <stop offset="100%" stop-color="#00c8b9"></stop>
                    </linearGradient>
                </defs>
                <circle class="gauge-track" cx="60" cy="60" r=RADIUS fill="none" stroke-width="9"></circle>
                <circle
                    class="gauge-arc"
                    cx="60"
                    cy="60"
                    r=RADIUS
                    fill="none"
                    stroke=stroke
                    stroke-width="9"
                    stroke-linecap="round"
                    stroke-dasharray=format!("{:.2}", geometry.circumference)
                    stroke-dashoffset=format!("{:.2}", geometry.dash_offset)
                    transform="rotate(-90 60 60)"
                ></circle>
            </svg>
            <div class="gauge-center">
                <span class="gauge-value">{format!("{rounded:.0}")}<small>"%"</small></span>
                <span class="gauge-caption">{caption}</span>
            </div>
        </div>
    }
}
