use leptos::prelude::*;

use super::menu::{Menu, MenuFoot, close_open_menu};
use crate::util::uid;

/// A row of filters over a list or table: a search field, dropdown [`Filter`]s that state their own value,
/// and a line of removable chips for what is active (put [`FilterActive`] last).
#[component]
pub fn FilterBar(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! { <div class="d-filterbar" role="group" aria-label=label>{children()}</div> }
}

/// The search field of a [`FilterBar`].
#[component]
pub fn FilterSearch(
    #[prop(into)] label: String,
    value: RwSignal<String>,
    #[prop(optional, into)] placeholder: Option<String>,
) -> impl IntoView {
    let id = uid("search");
    view! {
        <div class="d-filterbar__search">
            <label class="d-sr" for=id.clone()>{label}</label>
            <input
                class="d-input" id=id type="search" placeholder=placeholder
                prop:value=move || value.get()
                on:input=move |e| value.set(event_target_value(&e))
            />
        </div>
    }
}

/// The chip row for active filters. Render it only when something is active.
#[component]
pub fn FilterActive(children: Children) -> impl IntoView {
    view! { <div class="d-filterbar__active" aria-label="active filters">{children()}</div> }
}

/// One choice in a [`Filter`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilterOption {
    pub value: String,
    pub label: String,
    /// Secondary text in the menu row: a count, a balance, a hint.
    pub meta: Option<String>,
    /// Show the option as a coloured tag (categorical colour 1 to 6).
    pub cat: Option<u8>,
}

impl FilterOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        FilterOption { value: value.into(), label: label.into(), meta: None, cat: None }
    }

    pub fn meta(mut self, meta: impl Into<String>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn cat(mut self, cat: u8) -> Self {
        self.cat = Some(cat);
        self
    }
}

/// A dropdown filter. Its label is the noun (`type`); its value is what is chosen (`any`, a label, or
/// `3 selected`). With something chosen it shows `data-active`.
///
/// Multi-select by default: the menu stays open while you tick, with `clear` and `done`. With `single`, rows
/// are radios and picking one closes the menu.
#[component]
pub fn Filter(
    /// The noun: `type`, `account`, `tags`.
    #[prop(into)]
    label: String,
    #[prop(into)] options: Signal<Vec<FilterOption>>,
    /// The chosen option values.
    selected: RwSignal<Vec<String>>,
    #[prop(optional)] single: bool,
    /// Shown when nothing is chosen. Default `any`.
    #[prop(default = "any")]
    empty: &'static str,
) -> impl IntoView {
    let value_text = move || {
        let s = selected.get();
        match s.len() {
            0 => empty.to_string(),
            1 => options
                .get()
                .into_iter()
                .find(|o| o.value == s[0])
                .map(|o| o.label)
                .unwrap_or_else(|| s[0].clone()),
            n => format!("{n} selected"),
        }
    };
    let menu_label = label.clone();
    let trigger = {
        let label = label.clone();
        move || {
            let label = label.clone();
            view! {
                <button type="button" class="d-btn d-btn--secondary d-filter" data-active=move || (!selected.get().is_empty()).then_some("")>
                    {label}" "<span class="d-filter__value">{value_text}</span>
                </button>
            }
        }
    };
    view! {
        <Menu label=menu_label trigger=trigger multi=!single>
            {move || options.get().into_iter().map(|o| {
                let v = o.value.clone();
                let v2 = o.value.clone();
                let on = move || selected.get().contains(&v);
                let body = match o.cat {
                    Some(c) => view! { <span class="d-badge" data-cat=c.to_string()>{o.label.clone()}</span> }.into_any(),
                    None => o.label.clone().into_any(),
                };
                view! {
                    <button
                        type="button" tabindex="-1"
                        class="d-menu__item"
                        role=if single { "menuitemradio" } else { "menuitemcheckbox" }
                        aria-checked=move || if on() { "true" } else { "false" }
                        data-value=o.value.clone()
                        on:click=move |_| {
                            if single {
                                selected.set(vec![v2.clone()]);
                            } else {
                                selected.update(|s| {
                                    if let Some(i) = s.iter().position(|x| *x == v2) { s.remove(i); } else { s.push(v2.clone()); }
                                });
                            }
                        }
                    >
                        <span class="d-menu__mark"></span>
                        {body}
                        {o.meta.clone().map(|m| view! { <span class="d-menu__meta">{m}</span> })}
                    </button>
                }
            }).collect_view()}
            {(!single).then(|| view! {
                <MenuFoot>
                    <button type="button" class="d-btn d-btn--ghost d-btn--sm" on:click=move |_| selected.set(Vec::new())>"clear"</button>
                    <button type="button" class="d-btn d-btn--secondary d-btn--sm" on:click=move |_| close_open_menu()>"done"</button>
                </MenuFoot>
            })}
        </Menu>
    }
}
