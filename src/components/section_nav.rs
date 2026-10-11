//! The sections of one page, listed down its side: settings, an account, a long form.

use leptos::prelude::*;

use crate::icon::Icon;

/// A `<nav>` of links to the sections of one page, each its own route (`/settings/profile`). The current one
/// has `aria-current="page"`, `text-max` and a 2px left bar. Below `bp-md` the list wraps across the top.
///
/// For panels that switch in place without changing the address, use [`Tabs`](crate::Tabs) with a vertical
/// [`TabList`](crate::TabList).
#[component]
pub fn SectionNav(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! { <nav class="d-secnav" aria-label=label>{children()}</nav> }
}

/// One section. Without `href` it is a button (use `on:click`).
#[component]
pub fn SectionNavItem(
    #[prop(optional, into)] href: Option<String>,
    /// A Lucide icon name.
    #[prop(optional)]
    icon: Option<&'static str>,
    #[prop(into)] label: String,
    #[prop(optional, into)] current: Signal<bool>,
    /// A neutral count badge at the end of the row.
    #[prop(optional, into)]
    count: Option<Signal<String>>,
) -> impl IntoView {
    let inner = move || {
        view! {
            {icon.map(|i| view! { <Icon name=i/> })}
            <span>{label.clone()}</span>
            {count.map(|c| view! { <span class="d-badge d-badge--count">{move || c.get()}</span> })}
        }
    };
    match href {
        Some(h) => view! {
            <a class="d-secnav__item" href=h aria-current=move || current.get().then_some("page")>{inner()}</a>
        }
        .into_any(),
        None => view! {
            <button type="button" class="d-secnav__item" aria-current=move || current.get().then_some("true")>{inner()}</button>
        }
        .into_any(),
    }
}

/// A small heading between groups of sections. Not focusable.
#[component]
pub fn SectionNavLabel(children: Children) -> impl IntoView {
    view! { <div class="d-secnav__label">{children()}</div> }
}

/// The page around a [`SectionNav`]: the nav in a 216px column, the open section beside it scrolling on its
/// own. Give it a height (the space under the app bar) for the two to scroll separately. The last child is
/// the panel.
#[component]
pub fn SectionNavLayout(
    /// A CSS height, like `calc(100dvh - 80px)`. Unset: as tall as its content.
    #[prop(optional, into)]
    height: Option<String>,
    children: Children,
) -> impl IntoView {
    view! { <div class="d-secnav-layout" style=height.map(|h| format!("height:{h}"))>{children()}</div> }
}
