use leptos::prelude::*;

/// Accessible tab strip. The parent owns `active` and renders the matching panel.
#[component]
pub fn TabBar(
    tabs: Vec<(&'static str, &'static str)>,
    active: RwSignal<&'static str>,
) -> impl IntoView {
    let items = tabs
        .into_iter()
        .map(|(id, label)| {
            view! {
                <button
                    type="button"
                    role="tab"
                    class=move || if active.get() == id { "tab active" } else { "tab" }
                    aria-selected=move || if active.get() == id { "true" } else { "false" }
                    on:click=move |_| active.set(id)
                >
                    {label}
                </button>
            }
        })
        .collect_view();

    view! { <div class="tabbar" role="tablist">{items}</div> }
}
