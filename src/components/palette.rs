use leptos::html;
use leptos::prelude::*;

use crate::components::dialog::{backdrop_click, modal_core};
use crate::util::uid;

/// A fuzzy match: higher `score` is better, `at` are the matched character indexes.
#[derive(Clone, Debug, PartialEq)]
pub struct FuzzyMatch {
    pub score: f64,
    pub at: Vec<usize>,
}

fn is_boundary(c: char) -> bool {
    c.is_whitespace() || matches!(c, '-' | '_' | '/' | ':' | '.')
}

fn fuzzy_from(q: &[char], s: &[char], start: usize) -> Option<FuzzyMatch> {
    let mut at = Vec::with_capacity(q.len());
    let (mut score, mut i, mut last) = (0.0_f64, start, -2_isize);
    for (k, qc) in q.iter().enumerate() {
        let j = s[i.min(s.len())..].iter().position(|c| c == qc).map(|p| p + i)?;
        let run = if j as isize == last + 1 { 3.0 } else { 0.0 };
        let word = if j == 0 || is_boundary(s[j - 1]) { 2.0 } else { 0.0 };
        let gap = if k > 0 { ((j - i).min(3)) as f64 * 0.1 } else { 0.0 };
        score += 1.0 + run + word - gap;
        at.push(j);
        last = j as isize;
        i = j + 1;
    }
    Some(FuzzyMatch { score: score - s.len() as f64 * 0.001, at })
}

/// Subsequence match: word starts and runs score higher. Every start of the first character is tried and the
/// best kept. An empty query matches everything with score 0.
pub fn fuzzy(query: &str, text: &str) -> Option<FuzzyMatch> {
    let q: Vec<char> = query.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect();
    let s: Vec<char> = text.to_lowercase().chars().collect();
    if q.is_empty() {
        return Some(FuzzyMatch { score: 0.0, at: Vec::new() });
    }
    let mut best: Option<FuzzyMatch> = None;
    let mut from = 0;
    while let Some(i) = s[from..].iter().position(|c| *c == q[0]).map(|p| p + from) {
        let Some(m) = fuzzy_from(&q, &s, i) else { break };
        if best.as_ref().map(|b| m.score > b.score).unwrap_or(true) {
            best = Some(m);
        }
        from = i + 1;
    }
    best
}

/// One command in the palette.
#[derive(Clone, Debug, PartialEq)]
pub struct PaletteItem {
    pub id: String,
    /// A lowercase verb phrase: `go to tokens`.
    pub label: String,
    /// Group header. Items keep their order inside a group.
    pub group: Option<String>,
    /// Shortcut keycaps (`["g", "t"]`). Show the real shortcut.
    pub hint: Vec<String>,
    /// Extra text the filter matches but does not show.
    pub keywords: Option<String>,
    /// Stays listed (so people can find out it exists) but cannot run.
    pub disabled: bool,
}

