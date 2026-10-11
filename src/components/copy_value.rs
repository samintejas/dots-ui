use leptos::prelude::*;

use crate::icon::Icon;

/// A value you hand to someone (an id, an invite code): click to copy it. Long values are cut by the width,
/// the whole value is copied. It says `copied` to assistive tech for 1.5s.
#[component]
pub fn CopyValue(
    #[prop(into)] value: String,
    /// What it is, like `transaction id`: the button's name is `copy transaction id`.
    #[prop(into)]
    label: String,
    /// Cut the shown value at this width, like `"200px"`.
    #[prop(optional, into)]
    max_width: Option<String>,
) -> impl IntoView {
    let copied = RwSignal::new(false);
    let name = label.clone();
    let aria = move || if copied.get() { format!("{name} copied") } else { format!("copy {name}") };
    let v = value.clone();
    let on_click = move |_| {
        let v = v.clone();
        leptos::task::spawn_local(async move {
            if super::code_block::write_clipboard(&v).await {
                copied.set(true);
                set_timeout(move || copied.set(false), std::time::Duration::from_millis(1500));
            }
        });
    };
    view! {
        <button
            type="button" class="d-copyval" aria-label=aria data-copied=move || copied.get().then_some("")
            style=max_width.map(|w| format!("max-width:{w}")) on:click=on_click
        >
            <span class="d-copyval__v">{value}</span>
            <Icon name=Signal::derive(move || if copied.get() { "check" } else { "copy" })/>
        </button>
    }
}
