use leptos::prelude::*;

/// Slim determinate progress bar; `percent` is clamped to 0..=100.
#[component]
pub fn Progress(
    #[prop(into)] percent: Signal<f64>,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] label: String,
) -> impl IntoView {
    let width = move || format!("width: {:.1}%", percent.get().clamp(0.0, 100.0));
    view! {
        <div
            class=format!("progress {class}")
            role="progressbar"
            aria-label=label
            aria-valuemin="0"
            aria-valuemax="100"
            aria-valuenow=move || format!("{:.0}", percent.get().clamp(0.0, 100.0))
        >
            <div class="progress-fill" style=width></div>
        </div>
    }
}
