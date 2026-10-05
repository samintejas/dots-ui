//! The app layout: a fixed-height frame of tiled bars around the content. Sidebar left, app bar on top, a
//! right bar and a bottom bar. Each expands inside the frame and the content resizes; none covers it.
//!
//! Give the shell a fixed height (default `100dvh`). If the page itself scrolls, the bars are no longer tiles.

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::Element;

use crate::icon::Icon;
use crate::util::{dom, uid};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region {
    Left,
    Right,
    Bottom,
}

impl Region {
    fn storage_suffix(self) -> &'static str {
        match self {
            Region::Left => "",
            Region::Right => ":right",
            Region::Bottom => ":bottom",
        }
    }

    fn shortcut(self) -> &'static str {
        match self {
            Region::Left => "Control+B",
            Region::Right => "Control+Alt+B",
            Region::Bottom => "Control+J",
        }
    }

    fn default_icon(self) -> &'static str {
        match self {
            Region::Left => "panel-left",
            Region::Right => "panel-right",
            Region::Bottom => "panel-bottom",
        }
    }

}

/// What a collapsed sidebar leaves behind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Collapsible {
    /// A 44px icon rail.
    #[default]
    Icon,
    /// Nothing: zero wide, hidden and inert.
    Offcanvas,
}

/// The open/closed state of the shell's bars. Create one with [`ShellState::new`] to drive the shell from
/// outside, or call [`use_shell`] inside it.
#[derive(Clone, Copy)]
pub struct ShellState {
    left: RwSignal<bool>,
    right: RwSignal<bool>,
    bottom: RwSignal<bool>,
    mobile: RwSignal<bool>,
    collapsible: RwSignal<Collapsible>,
    persist: StoredValue<&'static str>,
    present: StoredValue<[bool; 3]>,
    ids: StoredValue<[String; 3]>,
}

impl Default for ShellState {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellState {
    /// Sidebar expanded, right bar and bottom panel collapsed.
    pub fn new() -> Self {
        Self::with_open(true, false, false)
    }

    pub fn with_open(left: bool, right: bool, bottom: bool) -> Self {
        ShellState {
            left: RwSignal::new(left),
            right: RwSignal::new(right),
            bottom: RwSignal::new(bottom),
            mobile: RwSignal::new(false),
            collapsible: RwSignal::new(Collapsible::Icon),
            persist: StoredValue::new(""),
            present: StoredValue::new([false; 3]),
            ids: StoredValue::new([uid("leftbar"), uid("rightbar"), uid("bottombar")]),
        }
    }

    fn sig(&self, r: Region) -> RwSignal<bool> {
        match r {
            Region::Left => self.left,
            Region::Right => self.right,
            Region::Bottom => self.bottom,
        }
    }

    fn idx(r: Region) -> usize {
        match r {
            Region::Left => 0,
            Region::Right => 1,
            Region::Bottom => 2,
        }
    }

    pub(crate) fn id(&self, r: Region) -> String {
        self.ids.with_value(|ids| ids[Self::idx(r)].clone())
    }

    pub(crate) fn register(&self, r: Region, present: bool) {
        self.present.update_value(|p| p[Self::idx(r)] = present);
    }

    fn has(&self, r: Region) -> bool {
        self.present.with_value(|p| p[Self::idx(r)])
    }

    /// Whether a bar is expanded. Reactive.
    pub fn is_open(&self, r: Region) -> bool {
        self.sig(r).get()
    }

    /// A reactive handle for the bar's open state.
    pub fn open_signal(&self, r: Region) -> Signal<bool> {
        self.sig(r).into()
    }

    /// Whether the shell is narrower than its breakpoint, so the tiles are stacked.
    pub fn is_mobile(&self) -> bool {
        self.mobile.get()
    }

    pub fn collapsible(&self) -> Collapsible {
        self.collapsible.get()
    }

    /// The sidebar is collapsed to the icon rail (labels clipped, tooltips on).
    pub fn icon_rail(&self) -> bool {
        !self.left.get() && self.collapsible.get() == Collapsible::Icon && !self.mobile.get()
    }

