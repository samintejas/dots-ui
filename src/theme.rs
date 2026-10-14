//! Theming. Dark is the reference; light is built from the same names. With no `data-theme` on the page
//! the tokens follow the OS (`prefers-color-scheme`), which is what [`Theme::System`] does.

use leptos::prelude::*;

/// The three choices an app offers: system (the default), dark and light.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Theme {
    /// Follow the OS. No `data-theme` on `<html>`.
    #[default]
    System,
    Dark,
    Light,
}

impl Theme {
    pub fn as_str(self) -> &'static str {
        match self {
            Theme::System => "system",
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }

    /// `Dark` or `Light`: what is on screen. `System` asks the OS (dark when there is no browser to ask).
    pub fn resolved(self) -> Self {
        match self {
            Theme::System => {
                #[cfg(target_arch = "wasm32")]
                {
                    let light = web_sys::window()
                        .and_then(|w| w.match_media("(prefers-color-scheme: light)").ok().flatten())
                        .is_some_and(|m| m.matches());
                    if light { Theme::Light } else { Theme::Dark }
                }
                #[cfg(not(target_arch = "wasm32"))]
                Theme::Dark
            }
            t => t,
        }
    }

    /// The opposite of what is on screen, as an explicit choice.
    pub fn toggled(self) -> Self {
        match self.resolved() {
            Theme::Light => Theme::Dark,
            _ => Theme::Light,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "system" => Some(Theme::System),
            "dark" => Some(Theme::Dark),
            "light" => Some(Theme::Light),
            _ => None,
        }
    }
}

/// The current theme. Requires a [`ThemeProvider`] (or `DotsProvider`) above it.
pub fn use_theme() -> RwSignal<Theme> {
    expect_context::<RwSignal<Theme>>()
}

/// Owns the theme signal and mirrors it to `data-theme` on `<html>` (removed for [`Theme::System`]).
#[component]
pub fn ThemeProvider(
    #[prop(optional)] initial: Theme,
    /// `localStorage` key to remember the choice under ("" = don't). Read after mount, so SSR output is always `initial`.
    #[prop(default = "")]
    persist: &'static str,
    children: Children,
) -> impl IntoView {
    let theme = RwSignal::new(initial);
    provide_context(theme);

    if !persist.is_empty() {
        let key = persist;
        Effect::new(move |first: Option<bool>| {
            if first.is_none() {
                let stored = window()
                    .local_storage()
                    .ok()
                    .flatten()
                    .and_then(|s| s.get_item(key).ok().flatten())
                    .and_then(|v| Theme::parse(&v));
                if let Some(t) = stored {
                    theme.set(t);
                }
            }
            true
        });
    }

    Effect::new(move |_| {
        let t = theme.get();
        if let Some(root) = document().document_element() {
            let _ = match t {
                Theme::System => root.remove_attribute("data-theme"),
                t => root.set_attribute("data-theme", t.as_str()),
            };
        }
        if !persist.is_empty() {
            if let Ok(Some(s)) = window().local_storage() {
                let _ = s.set_item(persist, t.as_str());
            }
        }
    });

    children()
}
