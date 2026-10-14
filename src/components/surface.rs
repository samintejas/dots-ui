use leptos::prelude::*;

/// Elevation levels. Forward is lighter: each step up is a lighter layer, and from raised up it casts a shadow.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Elevation {
    /// Sunk into the page: tracks and insets.
    Inset,
    /// The ground: the page and every layout tile (`bg-base`).
    Base,
    /// Rests on the page: cards, panels.
    #[default]
    Surface,
    /// Floats over it and the page stays usable: menus, popovers.
    Raised,
    /// Blocks the page: dialogs, sheets.
    Overlay,
}

/// A `div` carrying a whole elevation level (fill, border and shadow together).
#[component]
pub fn Surface(#[prop(optional)] level: Elevation, children: Children) -> impl IntoView {
    let class = match level {
        Elevation::Inset => "d-elev-inset",
        Elevation::Base => "d-elev-0",
        Elevation::Surface => "d-elev-1",
        Elevation::Raised => "d-elev-2",
        Elevation::Overlay => "d-elev-3",
    };
    view! { <div class=class>{children()}</div> }
}
