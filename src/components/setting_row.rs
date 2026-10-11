use leptos::prelude::*;

/// A list of [`SettingRow`]s, usually the body of a [`Card`](crate::Card).
#[component]
pub fn SettingRows(children: Children) -> impl IntoView {
    view! { <div class="d-setrows">{children()}</div> }
}

/// One setting: its name and a line of hint on the left, the control on the right.
///
/// For a checkbox or switch use `toggle`: the whole row becomes the control's label, so a click anywhere flips it.
#[component]
pub fn SettingRow(
    #[prop(into)] label: String,
    #[prop(optional, into)] hint: Option<String>,
    #[prop(optional)] toggle: bool,
    children: Children,
) -> impl IntoView {
    let text = view! { <span class="d-setrow__text"><b>{label}</b>{hint.map(|h| view! { <small>{h}</small> })}</span> };
    if toggle {
        view! { <label class="d-setrow">{text}{children()}</label> }.into_any()
    } else {
        view! { <div class="d-setrow">{text}{children()}</div> }.into_any()
    }
}
