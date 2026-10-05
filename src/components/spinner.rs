use leptos::prelude::*;

/// Three dots pulsing in sequence, for a short wait with no measurable progress. Nothing rotates.
///
/// Alone it is a `role="status"` with hidden text `label`. With a visible label beside it, set `decorative`.
#[component]
pub fn Spinner(
    /// Accessible name when the spinner stands alone.
    #[prop(optional, into)]
    label: Option<String>,
    /// Hide it from assistive tech because a visible label sits next to it.
    #[prop(optional)]
    decorative: bool,
) -> impl IntoView {
    if decorative {
        view! { <span class="d-spinner" aria-hidden="true"><i></i><i></i><i></i></span> }.into_any()
    } else {
        let label = label.unwrap_or_else(|| "loading".into());
        view! {
            <span class="d-spinner" role="status"><i></i><i></i><i></i><span class="d-sr">{label}</span></span>
        }
        .into_any()
    }
}
