//! Page rhythm: parts of a thing 8 apart, things 16, sections 32. Sections are split by space before any rule.

use leptos::prelude::*;

/// The page column: sections `gap-section` (32px) apart. Usually the child of [`Main`](crate::Main).
#[component]
pub fn Page(children: Children) -> impl IntoView {
    view! { <div class="d-page">{children()}</div> }
}

/// A section of a page: a head (title and an optional line of meta) and its things `gap-outer` (16px) apart.
#[component]
pub fn Section(
    /// The section's heading (an `<h2>`). Leave unset for a section with no head.
    #[prop(optional, into)]
    title: Option<String>,
    /// A short line beside the title, in secondary text.
    #[prop(optional, into)]
    meta: Option<String>,
    children: Children,
) -> impl IntoView {
    let head = title.map(|t| {
        view! {
            <div class="d-section__head">
                <h2 class="d-section__title">{t}</h2>
                {meta.map(|m| view! { <p>{m}</p> })}
            </div>
        }
    });
    view! { <section class="d-section">{head}{children()}</section> }
}

/// Parts of one thing, stacked `gap-inner` (8px) apart.
#[component]
pub fn Stack(children: Children) -> impl IntoView {
    view! { <div class="d-stack">{children()}</div> }
}

/// Fields stacked `gap-outer` (16px) apart. A `<form>`: pass `on:submit=…`.
#[component]
pub fn FormStack(children: Children) -> impl IntoView {
    view! { <form class="d-form">{children()}</form> }
}

/// Cards or panels in as many 280px-or-wider columns as fit, `gap-outer` apart.
#[component]
pub fn Grid(children: Children) -> impl IntoView {
    view! { <div class="d-grid">{children()}</div> }
}

/// A row of buttons, one gap between them wherever they sit.
#[component]
pub fn Actions(children: Children) -> impl IntoView {
    view! { <div class="d-actions">{children()}</div> }
}

/// Centres content at `content-max` (1200px) with side padding.
#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! { <div class="d-container">{children()}</div> }
}

/// A block of prose set in Geist, at most `prose-max` (68ch) wide. Every `<p>` already sets in Geist; use this
/// for mixed blocks (paragraphs, lists, inline code).
#[component]
pub fn Prose(children: Children) -> impl IntoView {
    view! { <div class="d-prose">{children()}</div> }
}

/// Why a control is off, said next to it. A control is never disabled without one.
///
/// Give it an `id` and point the control's `aria-describedby` at it, with `aria_disabled` (stays focusable)
/// or `disabled` on the control: `<Button aria_disabled=true attr:aria-describedby="why">…</Button>
/// <Reason id="why">"only an owner can remove members"</Reason>`.
#[component]
pub fn Reason(
    #[prop(into)] id: String,
    /// Lucide icon in front of the text. Default `lock`.
    #[prop(optional)]
    icon: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! { <p class="d-reason" id=id><crate::Icon name=icon.unwrap_or("lock")/>{children()}</p> }
}

/// An inline text link (`accent-info`, underlined).
#[component]
pub fn Link(#[prop(into)] href: String, children: Children) -> impl IntoView {
    view! { <a class="d-link" href=href>{children()}</a> }
}
