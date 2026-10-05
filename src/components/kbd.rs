use leptos::prelude::*;

/// A keycap. One per key, lowercase words for modifiers (`ctrl`, `shift`, `esc`), glyphs `↑ ↓ ← → ↵` for the rest.
#[component]
pub fn Kbd(children: Children) -> impl IntoView {
    view! { <kbd class="d-kbd">{children()}</kbd> }
}

/// A chord: one [`Kbd`] per key, separated by a space.
#[component]
pub fn Keys(#[prop(into)] keys: Vec<String>) -> impl IntoView {
    view! {
        {keys.into_iter().map(|k| view! { <kbd class="d-kbd">{k}</kbd>" " }).collect_view()}
    }
}
