# dots-ui

Leptos components for the **dots** design system: monochrome, monospace, dense. Grey carries structure, colour
appears only to signal state, dark is canonical.

Every component is a thin, accessible `view!` over the markup the design system documents. State lives in the
same native and ARIA attributes the stylesheet reads (`aria-selected`, `data-state`, …) and is driven by
signals. It does not need `dots.js`.

- Leptos **0.8** (stable). Features `csr`, `hydrate`, `ssr` are forwarded to `leptos`; enable the one your app uses.
- 70+ components and parts, the full stylesheet, tokens, Geist Mono and 76 Lucide icons are in the crate.
- Browser baseline: evergreen since early 2024 (`<dialog>`, `:has()`, container queries, `color-mix()`).

## use it

```toml
# Cargo.toml
[dependencies]
dots-ui = { version = "0.1", features = ["csr"] }
# or straight from git: dots-ui = { git = "https://github.com/samintejas/dots-ui", features = ["csr"] }
```

Load the styles once, wrap the app in `DotsProvider`, and serve the fonts:

```rust
use dots_ui::prelude::*;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <DotsStyles font_base="/fonts/"/>      // inlines tokens.css + dots.css
        <DotsProvider persist_theme="app:theme"> // theme, toasts, tooltips
            <SkipLink/>
            <Main>
                <Button variant=ButtonVariant::Primary>"deploy"</Button>
            </Main>
        </DotsProvider>
    }
}
```

Fonts: serve `dots_ui::assets::FONTS` yourself, or call `dots_ui::assets::write_to("style")` from a
`build.rs`/xtask to drop `tokens.css`, `dots.css` and `fonts/` into a directory that Trunk or cargo-leptos
bundles (then skip `DotsStyles`). `tokens.css` expects the fonts at `./fonts/` next to it.

The gallery in `gallery/` uses every component: `gallery/build.sh` then
`python3 -m http.server -d gallery/dist 8080` (needs the target `wasm32-unknown-unknown` and a
`wasm-bindgen` CLI matching `Cargo.lock`).

## conventions

- **Reactive props** take a value, a signal or a closure: `disabled=true`, `disabled=sig`, `disabled=move || x.get()`.
- **Two-way controls** take an `RwSignal`: `TextField value=…`, `Checkbox checked=…`, `Tabs value=…`, `Dialog open=…`.
- **Passthrough**: `on:click=…`, `attr:id=…` and `class:name=cond` work on every component and reach its root
  element. Do not use `attr:class`, it *replaces* the component's classes; use `class:name=true`.
- **Children are the label.** Labels follow the voice rules: lowercase verbs (`deploy`, `delete workspace`), errors
  start with `error:`, no `ok`/`submit`.
- **State is never colour-only**: severities pair colour with an icon, glyph or word, as the design system requires.
- **Native first**: `<dialog>`, `<details>`, `<select>`, checkbox, radio and range are the real elements. Focus
  trapping, esc and find-in-page come from the platform.
- Unique ids (labels, panels, menus) come from `leptos`' shared context under SSR/hydration, so they match.

## what is in it

