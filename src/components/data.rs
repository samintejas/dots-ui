//! Shares of a whole (Meter, Ring) and moving through a long list (Pager).

use leptos::prelude::*;

use crate::types::Severity;

fn pct(v: f64) -> f64 {
    v.clamp(0.0, 100.0)
}

/// A 4px bar for an amount or share. Always show the number as text beside it ([`MeterRow`]).
///
/// Grey by default; `severity` only when the value is in that state (over a limit), and say so in words.
#[component]
pub fn Meter(
    /// 0 to 100.
    #[prop(into)]
    value: Signal<f64>,
    /// Id of the element that names it.
    #[prop(optional, into)]
    labelledby: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    /// Text for assistive tech instead of the percentage, like `₹6,110`.
    #[prop(optional, into)]
    text: MaybeProp<String>,
    #[prop(optional, into)] severity: MaybeProp<Severity>,
    /// Categorical colour 1 to 6, when several meters are series that must be told apart.
    #[prop(optional, into)]
    cat: MaybeProp<u8>,
) -> impl IntoView {
    view! {
        <div
            class="d-meter" role="meter"
            aria-labelledby=labelledby aria-label=label
            aria-valuemin="0" aria-valuemax="100"
            aria-valuenow=move || pct(value.get()).round().to_string()
            aria-valuetext=move || text.get()
            data-severity=move || severity.get().map(Severity::as_str)
            data-cat=move || cat.get().map(|c| c.to_string())
            style=move || format!("--d-value:{}", pct(value.get()))
        >
            <div class="d-meter__bar"></div>
        </div>
    }
}

/// Label, bar and value, the way the design system sets them.
#[component]
pub fn MeterRow(
    #[prop(into)] label: String,
    #[prop(into)] value: Signal<f64>,
    /// The figure written out (`₹6,110`). Becomes the meter's text too.
    #[prop(into)]
    output: Signal<String>,
    #[prop(optional, into)] severity: MaybeProp<Severity>,
    #[prop(optional, into)] cat: MaybeProp<u8>,
) -> impl IntoView {
    let id = crate::util::uid("meter");
    view! {
        <div class="d-meter-row">
            <span id=id.clone()>{label}</span>
            <Meter value=value labelledby=id text=Signal::derive(move || Some(output.get())) severity=severity cat=cat/>
            <output>{move || output.get()}</output>
        </div>
    }
}

/// A ring with one number inside and a two-word caption. The one round data shape.
#[component]
pub fn Ring(
    /// 0 to 100.
    #[prop(into)]
    value: Signal<f64>,
    /// The number inside (`89`).
    #[prop(into)]
    number: Signal<String>,
    /// Two words under it (`months left`). Not shown in the `small` ring.
    #[prop(optional, into)]
    caption: Option<String>,
    /// Full sentence for assistive tech (`home loan, 91 of 180 months paid`).
    #[prop(into)]
    label: String,
    /// 48px.
    #[prop(optional)]
    small: bool,
    #[prop(optional, into)] severity: MaybeProp<Severity>,
) -> impl IntoView {
    view! {
        <div
            class=if small { "d-ring d-ring--sm" } else { "d-ring" }
            role="meter" aria-label=label
            aria-valuemin="0" aria-valuemax="100"
            aria-valuenow=move || pct(value.get()).round().to_string()
            data-severity=move || severity.get().map(Severity::as_str)
            style=move || format!("--d-value:{}", pct(value.get()))
        >
            <span class="d-ring__in">
                <b class="d-ring__value">{move || number.get()}</b>
                {caption.map(|c| view! { <span class="d-ring__label">{c}</span> })}
            </span>
        </div>
    }
}

/// Previous and next over a long list: the range in words, rows per page, `page N of M`.
///
/// You own the data. `page` is 1-based; changing the page size returns to page 1.
#[component]
pub fn Pager(
    /// Total rows (the filtered total when filters are on).
    #[prop(into)]
    total: Signal<usize>,
    page: RwSignal<usize>,
    page_size: RwSignal<usize>,
    #[prop(default = vec![10, 25, 50])] sizes: Vec<usize>,
    /// Pin it to the bottom of the scrolling page.
    #[prop(optional)]
    sticky: bool,
) -> impl IntoView {
    let pages = move || total.get().div_ceil(page_size.get().max(1)).max(1);
    // keep the page in range when the total shrinks (a filter changed)
    Effect::new(move |_| {
        let last = pages();
        if page.get_untracked() > last {
            page.set(last);
        }
    });
    let range = move || {
        let (t, s, p) = (total.get(), page_size.get().max(1), page.get());
        if t == 0 {
            "0 of 0".to_string()
        } else {
            format!("{} to {} of {}", (p - 1) * s + 1, (p * s).min(t), t)
        }
    };
    let id = crate::util::uid("rows");
    let for_id = id.clone();
    view! {
        <nav class=if sticky { "d-pager d-pager--sticky" } else { "d-pager" } aria-label="pages">
            <span>{range}</span>
            <div class="d-pager__nav">
                <label class="d-pager__size" for=for_id>
                    "rows per page"
                    <span class="d-select">
                        <select
                            class="d-input" id=id
                            on:change=move |e| {
                                if let Ok(n) = event_target_value(&e).parse::<usize>() {
                                    page_size.set(n);
                                    page.set(1);
                                }
                            }
                        >
                            {sizes.into_iter().map(|n| view! {
                                <option value=n.to_string() selected=move || page_size.get() == n>{n.to_string()}</option>
                            }).collect_view()}
                        </select>
                    </span>
                </label>
                <button type="button" class="d-btn d-btn--secondary" disabled={move || page.get() <= 1} on:click=move |_| page.update(|p| *p -= 1)>
                    "previous"
                </button>
                <span class="d-pager__page">{move || format!("page {} of {}", page.get(), pages())}</span>
                <button type="button" class="d-btn d-btn--secondary" disabled={move || page.get() >= pages()} on:click=move |_| page.update(|p| *p += 1)>
                    "next"
                </button>
            </div>
        </nav>
    }
}
