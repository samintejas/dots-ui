//! Dropdown and context menus: a raised list with 28px rows, one level, no submenus.
//!
//! The panel is moved to the end of `<body>` (or the open dialog) while it is shown, so nothing clips it.

use std::cell::RefCell;

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{Element, HtmlElement};

use crate::util::dom;

struct Open {
    menu: HtmlElement,
    trigger: Option<Element>,
    back: Option<Element>,
}

thread_local! {
    static OPEN: RefCell<Option<Open>> = const { RefCell::new(None) };
    static INSTALLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn close_menu(refocus: bool) {
    let Some(open) = OPEN.with(|o| o.borrow_mut().take()) else { return };
    open.menu.set_hidden(true);
    let _ = open.menu.class_list().remove_1("d-menu--floating");
    if let Some(t) = &open.trigger {
        let _ = t.set_attribute("aria-expanded", "false");
    }
    if refocus {
        if let Some(b) = &open.back {
            dom::focus(b);
        }
    }
}

fn install() {
    if INSTALLED.with(|i| i.replace(true)) {
        return;
    }
    dom::listen_document("pointerdown", false, |e| {
        let target = e.target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        let inside = OPEN.with(|o| {
            o.borrow().as_ref().map(|open| {
                let in_menu = target.as_ref().map(|t| open.menu.contains(Some(t))).unwrap_or(false);
                let in_trigger = match (&open.trigger, &target) {
                    (Some(tr), Some(t)) => tr.contains(Some(t)),
                    _ => false,
                };
                in_menu || in_trigger
            })
        });
        if inside == Some(false) {
            close_menu(false);
        }
    });
    dom::listen_window("blur", |_| close_menu(false));
    dom::listen_window("resize", |_| close_menu(false));
}

fn enabled_items(menu: &Element) -> Vec<Element> {
    dom::query_all(menu, "[role^=menuitem]").into_iter().filter(dom::enabled).collect()
}

fn place(menu: &HtmlElement, x: f64, y: f64, anchor: Option<&web_sys::DomRect>) {
    let style = menu.style();
    let _ = style.set_property("left", "0px");
    let _ = style.set_property("top", "0px");
    let r = menu.get_bounding_client_rect();
    let vw = window().inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(1024.0);
    let vh = window().inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(768.0);
    let (mut x, mut y) = (x, y);
    if x + r.width() > vw - 8.0 {
        x = (vw - r.width() - 8.0).max(8.0);
    }
    if y + r.height() > vh - 8.0 {
        y = match anchor {
            Some(a) => (a.top() - r.height() - 4.0).max(8.0),
            None => (vh - r.height() - 8.0).max(8.0),
        };
    }
    let _ = style.set_property("left", &format!("{}px", x.round()));
    let _ = style.set_property("top", &format!("{}px", y.round()));
}

fn show_menu(
    menu: &HtmlElement,
    x: f64,
    y: f64,
    back: Option<Element>,
    trigger: Option<Element>,
    anchor: Option<web_sys::DomRect>,
    match_width: bool,
    focus_last: bool,
) {
    install();
    close_menu(false);
    let host: Option<Element> = trigger
        .as_ref()
        .or(back.as_ref())
        .and_then(|t| t.closest("dialog[open]").ok().flatten())
        .or_else(|| document().body().map(Into::into));
    if let Some(host) = host {
        if menu.parent_node().as_ref() != Some(host.as_ref()) {
            let _ = host.append_child(menu);
        }
    }
    if match_width {
        if let Some(t) = &trigger {
            let _ = menu.style().set_property("min-width", &format!("{}px", t.get_bounding_client_rect().width().round()));
        }
    }
    menu.set_hidden(false);
    let _ = menu.class_list().add_1("d-menu--floating");
    place(menu, x, y, anchor.as_ref());
    if let Some(t) = &trigger {
        let _ = t.set_attribute("aria-expanded", "true");
    }
    OPEN.with(|o| *o.borrow_mut() = Some(Open { menu: menu.clone(), trigger, back }));
    let items = enabled_items(menu);
    let pick = if focus_last { items.last() } else { items.first() };
    if let Some(i) = pick {
        dom::focus(i);
    }
}

fn is_open(menu: &HtmlElement) -> bool {
    OPEN.with(|o| o.borrow().as_ref().map(|x| &x.menu == menu).unwrap_or(false))
}

fn menu_keydown(buf: StoredValue<(String, f64)>) -> impl Fn(leptos::ev::KeyboardEvent) + 'static {
    move |e| {
        let Some(menu) = e.current_target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        let items = enabled_items(&menu);
        let i = dom::active_element().and_then(|a| items.iter().position(|x| *x == a));
        let n = items.len();
        let key = e.key();
        let j = match key.as_str() {
            "ArrowDown" if n > 0 => Some(i.map(|i| (i + 1) % n).unwrap_or(0)),
            "ArrowUp" if n > 0 => Some(i.map(|i| (i + n - 1) % n).unwrap_or(n - 1)),
            "Home" if n > 0 => Some(0),
            "End" if n > 0 => Some(n - 1),
            "Escape" => {
                e.prevent_default();
                e.stop_propagation();
                close_menu(true);
                return;
            }
            "Tab" => {
                close_menu(false);
                return;
            }
            k if k.chars().count() == 1 && k != " " && !e.ctrl_key() && !e.meta_key() && !e.alt_key() => {
                let now = js_sys::Date::now();
                let (mut text, last) = buf.get_value();
                if now - last > 500.0 {
                    text.clear();
                }
                text.push_str(&k.to_lowercase());
                if let Some(hit) = items.iter().find(|it| {
                    it.text_content().unwrap_or_default().trim().to_lowercase().starts_with(&text)
                }) {
                    dom::focus(hit);
                }
                buf.set_value((text, now));
                return;
            }
            _ => None,
        };
        if let Some(j) = j {
            e.prevent_default();
            dom::focus(&items[j]);
        }
    }
}

