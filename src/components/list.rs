
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::types::Size;
use crate::util::dom;

/// Rows of `row-h` (36px; `row-h-lg` 44px with `lg`) split by `border-subtle` lines. For a bordered list, put it in a flush [`CardBody`](crate::CardBody).
#[component]
pub fn List(#[prop(optional)] size: Size, children: Children) -> impl IntoView {
    view! { <ul class=if size == Size::Lg { "d-list d-list--lg" } else { "d-list" }>{children()}</ul> }
}

/// A row in a [`List`]: a link when `href` is set, otherwise static text.
///
/// `current` marks the page you are on (`aria-current="page"`: a 2px accent bar and `text-max`).
#[component]
pub fn ListItem(
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] current: Signal<bool>,
    /// Secondary text, right aligned.
    #[prop(optional, into)]
    meta: Option<String>,
    children: Children,
) -> impl IntoView {
    let meta = meta.map(|m| view! { <span class="d-list__meta">{m}</span> });
    let row = match href {
        Some(href) => view! {
            <a class="d-list__row" href=href aria-current=move || current.get().then_some("page")>{children()}{meta}</a>
        }
        .into_any(),
        None => view! {
            <div class="d-list__row" aria-current=move || current.get().then_some("true")>{children()}{meta}</div>
        }
        .into_any(),
    };
    view! { <li>{row}</li> }
}

#[derive(Clone)]
struct ListboxCtx {
    value: RwSignal<String>,
    first: StoredValue<Option<String>>,
}

/// A single-select listbox. `value` is the selected option's value ("" for none).
///
/// Up/down move and select, home/end jump, disabled options are skipped, one option is in the tab order.
#[component]
pub fn Listbox(
    #[prop(into)] label: String,
    value: RwSignal<String>,
    #[prop(optional)] size: Size,
    children: Children,
) -> impl IntoView {
    provide_context(ListboxCtx { value, first: StoredValue::new(None) });
    let onkey = move |e: leptos::ev::KeyboardEvent| {
        let Some(list) = e.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        let ok: Vec<_> = dom::query_all(&list, "[role=option]").into_iter().filter(dom::enabled).collect();
        let active = dom::active_element();
        let Some(i) = active.and_then(|a| ok.iter().position(|o| *o == a)) else { return };
        let last = ok.len() - 1;
        let j = match e.key().as_str() {
            "ArrowDown" => (i + 1).min(last),
            "ArrowUp" => i.saturating_sub(1),
            "Home" => 0,
            "End" => last,
            _ => return,
        };
        e.prevent_default();
        dom::focus(&ok[j]);
        if let Some(h) = ok[j].dyn_ref::<web_sys::HtmlElement>() {
            h.click();
        }
    };
    view! {
        <div class=if size == Size::Lg { "d-list d-list--lg" } else { "d-list" } role="listbox" aria-label=label on:keydown=onkey>
            {children()}
        </div>
    }
}

/// An option in a [`Listbox`].
#[component]
pub fn ListOption(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] meta: Option<String>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<ListboxCtx>();
    if ctx.first.get_value().is_none() && !disabled.get_untracked() {
        ctx.first.set_value(Some(value.clone()));
    }
    let (v1, v2, v3) = (value.clone(), value.clone(), value);
    let selected = move || ctx.value.get() == v1;
    let tab = {
        let selected = selected.clone();
        move || {
            let none = ctx.value.get().is_empty();
            if selected() || (none && ctx.first.get_value().as_deref() == Some(v2.as_str())) { "0" } else { "-1" }
        }
    };
    view! {
        <div
            class="d-list__row" role="option"
            aria-selected=move || if selected() { "true" } else { "false" }
            aria-disabled=move || disabled.get().then_some("true")
            tabindex=tab
            on:click=move |_| { if !disabled.get_untracked() { ctx.value.set(v3.clone()) } }
        >
            {children()}
            {meta.map(|m| view! { <span class="d-list__meta">{m}</span> })}
        </div>
    }
}

