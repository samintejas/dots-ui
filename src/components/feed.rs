use leptos::prelude::*;

use crate::types::Severity;

/// A short list of things that happened, newest first: notifications, activity. Wrap [`FeedItem`]s.
#[component]
pub fn Feed(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! { <ul class="d-feed" aria-label=label>{children()}</ul> }
}

/// One entry of a [`Feed`]. With `href` it is a link, with `on_click` a button, with neither plain text.
/// An unread entry has a filled mark and a medium title; `severity` colours the mark.
#[component]
pub fn FeedItem(
    #[prop(into)] title: String,
    #[prop(optional, into)] text: Option<String>,
    /// Already formatted: `today`, `2h`, `05 oct`.
    #[prop(into)]
    time: String,
    #[prop(optional, into)] unread: Signal<bool>,
    #[prop(optional)] severity: Option<Severity>,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let sev = severity.map(Severity::as_str);
    let body = move || view! {
        <span class="d-feed__mark" aria-hidden="true"></span>
        <span class="d-feed__body">
            <span class="d-feed__title">
                {move || unread.get().then(|| view! { <span class="d-sr">"unread: "</span> })}
                {title.clone()}
            </span>
            {text.clone().map(|t| view! { <span class="d-feed__text">{t}</span> })}
        </span>
        <span class="d-feed__time">{time.clone()}</span>
    };
    let flag = move || unread.get().then_some("");
    view! {
        <li>
            {match (href, on_click) {
                (Some(h), _) => view! { <a class="d-feed__item" href=h data-unread=flag data-severity=sev>{body()}</a> }.into_any(),
                (None, Some(c)) => view! { <button type="button" class="d-feed__item" data-unread=flag data-severity=sev on:click=move |_| c.run(())>{body()}</button> }.into_any(),
                _ => view! { <div class="d-feed__item" data-unread=flag data-severity=sev>{body()}</div> }.into_any(),
            }}
        </li>
    }
}
