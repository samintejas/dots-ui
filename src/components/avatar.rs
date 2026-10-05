use leptos::prelude::*;

use crate::types::Size;
use crate::util::{tf, when_true};

fn avatar_class(size: Size) -> &'static str {
    match size {
        Size::Sm => "d-avatar d-avatar--sm",
        Size::Md => "d-avatar",
        Size::Lg => "d-avatar d-avatar--lg",
    }
}

/// A person or agent as two lowercase initials in a square box.
///
/// Give `name` when the avatar stands alone. When a name sits beside it, set `decorative`.
#[component]
pub fn Avatar(
    /// The initials, lowercase, usually two.
    #[prop(into)]
    initials: String,
    /// Accessible name (`aria-label`).
    #[prop(optional, into)]
    name: Option<String>,
    /// A name is shown beside it, so the initials are decoration.
    #[prop(optional)]
    decorative: bool,
    #[prop(optional)] size: Size,
) -> impl IntoView {
    if decorative {
        view! { <span class=avatar_class(size) aria-hidden="true">{initials}</span> }.into_any()
    } else {
        view! { <span class=avatar_class(size) role="img" aria-label=name>{initials}</span> }.into_any()
    }
}

/// A photo avatar. Photos render greyscale.
#[component]
pub fn AvatarImage(
    #[prop(into)] src: String,
    #[prop(into)] name: String,
    #[prop(optional)] size: Size,
) -> impl IntoView {
    view! { <span class=avatar_class(size)><img src=src alt=name/></span> }
}

/// An avatar as a button, for filtering by person or opening an account menu.
#[component]
pub fn AvatarButton(
    #[prop(into)] initials: String,
    #[prop(into)] label: String,
    #[prop(optional)] size: Size,
    #[prop(optional, into)] selected: MaybeProp<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class=avatar_class(size)
            aria-label=label
            aria-pressed=move || selected.get().map(tf)
            aria-disabled=move || when_true(disabled.get())
        >
            {initials}
        </button>
    }
}
