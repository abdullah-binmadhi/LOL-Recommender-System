use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[default]
    Md,
    Lg,
}

/// Rust/UI style button. Attach handlers with `on:click` directly on the tag.
#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: ButtonSize,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(optional)] submit: bool,
    children: Children,
) -> impl IntoView {
    let variant = match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Ghost => "btn-ghost",
    };
    let size = match size {
        ButtonSize::Md => "btn-md",
        ButtonSize::Lg => "btn-lg",
    };
    let kind = if submit { "submit" } else { "button" };

    view! {
        <button type=kind class=format!("btn {variant} {size} {class}") disabled=move || disabled.get()>
            {children()}
        </button>
    }
}