fn menu_click(e: leptos::ev::MouseEvent) {
    let Some(menu) = e.current_target().and_then(|t| t.dyn_into::<HtmlElement>().ok()) else { return };
    let item = e
        .target()
        .and_then(|t| t.dyn_into::<Element>().ok())
        .and_then(|t| t.closest("[role^=menuitem]").ok().flatten());
    let Some(item) = item else { return };
    if !dom::enabled(&item) {
        e.prevent_default();
        return;
    }
    // a multi-select menu stays open while checkboxes are toggled; its `done` button closes it
    let role = item.get_attribute("role").unwrap_or_default();
    if menu.has_attribute("data-multi") && role == "menuitemcheckbox" {
        return;
    }
    if is_open(&menu) {
        close_menu(true);
    }
}

/// A dropdown: `trigger` renders the button (one focusable element); `children` are the menu items.
///
/// The trigger gets `aria-haspopup`, `aria-expanded` and `aria-controls`, and no wrapper element is added, so
/// the layout around it is unchanged. Keyboard: enter/space/down on the trigger open and focus the first item
/// (up: the last); arrows, home/end and type-ahead move; enter/space activates; esc closes and returns focus;
/// tab closes.
#[component]
pub fn Menu(
    /// `aria-label` of the menu.
    #[prop(into)]
    label: String,
    #[prop(into)] trigger: ViewFn,
    /// Make the menu at least as wide as its trigger.
    #[prop(optional)]
    match_width: bool,
    /// Stay open while checkbox rows are toggled; give it a [`MenuFoot`] with `clear` and `done`, which stays pinned to the bottom while the rows scroll.
    #[prop(optional)]
    multi: bool,
    /// Extra classes on the panel, for a menu laid out differently (`d-apps` for [`AppSwitcher`](crate::AppSwitcher)).
    #[prop(optional)]
    panel_class: &'static str,
    children: Children,
) -> impl IntoView {
    let panel = NodeRef::<leptos::html::Div>::new();
    let buf = StoredValue::new((String::new(), 0.0));
    let id = crate::util::uid("menu");
    let trigger_el = StoredValue::new_local(None::<Element>);
    let listeners = StoredValue::new_local(Vec::<(&'static str, Closure<dyn FnMut(web_sys::Event)>)>::new());

    let open_at = move |focus_last: bool| {
        let (Some(t), Some(p)) = (trigger_el.get_value(), panel.get_untracked()) else { return };
        let r = t.get_bounding_client_rect();
        show_menu(&p, r.left(), r.bottom() + 4.0, Some(t.clone()), Some(t), Some(r), match_width, focus_last);
    };

    // the panel is the trigger's next sibling: wire the trigger once it is in the DOM
    Effect::new({
        let id = id.clone();
        move |_| {
            let Some(p) = panel.get() else { return };
            if trigger_el.get_value().is_some() {
                return;
            }
            let Some(t) = p.previous_element_sibling() else { return };
            let _ = t.set_attribute("aria-haspopup", "menu");
            let _ = t.set_attribute("aria-controls", &id);
            if !t.has_attribute("aria-expanded") {
                let _ = t.set_attribute("aria-expanded", "false");
            }
            let click = Closure::<dyn FnMut(web_sys::Event)>::new(move |_| {
                if let Some(p) = panel.get_untracked() {
                    if is_open(&p) {
                        close_menu(false);
                        return;
                    }
                }
                open_at(false);
            });
            let key = Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| {
                let Some(k) = e.dyn_ref::<web_sys::KeyboardEvent>() else { return };
                match k.key().as_str() {
                    "ArrowDown" => {
                        e.prevent_default();
                        open_at(false);
                    }
                    "ArrowUp" => {
                        e.prevent_default();
                        open_at(true);
                    }
                    _ => {}
                }
            });
            let _ = t.add_event_listener_with_callback("click", click.as_ref().unchecked_ref());
            let _ = t.add_event_listener_with_callback("keydown", key.as_ref().unchecked_ref());
            listeners.update_value(|l| {
                l.push(("click", click));
                l.push(("keydown", key));
            });
            trigger_el.set_value(Some(t));
        }
    });
    on_cleanup(move || {
        if let Some(t) = trigger_el.get_value() {
            listeners.with_value(|l| {
                for (name, cb) in l {
                    let _ = t.remove_event_listener_with_callback(name, cb.as_ref().unchecked_ref());
                }
            });
        }
        if let Some(p) = panel.get_untracked() {
            if is_open(&p) {
                close_menu(false);
            }
            p.remove();
        }
    });

    let keydown = menu_keydown(buf);
    view! {
        {trigger.run()}
        <div class=if panel_class.is_empty() { "d-menu".to_string() } else { format!("d-menu {panel_class}") } id=id role="menu" aria-label=label data-multi=multi.then_some("") hidden node_ref=panel on:keydown=keydown on:click=menu_click>
            {children()}
        </div>
    }
}

/// A context menu: right click, the menu key or shift+f10 on `target` opens `children` at the pointer.
#[component]
pub fn ContextMenu(
    #[prop(into)] label: String,
    /// The focusable region the menu belongs to.
    #[prop(into)]
    target: ViewFn,
    children: Children,
) -> impl IntoView {
    let region = NodeRef::<leptos::html::Div>::new();
    let panel = NodeRef::<leptos::html::Div>::new();
    let buf = StoredValue::new((String::new(), 0.0));
    on_cleanup(move || {
        if let Some(p) = panel.get_untracked() {
            if is_open(&p) {
                close_menu(false);
            }
            p.remove();
        }
    });
    let open_at_pointer = move |e: leptos::ev::MouseEvent| {
        e.prevent_default();
        let (Some(r), Some(p)) = (region.get_untracked(), panel.get_untracked()) else { return };
        let (mut x, mut y) = (f64::from(e.client_x()), f64::from(e.client_y()));
        if x == 0.0 && y == 0.0 {
            let b = r.get_bounding_client_rect();
            x = b.left() + 12.0;
            y = b.top() + 12.0;
        }
        show_menu(&p, x, y, Some(r.into()), None, None, false, false);
    };
    let onkey = move |e: leptos::ev::KeyboardEvent| {
        let is_menu_key = e.key() == "ContextMenu" || (e.shift_key() && e.key() == "F10");
        if !is_menu_key {
            return;
        }
        e.prevent_default();
        let (Some(r), Some(p)) = (region.get_untracked(), panel.get_untracked()) else { return };
        let b = r.get_bounding_client_rect();
        show_menu(&p, b.left() + 12.0, b.top() + 12.0, Some(r.into()), None, None, false, false);
    };
    let keydown = menu_keydown(buf);
    view! {
        <div node_ref=region tabindex="0" on:contextmenu=open_at_pointer on:keydown=onkey>{target.run()}</div>
        <div class="d-menu" role="menu" aria-label=label hidden node_ref=panel on:keydown=keydown on:click=menu_click>
            {children()}
        </div>
    }
}

/// A menu row. `on_select` runs when it is activated; the menu then closes and focus returns to the trigger.
#[component]
pub fn MenuItem(
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Last in the menu, after a separator, and its label names the action.
    #[prop(optional)]
    danger: bool,
    /// Secondary text, usually the shortcut.
    #[prop(optional, into)]
    meta: Option<String>,
    #[prop(optional, into)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type="button" role="menuitem" tabindex="-1"
            class=if danger { "d-menu__item d-menu__item--danger" } else { "d-menu__item" }
            aria-disabled=move || disabled.get().then_some("true")
            on:click=move |_| { if !disabled.get_untracked() { if let Some(cb) = on_select { cb.run(()) } } }
        >
            {children()}
            {meta.map(|m| view! { <span class="d-menu__meta">{m}</span> })}
        </button>
    }
}

