# dots-ui

Leptos 0.8 components for **dots**, pebblelab's design system: dark, monospace, dense. Grey for structure, the pink
accent only for state. Every component is an accessible `view!` over the system's markup; the stylesheet, tokens,
Geist Mono and 76 Lucide icons ship in the crate. No JavaScript needed.

```toml
dots-ui = { version = "0.1", features = ["csr"] }   # or "hydrate" / "ssr"
```

```rust
use dots_ui::prelude::*;

view! {
    <DotsStyles font_base="/fonts/"/>               // tokens.css + dots.css; serve dots_ui::assets::FONTS at /fonts/
    <DotsProvider persist_theme="app:theme">        // theme, toasts, tooltips
        <Shell>
            <Sidebar>
                <SidebarHead><Switchboard initials="rf" title="rao family" app="fin">…</Switchboard></SidebarHead>
                <SidebarContent>…</SidebarContent>
            </Sidebar>
            <ShellMain><AppBar>…</AppBar><Main>…</Main></ShellMain>
        </Shell>
    </DotsProvider>
}
```

## conventions

- Props take a value, a signal or a closure. Two-way controls take an `RwSignal` (`TextField value=…`).
- Children are the label: lowercase verbs, errors start with `error:`.
- `on:…` and `attr:…` reach the root element. Don't use `attr:class` (it replaces); use `class:name=true`.
- State is never colour-only, and native elements stay native (`<dialog>`, `<details>`, checkbox, range).

## layers

Every background is one of five layers, darker further back: `bg-well` (#000) where you type, `bg-base` the page and
every layout tile, `bg-surface` a component that holds content, `bg-raised` menus and toasts, `bg-overlay` dialogs and
the palette. The components already carry theirs. In your own markup: a section inside a component shares its fill
and gets a hairline, never a fill of its own; hover and pressed are `bg-hover` and `bg-press`, tints that work on any
layer; tracks and small chips use `bg-fill`. `Surface level=Elevation::…` wraps a block in one layer.

## what is in it

| area | components |
|---|---|
| layout | `Shell`, `Sidebar` (+ `Switchboard`, `SwitchboardApps`, groups, items), `AppBar`, `RightBar`, `BottomBar`, `SplitPane`, `SectionNav` |
| actions | `Button`, `Menu` (+ items, radio, checkbox), `ContextMenu`, `CommandPalette` |
| forms | `TextField`, `Select`, `Checkbox`, `Radio`, `Switch`, `Slider`, `Segmented`, `TagInput`, `InputBar`, `FilterBar`, `SettingRow` |
| content | `Card`, `Table`, `List`, `Avatar`, `Badge`, `CopyValue`, `CodeBlock`, `Accordion`, `HeroMetric`, `EmptyState` |
| status | `StatusPage` (with the street), `Pebbles`, `Toast`, `StatusChip`, `Progress`, `Meter`, `Stepper`, `Feed` |
| other | `Chat`, `AppSwitcher`, `Tabs`, `Breadcrumbs`, `Pager`, `WorkspaceDots`, `Dialog`, `Sheet`, `Icon` |

Run the gallery: `gallery/build.sh`, then `python3 -m http.server -d gallery/dist 8080`.

## keeping in step with the design system

`assets/dots.css` is the system's `components/bundle.css`, `assets/tokens.css` its `assets/Tokens/tokens.css`.
Class names and attributes are the contract: when a component's README changes there, its file in `src/components/`
changes here.

## licence

MIT. Geist Mono is SIL OFL 1.1 (`assets/fonts/LICENSE.txt`); Lucide icons are ISC (`assets/LICENSE-lucide.txt`).
