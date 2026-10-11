use leptos::prelude::*;

use crate::icon::Icon;

/// A page that is split into sections, with the section list beside the content. Put a [`SectionNav`] and the
/// section's content inside.
#[component]
pub fn SectionNavLayout(children: Children) -> impl IntoView {
    view! { <div class="d-secnav-layout">{children()}</div> }
}

/// Links to the sections of one page or settings area, with the current one marked.
#[component]
pub fn SectionNav(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! { <nav class="d-secnav" aria-label=label>{children()}</nav> }
}

/// One link of a [`SectionNav`]. A count goes in a `Badge count` child.
#[component]
pub fn SectionNavItem(
    #[prop(into)] href: String,
    #[prop(optional)] icon: Option<&'static str>,
    /// This is the section you are on (`aria-current="page"`).
    #[prop(optional, into)]
    current: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <a class="d-secnav__item" href=href aria-current=move || current.get().then_some("page")>
            {icon.map(|n| view! { <Icon name=n/> })}
            <span>{children()}</span>
        </a>
    }
}

/// A small heading between groups of links.
#[component]
pub fn SectionNavLabel(children: Children) -> impl IntoView {
    view! { <div class="d-secnav__label">{children()}</div> }
}
