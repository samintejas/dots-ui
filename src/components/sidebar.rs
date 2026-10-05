use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use super::menu::{Menu, MenuRadio, MenuRadioGroup};
use super::shell::{Region, ShellToggle, try_shell, use_shell};
use crate::icon::Icon;
use crate::util::dom;

/// The app sidebar, the left tile of the [`Shell`](crate::Shell): it collapses to icons or to nothing and the
/// content resizes with it. Elevation 0, never a floating panel.
#[component]
pub fn Sidebar(
    /// The landmark's name. Default `sidebar`.
    #[prop(optional, into)]
    label: Option<String>,
    children: Children,
) -> impl IntoView {
    let shell = use_shell();
    shell.register(Region::Left, true);
    on_cleanup(move || shell.register(Region::Left, false));
    let open = shell.open_signal(Region::Left);
    view! {
        <aside
            class="d-sidebar" id=shell.id(Region::Left)
            aria-label=label.unwrap_or_else(|| "sidebar".into())
            inert=move || !open.get() && (shell.collapsible() == crate::Collapsible::Offcanvas || shell.is_mobile())
        >
            <div class="d-sidebar__inner">{children()}</div>
            <button
                type="button" class="d-sidebar__rail" tabindex="-1" aria-label="toggle sidebar"
                on:click=move |_| shell.toggle(Region::Left)
            ></button>
        </aside>
    }
}

/// Fixed at the top: the company switcher.
#[component]
pub fn SidebarHead(children: Children) -> impl IntoView {
    view! { <div class="d-sidebar__head">{children()}</div> }
}

/// Fixed at the bottom: the account menu.
#[component]
pub fn SidebarFoot(children: Children) -> impl IntoView {
    view! { <div class="d-sidebar__foot">{children()}</div> }
}

/// One entry of a [`SidebarSwitcher`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SwitcherItem {
    pub value: String,
    pub name: String,
    /// Two lowercase letters for the avatar.
    pub initials: String,
    /// Second line (a plan, a role).
    pub meta: String,
}

impl SwitcherItem {
    pub fn new(value: impl Into<String>, name: impl Into<String>, initials: impl Into<String>, meta: impl Into<String>) -> Self {
        SwitcherItem { value: value.into(), name: name.into(), initials: initials.into(), meta: meta.into() }
    }
}

/// The company switcher: a two-line tile that opens a menu of radio items and mirrors the chosen one.
/// `children` are extra menu items after a separator (`add company`).
#[component]
pub fn SidebarSwitcher(
    /// `aria-label` of the tile and the menu heading (`companies`).
    #[prop(into)]
    label: String,
    items: Vec<SwitcherItem>,
    value: RwSignal<String>,
    #[prop(optional, into)] on_change: Option<Callback<String>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let shell = try_shell();
    let items = StoredValue::new(items);
    let current = Signal::derive(move || {
        let v = value.get();
        items.with_value(|i| i.iter().find(|x| x.value == v).cloned())
    });
    Effect::new(move |prev: Option<String>| {
        let v = value.get();
        if let (Some(p), Some(cb)) = (prev, on_change) {
            if p != v {
                cb.run(v.clone());
            }
        }
        v
    });
    let tile_label = label.clone();
    let trigger = move || {
        let tip_attr = move || {
            let rail = shell.map(|s| s.icon_rail()).unwrap_or(false);
            rail.then(|| current.get().map(|c| c.name)).flatten()
        };
        view! {
            <button type="button" class="d-sidebar__tile" aria-label=tile_label.clone() data-tooltip=tip_attr>
                <span class="d-avatar" aria-hidden="true">{move || current.get().map(|c| c.initials)}</span>
                <span class="d-sidebar__who">
                    <b>{move || current.get().map(|c| c.name)}</b>
                    <small>{move || current.get().map(|c| c.meta)}</small>
                </span>
                <Icon name="chevrons-up-down"/>
            </button>
        }
    };
    view! {
        <Menu label=label.clone() trigger=trigger match_width=true>
            <MenuRadioGroup label=label value=value>
                {items.get_value().into_iter().map(|i| view! {
                    <MenuRadio value=i.value.clone()>
                        <span class="d-avatar d-avatar--sm" aria-hidden="true">{i.initials.clone()}</span>{i.name.clone()}
                    </MenuRadio>
                }).collect_view()}
            </MenuRadioGroup>
            {children.map(|c| view! { <div class="d-menu__sep" role="separator"></div>{c()} })}
        </Menu>
    }
}