/// A menu row that navigates.
#[component]
pub fn MenuLink(#[prop(into)] href: String, children: Children) -> impl IntoView {
    view! { <a class="d-menu__item" role="menuitem" tabindex="-1" href=href>{children()}</a> }
}

/// A toggleable row with a tick in the mark column.
#[component]
pub fn MenuCheckbox(
    checked: RwSignal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type="button" class="d-menu__item" role="menuitemcheckbox" tabindex="-1"
            aria-checked=move || if checked.get() { "true" } else { "false" }
            aria-disabled=move || disabled.get().then_some("true")
            on:click=move |_| { if !disabled.get_untracked() { checked.update(|c| *c = !*c) } }
        >
            <span class="d-menu__mark"></span>{children()}
        </button>
    }
}

#[derive(Clone, Copy)]
struct MenuRadioCtx {
    value: RwSignal<String>,
}

/// A labelled group of [`MenuRadio`] rows sharing one value (a theme picker, a sort order).
#[component]
pub fn MenuRadioGroup(#[prop(into)] label: String, value: RwSignal<String>, children: Children) -> impl IntoView {
    provide_context(MenuRadioCtx { value });
    let name = label.clone();
    view! {
        <div role="group" aria-label=name>
            <div class="d-menu__label" role="presentation">{label}</div>
            {children()}
        </div>
    }
}

