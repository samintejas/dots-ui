//! The app switcher: the other apps under one account, from the app bar.

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
