use leptos::prelude::*;

use crate::icon::Icon;
use super::spinner::Spinner;

/// What a region shows when it has nothing to list: a title, one line of help, one secondary action.
#[component]
pub fn EmptyState(
    /// Lucide icon name. Defaults to `inbox`.
    #[prop(optional)]
    icon: Option<&'static str>,
    /// What is missing, in plain words.
    #[prop(into)]
    title: String,
    /// One line on what to do.
    #[prop(optional, into)]
    hint: Option<String>,
    /// The action, usually one secondary [`Button`](crate::Button).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="d-empty">
            <Icon name=icon.unwrap_or("inbox")/>
            <p class="d-empty__title">{title}</p>
            {hint.map(|h| view! { <p>{h}</p> })}
            {children.map(|c| c())}
        </div>
    }
}

/// The loading variant: a spinner and a title.
#[component]
pub fn EmptyLoading(#[prop(into)] title: String) -> impl IntoView {
    view! {
        <div class="d-empty" role="status">
            <Spinner decorative=true/>
            <p class="d-empty__title">{title}</p>
        </div>
    }
}

/// The error variant. The title is prefixed with `error:`; state the cause in `hint` and the fix in the action.
#[component]
pub fn EmptyError(
    #[prop(into)] title: String,
    #[prop(optional, into)] hint: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="d-empty" data-severity="danger" role="alert">
            <Icon name="octagon-alert"/>
            <p class="d-empty__title">"error: "{title}</p>
            {hint.map(|h| view! { <p>{h}</p> })}
            {children.map(|c| c())}
        </div>
    }
}
