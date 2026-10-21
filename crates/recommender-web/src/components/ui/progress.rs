#![allow(dead_code)]
use leptos::prelude::*;

#[component]
pub fn Progress(
    #[prop(into)] value: Signal<f64>,
    #[prop(into, optional)] max: Option<f64>,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let max_val = max.unwrap_or(100.0);
    let pct_style = move || {
        let v = value.get();
        let pct = if max_val > 0.0 {
            (v / max_val * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };
        format!("width: {pct:.1}%;")
    };

    view! {
        <div class=format!("ui-progress-track {class}") role="progressbar">
            <div class="ui-progress-indicator" style=pct_style />
        </div>
    }
}
