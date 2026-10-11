use leptos::prelude::*;

use crate::icon::Icon;

/// One app in an [`AppSwitcher`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppEntry {
    pub name: String,
    /// Lucide icon name for its mark.
    pub icon: &'static str,
    pub href: String,
    /// The app you are in.
    pub current: bool,
    /// Not available: drawn dashed, not a link.
    pub disabled: bool,
}

impl AppEntry {
    pub fn new(name: impl Into<String>, icon: &'static str, href: impl Into<String>) -> Self {
        Self { name: name.into(), icon, href: href.into(), current: false, disabled: false }
    }
    pub fn current(mut self) -> Self {
        self.current = true;
        self
    }
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// A grid of the apps that share one account, as a menu panel. Show it from a button in the app bar
/// (an `Icon grip` ghost button) with [`Menu`](crate::Menu), or place it as the apps row of a switchboard.
/// Children go below the grid, after a separator (for example a link to manage the account).
#[component]
pub fn AppSwitcher(
    apps: Vec<AppEntry>,
    #[prop(default = "apps".into(), into)] label: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="d-menu d-apps" role="menu" aria-label=label>
            <div class="d-apps__grid" role="none">
                {apps.into_iter().map(|a| {
                    let inner = view! {
                        <span class="d-apps__mark" aria-hidden="true"><Icon name=a.icon/></span>
                        <span class="d-apps__name">{a.name.clone()}</span>
                    };
                    if a.disabled {
                        view! { <a class="d-apps__tile" role="menuitem" tabindex="-1" aria-disabled="true">{inner}</a> }.into_any()
                    } else {
                        view! { <a class="d-apps__tile" role="menuitem" tabindex="-1" href=a.href aria-current=a.current.then_some("true")>{inner}</a> }.into_any()
                    }
                }).collect_view()}
            </div>
            {children.map(|c| view! { <div class="d-menu__sep" role="separator"></div>{c()} })}
        </div>
    }
}
