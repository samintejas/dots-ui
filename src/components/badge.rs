use leptos::prelude::*;

use crate::icon::Icon;
use crate::types::Severity;

/// What a badge reports. `Neutral` is the default and most badges are neutral.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tone {
    #[default]
    Neutral,
    Primary,
    Info,
    Success,
    Warning,
    Danger,
}

impl Tone {
    fn attr(self) -> Option<&'static str> {
        match self {
            Tone::Neutral => None,
            Tone::Primary => Some("primary"),
            Tone::Info => Some("info"),
            Tone::Success => Some("success"),
            Tone::Warning => Some("warning"),
            Tone::Danger => Some("danger"),
        }
    }
}

impl From<Severity> for Tone {
    fn from(s: Severity) -> Self {
        match s {
            Severity::Info => Tone::Info,
            Severity::Success => Tone::Success,
            Severity::Warning => Tone::Warning,
            Severity::Danger => Tone::Danger,
        }
    }
}

impl From<Severity> for Signal<Tone> {
    fn from(s: Severity) -> Self {
        Signal::stored(Tone::from(s))
    }
}

/// A stable categorical colour 1 to 6 for a tag name, so the same tag has the same colour everywhere.
/// Same hash as `Dots.cat` in the design system.
pub fn tag_cat(name: &str) -> u8 {
    let mut h: u32 = 0;
    for u in name.to_lowercase().encode_utf16() {
        h = (h * 31 + u as u32) % 997;
    }
    (h % 6) as u8 + 1
}

/// A small label for status, category or count.
///
/// Categories are grey: use a tone only when the badge reports a state, and let the word be the cue.
#[component]
pub fn Badge(
    #[prop(optional, into)] tone: Signal<Tone>,
    /// A count, with `aria-label` carrying the unit (`label="12 unread"`).
    #[prop(optional)]
    count: bool,
    #[prop(optional, into)] label: MaybeProp<String>,
    /// Categorical colour 1 to 6 for a user's tag. Use [`tag_cat`] to derive it from the name. Never with a tone.
    #[prop(optional, into)]
    cat: MaybeProp<u8>,
    /// Show the severity icon for the tone (use for warning and danger).
    #[prop(optional)]
    icon: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let icon_view = move || {
        icon.then(|| {
            let name = match tone.get() {
                Tone::Info | Tone::Primary => "info",
                Tone::Success => "circle-check",
                Tone::Warning => "triangle-alert",
                Tone::Danger => "octagon-alert",
                Tone::Neutral => return None,
            };
            Some(view! { <Icon name=name small=true/> })
        })
        .flatten()
    };
    view! {
        <span
            class=if count { "d-badge d-badge--count" } else { "d-badge" }
            data-severity=move || tone.get().attr()
            data-cat=move || cat.get().map(|c| c.to_string())
            aria-label=move || label.get()
            aria-disabled=move || disabled.get().then_some("true")
        >
            {icon_view}
            {children()}
        </span>
    }
}

/// A removable tag: a neutral [`Badge`] with a remove button. `on_remove` fires when it is pressed.
#[component]
pub fn Tag(
    /// The tag text. Also names the remove button (`remove rust`).
    #[prop(into)]
    text: String,
    #[prop(into)] on_remove: Callback<()>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let aria = format!("remove {text}");
    let cat = tag_cat(&text).to_string();
    view! {
        <span class="d-badge" data-cat=cat aria-disabled=move || disabled.get().then_some("true")>
            {text}
            <button type="button" class="d-badge__x" aria-label=aria on:click=move |_| on_remove.run(())>
                <Icon name="x" small=true/>
            </button>
        </span>
    }
}

