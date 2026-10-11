use leptos::prelude::*;

/// What a workspace dot says when it is not the active one. The app owns these.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DotState {
    /// 8×8 circle.
    #[default]
    Occupied,
    /// 6×6 square.
    Empty,
    /// 7×7 diamond, so it is not told by red alone.
    Urgent,
}

impl DotState {
    fn attr(self) -> &'static str {
        match self {
            DotState::Occupied => "occupied",
            DotState::Empty => "empty",
            DotState::Urgent => "urgent",
        }
    }

    fn suffix(self) -> &'static str {
        match self {
            DotState::Occupied => "",
            DotState::Empty => ", empty",
            DotState::Urgent => ", urgent",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DotItem {
    /// Names the dot for assistive tech (`workspace 2`). State is appended for empty and urgent dots.
    pub label: String,
    pub state: DotState,
}

impl DotItem {
    pub fn new(label: impl Into<String>, state: DotState) -> Self {
        DotItem { label: label.into(), state }
    }
}

/// The workspace indicator, also usable as pagination. Shape carries the state: pill, circle, square, diamond.
///
/// Every dot sits in a fixed slot, so the row never shifts; the active pill slides from slot to slot.
///
/// Dot buttons are under the 24px target size by design, so where they are the only way to navigate, offer
/// the same action elsewhere or use [`WorkspaceSteps`].
#[component]
pub fn WorkspaceDots(
    /// `aria-label` of the `<nav>`.
    #[prop(into)]
    label: String,
    #[prop(into)] items: Signal<Vec<DotItem>>,
    /// Index of the active dot. Pressing a dot sets it.
    current: RwSignal<usize>,
    /// Called with the new index after a press.
    #[prop(optional, into)]
    on_change: Option<Callback<usize>>,
) -> impl IntoView {
    view! {
        <nav aria-label=label>
            // fixed slots and one sliding highlight: data-glide plus --d-at, the active index
            <ol class="d-dots" data-glide="" style=move || format!("--d-at: {}", current.get())>
                {move || {
                    items.get().into_iter().enumerate().map(|(i, item)| {
                        let active = move || current.get() == i;
                        let name = format!("{}{}", item.label, if active() { "" } else { item.state.suffix() });
                        view! {
                            <li>
                                <button
                                    type="button" class="d-dot" aria-label=name
                                    data-state=move || if active() { "active" } else { item.state.attr() }
                                    aria-current=move || active().then_some("true")
                                    on:click=move |_| {
                                        current.set(i);
                                        if let Some(cb) = on_change { cb.run(i) }
                                    }
                                ></button>
                            </li>
                        }
                    }).collect_view()
                }}
            </ol>
        </nav>
    }
}

/// The labelled variant: one visible step per dot. Use it where the dots are the only navigation.
#[component]
pub fn WorkspaceSteps(
    #[prop(into)] label: String,
    #[prop(into)] steps: Signal<Vec<String>>,
    current: RwSignal<usize>,
    #[prop(optional, into)] on_change: Option<Callback<usize>>,
) -> impl IntoView {
    view! {
        <nav aria-label=label>
            <ol class="d-steps">
                {move || {
                    steps.get().into_iter().enumerate().map(|(i, text)| {
                        let active = move || current.get() == i;
                        view! {
                            <li>
                                <button
                                    type="button" class="d-step"
                                    aria-current=move || active().then_some("step")
                                    on:click=move |_| {
                                        current.set(i);
                                        if let Some(cb) = on_change { cb.run(i) }
                                    }
                                >
                                    <span class="d-dot" data-state=move || if active() { "active" } else { "occupied" }></span>
                                    {format!("{} {}", i + 1, text)}
                                </button>
                            </li>
                        }
                    }).collect_view()
                }}
            </ol>
        </nav>
    }
}

/// A non-interactive dot, for pagination that is not a control.
#[component]
pub fn Dot(#[prop(into)] state: Signal<DotState>, #[prop(optional, into)] active: Signal<bool>) -> impl IntoView {
    view! {
        <span class="d-dot" data-state=move || if active.get() { "active" } else { state.get().attr() }></span>
    }
}
