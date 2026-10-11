//! A segmented control: one choice from two to five options, drawn as one control.

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::types::Size;
use crate::util::dom;

#[derive(Clone, Copy)]
struct SegCtx {
    value: RwSignal<String>,
}

/// One choice out of a few, always visible (`expense / income / transfer`, `monthly / yearly`, a theme).
///
/// It is a `role="radiogroup"`: one tab stop, arrows move and select, home/end jump. Use it for two to five
/// short options that change what the form around it asks for; for more, or for a setting that is rarely
/// changed, use a [`Select`](crate::Select).
#[component]
pub fn Segmented(
    /// The selected option's value.
    value: RwSignal<String>,
    /// `aria-label` of the group. Or point at a visible label with `labelledby`.
    #[prop(optional, into)]
    label: Option<String>,
    /// Id of a visible label for the group.
    #[prop(optional, into)]
    labelledby: Option<String>,
    /// `Lg` is 36px tall, for side panels and forms drawn at the comfortable size.
    #[prop(optional)]
    size: Size,
    /// Stretch to the container, options sharing the width.
    #[prop(optional)]
    fill: bool,
    children: Children,
) -> impl IntoView {
    provide_context(SegCtx { value });
    let mut class = String::from("d-seg");
    if size == Size::Lg {
        class.push_str(" d-seg--lg");
    }
    if fill {
        class.push_str(" d-seg--fill");
    }
    let onkey = move |e: leptos::ev::KeyboardEvent| {
        let Some(root) = e.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        let ok: Vec<_> = dom::query_all(&root, ".d-seg__opt").into_iter().filter(dom::enabled).collect();
        let Some(i) = dom::active_element().and_then(|a| ok.iter().position(|o| *o == a)) else { return };
        let n = ok.len();
        let j = match e.key().as_str() {
            "ArrowRight" | "ArrowDown" => (i + 1) % n,
            "ArrowLeft" | "ArrowUp" => (i + n - 1) % n,
            "Home" => 0,
            "End" => n - 1,
            _ => return,
        };
        e.prevent_default();
        dom::focus(&ok[j]);
        if let Some(h) = ok[j].dyn_ref::<web_sys::HtmlElement>() {
            h.click();
        }
    };
    view! {
        <div class=class role="radiogroup" aria-label=label aria-labelledby=labelledby on:keydown=onkey>
            {children()}
        </div>
    }
}

/// One option. Children are its label; an icon-only option needs `label`.
#[component]
pub fn SegOption(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Accessible name and tooltip, for an option that shows only an icon.
    #[prop(optional, into)]
    label: Option<String>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<SegCtx>();
    let (a, b, c) = (value.clone(), value.clone(), value);
    let on = move || ctx.value.get() == a;
    view! {
        <button
            type="button" class="d-seg__opt" role="radio"
            aria-checked=move || if on() { "true" } else { "false" }
            tabindex=move || if ctx.value.get() == b { "0" } else { "-1" }
            aria-label=label.clone() data-tooltip=label
            disabled=move || disabled.get()
            on:click=move |_| ctx.value.set(c.clone())
        >
            {children()}
        </button>
    }
}
