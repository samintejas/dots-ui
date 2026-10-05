//! The stylesheet, tokens and fonts, embedded in the crate.
//!
//! Easiest: render [`DotsStyles`] once. To serve the files yourself, use the constants here.

use leptos::prelude::*;

/// `tokens.css`: primitives and semantic aliases. Dark in `:root`, light under `[data-theme="light"]`.
/// Its `@font-face` rules point at `./fonts/`.
pub const TOKENS_CSS: &str = include_str!("../assets/tokens.css");

/// `dots.css`: the component styles (class prefix `d-`). Needs [`TOKENS_CSS`].
pub const DOTS_CSS: &str = include_str!("../assets/dots.css");

/// Geist Mono, SIL OFL. Name and bytes for each weight, to serve under one directory (default `/fonts/`).
pub const FONTS: &[(&str, &[u8])] = &[
    ("GeistMono-Regular.woff2", include_bytes!("../assets/fonts/GeistMono-Regular.woff2")),
    ("GeistMono-Medium.woff2", include_bytes!("../assets/fonts/GeistMono-Medium.woff2")),
    ("GeistMono-SemiBold.woff2", include_bytes!("../assets/fonts/GeistMono-SemiBold.woff2")),
];

/// `tokens.css` with its font urls pointed at `font_base` (for example `"/fonts/"`).
pub fn tokens_css(font_base: &str) -> String {
    let base = if font_base.ends_with('/') { font_base.to_string() } else { format!("{font_base}/") };
    TOKENS_CSS.replace("./fonts/", &base)
}

/// Both stylesheets, ready to put in a `<style>` element.
pub fn css(font_base: &str) -> String {
    format!("{}\n{}", tokens_css(font_base), DOTS_CSS)
}

/// Write `tokens.css`, `dots.css` and `fonts/` into `dir`. For a `build.rs` or an xtask, so the files can be
/// bundled by Trunk or cargo-leptos instead of inlined.
#[cfg(not(target_arch = "wasm32"))]
pub fn write_to(dir: impl AsRef<std::path::Path>) -> std::io::Result<()> {
    let dir = dir.as_ref();
    std::fs::create_dir_all(dir.join("fonts"))?;
    std::fs::write(dir.join("tokens.css"), TOKENS_CSS)?;
    std::fs::write(dir.join("dots.css"), DOTS_CSS)?;
    for (name, bytes) in FONTS {
        std::fs::write(dir.join("fonts").join(name), bytes)?;
    }
    Ok(())
}

/// Inline the tokens and component styles. Serve the fonts at `font_base` (see [`FONTS`] or [`write_to`]).
///
/// Prefer linking real files in production for caching; this is the zero-setup option.
#[component]
pub fn DotsStyles(
    /// Where the Geist Mono files are served from. Default `/fonts/`.
    #[prop(optional, into)]
    font_base: Option<String>,
) -> impl IntoView {
    let css = css(font_base.as_deref().unwrap_or("/fonts/"));
    view! { <style data-dots="">{css}</style> }
}