/// The account tile at the foot of the sidebar. `children` are the menu items (profile, theme, sign out).
#[component]
pub fn SidebarAccount(
    /// Menu `aria-label`.
    #[prop(into)]
    label: String,
    #[prop(into)] initials: String,
    #[prop(into)] name: String,
    #[prop(optional, into)] meta: Option<String>,
    children: Children,
) -> impl IntoView {
    let shell = try_shell();
    let aria = format!("account: {name}");
    let tip = name.clone();
    let trigger = move || {
        let (initials, name, meta, aria, tip) = (initials.clone(), name.clone(), meta.clone(), aria.clone(), tip.clone());
        view! {
            <button
                type="button" class="d-sidebar__tile" aria-label=aria
                data-tooltip=move || shell.map(|s| s.icon_rail()).unwrap_or(false).then(|| tip.clone())
            >
                <span class="d-avatar" aria-hidden="true">{initials}</span>
                <span class="d-sidebar__who"><b>{name}</b>{meta.map(|m| view! { <small>{m}</small> })}</span>
                <Icon name="chevrons-up-down"/>
            </button>
        }
    };
    view! { <Menu label=label trigger=trigger match_width=true>{children()}</Menu> }
}

/// Filters the navigation as you type; esc clears. The match count is announced.
#[component]
pub fn SidebarFilter(#[prop(optional, into)] placeholder: Option<String>) -> impl IntoView {
    let status = RwSignal::new(String::new());
    let run = move |input: &HtmlElement, q: &str| {
        let Some(inner) = input.closest(".d-sidebar__inner").ok().flatten() else { return };
        status.set(filter_navigation(&inner, q));
    };
    view! {
        <div class="d-sidebar__search">
            <input
                class="d-input d-sidebar__filter" type="search"
                placeholder=placeholder.unwrap_or_else(|| "filter".into()) aria-label="filter navigation"
                on:input=move |e| {
                    if let Some(i) = e.target().and_then(|t| t.dyn_into::<HtmlElement>().ok()) {
                        let q = event_target_value(&e);
                        run(&i, &q);
                    }
                }
                on:keydown=move |e| {
                    if e.key() == "Escape" {
                        if let Some(i) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                            if !i.value().is_empty() {
                                e.prevent_default();
                                e.stop_propagation();
                                i.set_value("");
                                let el: HtmlElement = i.unchecked_into();
                                run(&el, "");
                            }
                        }
                    }
                }
            />
            <span class="d-sr" role="status">{move || status.get()}</span>
        </div>
    }
}

/// Hide rows, sub rows and groups that do not match. Returns the announcement text.
fn filter_navigation(inner: &Element, query: &str) -> String {
    let q = query.trim().to_lowercase();
    let mut n = 0;
    for g in dom::query_all(inner, ".d-sidebar__group") {
        let mut any = false;
        for li in dom::query_all(&g, ".d-sidebar__menu > .d-sidebar__item") {
            let own = li.query_selector(".d-sidebar__btn .d-sidebar__label").ok().flatten();
            let hit_own = q.is_empty()
                || own.map(|o| o.text_content().unwrap_or_default().to_lowercase().contains(&q)).unwrap_or(false);
            let mut sub_hit = false;
            for s in dom::query_all(&li, ".d-sidebar__sub > li") {
                let h = hit_own || s.text_content().unwrap_or_default().to_lowercase().contains(&q);
                set_hidden(&s, !h);
                if h && !hit_own {
                    sub_hit = true;
                }
                if h && !q.is_empty() {
                    n += 1;
                }
            }
            let hidden = !(hit_own || sub_hit);
            set_hidden(&li, hidden);
            if !hidden {
                any = true;
                if !q.is_empty() && hit_own {
                    n += 1;
                }
            }
            if !q.is_empty() && sub_hit {
                if let Some(d) = li.query_selector("details").ok().flatten() {
                    let _ = d.set_attribute("open", "");
                }
            }
        }
        set_hidden(&g, !any);
    }
    if q.is_empty() {
        String::new()
    } else if n == 1 {
        "1 match".into()
    } else {
        format!("{n} matches")
    }
}

fn set_hidden(el: &Element, hidden: bool) {
    if hidden {
        let _ = el.set_attribute("hidden", "");
    } else {
        let _ = el.remove_attribute("hidden");
    }
}

/// The scrolling navigation. Two levels at most; a third belongs in the page, as tabs.
#[component]
pub fn SidebarContent(#[prop(optional, into)] label: Option<String>, children: Children) -> impl IntoView {
    view! { <nav class="d-sidebar__content" aria-label=label.unwrap_or_else(|| "main".into())>{children()}</nav> }
}