    fn store(&self, r: Region, open: bool) {
        let key = self.persist.get_value();
        if key.is_empty() || self.mobile.get_untracked() {
            return;
        }
        if let Ok(Some(s)) = window().local_storage() {
            let _ = s.set_item(&format!("{key}{}", r.storage_suffix()), if open { "expanded" } else { "collapsed" });
        }
    }

    /// Expand or collapse a bar.
    pub fn set(&self, r: Region, open: bool) {
        self.sig(r).set(open);
        self.store(r, open);
    }

    pub fn toggle(&self, r: Region) {
        if !self.has(r) {
            return;
        }
        let next = !self.sig(r).get_untracked();
        self.set(r, next);
        // stacked layout: the sidebar opens between the bar and the content, so take focus there
        if r == Region::Left && self.mobile.get_untracked() && next {
            set_timeout(
                || {
                    if let Some(el) = document()
                        .query_selector(".d-sidebar a[href], .d-sidebar button:not([disabled]), .d-sidebar input")
                        .ok()
                        .flatten()
                    {
                        dom::focus(&el);
                    }
                },
                std::time::Duration::from_millis(0),
            );
        }
    }
}

/// The nearest shell's state. Call inside [`Shell`].
pub fn use_shell() -> ShellState {
    expect_context::<ShellState>()
}

pub(crate) fn try_shell() -> Option<ShellState> {
    use_context::<ShellState>()
}

/// The tiled app frame. Put [`Sidebar`](crate::Sidebar), [`ShellMain`] (with an [`AppBar`](crate::AppBar) and
/// [`Main`]) and optionally [`RightBar`] inside it.
///
/// Shortcuts: ctrl+b left, ctrl+alt+b right, ctrl+j bottom. Under the breakpoint the tiles stack, the sidebar
/// and right bar start closed, and opening one inserts it above the content.
#[component]
pub fn Shell(
    /// Drive the bars from outside. Default: a fresh [`ShellState`].
    #[prop(optional)]
    state: Option<ShellState>,
    #[prop(optional)] collapsible: Collapsible,
    /// `localStorage` key to remember the open state of all three bars under. Off when empty.
    #[prop(default = "")]
    persist: &'static str,
    /// Bind the keyboard shortcuts.
    #[prop(default = true)]
    hotkeys: bool,
    /// Shell width in px under which the tiles stack.
    #[prop(default = 768.0)]
    breakpoint: f64,
    /// CSS length of the frame (`--d-shell-h`). Default `100dvh`.
    #[prop(optional, into)]
    height: Option<String>,
    children: Children,
) -> impl IntoView {
    let shell = state.unwrap_or_default();
    shell.collapsible.set(collapsible);
    shell.persist.set_value(persist);
    provide_context(shell);

    let root = NodeRef::<html::Div>::new();
    let desk = StoredValue::new((true, false));
    let observer = StoredValue::new_local(None::<(web_sys::ResizeObserver, Closure<dyn FnMut(js_sys::Array)>)>);

    // restore remembered state (client only, so the server render is the default)
    if !persist.is_empty() {
        Effect::new(move |done: Option<bool>| {
            if done.is_none() {
                if let Ok(Some(s)) = window().local_storage() {
                    for r in [Region::Left, Region::Right, Region::Bottom] {
                        let stored = s.get_item(&format!("{persist}{}", r.storage_suffix())).ok().flatten();
                        match stored.as_deref() {
                            Some("expanded") => shell.sig(r).set(true),
                            Some("collapsed") => shell.sig(r).set(false),
                            _ => {}
                        }
                    }
                }
            }
            true
        });
    }

    // stack the tiles under the breakpoint
    Effect::new(move |_| {
        let Some(el) = root.get() else { return };
        if observer.with_value(|o| o.is_some()) {
            return;
        }
        let measure = {
            let el = el.clone();
            move || {
                let narrow = f64::from(el.client_width()) < breakpoint;
                if narrow == shell.mobile.get_untracked() {
                    return;
                }
                if narrow {
                    desk.set_value((shell.left.get_untracked(), shell.right.get_untracked()));
                    shell.mobile.set(true);
                    shell.left.set(false);
                    shell.right.set(false);
                } else {
                    shell.mobile.set(false);
                    let (l, r) = desk.get_value();
                    shell.left.set(l);
                    shell.right.set(r);
                }
            }
        };
        measure();
        let cb = Closure::<dyn FnMut(js_sys::Array)>::new(move |_| measure());
        if let Ok(ro) = web_sys::ResizeObserver::new(cb.as_ref().unchecked_ref()) {
            ro.observe(&el);
            observer.set_value(Some((ro, cb)));
        }
    });
    on_cleanup(move || {
        observer.with_value(|o| {
            if let Some((ro, _)) = o {
                ro.disconnect();
            }
        });
    });

    if hotkeys {
        window_event_listener(leptos::ev::keydown, move |e| {
            if !(e.ctrl_key() || e.meta_key()) || e.shift_key() {
                return;
            }
            let region = match e.key().to_lowercase().as_str() {
                "b" => Some(if e.alt_key() { Region::Right } else { Region::Left }),
                "j" if !e.alt_key() => Some(Region::Bottom),
                _ => None,
            };
            if let Some(r) = region {
                if shell.has(r) {
                    e.prevent_default();
                    shell.toggle(r);
                }
            }
        });
    }

    let on_click = move |e: leptos::ev::MouseEvent| {
        let Some(t) = e.target().and_then(|t| t.dyn_into::<Element>().ok()) else { return };
        // following a link in the stacked sidebar closes it
        if shell.mobile.get_untracked() && t.closest(".d-sidebar a[href]").ok().flatten().is_some() {
            shell.set(Region::Left, false);
            return;
        }
        // a parent row clicked while collapsed to icons: expand first, then open it
        if let Some(s) = t.closest("summary.d-sidebar__btn").ok().flatten() {
            if !shell.left.get_untracked()
                && shell.collapsible.get_untracked() == Collapsible::Icon
                && !shell.mobile.get_untracked()
            {
                e.prevent_default();
                shell.set(Region::Left, true);
                if let Some(p) = s.parent_element() {
                    let _ = p.set_attribute("open", "");
                }
                dom::focus(&s);
            }
        }
    };
    let on_keydown = move |e: leptos::ev::KeyboardEvent| {
        if e.key() != "Escape" || !shell.mobile.get_untracked() || !shell.left.get_untracked() || e.default_prevented() {
            return;
        }
        let in_sidebar = e
            .target()
            .and_then(|t| t.dyn_into::<Element>().ok())
            .map(|t| t.closest(".d-sidebar").ok().flatten().is_some())
            .unwrap_or(false);
        if !in_sidebar {
            return;
        }
        shell.set(Region::Left, false);
        if let Some(r) = root.get_untracked() {
            let toggle = dom::query_all(&r, "[data-d-sidebar-toggle]")
                .into_iter()
                .find(|x| x.closest(".d-sidebar").ok().flatten().is_none());
            if let Some(t) = toggle {
                dom::focus(&t);
            }
        }
    };

    view! {
        <div
            class="d-shell" node_ref=root
            data-state=move || if shell.left.get() { "expanded" } else { "collapsed" }
            data-right=move || if shell.right.get() { "expanded" } else { "collapsed" }
            data-bottom=move || if shell.bottom.get() { "expanded" } else { "collapsed" }
            data-collapsible=move || if shell.collapsible.get() == Collapsible::Icon { "icon" } else { "offcanvas" }
            data-mobile=move || shell.mobile.get()
            style=height.map(|h| format!("--d-shell-h:{h}"))
            on:click=on_click
            on:keydown=on_keydown
        >
            {children()}
        </div>
    }
}

/// The column holding the app bar, the content and the bottom bar.
#[component]
pub fn ShellMain(children: Children) -> impl IntoView {
    view! { <div class="d-shell__main">{children()}</div> }
}

/// The page content. Start every page with a [`SkipLink`] to it.
#[component]
pub fn Main(children: Children) -> impl IntoView {
    view! { <main id="main" tabindex="-1">{children()}</main> }
}

/// The first focusable element on the page; visible only on focus.
#[component]
pub fn SkipLink() -> impl IntoView {
    view! { <a class="d-skip" href="#main">"skip to content"</a> }
}

/// A button that toggles one of the shell's bars. Icon-only by default; give it children for a text button
/// (like `logs` in the status line). It carries `aria-expanded`, `aria-controls` and `aria-keyshortcuts`.
#[component]
pub fn ShellToggle(
    region: Region,
    /// `aria-label`, like `toggle sidebar`.
    #[prop(into)]
    label: String,
    /// Icon for the icon-only form. Defaults to the panel icon for the region.
    #[prop(optional)]
    icon: Option<&'static str>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let shell = use_shell();
    let controls = shell.id(region);
    let (is_left, is_right, is_bottom) = (region == Region::Left, region == Region::Right, region == Region::Bottom);
    let open = shell.open_signal(region);
    let class = if children.is_some() {
        "d-btn d-btn--ghost d-btn--sm"
    } else {
        "d-btn d-btn--ghost d-btn--icon d-btn--sm"
    };
    view! {
        <button
            type="button" class=class
            data-d-sidebar-toggle=is_left
            data-d-right-toggle=is_right
            data-d-bottom-toggle=is_bottom
            aria-label=label aria-controls=controls
            aria-expanded=move || if open.get() { "true" } else { "false" }
            aria-keyshortcuts=region.shortcut()
            on:click=move |_| shell.toggle(region)
        >
            {match children {
                Some(c) => c().into_any(),
                None => view! { <Icon name=icon.unwrap_or(region.default_icon())/> }.into_any(),
            }}
        </button>
    }
}

/// The right bar: detail about the current selection. Navigation stays on the left.
#[component]
pub fn RightBar(
    /// Names the landmark and heads the bar (`details`).
    #[prop(into)]
    title: String,
    children: Children,
) -> impl IntoView {
    let shell = use_shell();
    shell.register(Region::Right, true);
    on_cleanup(move || shell.register(Region::Right, false));
    let open = shell.open_signal(Region::Right);
    let close_label = format!("close {title}");
    let landmark = title.clone();
    view! {
        <aside class="d-rightbar" id=shell.id(Region::Right) aria-label=landmark inert=move || !open.get()>
            <div class="d-rightbar__inner">
                <header class="d-rightbar__head">
                    <h2 class="d-rightbar__title">{title}</h2>
                    <ShellToggle region=Region::Right label=close_label icon="x"/>
                </header>
                <div class="d-rightbar__body">{children()}</div>
            </div>
        </aside>
    }
}

/// The bottom bar: a status line that is always visible, and a panel above it that expands.
///
/// The line holds short facts (connection, branch): use a [`StatusChip`](crate::StatusChip) for anything in a
/// state, and a text [`ShellToggle`] for the panel.
#[component]
pub fn BottomBar(
    /// Names the panel region (`logs`).
    #[prop(into)]
    label: String,
    /// The content of the status line.
    #[prop(into)]
    line: ViewFn,
    /// The expandable panel.
    children: Children,
) -> impl IntoView {
    let shell = use_shell();
    shell.register(Region::Bottom, true);
    on_cleanup(move || shell.register(Region::Bottom, false));
    let open = shell.open_signal(Region::Bottom);
    view! {
        <footer class="d-bottombar">
            <div class="d-bottombar__panel" id=shell.id(Region::Bottom) role="region" aria-label=label inert=move || !open.get()>
                {children()}
            </div>
            <div class="d-bottombar__line">{line.run()}</div>
        </footer>
    }
}

#[component]
pub fn BottomSpacer() -> impl IntoView {
    view! { <span class="d-bottombar__spacer"></span> }
}
