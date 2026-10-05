use leptos::prelude::*;

/// A path back up the hierarchy. The last item is the current page: not a link, `aria-current="page"`.
#[component]
pub fn Breadcrumbs(children: Children) -> impl IntoView {
    view! {
        <nav class="d-crumbs" aria-label="breadcrumb"><ol>{children()}</ol></nav>
    }
}

/// A link in the trail.
#[component]
pub fn Crumb(#[prop(into)] href: String, children: Children) -> impl IntoView {
    view! { <li><a href=href>{children()}</a></li> }
}

/// The current page, last in the trail.
#[component]
pub fn CrumbCurrent(children: Children) -> impl IntoView {
    view! { <li><span aria-current="page">{children()}</span></li> }
}
