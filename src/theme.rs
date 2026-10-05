//! Dark/light theming. Dark is canonical; light is a remap of the same semantic tokens.

use leptos::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    pub fn as_str(self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }

    pub fn toggled(self) -> Self {
        match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
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

/// Owns the theme signal and mirrors it to `data-theme` on `<html>`.
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
            let _ = root.set_attribute("data-theme", t.as_str());
        }
        if !persist.is_empty() {
            if let Ok(Some(s)) = window().local_storage() {
                let _ = s.set_item(persist, t.as_str());
            }
        }
    });

    children()
}
