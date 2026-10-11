use std::time::Duration;

use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::icon::Icon;
use crate::util::dom;

pub(crate) async fn write_clipboard(text: &str) -> bool {
    let nav: JsValue = window().navigator().into();
    if let Ok(clip) = js_sys::Reflect::get(&nav, &"clipboard".into()) {
        if !clip.is_undefined() && !clip.is_null() {
            if let Ok(f) = js_sys::Reflect::get(&clip, &"writeText".into()) {
                if let Ok(f) = f.dyn_into::<js_sys::Function>() {
                    if let Ok(p) = f.call1(&clip, &text.into()) {
                        if let Ok(p) = p.dyn_into::<js_sys::Promise>() {
                            if JsFuture::from(p).await.is_ok() {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    legacy_copy(text)
}

/// `execCommand("copy")` through a throwaway textarea, for insecure contexts.
fn legacy_copy(text: &str) -> bool {
    let doc = document();
    let Ok(ta) = doc.create_element("textarea") else { return false };
    let Some(body) = doc.body() else { return false };
    let _ = ta.set_attribute("readonly", "");
    let _ = ta.set_attribute("style", "position:fixed;opacity:0;top:0;left:0");
    ta.set_text_content(Some(text));
    let _ = body.append_child(&ta);
    if let Some(select) = js_sys::Reflect::get(&ta, &"select".into()).ok().and_then(|f| f.dyn_into::<js_sys::Function>().ok()) {
        let _ = select.call0(&ta);
    }
    let doc_js: JsValue = doc.into();
    let ok = js_sys::Reflect::get(&doc_js, &"execCommand".into())
        .ok()
        .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
        .and_then(|f| f.call1(&doc_js, &"copy".into()).ok())
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    ta.remove();
    ok
}

/// Code and terminal output on the terminal palette, with a copy button, optional line numbers and an
/// optional blinking cursor. Dark in both themes.
///
/// `code` is plain text by default. Bring your own highlighter: set `html` and pass markup whose tokens use
/// the `tk-*` classes (`tk-kw`, `tk-str`, `tk-num`, `tk-fn`, `tk-type`, `tk-mac`, `tk-com`, `tk-ok`,
/// `tk-warn`, `tk-err`, `tk-info`). A token span must not cross a line break. With `html` you are responsible
/// for escaping.
#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    /// File name or caption, shown in the header and used as the region's name.
    #[prop(optional, into)]
    title: Option<String>,
    #[prop(optional)] numbered: bool,
    #[prop(optional)] html: bool,
    /// A blinking cursor after the last line (terminal output).
    #[prop(optional)]
    cursor: bool,
) -> impl IntoView {
    let code_ref = NodeRef::<leptos::html::Code>::new();
    let state = RwSignal::new(None::<bool>);

    let lines: Vec<String> = code.trim_end_matches('\n').split('\n').map(str::to_string).collect();
    let last = lines.len().saturating_sub(1);
    let rows = lines
        .into_iter()
        .enumerate()
        .map(|(i, l)| {
            let cur = (cursor && i == last).then(|| view! { <span class="d-code__cursor" aria-hidden="true"></span> });
            if html {
                view! { <span class="d-code__line"><span inner_html=l></span>{cur}</span> }.into_any()
            } else {
                view! { <span class="d-code__line">{l}{cur}</span> }.into_any()
            }
        })
        .collect_view();

    let copy = move |_| {
        let Some(code_el) = code_ref.get() else { return };
        let text = dom::query_all(&code_el, ".d-code__line")
            .iter()
            .map(|l| l.text_content().unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\n");
        spawn_local(async move {
            let ok = write_clipboard(&text).await;
            state.set(Some(ok));
            set_timeout(move || state.set(None), Duration::from_millis(1600));
        });
    };
    let icon = Signal::derive(move || match state.get() {
        Some(true) => "check",
        Some(false) => "x",
        None => "copy",
    });
    let label = move || match state.get() {
        Some(true) => "copied",
        Some(false) => "copy failed",
        None => "copy",
    };
    let live = move || match state.get() {
        Some(true) => "copied to clipboard",
        Some(false) => "copy failed. select the code and press ctrl+c",
        None => "",
    };
    let region = title.clone().unwrap_or_else(|| "code".into());

    view! {
        <figure class=if numbered { "d-code d-code--numbered" } else { "d-code" }>
            <figcaption class="d-code__head">
                <span>{title}</span>
                <button type="button" class="d-code__copy" data-copied=move || state.get() == Some(true) on:click=copy>
                    <Icon name=icon small=true/>
                    <span data-label>{label}</span>
                </button>
            </figcaption>
            <pre tabindex="0" role="region" aria-label=region><code node_ref=code_ref>{rows}</code></pre>
            <span class="d-sr" role="status">{live}</span>
        </figure>
    }
}