impl PaletteItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        PaletteItem { id: id.into(), label: label.into(), group: None, hint: Vec::new(), keywords: None, disabled: false }
    }

    pub fn group(mut self, g: impl Into<String>) -> Self {
        self.group = Some(g.into());
        self
    }

    pub fn hint<S: Into<String>>(mut self, keys: impl IntoIterator<Item = S>) -> Self {
        self.hint = keys.into_iter().map(Into::into).collect();
        self
    }

    pub fn keywords(mut self, k: impl Into<String>) -> Self {
        self.keywords = Some(k.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[derive(Clone, PartialEq)]
struct Row {
    item: PaletteItem,
    at: Vec<usize>,
    score: f64,
    index: usize,
}

#[derive(Clone, PartialEq)]
struct Group {
    name: String,
    rows: Vec<Row>,
    best: f64,
}

fn compute(items: &[PaletteItem], query: &str) -> Vec<Group> {
    let q = query.trim();
    let mut groups: Vec<Group> = Vec::new();
    for (idx, it) in items.iter().enumerate() {
        let m = if q.is_empty() {
            Some(FuzzyMatch { score: -(idx as f64), at: Vec::new() })
        } else {
            fuzzy(q, &it.label).or_else(|| {
                it.keywords.as_deref().and_then(|k| fuzzy(q, k)).map(|_| FuzzyMatch { score: 0.5, at: Vec::new() })
            })
        };
        let Some(m) = m else { continue };
        let name = it.group.clone().unwrap_or_default();
        let g = match groups.iter().position(|g| g.name == name) {
            Some(p) => &mut groups[p],
            None => {
                groups.push(Group { name, rows: Vec::new(), best: f64::MIN });
                groups.last_mut().unwrap()
            }
        };
        g.best = g.best.max(m.score);
        g.rows.push(Row { item: it.clone(), at: m.at, score: m.score, index: 0 });
    }
    if !q.is_empty() {
        groups.sort_by(|a, b| b.best.total_cmp(&a.best));
        for g in &mut groups {
            g.rows.sort_by(|a, b| b.score.total_cmp(&a.score));
        }
    }
    let mut n = 0;
    for g in &mut groups {
        for r in &mut g.rows {
            r.index = n;
            n += 1;
        }
    }
    groups
}

fn highlight(label: &str, at: &[usize]) -> impl IntoView + use<> {
    let chars: Vec<char> = label.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if at.contains(&i) {
            out.push(view! { <mark>{chars[i].to_string()}</mark> }.into_any());
            i += 1;
        } else {
            let start = i;
            while i < chars.len() && !at.contains(&i) {
                i += 1;
            }
            out.push(chars[start..i].iter().collect::<String>().into_any());
        }
    }
    out.collect_view()
}

/// The keyboard-first launcher: a prompt, a fuzzy-filtered list of five visible rows, group headers and
/// shortcut hints. Opens with ctrl+k / cmd+k.
///
/// Focus never leaves the input (`aria-activedescendant`); as a `<dialog>` it traps focus and makes the page inert.
#[component]
pub fn CommandPalette(
    #[prop(into)] items: Signal<Vec<PaletteItem>>,
    /// Open state of the overlay. Ignored when `inline`.
    open: RwSignal<bool>,
    /// Runs the chosen command (after the palette closes).
    #[prop(into)]
    on_run: Callback<PaletteItem>,
    #[prop(optional, into)] placeholder: Option<String>,
    /// Bind ctrl+k / cmd+k to toggle. Default true; ignored when `inline`.
    #[prop(default = true)]
    hotkey: bool,
    /// Render in place instead of as an overlay (for docs and embedded launchers).
    #[prop(optional)]
    inline: bool,
) -> impl IntoView {
    let placeholder = placeholder.unwrap_or_else(|| "type a command".into());
    let base = uid("pal");
    let list_id = format!("{base}-list");
    let query = RwSignal::new(String::new());
    let sel = RwSignal::new(0usize);
    let input = NodeRef::<html::Input>::new();
    let dialog = NodeRef::<html::Dialog>::new();
    let list = NodeRef::<html::Div>::new();

    let groups = Memo::new(move |_| compute(&items.get(), &query.get()));
    let flat = Memo::new(move |_| groups.get().into_iter().flat_map(|g| g.rows).collect::<Vec<_>>());

    let first_enabled = move || flat.with(|f| f.iter().position(|r| !r.item.disabled).unwrap_or(0));
    // a new result set resets the selection to the first runnable row
    Effect::new(move |_| {
        flat.track();
        sel.set(first_enabled());
    });

    let (close, on_native_close) = modal_core(dialog, open, None);
    let backdrop = backdrop_click(dialog, true, close);

    // opening clears the prompt
    Effect::new(move |prev: Option<bool>| {
        let now = open.get();
        if now && prev != Some(true) {
            query.set(String::new());
            if let Some(i) = input.get_untracked() {
                i.set_value("");
            }
        }
        now
    });

    if hotkey && !inline {
        window_event_listener(leptos::ev::keydown, move |e| {
            if (e.meta_key() || e.ctrl_key()) && !e.shift_key() && !e.alt_key() && e.key().to_lowercase() == "k" {
                e.prevent_default();
                open.update(|o| *o = !*o);
            }
        });
    }

    let run = move |index: usize| {
        let Some(row) = flat.with(|f| f.get(index).cloned()) else { return };
        if row.item.disabled {
            return;
        }
        if !inline {
            open.set(false);
        }
        on_run.run(row.item);
    };

    let mv = move |d: isize| {
        let n = flat.with(|f| f.len());
        if n == 0 {
            return;
        }
        let mut i = sel.get_untracked();
        for _ in 0..n {
            i = ((i as isize + d).rem_euclid(n as isize)) as usize;
            if !flat.with(|f| f[i].item.disabled) {
                break;
            }
        }
        sel.set(i);
    };

    let onkey = {
        move |e: leptos::ev::KeyboardEvent| match e.key().as_str() {
            "ArrowDown" => {
                e.prevent_default();
                mv(1);
            }
            "ArrowUp" => {
                e.prevent_default();
                mv(-1);
            }
            "Enter" => {
                e.prevent_default();
                run(sel.get_untracked());
            }
            "Escape" if inline && !query.get_untracked().is_empty() => {
                e.prevent_default();
                query.set(String::new());
                if let Some(i) = input.get_untracked() {
                    i.set_value("");
                }
            }
            _ => {}
        }
    };

    // keep the selected row in view
    {
        let base = base.clone();
        Effect::new(move |_| {
            let i = sel.get();
            if i == 0 {
                if let Some(l) = list.get() {
                    l.set_scroll_top(0);
                }
            } else if let Some(el) = document().get_element_by_id(&format!("{base}-o{i}")) {
                let opts = web_sys::ScrollIntoViewOptions::new();
                opts.set_block(web_sys::ScrollLogicalPosition::Nearest);
                el.scroll_into_view_with_scroll_into_view_options(&opts);
            }
        });
    }

    let status = move || {
        let n = flat.with(|f| f.len());
        match n {
            0 => "no matches".to_string(),
            1 => "1 result".to_string(),
            n => format!("{n} results"),
        }
    };

    let body = {
        let base = base.clone();
        let list_id = list_id.clone();
        let placeholder = placeholder.clone();
        let base_active = base.clone();
        view! {
            <div class="d-palette__prompt">
                <span aria-hidden="true">">"</span>
                <input
                    node_ref=input class="d-palette__input" type="text" role="combobox"
                    aria-expanded="true" aria-controls=list_id.clone() aria-autocomplete="list"
                    aria-label=placeholder.clone() placeholder=placeholder
                    autocomplete="off" spellcheck="false"
                    aria-activedescendant=move || flat.with(|f| (!f.is_empty()).then(|| format!("{base_active}-o{}", sel.get())))
                    on:input=move |e| query.set(event_target_value(&e))
                    on:keydown=onkey
                />
            </div>
            <div
                node_ref=list class="d-palette__list" id=list_id role="listbox" aria-label="commands"
                on:mousedown=move |e| e.prevent_default()
            >
                {move || {
                    let gs = groups.get();
                    if gs.iter().all(|g| g.rows.is_empty()) {
                        return view! { <div class="d-palette__empty">{format!("no matches for \"{}\"", query.get())}</div> }.into_any();
                    }
                    gs.into_iter().map(|g| {
                        let head_id = uid("pg");
                        let head = (!g.name.is_empty()).then(|| view! { <div class="d-palette__group" id=head_id.clone() role="presentation">{g.name.clone()}</div> });
                        let labelled = (!g.name.is_empty()).then(|| head_id.clone());
                        let rows = g.rows.into_iter().map(|r| {
                            let idx = r.index;
                            let id = format!("{base}-o{idx}");
                            let hint = (!r.item.hint.is_empty()).then(|| view! {
                                <span class="d-palette__hint">
                                    {r.item.hint.iter().map(|k| view! { <kbd class="d-kbd">{k.clone()}</kbd> }).collect_view()}
                                </span>
                            });
                            view! {
                                <div
                                    class="d-palette__opt" id=id role="option"
                                    aria-selected=move || if sel.get() == idx { "true" } else { "false" }
                                    aria-disabled=r.item.disabled.then_some("true")
                                    on:mousemove=move |_| {
                                        let disabled = flat.with(|f| f.get(idx).map(|r| r.item.disabled).unwrap_or(true));
                                        if sel.get_untracked() != idx && !disabled { sel.set(idx) }
                                    }
                                    on:click=move |_| run(idx)
                                >
                                    <span>{highlight(&r.item.label, &r.at)}</span>
                                    {hint}
                                </div>
                            }
                        }).collect_view();
                        view! { <div role="group" aria-labelledby=labelled>{head}{rows}</div> }
                    }).collect_view().into_any()
                }}
            </div>
            <div class="d-palette__foot">
                <span><kbd class="d-kbd">"↑↓"</kbd>" move"</span>
                <span><kbd class="d-kbd">"↵"</kbd>" run"</span>
                <span><kbd class="d-kbd">"esc"</kbd>" close"</span>
            </div>
            <div class="d-sr" role="status">{status}</div>
        }
    };

    if inline {
        view! { <div class="d-palette d-palette--static" aria-label="command palette">{body}</div> }.into_any()
    } else {
        view! {
            <dialog class="d-palette" node_ref=dialog aria-label="command palette" on:close=on_native_close on:click=backdrop>
                {body}
            </dialog>
        }
        .into_any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_query_matches_everything() {
        assert_eq!(fuzzy("", "anything").unwrap().at.len(), 0);
    }

    #[test]
    fn subsequence_matches_and_reports_indexes() {
        let m = fuzzy("gt", "go to tokens").unwrap();
        assert_eq!(m.at.len(), 2);
        assert_eq!(m.at[0], 0);
    }

    #[test]
    fn non_subsequence_is_none() {
        assert!(fuzzy("zz", "go to tokens").is_none());
    }

    #[test]
    fn word_starts_and_runs_score_higher() {
        let word = fuzzy("sw", "switch").unwrap().score;
        let scattered = fuzzy("sw", "assorted wine").unwrap().score;
        assert!(word > scattered);
    }

    #[test]
    fn groups_keep_order_without_a_query_and_rank_with_one() {
        let items = vec![
            PaletteItem::new("a", "go to tokens").group("navigate"),
            PaletteItem::new("b", "switch to light theme").group("theme"),
            PaletteItem::new("c", "go to components").group("navigate"),
        ];
        let g = compute(&items, "");
        assert_eq!(g[0].name, "navigate");
        assert_eq!(g[0].rows.len(), 2);
        let g = compute(&items, "light");
        assert_eq!(g.len(), 1);
        assert_eq!(g[0].rows[0].item.id, "b");
    }
}
