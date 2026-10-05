use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaneContext {
    /// Pink: the main context (the laptop screen).
    Primary,
    /// Blue: the secondary context (the external monitor). Use the pairing only for two simultaneous contexts.
    Secondary,
}

#[derive(Clone, Copy)]
struct SplitCtx {
    active: RwSignal<String>,
}

/// Side-by-side panes with a focus indicator. `active` is the id of the active pane.
#[component]
pub fn SplitPane(active: RwSignal<String>, children: Children) -> impl IntoView {
    provide_context(SplitCtx { active });
    view! { <div class="d-split">{children()}</div> }
}

/// A pane. Focus is depth: the active pane sits a step up and the rest dim. It activates on focus or press.
#[component]
pub fn Pane(
    #[prop(into)] id: String,
    /// Names the pane and is set in its header next to the on/off glyph.
    #[prop(into)]
    title: String,
    #[prop(optional)] context: Option<PaneContext>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<SplitCtx>();
    let (a, b, c) = (id.clone(), id.clone(), id);
    let is_active = Signal::derive(move || ctx.active.get() == a);
    let name = title.clone();
    let activate = {
        let set = move || {
            if ctx.active.get_untracked() != c {
                ctx.active.set(c.clone());
            }
        };
        set
    };
    let activate2 = activate.clone();
    view! {
        <section
            class="d-pane"
            data-context=context.map(|c| if c == PaneContext::Primary { "primary" } else { "secondary" })
            data-active=move || is_active.get()
            aria-current=move || is_active.get().then_some("true")
            tabindex="0"
            aria-label=name
            on:focusin=move |_| activate()
            on:pointerdown=move |_| activate2()
        >
            <header class="d-pane__head">
                <span class="d-status" data-state=move || if ctx.active.get() == b { "on" } else { "off" }>{title}</span>
            </header>
            <div class="d-pane__body">{children()}</div>
        </section>
    }
}
