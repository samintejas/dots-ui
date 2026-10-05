use leptos::prelude::*;

use crate::types::Size;
use crate::util::{tf, when_true};

/// Avatar diameter: xs 16, sm 20, md 24 (default), lg 32, xl 48, xxl 64px.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Xxl,
}

impl From<Size> for AvatarSize {
    fn from(s: Size) -> Self {
        match s {
            Size::Sm => AvatarSize::Sm,
            Size::Md => AvatarSize::Md,
            Size::Lg => AvatarSize::Lg,
        }
    }
}

/// Presence mark on the edge of a person's avatar. The state also belongs in the accessible name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Presence {
    Online,
    Away,
    Busy,
    Offline,
}

impl Presence {
    pub fn as_str(self) -> &'static str {
        match self {
            Presence::Online => "online",
            Presence::Away => "away",
            Presence::Busy => "busy",
            Presence::Offline => "offline",
        }
    }
}

fn avatar_class(size: AvatarSize, square: bool, color: bool) -> String {
    let mut c = String::from("d-avatar");
    c.push_str(match size {
        AvatarSize::Xs => " d-avatar--xs",
        AvatarSize::Sm => " d-avatar--sm",
        AvatarSize::Md => "",
        AvatarSize::Lg => " d-avatar--lg",
        AvatarSize::Xl => " d-avatar--xl",
        AvatarSize::Xxl => " d-avatar--2xl",
    });
    if square {
        c.push_str(" d-avatar--square");
    }
    if color {
        c.push_str(" d-avatar--color");
    }
    c
}

/// A person (circle) or an organisation, team or project (`square`), as two lowercase initials, or a photo
/// with the initials underneath while it loads.
///
/// Give `name` when the avatar stands alone. When a name sits beside it, set `decorative`. With a `status`
/// the state is added to the accessible name (`samin t, online`).
#[component]
pub fn Avatar(
    /// The initials, lowercase, usually two.
    #[prop(into)]
    initials: String,
    /// Accessible name (`aria-label`).
    #[prop(optional, into)]
    name: Option<String>,
    /// A name is shown beside it, so the initials are decoration.
    #[prop(optional)]
    decorative: bool,
    #[prop(optional, into)] size: AvatarSize,
    /// Organisation, team or project: a square instead of a circle.
    #[prop(optional)]
    square: bool,
    /// Show the photo in colour (default is greyscale).
    #[prop(optional)]
    color: bool,
    /// Photo URL. The initials show while it loads or if it fails.
    #[prop(optional, into)]
    src: Option<String>,
    #[prop(optional, into)] status: MaybeProp<Presence>,
    #[prop(optional, into)] busy: Signal<bool>,
) -> impl IntoView {
    let class = avatar_class(size, square, color);
    let label = name.map(|n| move || match status.get() {
        Some(s) => format!("{n}, {}", s.as_str()),
        None => n.clone(),
    });
    let body = match src {
        Some(src) => view! {
            <img src=src alt="" />
            <span class="d-avatar__fallback" aria-hidden="true">{initials}</span>
        }
        .into_any(),
        None => initials.into_any(),
    };
    let mark = move || {
        status
            .get()
            .map(|s| view! { <span class="d-avatar__status" data-status=s.as_str()></span> })
    };
    if decorative {
        view! { <span class=class aria-hidden="true" aria-busy=move || when_true(busy.get())>{body}{mark}</span> }.into_any()
    } else {
        view! { <span class=class role="img" aria-label=label aria-busy=move || when_true(busy.get())>{body}{mark}</span> }.into_any()
    }
}

/// A photo avatar. Photos render greyscale unless `color` is set.
#[component]
pub fn AvatarImage(
    #[prop(into)] src: String,
    #[prop(into)] name: String,
    /// Initials shown under the photo until it loads.
    #[prop(optional, into)]
    initials: Option<String>,
    #[prop(optional, into)] size: AvatarSize,
    #[prop(optional)] square: bool,
    #[prop(optional)] color: bool,
) -> impl IntoView {
    view! {
        <span class=avatar_class(size, square, color) role="img" aria-label=name>
            <img src=src alt="" />
            {initials.map(|i| view! { <span class="d-avatar__fallback" aria-hidden="true">{i}</span> })}
        </span>
    }
}

/// An avatar as a button, for filtering by person or opening an account menu.
#[component]
pub fn AvatarButton(
    #[prop(into)] initials: String,
    #[prop(into)] label: String,
    #[prop(optional, into)] size: AvatarSize,
    #[prop(optional)] square: bool,
    #[prop(optional, into)] selected: MaybeProp<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class=avatar_class(size, square, false)
            aria-label=label
            aria-pressed=move || selected.get().map(tf)
            aria-disabled=move || when_true(disabled.get())
        >
            {initials}
        </button>
    }
}

/// Overlapping avatars with a ring. Show at most four, then a count (`+2`).
#[component]
pub fn AvatarGroup(#[prop(into)] label: String, children: Children) -> impl IntoView {
    view! { <span class="d-avatars" role="group" aria-label=label>{children()}</span> }
}

/// An avatar next to a name and a second line. The avatar is decoration: pass a `decorative` [`Avatar`].
#[component]
pub fn Identity(
    #[prop(into)] name: String,
    #[prop(optional, into)] detail: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <span class="d-identity">
            {children()}
            <span class="d-identity__text"><b>{name}</b>{detail.map(|d| view! { <small>{d}</small> })}</span>
        </span>
    }
}
