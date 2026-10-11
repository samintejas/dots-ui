//! A feed: notifications and activity, newest first.

use leptos::prelude::*;

use crate::types::Severity;

/// A list of [`FeedItem`]s, ruled between rows. Put it in a right bar, a card or a menu.
#[component]
pub fn Feed(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! { <ul class="d-feed" aria-label=label>{children()}</ul> }
}

/// One notification or event: a mark, a title, optional text, and when it happened.
///
/// Unread rows show a filled 6px mark *and* a medium-weight title, and say `unread` to screen readers, so the
/// state is never the dot's colour alone. `severity` colours the unread mark (the title should say it too).
/// With `href` the row is a link; with `on_select` a button; with neither, plain text.
#[component]
pub fn FeedItem(
    #[prop(into)] title: Signal<String>,
    #[prop(optional, into)] text: Option<Signal<String>>,
    /// Already formatted: `today`, `2h`, `05 oct`.
    #[prop(into)]
    time: Signal<String>,
    #[prop(optional, into)] unread: Signal<bool>,
    #[prop(optional)] severity: Option<Severity>,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] on_select: Option<Callback<()>>,
) -> impl IntoView {
    let body = move || {
        view! {
            <span class="d-feed__mark" aria-hidden="true"></span>
            <span class="d-feed__body">
                <span class="d-feed__title">
                    {move || unread.get().then_some(view! { <span class="d-sr">"unread: "</span> })}
                    {move || title.get()}
                </span>
                {text.map(|t| view! { <span class="d-feed__text">{move || t.get()}</span> })}
            </span>
            <span class="d-feed__time">{move || time.get()}</span>
        }
    };
    let unread_attr = move || unread.get().then_some("");
    let sev = severity.map(|s| s.as_str());
    let row = match (href, on_select) {
        (Some(h), _) => view! { <a class="d-feed__item" href=h data-unread=unread_attr data-severity=sev>{body()}</a> }.into_any(),
        (None, Some(cb)) => view! {
            <button type="button" class="d-feed__item" data-unread=unread_attr data-severity=sev on:click=move |_| cb.run(())>{body()}</button>
        }
        .into_any(),
        (None, None) => view! { <div class="d-feed__item" data-unread=unread_attr data-severity=sev>{body()}</div> }.into_any(),
    };
    view! { <li>{row}</li> }
}