| area | components |
|---|---|
| actions | `Button`, `ButtonLink`, `Menu`, `ContextMenu`, `MenuItem`, `MenuLink`, `MenuCheckbox`, `MenuRadioGroup`, `MenuRadio`, `MenuSeparator`, `MenuLabel`, `MenuHead`, `MenuFoot` |
| forms | `Segmented` + `SegOption`, `SettingRows` + `SettingRow`, `SettingGrid`, `FilterBar` (`FilterSearch`, `Filter`, `FilterActive`), `InputBar` (`InputSeg`, `InputBarSubmit`), `TagInput`, `TextField`, `Select`, `Checkbox`, `RadioGroup` + `Radio`, `Switch`, `Slider`, `Field`, `Label`, `Hint`, `ErrorText` |
| status | `StatusPage` (loading, not found, error, offline), `Pebbles` (loader), `Stepper`, `Feed` + `FeedItem`, `StatusChip`, `StatusButton`, `Badge`, `Tag`, `Progress`, `ProgressRow`, `Meter`, `MeterRow`, `Ring`, `Spinner`, `Toast` + `ToastProvider` + `use_toasts()` |
| content | `Card` (`CardHead`, `CardBody`, `CardFoot`, `CardButton`, `CardLink`), `List`, `ListItem`, `Listbox`, `ListOption`, `Table` (`Th`, `Tr`, `Td`, `EditCell`), `Accordion`, `CodeBlock`, `HeroMetric`, `HeroClock`, `EmptyState`, `EmptyLoading`, `EmptyError`, `Avatar` (`AvatarImage`, `AvatarGroup`, `Identity`), `Kbd`/`Keys`, `Divider`, `Surface` (elevation) |
| navigation | `AppSwitcher` + `AppTile`, `SectionNav` (`SectionNavItem`, `SectionNavLabel`, `SectionNavLayout`), `Tabs` (`TabList` across or `vertical`, `Tab`, `TabPanel`), `Breadcrumbs`, `Pager`, `Chat`, `WorkspaceDots`, `WorkspaceSteps`, `CommandPalette` |
| layout | `Shell`, `ShellMain`, `Main`, `SkipLink`, `ShellToggle`, `AppBar` (+ parts), `Sidebar` (+ switcher, filter, groups, items, parents, actions, account), `RightBar`, `BottomBar`, `SplitPane` + `Pane` |
| overlays | `Dialog`, `Sheet`, `DialogClose`, `use_dialog()`, tooltips via `data-tooltip` |
| setup | `DotsProvider`, `ThemeProvider`, `use_theme()`, `DotsStyles`, `Icon`, `icon_names()` |

## behaviour you should know

- **Menu**: `trigger` must render one focusable element; it is wired in place (no wrapper element). The panel is
  moved to `<body>` (or the open dialog) while shown so nothing clips it. One level, no submenus.
- **Tooltips**: any element with `data-tooltip="…"` gets one (300ms hover, immediate on keyboard focus, esc
  closes). `Button` has a `tooltip` prop.
- **Table**: you own the data and the sort. `Th sort=… on_sort=…`, `Tr selectable selected=… on_select=…`; see the
  gallery for a sortable, selectable table.
- **CodeBlock**: plain text by default. For highlighting set `html` and pass markup using the `tk-*` token
  classes (one token span never crosses a line break; you escape).
- **Shell**: needs a fixed height (`100dvh` by default). Ctrl+B / Ctrl+Alt+B / Ctrl+J toggle the bars, the tiles
  stack under 768px, and `persist="key"` remembers the state. Drive it from outside with `ShellState`.
- **Segmented** is a radio group: one tab stop, arrows select. Two to five options; more is a `Select`.
- **SectionNav** is for sections that are routes (`/settings/profile`); a vertical `TabList` is for panels that
  switch in place. Both sit in a `SectionNavLayout`, which stacks the nav on top under 768px.
- **AppSwitcher** is a `Menu` with a grid panel (`panel_class="d-apps"`); any `Menu` takes `panel_class`.
- **Glass**: menus, toasts, tooltips, dialogs and the palette blur what is behind them and keep 90% (94% for
  dialogs) of their fill, so text still reads 4.5:1 over anything. Browsers without `backdrop-filter`, and people who
  ask for reduced transparency, get the solid fill.
- **Corners**: 2px small held things, 3px controls, 4px cards, 6px floating layers; layout tiles stay square.
- **Dialog**: `dismiss=false` ignores the scrim and focuses the first footer button (cancel).
- **Theme**: `use_theme()` is an `RwSignal<Theme>` mirrored to `data-theme` on `<html>`.

## refreshing from the design system

`assets/dots.css` is the design system's `components/bundle.css`, `assets/tokens.css` is `assets/Tokens/tokens.css`,
fonts are `fonts/`. To pick up a new release, copy them over and regenerate the icon table:

```sh
scripts/gen_icons.py path/to/components/bundle.js
```

Class names and attributes are the contract: when a component's README changes, change the matching file in
`src/components/`.

## licences

Geist Mono is SIL OFL 1.1 (`assets/fonts/LICENSE.txt`); Lucide icons are ISC/MIT (`assets/LICENSE-lucide.txt`).
Add a licence for this crate before publishing it.

## licence

MIT (see `LICENSE`). Geist Mono is bundled under the SIL Open Font License 1.1 (`assets/fonts/LICENSE.txt`) and the
icons come from Lucide, ISC (`assets/LICENSE-lucide.txt`).
