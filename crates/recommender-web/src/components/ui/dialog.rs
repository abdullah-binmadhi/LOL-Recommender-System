use leptos::ev;
use leptos::prelude::*;

/// Modal overlay. Clicking the backdrop or the close button calls `on_close`;
/// Escape is handled once at the application root.
#[component]
pub fn Dialog(
    #[prop(into)] on_close: Callback<()>,
    #[prop(into)] label: String,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="modal active" role="presentation" on:click=move |_| on_close.run(())>
            <section
                class=format!("modal-content {class}")
                role="dialog"
                aria-modal="true"
                aria-label=label
                on:click=move |event: ev::MouseEvent| event.stop_propagation()
            >
                <button
                    type="button"
                    class="modal-close"
                    aria-label="Close dialog"
                    on:click=move |_| on_close.run(())
                >
                    "×"
                </button>
                {children()}
            </section>
        </div>
    }
}
