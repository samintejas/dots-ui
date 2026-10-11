use leptos::prelude::*;

/// The five background layers, darker is further back. Only raised and overlay cast a shadow.
/// Sections inside a layer share its fill and are split by a hairline; state is a tint
/// (`bg-hover`, `bg-press`) over whatever layer it sits on, never a layer of its own.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Elevation {
    /// The darkest: where you type or a value sits (`bg-well`).
    Well,
    /// The old name for `Well`.
    Inset,
    /// The ground: the page and layout tiles (`bg-base`).
    Base,
    /// One step up: a component that holds content, like a card or a panel (`bg-surface`).
    #[default]
    Surface,
    /// Floats over the page and the page stays usable: menus, popovers (`bg-raised`).
    Raised,
    /// Blocks the page: dialogs, sheets (`bg-overlay`).
    Overlay,
}

/// A `div` carrying a whole layer (fill, border and shadow together).
#[component]
pub fn Surface(#[prop(optional)] level: Elevation, children: Children) -> impl IntoView {
    let class = match level {
        Elevation::Well | Elevation::Inset => "d-elev-inset",
        Elevation::Base => "d-elev-0",
        Elevation::Surface => "d-elev-1",
        Elevation::Raised => "d-elev-2",
        Elevation::Overlay => "d-elev-3",
    };
    view! { <div class=class>{children()}</div> }
}
