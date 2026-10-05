use leptos::prelude::*;

use crate::util::{tf, when_true};

/// A bounded surface at elevation 1. It rests on the page, so it never casts a shadow.
#[component]
pub fn Card(
    /// Marks the card as selected (left accent bar). For a static card this is `aria-current`.
    #[prop(optional, into)]
    selected: MaybeProp<bool>,
    #[prop(optional, into)] busy: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <section
            class="d-card"
            aria-current=move || selected.get().and_then(|s| s.then_some("true"))
            aria-busy=move || when_true(busy.get())
            aria-invalid=move || when_true(invalid.get())
        >
            {children()}
        </section>
    }
}

/// The card header: a lowercase title and optional meta text.
#[component]
pub fn CardHead(
    #[prop(into)] title: String,
    #[prop(optional, into)] meta: Option<String>,
) -> impl IntoView {
    view! {
        <header class="d-card__head">
            <h3 class="d-card__title">{title}</h3>
            {meta.map(|m| view! { <span class="d-card__meta">{m}</span> })}
        </header>
    }
}

#[component]
pub fn CardBody(
    /// Remove the padding for a list or table inside.
    #[prop(optional)]
    flush: bool,
    children: Children,
) -> impl IntoView {
    view! { <div class=if flush { "d-card__body d-card__body--flush" } else { "d-card__body" }>{children()}</div> }
}

#[component]
pub fn CardFoot(children: Children) -> impl IntoView {
    view! { <footer class="d-card__foot">{children()}</footer> }
}

/// An interactive card: the whole card is one button. Children must be `<span>`s.
#[component]
pub fn CardButton(
    #[prop(optional, into)] selected: MaybeProp<bool>,
    #[prop(optional, into)] busy: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="d-card"
            aria-pressed=move || selected.get().map(tf)
            aria-busy=move || when_true(busy.get())
            aria-invalid=move || when_true(invalid.get())
            disabled=move || disabled.get()
        >
            {children()}
        </button>
    }
}

/// An interactive card that links somewhere.
#[component]
pub fn CardLink(
    #[prop(into)] href: String,
    #[prop(optional, into)] current: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! { <a href=href class="d-card" aria-current=move || current.get().then_some("page")>{children()}</a> }
}
