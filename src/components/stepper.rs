use leptos::prelude::*;

use crate::icon::Icon;

/// Where you are in a short, ordered flow: done steps get a check, the current one is marked, the rest are ahead.
///
/// `current` is the zero-based index of the step you are on. It is a status display, not navigation.
#[component]
pub fn Stepper(
    steps: Vec<String>,
    #[prop(into)] current: Signal<usize>,
    /// Stack the steps in a column.
    #[prop(optional)]
    vertical: bool,
    #[prop(default = "steps".into(), into)] label: String,
) -> impl IntoView {
    let class = if vertical { "d-stepper d-stepper--vertical" } else { "d-stepper" };
    view! {
        <ol class=class aria-label=label>
            {steps.into_iter().enumerate().map(|(i, name)| view! {
                <li
                    class="d-stepper__step"
                    data-state=move || (i < current.get()).then_some("done")
                    aria-current=move || (i == current.get()).then_some("step")
                >
                    <span class="d-stepper__n" aria-hidden="true">
                        {move || if i < current.get() { view! { <Icon name="check"/> }.into_any() } else { (i + 1).to_string().into_any() }}
                    </span>
                    <span>{name}{move || (i < current.get()).then(|| view! { <span class="d-sr">", done"</span> })}</span>
                </li>
            }).collect_view()}
        </ol>
    }
}
