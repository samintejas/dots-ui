use std::time::Duration;

use leptos::prelude::*;

use crate::components::kbd::Keys;
use crate::icon::Icon;
use crate::util::{dom, uid};

#[derive(Clone)]
struct AppBarCtx {
    open: RwSignal<bool>,
    panel_id: String,
}

/// The top bar, `bar-h` tall. Left: where you are. Right: search, status, notifications and the account menu.
///
/// One bar per page. Below 768px what does not fit (put it in [`AppBarCollapse`]) moves to a second line
/// inside the bar, opened by [`AppBarMenu`]. Page-level actions belong in the page, not here.
#[component]
pub fn AppBar(children: Children) -> impl IntoView {
    let open = RwSignal::new(false);
    provide_context(AppBarCtx { open, panel_id: uid("appbar-panel") });
    view! {
        <header
            class="d-appbar" data-open=move || open.get()
            on:keydown=move |e| {
                if e.key() == "Escape" && open.get_untracked() {
                    open.set(false);
                    if let Some(b) = document().query_selector(".d-appbar__menu").ok().flatten() {
                        dom::focus(&b);
                    }
                }
            }
        >
            {children()}
        </header>
    }
}

/// A 1px divider between the sidebar trigger and the breadcrumb.
#[component]
pub fn AppBarSep() -> impl IntoView {
    view! { <span class="d-appbar__sep" aria-hidden="true"></span> }
}

#[component]
pub fn AppBarSpacer() -> impl IntoView {
    view! { <span class="d-appbar__spacer"></span> }
}

/// A plain context title, when there is no breadcrumb.
#[component]
pub fn AppBarTitle(children: Children) -> impl IntoView {
    view! { <span class="d-appbar__title">{children()}</span> }
}

/// Top-level links, for apps without a sidebar.
#[component]
pub fn AppBarNav(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! { <nav class="d-appbar__nav" aria-label=label>{children()}</nav> }
}

#[component]
pub fn AppBarLink(
    #[prop(into)] href: String,
    #[prop(optional, into)] current: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <a class="d-appbar__link" href=href aria-current=move || current.get().then_some("page")>{children()}</a>
    }
}

/// A button styled as a field that opens the command palette. An icon below 768px.
#[component]
pub fn AppBarSearch(
    /// Defaults to `search`.
    #[prop(optional, into)]
    label: Option<String>,
    /// The shortcut shown as keycaps. Defaults to ctrl k.
    #[prop(optional, into)]
    keys: Option<Vec<String>>,
) -> impl IntoView {
    let keys = keys.unwrap_or_else(|| vec!["ctrl".into(), "k".into()]);
    view! {
        <button type="button" class="d-appbar__search" aria-keyshortcuts="Control+K">
            <Icon name="search" small=true/><span>{label.unwrap_or_else(|| "search".into())}</span>
            <Keys keys=keys/>
        </button>
    }
}

/// The button that opens the second line below 768px. Icon-only: give it a label (`status`).
#[component]
pub fn AppBarMenu(#[prop(into)] label: String) -> impl IntoView {
    let ctx = expect_context::<AppBarCtx>();
    let panel_id = ctx.panel_id.clone();
    let focus_id = ctx.panel_id.clone();
    let open = ctx.open;
    view! {
        <button
            type="button" class="d-btn d-btn--ghost d-btn--icon d-btn--sm d-appbar__menu"
            aria-label=label aria-controls=panel_id
            aria-expanded=move || if open.get() { "true" } else { "false" }
            on:click=move |_| {
                let next = !open.get_untracked();
                open.set(next);
                if next {
                    let id = focus_id.clone();
                    set_timeout(move || {
                        if let Some(p) = document().get_element_by_id(&id) {
                            if let Some(f) = p.query_selector("a[href], button:not([disabled]), [tabindex=\"0\"]").ok().flatten() {
                                dom::focus(&f);
                            }
                        }
                    }, Duration::from_millis(0));
                }
            }
        >
            <Icon name="ellipsis"/>
        </button>
    }
}

/// What moves to the second line on small screens: status chips and the clock.
#[component]
pub fn AppBarCollapse(children: Children) -> impl IntoView {
    let ctx = expect_context::<AppBarCtx>();
    view! { <div class="d-appbar__collapse" id=ctx.panel_id>{children()}</div> }
}

/// Status chips and the clock, grouped and labelled. Chips keep their words.
#[component]
pub fn AppBarCluster(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! { <div class="d-appbar__cluster" role="group" aria-label=label>{children()}</div> }
}

/// A `<time>` showing `hh:mm`. Not a live region; never announced on its own.
#[component]
pub fn AppBarClock() -> impl IntoView {
    let time = RwSignal::new(String::from("--:--"));
    let tick = move || {
        let d = js_sys::Date::new_0();
        let next = format!("{:02}:{:02}", d.get_hours(), d.get_minutes());
        if time.get_untracked() != next {
            time.set(next);
        }
    };
    Effect::new(move |_| tick());
    Effect::new(move |_| {
        if let Ok(h) = set_interval_with_handle(tick, Duration::from_secs(1)) {
            on_cleanup(move || h.clear());
        }
    });
    view! { <time class="d-appbar__clock" datetime=move || time.get()>{move || time.get()}</time> }
}

/// Always visible: notifications and the account avatar.
#[component]
pub fn AppBarActions(children: Children) -> impl IntoView {
    view! { <div class="d-appbar__actions">{children()}</div> }
}
