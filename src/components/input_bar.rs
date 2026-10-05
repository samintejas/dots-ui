use leptos::prelude::*;

use crate::util::when_true;

/// Several fields joined into one bar with one action: the quick-entry control for a screen where adding a
/// row is the main job. Use it once per screen. Enter submits from any field.
///
/// Children are [`InputSeg`]s, then an [`InputBarSubmit`], and the error line comes from `error`.
#[component]
pub fn InputBar(
    #[prop(into)] label: String,
    /// Called on submit (the form's default navigation is prevented).
    #[prop(into)]
    on_submit: Callback<()>,
    /// Shows `error: …` under the bar and turns its border `accent-danger`.
    #[prop(optional, into)]
    error: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <form
            class="d-inputbar" aria-label=label
            aria-invalid=move || when_true(error.get().is_some())
            on:submit=move |e| {
                e.prevent_default();
                on_submit.run(());
            }
        >
            {children()}
            {move || error.get().map(|m| {
                let m = if m.starts_with("error:") { m } else { format!("error: {m}") };
                view! { <span class="d-inputbar__error" role="alert">{m}</span> }
            })}
        </form>
    }
}

/// One segment of an [`InputBar`]: a caption over a borderless field.
///
/// Wraps a native control in a `<label>` by default. For a composite control (a `TagInput`) pass
/// `composite` and `for_id` so the caption labels it instead.
#[component]
pub fn InputSeg(
    #[prop(into)] caption: String,
    /// CSS `flex`, like `"3 1 220px"`: give the free-text segment the room.
    #[prop(optional, into)]
    flex: Option<String>,
    /// Amount: right-aligned, tabular, narrow.
    #[prop(optional)]
    num: bool,
    /// The control is composite: render a `<div>` and a caption `<label for=for_id>`.
    #[prop(optional)]
    composite: bool,
    #[prop(optional, into)] for_id: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = if num { "d-inputbar__seg d-inputbar__seg--num" } else { "d-inputbar__seg" };
    let style = flex.map(|f| format!("flex: {f}"));
    if composite {
        view! {
            <div class=class style=style>
                <label class="d-inputbar__cap" for=for_id>{caption}</label>
                {children()}
            </div>
        }
        .into_any()
    } else {
        view! {
            <label class=class style=style>
                <span class="d-inputbar__cap">{caption}</span>
                {children()}
            </label>
        }
        .into_any()
    }
}

/// The bar's one primary button, with the Enter keycap.
#[component]
pub fn InputBarSubmit(#[prop(optional, into)] busy: Signal<bool>, children: Children) -> impl IntoView {
    view! {
        <button type="submit" class="d-btn d-btn--primary" aria-busy=move || when_true(busy.get())>
            {children()}" "<kbd class="d-kbd">"↵"</kbd>
        </button>
    }
}