/// A labelled section of rows. Keep it to about seven; use the filter and the palette for the long tail.
#[component]
pub fn SidebarGroup(#[prop(into)] label: String, children: Children) -> impl IntoView {
    let id = crate::util::uid("sbg");
    let labelled = id.clone();
    view! {
        <div class="d-sidebar__group" role="group" aria-labelledby=labelled>
            <h2 class="d-sidebar__group-label" id=id>{label}</h2>
            <ul class="d-sidebar__menu">{children()}</ul>
        </div>
    }
}

/// A top-level row: a 16px icon, a lowercase label and an optional count. Collapsed to the icon rail it shows
/// its label as a tooltip. Without `href` it is disabled.
#[component]
pub fn SidebarItem(
    #[prop(optional, into)] href: Option<String>,
    /// A Lucide icon name; top-level rows only.
    #[prop(optional)]
    icon: Option<&'static str>,
    #[prop(into)] label: String,
    #[prop(optional, into)] current: Signal<bool>,
    /// A neutral count badge.
    #[prop(optional, into)]
    count: Option<String>,
    /// A per-row menu button (see [`SidebarAction`]), shown on hover and focus.
    #[prop(optional, into)]
    action: Option<ViewFn>,
) -> impl IntoView {
    let shell = try_shell();
    let tip = label.clone();
    let disabled = href.is_none();
    view! {
        <li class="d-sidebar__item">
            <a
                class="d-sidebar__btn" href=href
                aria-current=move || current.get().then_some("page")
                aria-disabled=disabled.then_some("true")
                data-tooltip=move || shell.map(|s| s.icon_rail()).unwrap_or(false).then(|| tip.clone())
            >
                {icon.map(|i| view! { <Icon name=i/> })}
                <span class="d-sidebar__label">{label}</span>
                {count.map(|c| view! { <span class="d-badge d-badge--count">{c}</span> })}
            </a>
            {action.map(|a| a.run())}
        </li>
    }
}

/// A row with children, built on native `<details>`. `children` are [`SidebarSubItem`]s.
#[component]
pub fn SidebarParent(
    #[prop(optional)] icon: Option<&'static str>,
    #[prop(into)] label: String,
    #[prop(optional)] open: bool,
    children: Children,
) -> impl IntoView {
    let shell = try_shell();
    let tip = label.clone();
    view! {
        <li class="d-sidebar__item">
            <details open=open>
                <summary
                    class="d-sidebar__btn"
                    data-tooltip=move || shell.map(|s| s.icon_rail()).unwrap_or(false).then(|| tip.clone())
                >
                    {icon.map(|i| view! { <Icon name=i/> })}
                    <span class="d-sidebar__label">{label}</span>
                    <span class="d-sidebar__chev"><Icon name="chevron-right" small=true/></span>
                </summary>
                <ul class="d-sidebar__sub">{children()}</ul>
            </details>
        </li>
    }
}

/// A child of a [`SidebarParent`]. No icon.
#[component]
pub fn SidebarSubItem(
    #[prop(into)] href: String,
    #[prop(optional, into)] current: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <li><a class="d-sidebar__subbtn" href=href aria-current=move || current.get().then_some("page")>{children()}</a></li>
    }
}

/// A per-row `more` menu for a [`SidebarItem`]. `children` are the menu items.
#[component]
pub fn SidebarAction(
    /// The row it belongs to; the button is named `more: {row}`.
    #[prop(into)]
    row: String,
    children: Children,
) -> impl IntoView {
    let aria = format!("more: {row}");
    let trigger = move || {
        view! {
            <button type="button" class="d-btn d-btn--ghost d-btn--icon d-btn--sm d-sidebar__action" aria-label=aria.clone()>
                <Icon name="ellipsis"/>
            </button>
        }
    };
    let menu_label = format!("{row} actions");
    view! { <Menu label=menu_label trigger=trigger>{children()}</Menu> }
}

/// Loading placeholder rows on the system pulse.
#[component]
pub fn SidebarSkeleton(#[prop(default = 3)] rows: usize) -> impl IntoView {
    view! {
        <div class="d-sidebar__skel" aria-hidden="true">
            {(0..rows).map(|_| view! { <span class="d-skel"></span> }).collect_view()}
        </div>
    }
}

/// A sidebar toggle for the app bar: the panel icon, `toggle sidebar`.
#[component]
pub fn SidebarToggle() -> impl IntoView {
    view! { <ShellToggle region=Region::Left label="toggle sidebar"/> }
}

