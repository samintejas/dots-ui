use leptos::prelude::*;
use wasm_bindgen::JsCast;

use super::badge::tag_cat;
use crate::icon::Icon;
use crate::util::{dom, uid};

fn norm(s: &str) -> String {
    s.trim().to_lowercase()
}

/// Pick several tags from a list or type new ones: chips in the field, a filtered multi-select list under
/// it, comma or enter to create. Tags are lowercased and trimmed; duplicates are ignored. Each tag gets its
/// categorical colour from its name.
///
/// `bare` removes the well, for use inside an [`InputSeg`](super::InputSeg) where the bar is the well; then
/// pass the `id` the segment's label points at.
#[component]
pub fn TagInput(
    /// The chosen tags.
    value: RwSignal<Vec<String>>,
    /// Existing tags offered in the list.
    #[prop(into)]
    suggestions: Signal<Vec<String>>,
    /// Visible label. Omit with `bare`, where the segment caption is the label.
    #[prop(optional, into)]
    label: Option<String>,
    #[prop(optional, into)] id: Option<String>,
    #[prop(optional, into)] hint: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    /// Offer `add new tag: …` for names not in the list (default true).
    #[prop(default = true)]
    create: bool,
    #[prop(optional)] bare: bool,
    #[prop(optional, into)] invalid: Signal<bool>,
) -> impl IntoView {
    let id = id.unwrap_or_else(|| uid("tags"));
    let hint_id = format!("{id}-h");
    let list_id = format!("{id}-list");
    let root = NodeRef::<leptos::html::Div>::new();
    let list = NodeRef::<leptos::html::Div>::new();
    let input = NodeRef::<leptos::html::Input>::new();
    let open = RwSignal::new(false);
    let query = RwSignal::new(String::new());
    let placeholder = StoredValue::new(placeholder.unwrap_or_default());

    let add = move |name: &str| {
        let n = norm(name);
        if n.is_empty() {
            return;
        }
        value.update(|v| {
            if !v.contains(&n) {
                v.push(n)
            }
        });
    };
    let remove = move |name: &str| value.update(|v| v.retain(|x| x != name));
    let commit = move || {
        for p in query.get_untracked().split(',') {
            add(p);
        }
        query.set(String::new());
    };
    let visible = move || {
        let q = norm(&query.get());
        suggestions.get().into_iter().map(|s| norm(&s)).filter(|s| q.is_empty() || s.contains(&q)).collect::<Vec<_>>()
    };
    let can_create = move || {
        let q = norm(&query.get());
        create && !q.is_empty() && !suggestions.get().iter().any(|s| norm(s) == q) && !value.get().contains(&q)
    };
    let focus_option = move |first: bool| {
        let Some(l) = list.get_untracked() else { return };
        let opts = dom::query_all(&l, "[role=option]:not([hidden])");
        if let Some(o) = if first { opts.first() } else { opts.last() } {
            dom::focus(o);
        }
    };

    let on_input = move |e: leptos::ev::Event| {
        let v = event_target_value(&e);
        if let Some((done, rest)) = v.rsplit_once(',') {
            for p in done.split(',') {
                add(p);
            }
            query.set(rest.trim_start().to_string());
            if let Some(i) = input.get_untracked() {
                i.set_value(&query.get_untracked());
            }
        } else {
            query.set(v);
        }
        open.set(true);
    };
    let on_key = move |e: leptos::ev::KeyboardEvent| match e.key().as_str() {
        "Enter" if !norm(&query.get_untracked()).is_empty() => {
            e.prevent_default();
            commit();
        }
        "Backspace" if query.get_untracked().is_empty() => {
            if let Some(last) = value.get_untracked().last().cloned() {
                remove(&last);
            }
        }
        "Escape" if open.get_untracked() => {
            e.stop_propagation();
            open.set(false);
        }
        "ArrowDown" => {
            e.prevent_default();
            open.set(true);
            focus_option(true);
        }
        "ArrowUp" => {
            e.prevent_default();
            open.set(true);
            focus_option(false);
        }
        _ => {}
    };
    let on_list_key = move |e: leptos::ev::KeyboardEvent| {
        let Some(l) = list.get_untracked() else { return };
        let opts: Vec<_> = dom::query_all(&l, "[role=option]:not([hidden])");
        let i = dom::active_element().and_then(|a| opts.iter().position(|o| *o == a));
        match (e.key().as_str(), i) {
            ("ArrowDown", Some(i)) if i + 1 < opts.len() => {
                e.prevent_default();
                dom::focus(&opts[i + 1]);
            }
            ("ArrowUp", Some(i)) if i > 0 => {
                e.prevent_default();
                dom::focus(&opts[i - 1]);
            }
            ("ArrowUp", Some(0)) | ("Escape", _) => {
                e.prevent_default();
                e.stop_propagation();
                if e.key() == "Escape" {
                    open.set(false);
                }
                if let Some(inp) = input.get_untracked() {
                    let _ = inp.focus();
                }
            }
            _ => {}
        }
    };
    // close when focus leaves the whole control
    let on_focusout = move |e: leptos::ev::FocusEvent| {
        let to = e.related_target().and_then(|t| t.dyn_into::<web_sys::Node>().ok());
        let inside = match (root.get_untracked(), to) {
            (Some(r), Some(t)) => r.contains(Some(&t)),
            _ => false,
        };
        if !inside {
            open.set(false);
        }
    };
    let on_root_click = move |e: leptos::ev::MouseEvent| {
        let target = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok());
        let in_list = match (list.get_untracked(), &target) {
            (Some(l), Some(t)) => l.contains(Some(t)),
            _ => false,
        };
        let on_x = target.as_ref().map(|t| t.closest(".d-badge__x").ok().flatten().is_some()).unwrap_or(false);
        if !in_list && !on_x {
            if let Some(i) = input.get_untracked() {
                let _ = i.focus();
            }
        }
    };

    let control = view! {
        <div
            class=if bare { "d-taginput d-taginput--bare" } else { "d-taginput" }
            node_ref=root
            aria-invalid=move || invalid.get().then_some("true")
            on:click=on_root_click
            on:focusout=on_focusout
        >
            {move || value.get().into_iter().map(|t| {
                let c = tag_cat(&t).to_string();
                let (name, name2) = (t.clone(), t.clone());
                view! {
                    <span class="d-badge" data-value=name data-cat=c>
                        {t}
                        <button type="button" class="d-badge__x" aria-label=format!("remove {name2}") on:click=move |_| remove(&name2)>
                            <Icon name="x" small=true/>
                        </button>
                    </span>
                }
            }).collect_view()}
            <input
                class="d-taginput__input" id=id.clone() type="text" node_ref=input
                role="combobox" aria-autocomplete="list" aria-controls=list_id.clone()
                aria-expanded=move || open.get().to_string()
                aria-describedby=hint.as_ref().map(|_| hint_id.clone())
                placeholder=move || if value.get().is_empty() { placeholder.get_value() } else { String::new() }
                prop:value=move || query.get()
                on:input=on_input
                on:keydown=on_key
                on:focus=move |_| open.set(true)
            />
            <div
                class="d-menu d-taginput__list" id=list_id node_ref=list
                role="listbox" aria-multiselectable="true" aria-label="tags"
                hidden=move || !open.get()
                on:keydown=on_list_key
                on:mousedown=move |e| {
                    // keep focus in the field unless an option itself is being focused
                    if e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()).map(|t| t.closest("[role=option]").ok().flatten().is_none()).unwrap_or(true) {
                        e.prevent_default();
                    }
                }
            >
                {move || visible().into_iter().map(|s| {
                    let (a, b) = (s.clone(), s.clone());
                    let cat = tag_cat(&s).to_string();
                    let dv = s.clone();
                    let selected = move || value.get().contains(&a);
                    view! {
                        <button
                            type="button" class="d-menu__item" role="option" tabindex="-1"
                            data-value=dv
                            aria-selected=move || if selected() { "true" } else { "false" }
                            on:click=move |_| {
                                if value.get_untracked().contains(&b) { remove(&b) } else { add(&b) }
                                query.set(String::new());
                                if let Some(i) = input.get_untracked() { let _ = i.focus(); }
                            }
                        >
                            <span class="d-menu__mark"></span>
                            <span class="d-badge" data-cat=cat>{s}</span>
                        </button>
                    }
                }).collect_view()}
                {move || can_create().then(|| view! {
                    <button
                        type="button" class="d-menu__item d-taginput__create" tabindex="-1"
                        on:click=move |_| { commit(); if let Some(i) = input.get_untracked() { let _ = i.focus(); } }
                    >
                        {format!("add new tag: {}", norm(&query.get()))}
                    </button>
                })}
            </div>
        </div>
    };
    if bare {
        control.into_any()
    } else {
        view! {
            <div class="d-field">
                {label.map(|l| view! { <label class="d-label" for=id.clone()>{l}</label> })}
                {control}
                {hint.map(|h| view! { <span class="d-hint" id=hint_id.clone()>{h}</span> })}
            </div>
        }
        .into_any()
    }
}
