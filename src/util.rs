//! Small shared helpers. Nothing here is public API.

use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::prelude::*;
use wasm_bindgen::JsCast;

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// A document-unique id for linking labels, panels and descriptions.
///
/// With SSR/hydration this draws from the framework's shared counter so server and client agree; in plain
/// CSR it falls back to a process counter.
pub(crate) fn uid(prefix: &str) -> String {
    let n = match Owner::current_shared_context() {
        Some(sc) => sc.next_id().into_inner(),
        None => NEXT_ID.fetch_add(1, Ordering::Relaxed),
    };
    format!("d-{prefix}-{n}")
}

/// `aria-*` attributes that must say "true"/"false" explicitly.
pub(crate) fn tf(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

/// An attribute that is present as `"true"` or absent.
pub(crate) fn when_true(value: bool) -> Option<&'static str> {
    value.then_some("true")
}

/// DOM helpers used from event handlers and effects (client only).
pub(crate) mod dom {
    use super::*;
    use web_sys::{Element, HtmlElement};

    pub fn reduced_motion() -> bool {
        window()
            .match_media("(prefers-reduced-motion: reduce)")
            .ok()
            .flatten()
            .map(|m| m.matches())
            .unwrap_or(false)
    }

    /// Anything that is not `disabled` or `aria-disabled="true"`.
    pub fn enabled(el: &Element) -> bool {
        !el.has_attribute("disabled") && el.get_attribute("aria-disabled").as_deref() != Some("true")
    }

    pub fn query_all(root: &Element, selector: &str) -> Vec<Element> {
        let Ok(list) = root.query_selector_all(selector) else {
            return Vec::new();
        };
        (0..list.length())
            .filter_map(|i| list.item(i))
            .filter_map(|n| n.dyn_into::<Element>().ok())
            .collect()
    }

    pub fn active_element() -> Option<Element> {
        document().active_element()
    }

    pub fn focus(el: &Element) {
        if let Some(h) = el.dyn_ref::<HtmlElement>() {
            let _ = h.focus();
        }
    }

    /// Add a listener to `document` for the life of the page (the closure is leaked on purpose).
    pub fn listen_document(name: &str, capture: bool, f: impl FnMut(web_sys::Event) + 'static) {
        let cb = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(f);
        let opts = web_sys::AddEventListenerOptions::new();
        opts.set_capture(capture);
        let _ = document().add_event_listener_with_callback_and_add_event_listener_options(
            name,
            cb.as_ref().unchecked_ref(),
            &opts,
        );
        cb.forget();
    }

    /// Add a listener to `window` for the life of the page.
    pub fn listen_window(name: &str, f: impl FnMut(web_sys::Event) + 'static) {
        let cb = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::Event)>::new(f);
        let _ = window().add_event_listener_with_callback(name, cb.as_ref().unchecked_ref());
        cb.forget();
    }
}
