use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Gold,
    Cyan,
}

#[component]
pub fn Badge(
    #[prop(optional)] variant: BadgeVariant,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let variant = match variant {
        BadgeVariant::Gold => "chip-gold",
        BadgeVariant::Cyan => "chip-cyan",
    };
    view! { <span class=format!("chip {variant} {class}")>{children()}</span> }
}
