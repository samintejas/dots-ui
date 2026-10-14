//! Modal surfaces on the native `<dialog>`: focus is trapped and restored, esc closes, the page behind is inert.
//! Do not re-implement any of that; the platform does it.

use std::time::Duration;

use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::button::{Button, ButtonVariant};
use crate::icon::Icon;
use crate::util::{dom, uid};

/// Wires a `<dialog>` to an `open` signal. Returns `close(value)`, which sets the dialog's `returnValue`.
pub(crate) fn modal_core(
    node: NodeRef<html::Dialog>,
    open: RwSignal<bool>,
    on_close: Option<Callback<String>>,
) -> (Callback<String>, impl Fn(leptos::ev::Event) + Clone + 'static) {
    let ret = StoredValue::new(String::new());
    let close = Callback::new(move |v: String| {
        ret.set_value(v);
        open.set(false);
    });
    Effect::new(move |_| {
        let want = open.get();
        let Some(el) = node.get() else { return };
        if want && !el.open() {
            let _ = el.remove_attribute("data-closing");
            let _ = el.show_modal();
            // destructive dialogs (`dismiss=false`) focus the first footer button, which is cancel
            if el.get_attribute("data-dismiss").as_deref() == Some("false") {
                if let Ok(Some(first)) = el.query_selector(".d-dialog__foot button, .d-sheet__foot button") {
                    dom::focus(&first);
                }
            }
        } else if !want && el.open() {
            let value = ret.get_value();
            if dom::reduced_motion() {
                el.close_with_return_value(&value);
            } else {
                let _ = el.set_attribute("data-closing", "");
                set_timeout(
                    move || {
                        let _ = el.remove_attribute("data-closing");
                        if el.open() {
                            el.close_with_return_value(&value);
                        }
                    },
                    Duration::from_millis(150),
                );
            }
        }
    });
    // the native close event also fires for esc, which skips our animation
    let on_native_close = move |e: leptos::ev::Event| {
        if open.get_untracked() {
            open.set(false);
        }
        if let (Some(cb), Some(d)) = (on_close, e.target().and_then(|t| t.dyn_into::<web_sys::HtmlDialogElement>().ok())) {
            cb.run(d.return_value());
        }
    };
    (close, on_native_close)
}

/// A click on the scrim, outside the dialog's box, dismisses it.
pub(crate) fn backdrop_click(
    node: NodeRef<html::Dialog>,
    dismiss: bool,
    close: Callback<String>,
) -> impl Fn(leptos::ev::MouseEvent) + 'static {
    move |e| {
        let Some(d) = node.get_untracked() else { return };
        let on_dialog = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()).map(|t| t == **d).unwrap_or(false);
        if !on_dialog || !dismiss {
            return;
        }
        let r = d.get_bounding_client_rect();
        let (x, y) = (f64::from(e.client_x()), f64::from(e.client_y()));
        if x < r.left() || x > r.right() || y < r.top() || y > r.bottom() {
            close.run("dismiss".into());
        }
    }
}

/// Close handle for buttons inside a [`Dialog`] or [`Sheet`]. Get it with [`use_dialog`].
#[derive(Clone, Copy)]
pub struct DialogHandle {
    close: Callback<String>,
}

impl DialogHandle {
    /// Close the dialog. `value` becomes its `returnValue` (delivered to `on_close`).
    pub fn close(&self, value: impl Into<String>) {
        self.close.run(value.into());
    }
}

pub fn use_dialog() -> DialogHandle {
    expect_context::<DialogHandle>()
}

/// A button that closes the surrounding [`Dialog`] or [`Sheet`] with a return value.
#[component]
pub fn DialogClose(
    /// Delivered to `on_close` as the return value (`"delete"`, `"save"`).
    #[prop(optional, into)]
    value: String,
    #[prop(optional)] variant: ButtonVariant,
    children: Children,
) -> impl IntoView {
    let h = use_dialog();
    view! { <Button variant=variant on:click=move |_| h.close(value.clone())>{children()}</Button> }
}

