use leptos::prelude::*;

/// A 1px line between groups. Prefer spacing; use a divider only where two groups would read as one.
#[component]
pub fn Divider(
    /// `border-focus-ring-quiet`: inside sheets and between major regions.
    #[prop(optional)]
    strong: bool,
    #[prop(optional)] vertical: bool,
    /// Text in the middle of the line (`or`).
    #[prop(optional, into)]
    label: Option<String>,
) -> impl IntoView {
    if let Some(label) = label {
        view! { <div class="d-divider d-divider--label" role="separator">{label}</div> }.into_any()
    } else if vertical {
        view! { <span class="d-divider d-divider--vertical" role="separator" aria-orientation="vertical"></span> }
            .into_any()
    } else if strong {
        view! { <hr class="d-divider d-divider--strong"/> }.into_any()
    } else {
        view! { <hr class="d-divider"/> }.into_any()
    }
}
