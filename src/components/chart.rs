//! Chart scaffolding: the frame, the legend and the table view. You draw the marks in an `<svg>`.
//!
//! Rules from the design system: series take `cat-1` … `cat-6` in order and never cycle (a seventh folds into
//! "other"); one y axis; a legend for two series or more; a table view for every chart. Marks use the classes
//! `d-chart__bar`, `d-chart__line` (with `data-cat="1"…"6"`), `d-chart__grid`, `d-chart__axis` and `d-chart__tick`.

use leptos::prelude::*;

/// A chart: a `<figure class="d-chart">` holding your `<svg>`, a [`Legend`] and a [`ChartTable`].
#[component]
pub fn Chart(
    /// What the chart shows, for its accessible name.
    #[prop(into)]
    label: String,
    children: Children,
) -> impl IntoView {
    view! { <figure class="d-chart" role="group" aria-label=label>{children()}</figure> }
}

/// The table view every chart carries, folded under a summary line.
#[component]
pub fn ChartTable(
    /// The summary text. Default `table`.
    #[prop(optional, into)]
    summary: Option<String>,
    /// Usually a [`Table`](crate::Table) with the same numbers.
    children: Children,
) -> impl IntoView {
    view! {
        <div class="d-accordion">
            <details>
                <summary>{summary.unwrap_or_else(|| "table".into())}</summary>
                <div class="d-accordion__body">{children()}</div>
            </details>
        </div>
    }
}

/// The key for two series or more. Text stays ink; the swatch carries the series.
#[component]
pub fn Legend(
    #[prop(optional, into)] label: Option<String>,
    /// [`LegendItem`]s, in series order.
    children: Children,
) -> impl IntoView {
    view! { <ul class="d-legend" aria-label=label>{children()}</ul> }
}

/// One series in a [`Legend`].
#[component]
pub fn LegendItem(
    /// Categorical colour 1 to 6: the series' position.
    cat: u8,
    /// A line swatch instead of a box, for a line series.
    #[prop(optional)]
    line: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <li data-cat=cat.clamp(1, 6).to_string() data-shape=line.then_some("line")>{children()}</li>
    }
}
