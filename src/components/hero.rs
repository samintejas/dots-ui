use std::time::Duration;

use leptos::prelude::*;

/// One big number: a metric at 6rem. One per view; for a headline, lock screen or timer, not decoration.
#[component]
pub fn HeroMetric(
    #[prop(into)] value: Signal<String>,
    #[prop(optional, into)] unit: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    /// Extra content after the label, like a delta [`Badge`](crate::Badge) (`↑ 0.02`).
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="d-hero">
            <span class="d-hero__value">{move || value.get()}{unit.map(|u| view! { <span class="d-hero__unit">{u}</span> })}</span>
            <span class="d-hero__label">{label}{children.map(|c| view! { " " {c()} })}</span>
        </div>
    }
}

/// A live `hh:mm` clock at 6rem. `blink` flashes the colon at 1Hz as a liveness signal (static under reduced motion).
#[component]
pub fn HeroClock(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional)] blink: bool,
) -> impl IntoView {
    let time = RwSignal::new(("--".to_string(), "--".to_string()));
    let tick = move || {
        let d = js_sys::Date::new_0();
        let next = (format!("{:02}", d.get_hours()), format!("{:02}", d.get_minutes()));
        if time.get_untracked() != next {
            time.set(next);
        }
    };
    Effect::new(move |_| tick());
    Effect::new(move |_| {
        if let Ok(h) = set_interval_with_handle(tick, Duration::from_secs(1)) {
            on_cleanup(move || h.clear());
        }
    });
    view! {
        <div class="d-hero">
            <time class="d-hero__value" datetime=move || { let (h, m) = time.get(); format!("{h}:{m}") }>
                <span>{move || time.get().0}</span>
                <span class=blink.then_some("d-hero__colon")>":"</span>
                <span>{move || time.get().1}</span>
            </time>
            <span class="d-hero__label">{label}</span>
        </div>
    }
}
