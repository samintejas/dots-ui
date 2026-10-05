use leptos::prelude::*;

use super::spinner::Spinner;
use crate::types::Size;
use crate::util::{tf, when_true};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// `accent-primary` fill. One per view.
    Primary,
    /// 1px `border-strong`, no fill. The default choice.
    #[default]
    Secondary,
    /// Text only. Toolbars, row actions.
    Ghost,
    /// `accent-danger` tint. The label names what gets destroyed.
    Danger,
}

impl ButtonVariant {
    fn class(self) -> &'static str {
        match self {
            ButtonVariant::Primary => "d-btn--primary",
            ButtonVariant::Secondary => "d-btn--secondary",
            ButtonVariant::Ghost => "d-btn--ghost",
            ButtonVariant::Danger => "d-btn--danger",
        }
    }
}

pub(crate) fn button_class(variant: ButtonVariant, size: Size, icon: bool) -> String {
    let mut c = format!("d-btn {}", variant.class());
    match size {
        Size::Sm => c.push_str(" d-btn--sm"),
        Size::Lg => c.push_str(" d-btn--lg"),
        Size::Md => {}
    }
    if icon {
        c.push_str(" d-btn--icon");
    }
    c
}

/// Triggers an action. Pass `on:click=…` like on any element; `class`, `id` and other attributes forward too.
///
/// Labels are lowercase verbs (`deploy`, `delete workspace`). An icon-only button (`icon`) needs `label`.
#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: Size,
    /// Square, for an icon-only button. Give it a `label`.
    #[prop(optional)]
    icon: bool,
    /// A toggle: sets `aria-pressed`. Leave unset for a normal button.
    #[prop(optional, into)]
    pressed: MaybeProp<bool>,
    /// Loading: shows a spinner, sets `aria-busy` and ignores pointer input. Keep the label text.
    #[prop(optional, into)]
    busy: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// `aria-disabled`: looks disabled but stays focusable.
    #[prop(optional, into)]
    aria_disabled: Signal<bool>,
    /// `aria-label`, required for icon-only buttons.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Tooltip text (`data-tooltip`).
    #[prop(optional, into)]
    tooltip: MaybeProp<String>,
    /// `type="submit"` instead of `type="button"`.
    #[prop(optional)]
    submit: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type=if submit { "submit" } else { "button" }
            class=button_class(variant, size, icon)
            aria-pressed=move || pressed.get().map(tf)
            aria-busy=move || when_true(busy.get())
            aria-disabled=move || when_true(aria_disabled.get())
            aria-label=move || label.get()
            data-tooltip=move || tooltip.get()
            disabled=move || disabled.get()
        >
            {move || busy.get().then(|| view! { <Spinner decorative=true/> })}
            {children()}
        </button>
    }
}

/// A link that looks like a [`Button`].
#[component]
pub fn ButtonLink(
    #[prop(into)] href: String,
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional)] size: Size,
    #[prop(optional)] icon: bool,
    #[prop(optional, into)] label: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <a href=href class=button_class(variant, size, icon) aria-label=move || label.get()>{children()}</a>
    }
}
