use leptos::prelude::*;

/// Who sent a [`ChatMessage`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum From {
    Me,
    Them,
}

/// One message in a [`Chat`]. A `pending` message shows three pulsing dots until its text is set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatMessage {
    pub id: u64,
    pub from: From,
    pub text: String,
    pub pending: bool,
}

/// A conversation with an assistant: a scrolling log, suggested questions and a composer at the bottom.
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
                {let name = name.clone(); move || messages.get().into_iter().map(|m| {
                    let from = match m.from { From::Me => "me", From::Them => "them" };
                    let who = match m.from { From::Me => "you".to_string(), From::Them => name.clone() };
                    view! {
                        <div class="d-chat__msg" data-from=from>
                            <span class="d-chat__who">{who}</span>
                            <div class="d-chat__text">
                                {if m.pending {
                                    view! { <span class="d-spinner" role="status"><i></i><i></i><i></i><span class="d-sr">"thinking"</span></span> }.into_any()
                                } else {
                                    m.text.into_any()
                                }}
                            </div>
                        </div>
                    }
                }).collect_view()}
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
                <button type="submit" class="d-btn d-btn--primary" disabled=move || busy.get()>"ask"</button>
            </form>
        </section>
    }
}