/// A modal for a decision that blocks everything else. Up to `palette-w` (600px) wide, centred on the scrim.
///
/// The title is the question; the confirming button repeats the verb and the object (`delete workspace`,
/// never `ok`). Put cancel first in `footer`, so it is focused first. Do not open a dialog from a dialog.
#[component]
pub fn Dialog(
    open: RwSignal<bool>,
    #[prop(into)] title: String,
    /// The scrim click does not close it when false. Use for destructive actions.
    #[prop(default = true)]
    dismiss: bool,
    /// Called after it closes with the `returnValue` (`"dismiss"` for a scrim click).
    #[prop(optional, into)]
    on_close: Option<Callback<String>>,
    /// Footer buttons: cancel on the left, the action on the right.
    #[prop(optional, into)]
    footer: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let node = NodeRef::<html::Dialog>::new();
    let (close, on_native_close) = modal_core(node, open, on_close);
    provide_context(DialogHandle { close });
    let (title_id, body_id) = (uid("dlg-t"), uid("dlg-d"));
    let (labelled, described) = (title_id.clone(), body_id.clone());
    let click = backdrop_click(node, dismiss, close);
    view! {
        <dialog
            class="d-dialog" node_ref=node
            aria-labelledby=labelled aria-describedby=described
            data-dismiss=if dismiss { "true" } else { "false" }
            on:close=on_native_close on:click=click
        >
            <header class="d-dialog__head">
                <h2 class="d-dialog__title" id=title_id>{title}</h2>
                <button type="button" class="d-btn d-btn--ghost d-btn--icon" aria-label="close" on:click=move |_| close.run(String::new())>
                    <Icon name="x"/>
                </button>
            </header>
            <div class="d-dialog__body" id=body_id>{children()}</div>
            {footer.map(|f| view! { <footer class="d-dialog__foot">{f.run()}</footer> })}
        </dialog>
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetSide {
    #[default]
    Right,
    Left,
    Bottom,
    Top,
}

impl SheetSide {
    fn attr(self) -> &'static str {
        match self {
            SheetSide::Right => "right",
            SheetSide::Left => "left",
            SheetSide::Bottom => "bottom",
            SheetSide::Top => "top",
        }
    }
}

/// A modal drawer that slides in from an edge, for a short task that must block the page. Not how bars open:
/// the sidebar, right bar and bottom bar of the [`Shell`](crate::Shell) expand inside the layout.
#[component]
pub fn Sheet(
    open: RwSignal<bool>,
    #[prop(into)] title: String,
    #[prop(optional)] side: SheetSide,
    #[prop(default = true)] dismiss: bool,
    #[prop(optional, into)] on_close: Option<Callback<String>>,
    #[prop(optional, into)] footer: Option<ViewFn>,
    children: Children,
) -> impl IntoView {
    let node = NodeRef::<html::Dialog>::new();
    let (close, on_native_close) = modal_core(node, open, on_close);
    provide_context(DialogHandle { close });
    let title_id = uid("sheet-t");
    let labelled = title_id.clone();
    let click = backdrop_click(node, dismiss, close);
    let close_label = format!("close {title}");
    view! {
        <dialog
            class="d-sheet" node_ref=node data-side=side.attr()
            aria-labelledby=labelled
            data-dismiss=if dismiss { "true" } else { "false" }
            on:close=on_native_close on:click=click
        >
            <header class="d-sheet__head">
                <h2 class="d-sheet__title" id=title_id>{title}</h2>
                <button type="button" class="d-btn d-btn--ghost d-btn--icon" aria-label=close_label on:click=move |_| close.run(String::new())>
                    <Icon name="x"/>
                </button>
            </header>
            <div class="d-sheet__body">{children()}</div>
            {footer.map(|f| view! { <footer class="d-sheet__foot">{f.run()}</footer> })}
        </dialog>
    }
}
