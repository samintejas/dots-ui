//! Delegated tooltips: any element with `data-tooltip="text"` gets one on hover (after 300ms) and on
//! keyboard focus. Hoverable, dismissible with esc, closed on leave, blur or scroll (WCAG 1.4.13).
//!
//! Install once by rendering [`TooltipLayer`] (included in `DotsProvider`).

use std::cell::{Cell, RefCell};
use std::time::Duration;

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{AddEventListenerOptions, Element, Event, HtmlElement, KeyboardEvent, MouseEvent, Node};

use crate::util::uid;

struct Tip {
    el: HtmlElement,
    target: Element,
}

thread_local! {
    static INSTALLED: Cell<bool> = const { Cell::new(false) };
    static TIP: RefCell<Option<Tip>> = const { RefCell::new(None) };
    static TIMER: RefCell<Option<TimeoutHandle>> = const { RefCell::new(None) };
}

fn clear_timer() {
    if let Some(h) = TIMER.with(|t| t.borrow_mut().take()) {
        h.clear();
    }
}

fn hide_tip() {
    clear_timer();
    if let Some(tip) = TIP.with(|t| t.borrow_mut().take()) {
        let _ = tip.target.remove_attribute("aria-describedby");
        tip.el.remove();
    }
}

fn place_tip() {
    TIP.with(|t| {
        let t = t.borrow();
        let Some(tip) = t.as_ref() else { return };
        let r = tip.target.get_bounding_client_rect();
        let tr = tip.el.get_bounding_client_rect();
        let vw = window().inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1024.0);
        let x = (r.left() + r.width() / 2.0 - tr.width() / 2.0).max(8.0).min(vw - tr.width() - 8.0);
        let mut y = r.top() - tr.height() - 6.0;
        if y < 8.0 {
            y = r.bottom() + 6.0;
        }
        let style = tip.el.style();
        let _ = style.set_property("left", &format!("{}px", x.round()));
        let _ = style.set_property("top", &format!("{}px", y.round()));
    });
}

fn show_tip(target: Element) {
    hide_tip();
    let Some(text) = target.get_attribute("data-tooltip") else { return };
    let doc = document();
    let Ok(el) = doc.create_element("div") else { return };
    let Ok(el) = el.dyn_into::<HtmlElement>() else { return };
    el.set_class_name("d-tooltip");
    let id = uid("tip");
    el.set_id(&id);
    let _ = el.set_attribute("role", "tooltip");
    el.set_text_content(Some(&text));
    let host: Option<Element> = target.closest("dialog[open]").ok().flatten().or_else(|| doc.body().map(Into::into));
    let Some(host) = host else { return };
    let _ = host.append_child(&el);
    let _ = target.set_attribute("aria-describedby", &id);

    let leave_target = target.clone();
    let on_leave = Closure::<dyn FnMut(MouseEvent)>::new(move |e: MouseEvent| {
        let related = e.related_target().and_then(|t| t.dyn_into::<Node>().ok());
        if !related.map(|r| leave_target.contains(Some(&r))).unwrap_or(false) {
            hide_tip();
        }
    });
    let _ = el.add_event_listener_with_callback("mouseleave", on_leave.as_ref().unchecked_ref());
    on_leave.forget();

    TIP.with(|t| *t.borrow_mut() = Some(Tip { el, target }));
    place_tip();
}

fn closest_target(e: &Event) -> Option<Element> {
    let el = e.target()?.dyn_into::<Element>().ok()?;
    el.closest("[data-tooltip]").ok().flatten()
}

fn listen(name: &str, capture: bool, f: impl FnMut(Event) + 'static) {
    let cb = Closure::<dyn FnMut(Event)>::new(f);
    let opts = AddEventListenerOptions::new();
    opts.set_capture(capture);
    let _ = document().add_event_listener_with_callback_and_add_event_listener_options(
        name,
        cb.as_ref().unchecked_ref(),
        &opts,
    );
    cb.forget();
}

fn install() {
    if INSTALLED.with(|i| i.replace(true)) {
        return;
    }
    listen("mouseover", false, |e| {
        let Some(t) = closest_target(&e) else { return };
        if TIP.with(|tip| tip.borrow().as_ref().map(|x| x.target == t).unwrap_or(false)) {
            return;
        }
        clear_timer();
        let handle = set_timeout_with_handle(move || show_tip(t), Duration::from_millis(300)).ok();
        TIMER.with(|slot| *slot.borrow_mut() = handle);
    });
    listen("mouseout", false, |e| {
        let Some(t) = closest_target(&e) else { return };
        let related = e
            .dyn_ref::<MouseEvent>()
            .and_then(|m| m.related_target())
            .and_then(|r| r.dyn_into::<Node>().ok());
        if let Some(r) = &related {
            let in_target = t.contains(Some(r));
            let in_tip = TIP.with(|tip| tip.borrow().as_ref().map(|x| x.el.contains(Some(r))).unwrap_or(false));
            if in_target || in_tip {
                return;
            }
        }
        let focused = document().active_element().map(|a| a == t).unwrap_or(false)
            && t.matches(":focus-visible").unwrap_or(false);
        let is_current = TIP.with(|tip| tip.borrow().as_ref().map(|x| x.target == t).unwrap_or(false));
        if is_current && focused {
            return;
        }
        clear_timer();
        if is_current {
            hide_tip();
        }
    });
    listen("focusin", false, |e| {
        if let Some(t) = closest_target(&e) {
            if t.matches(":focus-visible").unwrap_or(false) {
                show_tip(t);
            }
        }
    });
    listen("focusout", false, |e| {
        let Some(el) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        if TIP.with(|tip| tip.borrow().as_ref().map(|x| x.target == el).unwrap_or(false)) {
            hide_tip();
        }
    });
    listen("keydown", false, |e| {
        if e.dyn_ref::<KeyboardEvent>().map(|k| k.key() == "Escape").unwrap_or(false)
            && TIP.with(|tip| tip.borrow().is_some())
        {
            hide_tip();
        }
    });
    listen("scroll", true, |_| hide_tip());
    let resize = Closure::<dyn FnMut()>::new(place_tip);
    let _ = window().add_event_listener_with_callback("resize", resize.as_ref().unchecked_ref());
    resize.forget();
}

/// Installs the document-level tooltip listeners, once. Renders nothing.
#[component]
pub fn TooltipLayer() -> impl IntoView {
    Effect::new(|_| install());
}