#[component]
pub fn MenuRadio(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<MenuRadioCtx>();
    let (a, b) = (value.clone(), value);
    view! {
        <button
            type="button" class="d-menu__item" role="menuitemradio" tabindex="-1"
            aria-checked=move || if ctx.value.get() == a { "true" } else { "false" }
            aria-disabled=move || disabled.get().then_some("true")
            on:click=move |_| { if !disabled.get_untracked() { ctx.value.set(b.clone()) } }
        >
            <span class="d-menu__mark"></span>{children()}
        </button>
    }
}

#[component]
pub fn MenuSeparator() -> impl IntoView {
    view! { <div class="d-menu__sep" role="separator"></div> }
}

/// A small group heading. Not focusable.
#[component]
pub fn MenuLabel(children: Children) -> impl IntoView {
    view! { <div class="d-menu__label" role="presentation">{children()}</div> }
}

/// The account menu's header: name and email. Not focusable.
#[component]
pub fn MenuHead(children: Children) -> impl IntoView {
    view! { <div class="d-menu__head" role="presentation">{children()}</div> }
}


/// The footer of a multi-select menu: `clear` and `done` buttons. Close the menu from `done` with
/// [`close_open_menu`].
#[component]
pub fn MenuFoot(children: Children) -> impl IntoView {
    view! { <div class="d-menu__foot">{children()}</div> }
}

/// Close whichever menu is open and return focus to its trigger.
pub fn close_open_menu() {
    close_menu(true);
}
