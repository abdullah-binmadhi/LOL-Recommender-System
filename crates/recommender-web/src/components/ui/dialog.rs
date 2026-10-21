#![allow(dead_code)]
use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[component]
pub fn Dialog(
    #[prop(into)] on_close: Callback<()>,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class="ui-dialog-overlay"
            on:click=move |ev: ev::MouseEvent| {
                if let Some(target) = ev.target() {
                    if let Ok(el) = target.dyn_into::<web_sys::Element>() {
                        if el.class_list().contains("ui-dialog-overlay") {
                            on_close.run(());
                        }
                    }
                }
            }
        >
            <div class=format!("ui-dialog-content {class}") role="dialog" aria-modal="true">
                <button
                    type="button"
                    class="ui-dialog-close"
                    aria-label="Close dialog"
                    on:click=move |_| on_close.run(())
                >
                    "✕"
                </button>
                {children()}
            </div>
        </div>
    }
}
