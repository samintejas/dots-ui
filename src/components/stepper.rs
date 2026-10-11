//! Where you are in a short, ordered flow.

use leptos::prelude::*;

use crate::icon::Icon;

/// The steps of a flow of two to five screens (`kind / details / review`).
///
/// Steps before `current` are done and show a tick instead of their number; the current one has
/// `aria-current="step"` and an `accent-primary` ring; later ones are grey. It only shows progress: going
/// back is the flow's own `back` button. `current` counts from 0.
#[component]
pub fn Stepper(
    #[prop(into)] steps: Vec<String>,
    #[prop(into)] current: Signal<usize>,
    /// `aria-label` of the list.
    #[prop(default = "steps".into(), into)]
    label: String,
    /// Stacked down the side of a long form instead of across its top.
    #[prop(optional)]
    vertical: bool,
) -> impl IntoView {
    let class = if vertical { "d-stepper d-stepper--vertical" } else { "d-stepper" };
    let rows = steps
        .into_iter()
        .enumerate()
        .map(|(i, name)| {
            let state = move || {
                let c = current.get();
                if i < c { Some("done") } else { None }
            };
            view! {
                <li class="d-stepper__step" aria-current=move || (current.get() == i).then_some("step") data-state=state>
                    <span class="d-stepper__n" aria-hidden="true">
                        {move || if i < current.get() { view! { <Icon name="check" small=true/> }.into_any() } else { (i + 1).to_string().into_any() }}
                    </span>
                    <span>{name}{move || (i < current.get()).then_some(view! { <span class="d-sr">", done"</span> })}</span>
                </li>
            }
        })
        .collect_view();
    view! { <ol class=class aria-label=label>{rows}</ol> }
}
