use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::util::dom;

#[derive(Clone)]
struct SegCtx {
    value: RwSignal<String>,
}

/// A radio group drawn as joined buttons: pick one of two to five short options (theme, billing cycle, kind).
///
/// `value` is the selected option's value. Left/right (and up/down) move and select, home/end jump.
/// Give it a `label`; for more than five options use a [`Select`](crate::Select).
#[component]
pub fn Segmented(
    value: RwSignal<String>,
    #[prop(into)] label: String,
    /// `control-h-lg` (40px) instead of `control-h` (32px).
    #[prop(optional)]
    large: bool,
    /// Full width: the options share it.
    #[prop(optional)]
    fill: bool,
    children: Children,
) -> impl IntoView {
    provide_context(SegCtx { value });
    let class = format!("d-seg{}{}", if large { " d-seg--lg" } else { "" }, if fill { " d-seg--fill" } else { "" });
    let onkey = move |e: leptos::ev::KeyboardEvent| {
        let Some(group) = e.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        let ok: Vec<_> = dom::query_all(&group, "[role=radio]").into_iter().filter(dom::enabled).collect();
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
    view! { <div class=class role="radiogroup" aria-label=label on:keydown=onkey>{children()}</div> }
}

/// One option of a [`Segmented`]. An icon-only option needs an `aria-label` (and usually a `data-tooltip`).
#[component]
pub fn SegOption(
    #[prop(into)] value: String,
    #[prop(optional, into)] aria_label: Option<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<SegCtx>();
    let (v1, v2, v3) = (value.clone(), value.clone(), value);
    let tip = aria_label.clone();
    view! {
        <button
            type="button" class="d-seg__opt" role="radio" aria-label=aria_label data-tooltip=tip
            aria-checked=move || if ctx.value.get() == v1 { "true" } else { "false" }
            tabindex=move || if ctx.value.get() == v2 { "0" } else { "-1" }
            disabled=move || disabled.get()
            on:click=move |_| ctx.value.set(v3.clone())
        >
            {children()}
        </button>
    }
}
