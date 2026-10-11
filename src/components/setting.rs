//! Setting rows: what a setting is, and the control that changes it, on one line.

use leptos::prelude::*;

/// A stack of [`SettingRow`]s, ruled between rows. Usually inside a `CardBody`.
#[component]
pub fn SettingRows(children: Children) -> impl IntoView {
    view! { <div class="d-setrows">{children()}</div> }
}

/// One setting: `label` (and an optional `hint` under it) on the left, the control (`children`) on the right.
///
/// With `r#for` the label is a `<label>` for that control's id; otherwise wrap a checkbox or switch so the
/// whole row toggles it: `<SettingRow label="…" toggle=true>` renders the row as a `<label>`.
#[component]
pub fn SettingRow(
    #[prop(into)] label: String,
    #[prop(optional, into)] hint: Option<String>,
    /// Id of the control, for a `<label for>`.
    #[prop(optional, into)]
    r#for: Option<String>,
    /// Render the whole row as a `<label>` around a checkbox or switch.
    #[prop(optional)]
    toggle: bool,
    children: Children,
) -> impl IntoView {
    let text = match r#for {
        Some(f) => view! { <label class="d-setrow__text" for=f><b>{label}</b>{hint.map(|h| view! { <small>{h}</small> })}</label> }.into_any(),
        None => view! { <span class="d-setrow__text"><b>{label}</b>{hint.map(|h| view! { <small>{h}</small> })}</span> }.into_any(),
    };
    if toggle {
        view! { <label class="d-setrow">{text}{children()}</label> }.into_any()
    } else {
        view! { <div class="d-setrow">{text}{children()}</div> }.into_any()
    }
}

/// Cards of settings side by side, at least 340px each, wrapping to one column.
#[component]
pub fn SettingGrid(children: Children) -> impl IntoView {
    view! { <div class="d-setgrid">{children()}</div> }
}
