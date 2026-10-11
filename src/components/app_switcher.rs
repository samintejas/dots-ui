//! The app switcher: the other apps under one account. In a shell with a sidebar it lives inside the
//! [`Switchboard`], the one menu behind the sidebar's head tile; without a sidebar, [`AppSwitcher`] puts it in the app bar.

use leptos::prelude::*;

use super::menu::Menu;
use crate::icon::Icon;

/// A `grip` button in the app bar that opens a raised grid of [`AppTile`]s, one per app the account can use.
///
/// It is a [`Menu`]: arrows move between tiles, esc closes. Put the current app first and mark it `current`.
/// `foot` goes under the grid, usually a link to the account (`manage your account`).
#[component]
pub fn AppSwitcher(
    /// The trigger's accessible name and tooltip.
    #[prop(default = "apps".into(), into)]
    label: String,
    #[prop(optional, into)] foot: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let tip = label.clone();
    let name = label.clone();
    view! {
        <Menu
            label=label panel_class="d-apps"
            trigger=move || view! {
                <button type="button" class="d-btn d-btn--ghost d-btn--icon" aria-label=name.clone() data-tooltip=tip.clone()>
                    <Icon name="grip"/>
                </button>
            }
        >
            <div class="d-apps__grid" role="none">{children()}</div>
            {foot.map(|f| view! { <div class="d-menu__sep" role="separator"></div>{f.run()} })}
        </Menu>
    }
}

/// One app: its mark (children; or a Lucide `icon`), its name, and where it lives.
#[component]
pub fn AppTile(
    #[prop(into)] href: String,
    #[prop(into)] name: String,
    /// A Lucide icon when the app has no mark of its own.
    #[prop(optional)]
    icon: Option<&'static str>,
    /// This is the app you are in.
    #[prop(optional)]
    current: bool,
    /// Not available to this account yet (`soon`).
    #[prop(optional)]
    disabled: bool,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let mark = match (children, icon) {
        (Some(c), _) => c().into_any(),
        (None, Some(i)) => view! { <Icon name=i/> }.into_any(),
        (None, None) => name.chars().next().map(|c| c.to_string()).unwrap_or_default().into_any(),
    };
    view! {
        <a
            class="d-apps__tile" role="menuitem" tabindex="-1"
            href=(!disabled).then_some(href)
            aria-current=current.then_some("true")
            aria-disabled=disabled.then_some("true")
        >
            <span class="d-apps__mark" aria-hidden="true">{mark}</span>
            <span class="d-apps__name">{name}</span>
        </a>
    }
}

/// Everything that switches, behind one tile at the top of the sidebar: who you are, which app, which company
/// (or household), the theme, and signing out. Nothing else in the chrome switches.
///
/// The tile shows the current company (`initials`, `title`) and, underneath, the app you are in (`app`,
/// `app_icon`). Fill the panel with a [`MenuHead`](crate::MenuHead) for the person, [`SwitchboardApps`], a
/// labelled group of radio rows for companies, and the account rows. Put it in `d-sidebar__head`.
#[component]
pub fn Switchboard(
    /// Square avatar text for the current company or household (`ac`).
    #[prop(into)]
    initials: Signal<String>,
    /// The current company or household.
    #[prop(into)]
    title: Signal<String>,
    /// The app you are in, under the title.
    #[prop(into)]
    app: String,
    #[prop(optional)] app_icon: Option<&'static str>,
    /// Drawn instead of the initials, for a product with a mark of its own.
    #[prop(optional, into)]
    mark: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let app2 = app.clone();
    view! {
        <Menu
            label="switch" panel_class="d-switchboard" match_width=true
            trigger=move || {
                let app = app2.clone();
                view! {
                    <button type="button" class="d-sidebar__tile" aria-label=move || format!("switch app, company or account. now {} in {}", app, title.get())>
                        {match mark.clone() {
                            Some(m) => view! { <span class="d-sidebar__mark" aria-hidden="true">{m.run()}</span> }.into_any(),
                            None => view! { <span class="d-avatar d-avatar--square" aria-hidden="true">{move || initials.get()}</span> }.into_any(),
                        }}
                        <span class="d-sidebar__who">
                            <b>{move || title.get()}</b>
                            <small>{app_icon.map(|i| view! { <Icon name=i/> })}{app.clone()}</small>
                        </span>
                        <Icon name="chevrons-up-down"/>
                    </button>
                }
            }
        >
            {children()}
        </Menu>
    }
}

/// The apps row inside a [`Switchboard`]: a label and a grid of [`AppTile`]s, four across.
#[component]
pub fn SwitchboardApps(#[prop(default = "apps".into(), into)] label: String, children: Children) -> impl IntoView {
    view! {
        <div class="d-menu__label" role="presentation">{label}</div>
        <div class="d-apps__grid" role="none" style="grid-template-columns:repeat(4,minmax(0,1fr))">{children()}</div>
        <div class="d-menu__sep" role="separator"></div>
    }
}
