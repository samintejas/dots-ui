use leptos::prelude::*;

/// Stacked disclosure rows built on native `<details>`: enter and space toggle, find-in-page opens them.
#[component]
pub fn Accordion(children: Children) -> impl IntoView {
    view! { <div class="d-accordion">{children()}</div> }
}

/// One row. Give items a shared `name` to allow only one open at a time.
#[component]
pub fn AccordionItem(
    #[prop(into)] title: String,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] open: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <details name=name open=open>
            <summary
                aria-disabled=move || disabled.get().then_some("true")
                tabindex=move || disabled.get().then_some("-1")
            >
                {title}
            </summary>
            <div class="d-accordion__body">{children()}</div>
        </details>
    }
}
