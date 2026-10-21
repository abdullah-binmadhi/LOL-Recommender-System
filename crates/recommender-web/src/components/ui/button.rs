use leptos::ev;
use leptos::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Destructive,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Default,
    Lg,
    Icon,
}

#[allow(dead_code)]
#[component]
pub fn Button(
    #[prop(into, optional)] variant: ButtonVariant,
    #[prop(into, optional)] size: ButtonSize,
    #[prop(into, optional)] class: String,
    #[prop(into, optional)] disabled: Signal<bool>,
    #[prop(into, optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(into, optional)] button_type: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Primary => "ui-btn-primary",
        ButtonVariant::Secondary => "ui-btn-secondary",
        ButtonVariant::Outline => "ui-btn-outline",
        ButtonVariant::Ghost => "ui-btn-ghost",
        ButtonVariant::Destructive => "ui-btn-destructive",
    };

    let size_class = match size {
        ButtonSize::Sm => "ui-btn-sm",
        ButtonSize::Default => "ui-btn-md",
        ButtonSize::Lg => "ui-btn-lg",
        ButtonSize::Icon => "ui-btn-icon",
    };

    let full_class = format!("ui-btn {variant_class} {size_class} {class}");
    let b_type = button_type.unwrap_or("button");

    view! {
        <button
            type=b_type
            class=full_class
            disabled=move || disabled.get()
            on:click=move |ev| {
                if let Some(cb) = on_click {
                    cb.run(ev);
                }
            }
        >
            {children()}
        </button>
    }
}
