use leptos::prelude::*;

use crate::icon::Icon;

/// Who sent a [`ChatMessage`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum From {
    Me,
    Them,
}

/// Where one step of the work behind an answer stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepState {
    Done,
    Active,
    Todo,
}

/// One line of the work behind an answer, folded above it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatStep {
    pub text: String,
    pub state: StepState,
}

impl ChatStep {
    pub fn new(text: impl Into<String>, state: StepState) -> Self {
        Self { text: text.into(), state }
    }
}

/// One message in a [`Chat`]. A `pending` message shows `thinking` (or its steps) until its text is set.
/// An answer with `steps` folds the work above the text; a finished answer gets copy and retry buttons.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatMessage {
    pub id: u64,
    pub from: From,
    pub text: String,
    pub pending: bool,
    pub steps: Vec<ChatStep>,
}

impl ChatMessage {
    pub fn new(id: u64, from: From, text: impl Into<String>) -> Self {
        Self { id, from, text: text.into(), pending: false, steps: Vec::new() }
    }
    pub fn pending(mut self) -> Self {
        self.pending = true;
        self
    }
    pub fn steps(mut self, steps: Vec<ChatStep>) -> Self {
        self.steps = steps;
        self
    }
}

/// A conversation with an assistant: a scrolling log, suggested questions and a composer at the bottom.
/// The composer is a field over a bar: put extra controls (attach, a scope menu) in `bar`; send is the arrow at its end.
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
    /// Controls on the composer's bar, before the send arrow.
    #[prop(optional)]
    bar: Option<Children>,
    /// Called with the question an answer was given to, when its retry button is pressed.
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
                    let them = m.from == From::Them;
                    let from = if them { "them" } else { "me" };
                    let who = if them { name.clone() } else { "you".to_string() };
                    let ask = all[..i].iter().rev().find(|p| p.from == From::Me).map(|p| p.text.clone()).unwrap_or_default();
                    let text = m.text.clone();
                    let steps = (!m.steps.is_empty()).then(|| {
                        let n = m.steps.len();
                        let summary = if m.pending {
                            view! { <span class="d-chat__thinking" role="status">"working"</span> }.into_any()
                        } else {
                            format!("worked through {n} steps").into_any()
                        };
                        view! {
                            <details class="d-chat__steps" open=m.pending>
                                <summary>{summary}</summary>
                                <ol>{m.steps.iter().map(|s| view! { <li data-state=match s.state { StepState::Done => "done", StepState::Active => "active", StepState::Todo => "todo" }>{s.text.clone()}</li> }).collect_view()}</ol>
                            </details>
                        }
                    });
                    let body = if m.pending {
                        if m.steps.is_empty() {
                            view! { <span class="d-chat__thinking" role="status">"thinking"</span> }.into_any()
                        } else {
                            ().into_any()
                        }
                    } else {
                        text.clone().into_any()
                    };
                    let actions = (them && !m.pending).then(|| {
                        let copy_text = text.clone();
                        let copied = RwSignal::new(false);
                        view! {
                            <div class="d-chat__actions">
                                <button
                                    type="button" class="d-btn d-btn--ghost d-btn--icon d-btn--sm" aria-label="copy"
                                    data-tooltip=move || if copied.get() { "copied" } else { "copy" }
                                    on:click=move |_| {
                                        let t = copy_text.clone();
                                        leptos::task::spawn_local(async move {
                                            if super::code_block::write_clipboard(&t).await {
                                                copied.set(true);
                                                set_timeout(move || copied.set(false), std::time::Duration::from_millis(1200));
                                            }
                                        });
                                    }
                                ><Icon name="copy"/></button>
                                {on_retry.map(|r| view! {
                                    <button type="button" class="d-btn d-btn--ghost d-btn--icon d-btn--sm" aria-label="retry" data-tooltip="retry"
                                        on:click=move |_| r.run(ask.clone())><Icon name="repeat"/></button>
                                })}
                            </div>
                        }
                    });
                    view! {
                        <div class="d-chat__msg" data-from=from aria-busy=m.pending.then_some("true")>
                            <span class="d-chat__who">{who}</span>
                            {steps}
                            <div class="d-chat__text">{body}</div>
                            {actions}
                        </div>
                    }
                }).collect_view()}}
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
                    class="d-chat__input" id=id rows="1" node_ref=input
                    placeholder=placeholder
                    prop:value=move || text.get()
                    on:input=move |e| {
                        text.set(event_target_value(&e));
                        if let Some(i) = input.get_untracked() {
                            let _ = i.style(("height", "auto"));
                            let h = i.scroll_height().min(160);
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
                    {bar.map(|b| b())}
                    <button
                        type="submit" class="d-btn d-btn--primary d-chat__send" aria-label="send"
                        disabled=move || busy.get() || text.with(|t| t.trim().is_empty())
                    ><Icon name="arrow-up"/></button>
                </div>
            </form>
        </section>
    }
}
