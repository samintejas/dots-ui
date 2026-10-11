use leptos::prelude::*;

/// Which still-water variant a [`StatusPage`] draws.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatusKind {
    /// Faster rings: it is on its way.
    Loading,
    /// Slow rings and a point of accent, with the big `404`.
    #[default]
    NotFound,
    /// The rings stop where they are.
    Error,
    /// Flat water.
    Offline,
}

impl StatusKind {
    pub fn as_str(self) -> &'static str {
        match self {
            StatusKind::Loading => "loading",
            StatusKind::NotFound => "not-found",
            StatusKind::Error => "error",
            StatusKind::Offline => "offline",
        }
    }
}

/// Three pebbles dropping into water: a loading mark. `large` for a [`StatusPage`].
#[component]
pub fn Pebbles(#[prop(into)] label: String, #[prop(optional)] large: bool) -> impl IntoView {
    view! {
        <span class=if large { "d-pebbles d-pebbles--lg" } else { "d-pebbles" } role="status" aria-label=label>
            <i></i><i></i><i></i>
        </span>
    }
}

/// A whole-page state: loading, not found, error, offline. Quiet rings behind a title and one line.
///
/// `children` is the line under the title (a link home for not-found) and `actions` the buttons below it.
#[component]
pub fn StatusPage(
    kind: StatusKind,
    #[prop(into)] title: String,
    /// Shown small at the bottom, like the app's name.
    #[prop(optional, into)]
    foot: Option<String>,
    #[prop(optional)] children: Option<Children>,
    #[prop(optional)] actions: Option<Children>,
) -> impl IntoView {
    view! {
        <main class="d-status-page" data-kind=kind.as_str()>
            <div class="d-street" aria-hidden="true"><div class="d-street__world"><i></i><i></i><i></i></div></div>
            <div class="d-status-page__inner">
                {match kind {
                    StatusKind::NotFound => view! { <div class="d-status-page__code" aria-hidden="true">"4"<b></b>"4"</div> }.into_any(),
                    StatusKind::Loading => view! { <Pebbles label=title.clone() large=true/> }.into_any(),
                    _ => ().into_any(),
                }}
                <h1 class="d-status-page__title">{title}</h1>
                {children.map(|c| view! { <p class="d-status-page__text">{c()}</p> })}
                {actions.map(|a| view! { <div class="d-status-page__actions">{a()}</div> })}
            </div>
            {foot.map(|f| view! { <div class="d-status-page__foot">{f}</div> })}
        </main>
    }
}
