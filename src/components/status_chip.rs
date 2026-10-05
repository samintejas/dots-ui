use leptos::prelude::*;

use crate::types::Severity;

/// One fixed glyph, filled when on and hollow when off. Colour is severity; the text says the state.
#[component]
pub fn StatusChip(
    /// `●` when true, `○` when false.
    #[prop(into)]
    on: Signal<bool>,
    /// Leave unset for nominal (`text-strong`).
    #[prop(optional, into)]
    severity: MaybeProp<Severity>,
    /// In progress only: charging, syncing, reconnecting.
    #[prop(optional, into)]
    pulse: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <span
            class="d-status"
            data-state=move || if on.get() { "on" } else { "off" }
            data-severity=move || severity.get().map(Severity::as_str)
            data-pulse=move || pulse.get()
        >
            {children()}
        </span>
    }
}

/// A glyph-only chip as a button. Needs its words as `label`, which also becomes the tooltip.
#[component]
pub fn StatusButton(
    #[prop(into)] on: Signal<bool>,
    #[prop(optional, into)] severity: MaybeProp<Severity>,
    #[prop(into)] label: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="d-status"
            data-state=move || if on.get() { "on" } else { "off" }
            data-severity=move || severity.get().map(Severity::as_str)
            aria-label=label.clone()
            data-tooltip=label
        ></button>
    }
}
