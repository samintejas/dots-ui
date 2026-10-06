//! Lucide icons, stroke 1.5, inheriting `currentColor`.

use leptos::prelude::*;

use crate::icon_data;

/// Names of every bundled icon.
pub fn icon_names() -> &'static [&'static str] {
    icon_data::NAMES
}

fn svg(name: &str) -> String {
    debug_assert!(
        name.is_empty() || icon_data::inner(name).is_some(),
        "dots-ui: unknown icon `{name}`"
    );
    format!(
        "<svg viewBox=\"0 0 24 24\" focusable=\"false\">{}</svg>",
        icon_data::inner(name).unwrap_or("")
    )
}

/// A decorative icon (`aria-hidden`). The meaning belongs in adjacent text or an `aria-label` on the control.
#[component]
pub fn Icon(
    /// A Lucide name from [`icon_names`], like `"circle-check"`. May be reactive.
    #[prop(into)]
    name: Signal<&'static str>,
    /// 12px instead of 16px, for badges, toasts and copy buttons.
    #[prop(optional)]
    small: bool,
) -> impl IntoView {
    let class = if small { "d-icon d-icon--sm" } else { "d-icon" };
    view! {
        <span class=class aria-hidden="true" data-icon=move || name.get() inner_html=move || svg(name.get())></span>
    }
}
