use leptos::prelude::*;

use crate::icon::Icon;

/// Who sent a [`ChatMessage`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum From {
    Me,
    Them,
}

/// One message in a [`Chat`]. A `pending` message shows a shimmering `thinking` (or its steps) until its text is set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatMessage {
    pub id: u64,
    pub from: From,
    pub text: String,
    pub pending: bool,
    /// The work behind an answer, shown folded above it (`worked through 3 steps`). Empty for none.
    pub steps: Vec<String>,
    /// How many of `steps` are finished while the answer is pending; the next one shows as active.
    pub steps_done: usize,
}

impl ChatMessage {
    pub fn me(id: u64, text: impl Into<String>) -> Self {
        ChatMessage { id, from: From::Me, text: text.into(), pending: false, steps: vec![], steps_done: 0 }
    }
    pub fn them(id: u64, text: impl Into<String>) -> Self {
        ChatMessage { id, from: From::Them, text: text.into(), pending: false, steps: vec![], steps_done: 0 }
    }
    /// A reply still being worked on.
    pub fn pending(id: u64, steps: Vec<String>) -> Self {
        ChatMessage { id, from: From::Them, text: String::new(), pending: true, steps, steps_done: 0 }
    }
}

/// The assistant screen. Empty, it is a title and one big composer with suggestions under it; once a question
/// is asked the composer docks at the bottom and the conversation reads like a document: your turns are small
/// bubbles on the right, answers run across the column with their work folded above and copy / retry under.
///
/// You own `messages`. `on_send` fires with the question (the person's message is not added for you; push it
/// and then a `pending` reply, and replace the reply's text when the answer arrives). Give the chat a height.
/// Enter sends; shift+enter adds a line.
#[component]
pub fn Chat(
    /// The assistant's name, shown on its messages (`data-name`).
    #[prop(into)]
    name: String,
    #[prop(into)] label: String,
    messages: RwSignal<Vec<ChatMessage>>,
    #[prop(into)] on_send: Callback<String>,
    /// Empty-state title, like `ask tracer`.
    #[prop(into)]
    title: String,
    /// Empty-state line: say where answers come from.
    #[prop(into)]
    intro: String,
    #[prop(optional, into)] suggestions: Vec<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    /// Disable the composer while a reply is pending.
    #[prop(optional, into)]
    busy: Signal<bool>,
    /// The showcase entry point: only until the first message.
    #[prop(optional)]
    hero: bool,
    /// Extra controls for the composer's tool row, left of the send button (an attach button, a scope picker).
    #[prop(optional)]
    tools: Option<ChildrenFn>,
    /// Retry fires with the question an answer was for. Without it there is no retry button.
    #[prop(optional, into)]
    on_retry: Option<Callback<String>>,
) -> impl IntoView {
    let text = RwSignal::new(String::new());
    let log = NodeRef::<leptos::html::Div>::new();
    let input = NodeRef::<leptos::html::Textarea>::new();
    let id = crate::util::uid("ask");
    let started = move || !messages.with(|m| m.is_empty());

    let send = move |q: String| {
        let q = q.trim().to_string();
        if q.is_empty() || busy.get_untracked() {
            return;
        }
        text.set(String::new());
        if let Some(i) = input.get_untracked() {
            let _ = i.style(("height", "auto"));
        }
        on_send.run(q);
    };
    // keep the newest message in view
    Effect::new(move |_| {
        messages.track();
        if let Some(l) = log.get() {
            l.set_scroll_top(l.scroll_height());
        }
    });
    let send_chips = send.clone();
    let name_attr = name.clone();
    view! {
        <section
            class=if hero { "d-chat d-chat--hero" } else { "d-chat" }
            data-name=name_attr aria-label=label
            data-started=move || started().then_some("")
        >
            <div class="d-chat__log" role="log" aria-live="polite" tabindex="0" aria-label="conversation" node_ref=log>
                {move || (!started()).then(|| view! {
                    <div class="d-chat__empty">
                        {hero.then(|| view! { <span class="d-chat__mark" aria-hidden="true"></span> })}
                        <h2 class="d-chat__title">{title.clone()}</h2>
                        <p>{intro.clone()}</p>
                    </div>
                })}
                {let name = name.clone(); move || {
                    let all = messages.get();
                    all.iter().enumerate().map(|(i, m)| {
                        let m = m.clone();
                        let from = match m.from { From::Me => "me", From::Them => "them" };
                        let who = match m.from { From::Me => "you".to_string(), From::Them => name.clone() };
                        // the question this answer was for, for retry
                        let asked = all[..i].iter().rev().find(|p| p.from == From::Me).map(|p| p.text.clone());
                        let steps = (!m.steps.is_empty()).then(|| {
                            let n = m.steps.len();
                            let summary = if m.pending { None } else { Some(format!("worked through {n} steps")) };
                            view! {
                                <details class="d-chat__steps" open=m.pending>
                                    <summary>{match summary {
                                        Some(t) => t.into_any(),
                                        None => view! { <span class="d-chat__thinking" role="status">"working"</span> }.into_any(),
                                    }}</summary>
                                    <ol>{m.steps.iter().enumerate().map(|(j, t)| {
                                        let state = if !m.pending || j < m.steps_done { "done" } else if j == m.steps_done { "active" } else { "todo" };
                                        view! { <li data-state=state>{t.clone()}</li> }
                                    }).collect_view()}</ol>
                                </details>
                            }
                        });
                        let text = m.text.clone();
                        let actions = (m.from == From::Them && !m.pending).then(|| {
                            let copied = RwSignal::new(false);
                            let copy_text = text.clone();
                            view! {
                                <div class="d-chat__actions">
                                    <button type="button" class="d-btn d-btn--ghost d-btn--icon d-btn--sm"
                                        aria-label=move || if copied.get() { "copied" } else { "copy" }
                                        data-tooltip=move || if copied.get() { "copied" } else { "copy" }
                                        on:click=move |_| {
                                            let t = copy_text.clone();
                                            leptos::task::spawn_local(async move {
                                                if crate::components::code_block::write_clipboard(&t).await { copied.set(true) }
                                            });
                                        }
                                    ><Icon name="copy"/></button>
                                    {on_retry.zip(asked.clone()).map(|(cb, q)| view! {
                                        <button type="button" class="d-btn d-btn--ghost d-btn--icon d-btn--sm" aria-label="retry" data-tooltip="retry"
                                            on:click=move |_| cb.run(q.clone())><Icon name="repeat"/></button>
                                    })}
                                </div>
                            }
                        });
                        view! {
                            <div class="d-chat__msg" data-from=from aria-busy=m.pending.then_some("true")>
                                <span class="d-chat__who">{who}</span>
                                {steps}
                                <div class="d-chat__text">
                                    {if m.pending && m.steps.is_empty() {
                                        view! { <span class="d-chat__thinking" role="status">"thinking"</span> }.into_any()
                                    } else {
                                        text.into_any()
                                    }}
                                </div>
                                {actions}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>
            {(!suggestions.is_empty()).then(|| view! {
                <div class="d-chat__suggest">
                    {suggestions.into_iter().map(|s| {
                        let (a, send) = (s.clone(), send_chips.clone());
                        view! {
                            <button type="button" class="d-btn d-btn--secondary d-btn--sm" disabled=move || busy.get() on:click=move |_| send(a.clone())>{s}</button>
                        }
                    }).collect_view()}
                </div>
            })}
            <form class="d-chat__composer" on:submit={let send = send.clone(); move |e| { e.prevent_default(); send(text.get_untracked()); }}>
                <label class="d-sr" for=id.clone()>"your question"</label>
                <textarea
                    class="d-chat__input" id=id rows="2" node_ref=input
                    placeholder=placeholder
                    prop:value=move || text.get()
                    on:input=move |e| {
                        text.set(event_target_value(&e));
                        if let Some(i) = input.get_untracked() {
                            let _ = i.style(("height", "auto"));
                            let h = i.scroll_height().min(200);
                            let _ = i.style(("height", format!("{h}px")));
                        }
                    }
                    on:keydown=move |e| {
                        if e.key() == "Enter" && !e.shift_key() && !e.is_composing() {
                            e.prevent_default();
                            send(text.get_untracked());
                        }
                    }
                ></textarea>
                <div class="d-chat__bar">
                    {tools.as_ref().map(|t| t())}
                    <button type="submit" class="d-btn d-btn--primary d-chat__send" aria-label="send"
                        disabled=move || busy.get() || text.with(|t| t.trim().is_empty())
                    ><Icon name="arrow-up"/></button>
                </div>
            </form>
        </section>
    }
}
