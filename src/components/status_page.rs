//! Whole-screen states: loading, not found, error, offline. And the pebbles loader they share.

use leptos::prelude::*;

/// Three pebbles rising in turn, the largest dropping a ripple: "working on it". It carries an accessible
/// `label` (`loading transactions`); with reduced motion the pebbles sit still and the label still says it.
#[component]
pub fn Pebbles(
    #[prop(default = "loading".into(), into)] label: String,
    /// The 52px version, for a status page.
    #[prop(optional)]
    large: bool,
) -> impl IntoView {
    let class = if large { "d-pebbles d-pebbles--lg" } else { "d-pebbles" };
    view! { <span class=class role="status" aria-label=label><i></i><i></i><i></i></span> }
}

/// Which whole-screen state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatusKind {
    /// Starting up, or waiting on the first answer. Pebbles and one quiet line.
    #[default]
    Loading,
    /// No page at this address. A large 404 whose zero is a pebble.
    NotFound,
    /// Something broke: say what, and what to do.
    Error,
    /// The server cannot be reached.
    Offline,
}

impl StatusKind {
    fn as_str(self) -> &'static str {
        match self {
            StatusKind::Loading => "loading",
            StatusKind::NotFound => "not-found",
            StatusKind::Error => "error",
            StatusKind::Offline => "offline",
        }
    }
}

/// A whole screen for when there is no screen to show. `title` says what happened in the system's voice
/// (`nothing at this address`, `error: fin could not start`); `children` add a line of detail; `actions` are
/// buttons (a verb each: `go to transactions`, `retry now`); `foot` is a small line at the bottom (a request id,
/// the app's name).
///
/// Behind it is the [`Street`]: a wireframe street with a few people on it. Loading, they walk away down it;
/// not found, they search it with torches; error, they stop; offline, nobody is out.
#[component]
pub fn StatusPage(
    #[prop(optional)] kind: StatusKind,
    #[prop(into)] title: String,
    #[prop(optional, into)] actions: Option<ViewFn>,
    #[prop(optional, into)] foot: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let mark = match kind {
        StatusKind::Loading => view! { <Pebbles large=true label=title.clone()/> }.into_any(),
        StatusKind::NotFound => view! { <div class="d-status-page__code" aria-hidden="true">"4"<b></b>"4"</div> }.into_any(),
        StatusKind::Error | StatusKind::Offline => view! { <Pebbles large=true label=title.clone()/> }.into_any(),
    };
    let still = matches!(kind, StatusKind::Error | StatusKind::Offline);
    view! {
        <main id="main" tabindex="-1" class="d-status-page" data-kind=kind.as_str()>
            <Street/>
            <div class="d-status-page__inner">
                {if still { ().into_any() } else { mark }}
                <h1 class="d-status-page__title">{title}</h1>
                {children.map(|c| view! { <p class="d-status-page__text">{c()}</p> })}
                {actions.map(|a| view! { <div class="d-status-page__actions">{a.run()}</div> })}
            </div>
            {foot.map(|f| view! { <div class="d-status-page__foot">{f}</div> })}
        </main>
    }
}

/// The wireframe street behind a [`StatusPage`]: decorative (`aria-hidden`), pure css, four people on the pavements.
/// What they do comes from the page's `data-kind`.
#[component]
pub fn Street() -> impl IntoView {
    // (x across, z toward or away, delay, how far they wander, torch faces left)
    const PEOPLE: [(i32, i32, i32, i32, bool); 4] = [(-280, 200, -2, 70, false), (300, 120, -7, -90, true), (-330, -240, -4, 90, false), (320, -420, -11, -80, true)];
    view! {
        <div class="d-street" aria-hidden="true">
            <div class="d-street__world">
                <i class="d-street__ground"></i>
                <i class="d-street__wall d-street__wall--l"></i>
                <i class="d-street__wall d-street__wall--r"></i>
                {PEOPLE.iter().map(|&(x, z, d, dx, left)| view! {
                    <span class="d-street__walker" style=format!("--x:{x}px;--z:{z}px;--w-delay:{d}s;--dx:{dx}px")>
                        <b></b><i></i><i></i><u style=left.then_some("--dir:-1")></u>
                    </span>
                }).collect_view()}
            </div>
        </div>
    }
}
