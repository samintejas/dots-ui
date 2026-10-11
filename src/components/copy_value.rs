//! A value you copy in one click: a transaction id, an api key prefix, an invite code.

use std::time::Duration;

use leptos::prelude::*;

use crate::icon::Icon;

/// The value in small mono with a copy icon; long values are cut with an ellipsis (the whole value is copied).
/// `label` names what it is (`transaction id`); the button says `copy transaction id`, then `copied` for 1.5s.
#[component]
pub fn CopyValue(#[prop(into)] value: Signal<String>, #[prop(into)] label: String) -> impl IntoView {
    let copied = RwSignal::new(false);
    let name = label.clone();
    view! {
        <button type="button" class="d-copyval" data-copied=move || copied.get().then_some("")
            aria-label=move || if copied.get() { format!("{name} copied") } else { format!("copy {name}") }
            title=move || value.get()
            on:click=move |_| {
                let v = value.get_untracked();
                leptos::task::spawn_local(async move {
                    if crate::components::code_block::write_clipboard(&v).await {
                        copied.set(true);
                        set_timeout(move || copied.set(false), Duration::from_millis(1500));
                    }
                });
            }
        >
            <span class="d-copyval__v">{move || value.get()}</span>
            <Icon name=Signal::derive(move || if copied.get() { "check" } else { "copy" })/>
        </button>
    }
}
