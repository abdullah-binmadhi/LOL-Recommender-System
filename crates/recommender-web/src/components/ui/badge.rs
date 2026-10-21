use leptos::prelude::*;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Outline,
    Cyan,
    Success,
    Destructive,
}

#[allow(dead_code)]
#[component]
pub fn Badge(
    #[prop(into, optional)] variant: BadgeVariant,
    #[prop(into, optional)] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        BadgeVariant::Default => "ui-badge-gold",
        BadgeVariant::Secondary => "ui-badge-secondary",
        BadgeVariant::Outline => "ui-badge-outline",
        BadgeVariant::Cyan => "ui-badge-cyan",
        BadgeVariant::Success => "ui-badge-success",
        BadgeVariant::Destructive => "ui-badge-destructive",
    };

    view! {
        <span class=format!("ui-badge {variant_class} {class}")>
            {children()}
        </span>
    }
}
