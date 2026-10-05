//! Leptos components for the **dots** design system: monochrome, monospace, dense.
//!
//! Every component is a thin, accessible `view!` over the documented dots markup. State lives in the same
//! native and ARIA attributes the stylesheet reads (`aria-selected`, `data-state`, …) and is driven by
//! signals; no JavaScript from `dots.js` is needed.
//!
//! ```ignore
//! use dots_design::prelude::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <DotsProvider>
//!             <Button variant=ButtonVariant::Primary>"deploy"</Button>
//!         </DotsProvider>
//!     }
//! }
//! ```
//!
//! Load the styles with [`DotsStyles`] (or copy the files from [`assets`]) and wrap the app in
//! [`DotsProvider`].

mod icon_data;
mod util;

pub mod assets;
pub mod components;
pub mod icon;
pub mod theme;
pub mod types;

pub use components::*;
pub use icon::Icon;
pub use theme::{Theme, ThemeProvider, use_theme};
pub use types::*;

/// Everything you normally want in scope: `use dots_design::prelude::*;`.
pub mod prelude {
    pub use crate::assets::DotsStyles;
    pub use crate::components::*;
    pub use crate::icon::{Icon, icon_names};
    pub use crate::theme::{Theme, ThemeProvider, use_theme};
    pub use crate::types::*;
    pub use crate::DotsProvider;
}

use leptos::prelude::*;

/// App-level setup in one wrapper: theme, toasts and tooltips.
///
/// Put it once near the root of the app. It sets `data-theme` on `<html>`, renders the toast region and
/// installs the delegated tooltip layer (any element with a `data-tooltip` attribute gets a tooltip).
#[component]
pub fn DotsProvider(
    /// Theme to start in. Defaults to dark, the canonical theme.
    #[prop(optional)]
    theme: Option<Theme>,
    /// `localStorage` key to remember the theme under. Off unless set.
    #[prop(default = "")]
    persist_theme: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <ThemeProvider initial=theme.unwrap_or_default() persist=persist_theme>
            <components::tooltip::TooltipLayer/>
            <ToastProvider>{children()}</ToastProvider>
        </ThemeProvider>
    }
}
