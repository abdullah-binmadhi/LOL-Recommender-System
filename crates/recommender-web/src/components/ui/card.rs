#![allow(dead_code)]
use leptos::prelude::*;

#[component]
pub fn Card(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("ui-card {class}")>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("ui-card-header {class}")>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <h3 class=format!("ui-card-title {class}")>
            {children()}
        </h3>
    }
}

#[component]
pub fn CardDescription(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <p class=format!("ui-card-description {class}")>
            {children()}
        </p>
    }
}

#[component]
pub fn CardContent(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("ui-card-content {class}")>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div class=format!("ui-card-footer {class}")>
            {children()}
        </div>
    }
}
