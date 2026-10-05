use leptos::prelude::*;

/// A 4px bar for a task with known or unknown length. Pair it with a label and a value in an `<output>`
/// (see [`ProgressRow`]).
#[component]
pub fn Progress(
    /// 0 to 100. `None` is indeterminate: the whole bar pulses.
    #[prop(optional, into)]
    value: MaybeProp<f64>,
    /// `success` or `danger`, only at the end, with the word `done` or `failed` in the output.
    #[prop(optional, into)]
    severity: MaybeProp<crate::types::Severity>,
    /// Id of the element that names it.
    #[prop(optional, into)]
    labelledby: Option<String>,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    view! {
        <div
            class="d-progress"
            role="progressbar"
            aria-labelledby=labelledby
            aria-label=label
            aria-valuemin="0"
            aria-valuemax="100"
            aria-valuenow=move || value.get().map(|v| format!("{}", v.clamp(0.0, 100.0)))
            data-indeterminate=move || value.get().is_none()
            data-severity=move || severity.get().map(crate::types::Severity::as_str)
            style=move || value.get().map(|v| format!("--d-value:{}", v.clamp(0.0, 100.0)))
        >
            <div class="d-progress__bar"></div>
        </div>
    }
}

/// Label, value and bar together, the way the design system sets them.
#[component]
pub fn ProgressRow(
    #[prop(into)] label: String,
    #[prop(optional, into)] value: MaybeProp<f64>,
    /// Text in the output; defaults to `NN%`.
    #[prop(optional, into)]
    output: MaybeProp<String>,
    #[prop(optional, into)] severity: MaybeProp<crate::types::Severity>,
) -> impl IntoView {
    let id = crate::util::uid("progress");
    let text = move || {
        output
            .get()
            .or_else(|| value.get().map(|v| format!("{}%", v.round())))
            .unwrap_or_default()
    };
    view! {
        <div class="d-progress-row">
            <span id=id.clone()>{label}</span>
            <output>{text}</output>
            <Progress value=value severity=severity labelledby=id/>
        </div>
    }
}
