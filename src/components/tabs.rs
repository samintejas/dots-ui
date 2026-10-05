use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::util::{dom, uid};

#[derive(Clone)]
struct TabsCtx {
    value: RwSignal<String>,
    base: String,
}

fn slug(s: &str) -> String {
    s.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect()
}

/// Switches between panels in place. `value` is the selected tab's value.
///
/// Compose it from [`TabList`], [`Tab`] and [`TabPanel`]. Two to six tabs; for navigation between pages use links.
#[component]
pub fn Tabs(value: RwSignal<String>, children: Children) -> impl IntoView {
    provide_context(TabsCtx { value, base: uid("tabs") });
    children()
}

/// The `role="tablist"`. Left/right move and select (wrapping), home/end jump.
#[component]
pub fn TabList(#[prop(into)] label: String, children: Children) -> impl IntoView {
    let onkey = move |e: leptos::ev::KeyboardEvent| {
        let Some(list) = e.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        let ok: Vec<_> = dom::query_all(&list, "[role=tab]").into_iter().filter(dom::enabled).collect();
        let Some(i) = dom::active_element().and_then(|a| ok.iter().position(|o| *o == a)) else { return };
        let n = ok.len();
        let j = match e.key().as_str() {
            "ArrowRight" => (i + 1) % n,
            "ArrowLeft" => (i + n - 1) % n,
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
    view! { <div class="d-tabs" role="tablist" aria-label=label on:keydown=onkey>{children()}</div> }
}

/// One tab. A count goes in a `Badge count`.
#[component]
pub fn Tab(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<TabsCtx>();
    let key = slug(&value);
    let id = format!("{}-t-{key}", ctx.base);
    let panel = format!("{}-p-{key}", ctx.base);
    let (v1, v2, v3) = (value.clone(), value.clone(), value);
    view! {
        <button
            type="button" class="d-tab" role="tab" id=id aria-controls=panel
            aria-selected=move || if ctx.value.get() == v1 { "true" } else { "false" }
            tabindex=move || if ctx.value.get() == v2 { "0" } else { "-1" }
            disabled=move || disabled.get()
            on:click=move |_| ctx.value.set(v3.clone())
        >
            {children()}
        </button>
    }
}

/// The panel for a tab. Hidden (not removed) while another tab is selected, so its state is kept.
#[component]
pub fn TabPanel(#[prop(into)] value: String, children: Children) -> impl IntoView {
    let ctx = expect_context::<TabsCtx>();
    let key = slug(&value);
    let id = format!("{}-p-{key}", ctx.base);
    let tab = format!("{}-t-{key}", ctx.base);
    view! {
        <div class="d-tabpanel" role="tabpanel" id=id aria-labelledby=tab tabindex="0" hidden=move || ctx.value.get() != value>
            {children()}
        </div>
    }
}
