use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::types::Size;

/// A dense data table: `row-h` rows, `border-subtle` separators, sticky header, tabular numerals, no zebra striping.
///
/// You own the data and the sort; render `<thead>`/`<tbody>` as usual with [`Th`], [`Tr`] and [`Td`].
#[component]
pub fn Table(
    /// Names the scroll region (`aria-label`).
    #[prop(into)]
    label: String,
    #[prop(optional)] size: Size,
    /// CSS `max-height` of the scroll area, like `"200px"`.
    #[prop(optional, into)]
    max_height: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="d-table-wrap" tabindex="0" role="region" aria-label=label style=max_height.map(|h| format!("max-height:{h}"))>
            <table class=if size == Size::Lg { "d-table d-table--lg" } else { "d-table" }>{children()}</table>
        </div>
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SortDirection {
    #[default]
    None,
    Ascending,
    Descending,
}

impl SortDirection {
    fn attr(self) -> &'static str {
        match self {
            SortDirection::None => "none",
            SortDirection::Ascending => "ascending",
            SortDirection::Descending => "descending",
        }
    }

    /// The next direction when a sortable header is pressed: ascending first, then flipping.
    pub fn next(self) -> Self {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            _ => SortDirection::Ascending,
        }
    }
}

/// A header cell. Pass `sort` and `on_sort` to make it a sort button; the arrow glyph follows `aria-sort`.
#[component]
pub fn Th(
    /// Right-align (numbers).
    #[prop(optional)]
    num: bool,
    #[prop(optional, into)] sort: Option<Signal<SortDirection>>,
    #[prop(optional, into)] on_sort: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let class = num.then_some("d-num");
    match sort {
        Some(sort) => view! {
            <th scope="col" class=class aria-sort=move || sort.get().attr()>
                <button type="button" class="d-table__sort" on:click=move |_| { if let Some(cb) = on_sort { cb.run(()) } }>
                    {children()}
                </button>
            </th>
        }
        .into_any(),
        None => view! { <th scope="col" class=class>{children()}</th> }.into_any(),
    }
}

/// A body row. With `selectable`, click/enter/space call `on_select`, and up/down move between rows.
#[component]
pub fn Tr(
    #[prop(optional)] selectable: bool,
    #[prop(optional, into)] selected: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] on_select: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let pick = move || {
        if !disabled.get_untracked() {
            if let Some(cb) = on_select {
                cb.run(());
            }
        }
    };
    let onkey = move |e: leptos::ev::KeyboardEvent| {
        let Some(row) = e.current_target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else { return };
        // only when the row itself has focus, not a control inside it
        if e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()).as_ref() != Some(&row) {
            return;
        }
        let focus = |el: Option<web_sys::Element>| {
            if let Some(h) = el.and_then(|n| n.dyn_into::<web_sys::HtmlElement>().ok()) {
                let _ = h.focus();
            }
        };
        match e.key().as_str() {
            "Enter" | " " => {
                e.prevent_default();
                pick();
            }
            "ArrowDown" if row.next_element_sibling().is_some() => {
                e.prevent_default();
                focus(row.next_element_sibling());
            }
            "ArrowUp" if row.previous_element_sibling().is_some() => {
                e.prevent_default();
                focus(row.previous_element_sibling());
            }
            _ => {}
        }
    };
    if selectable {
        view! {
            <tr
                tabindex="0"
                aria-selected=move || if selected.get() { "true" } else { "false" }
                aria-disabled=move || disabled.get().then_some("true")
                on:click=move |_| pick()
                on:keydown=onkey
            >
                {children()}
            </tr>
        }
        .into_any()
    } else {
        view! { <tr>{children()}</tr> }.into_any()
    }
}

/// A cell. `num` right-aligns with tabular numerals.
#[component]
pub fn Td(#[prop(optional)] num: bool, children: Children) -> impl IntoView {
    view! { <td class=num.then_some("d-num")>{children()}</td> }
}

/// How an [`EditCell`] edits: text, number or date.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EditKind {
    #[default]
    Text,
    Number,
    Date,
}

impl EditKind {
    fn attr(self) -> &'static str {
        match self {
            EditKind::Text => "text",
            EditKind::Number => "number",
            EditKind::Date => "date",
        }
    }
}

/// A cell that edits in place: click it, or focus it and press enter or f2, and it becomes an input. Enter or
/// leaving the field calls `on_save` with the new text; esc cancels. Do not make a row both selectable and
/// editable.
///
/// `children` render the display (formatted) value; `value` is the raw text the input starts with.
#[component]
pub fn EditCell(
    /// Names the input (`description`, `amount`).
    #[prop(into)]
    label: String,
    #[prop(optional)] kind: EditKind,
    /// The raw value the input starts with.
    #[prop(into)]
    value: Signal<String>,
    #[prop(into)] on_save: Callback<String>,
    /// Right-align with tabular numerals.
    #[prop(optional)]
    num: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let editing = RwSignal::new(false);
    let input = NodeRef::<leptos::html::Input>::new();
    let begin = move || {
        editing.set(true);
    };
    Effect::new(move |_| {
        if editing.get() {
            if let Some(i) = input.get() {
                let _ = i.focus();
                i.select();
            }
        }
    });
    let finish = move |save: bool| {
        if !editing.get_untracked() {
            return;
        }
        editing.set(false);
        if save {
            if let Some(i) = input.get_untracked() {
                let v = i.value();
                if v != value.get_untracked() {
                    on_save.run(v);
                }
            }
        }
    };
    let cell_label = label.clone();
    view! {
        <td class=num.then_some("d-num") data-edit=kind.attr() data-label=cell_label>
            {move || if editing.get() {
                view! {
                    <input
                        class="d-table__input" node_ref=input aria-label=label.clone()
                        type=match kind { EditKind::Date => "date", _ => "text" }
                        inputmode=(kind == EditKind::Number).then_some("decimal")
                        prop:value=move || value.get()
                        on:keydown=move |e| match e.key().as_str() {
                            "Enter" => { e.prevent_default(); finish(true) }
                            "Escape" => { e.prevent_default(); e.stop_propagation(); finish(false) }
                            _ => {}
                        }
                        on:blur=move |_| finish(true)
                    />
                }.into_any()
            } else {
                view! {
                    <span
                        tabindex="0" role="button" style="display:block"
                        on:click=move |_| begin()
                        on:keydown=move |e| if e.key() == "Enter" || e.key() == "F2" { e.prevent_default(); begin() }
                    >
                        {children()}
                    </span>
                }.into_any()
            }}
        </td>
    }
}
