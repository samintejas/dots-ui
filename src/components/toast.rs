use std::time::Duration;

use leptos::prelude::*;

use crate::icon::Icon;
use crate::util::dom;

/// How urgent a toast is. `Low` is a ghost bar; `Danger` is an alert that never times out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastSeverity {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
    Low,
}

impl ToastSeverity {
    fn attr(self) -> &'static str {
        match self {
            ToastSeverity::Info => "info",
            ToastSeverity::Success => "success",
            ToastSeverity::Warning => "warning",
            ToastSeverity::Danger => "danger",
            ToastSeverity::Low => "low",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            ToastSeverity::Info => "info",
            ToastSeverity::Success => "circle-check",
            ToastSeverity::Warning => "triangle-alert",
            ToastSeverity::Danger => "octagon-alert",
            ToastSeverity::Low => "bell",
        }
    }
}

impl From<crate::types::Severity> for ToastSeverity {
    fn from(s: crate::types::Severity) -> Self {
        use crate::types::Severity::*;
        match s {
            Info => ToastSeverity::Info,
            Success => ToastSeverity::Success,
            Warning => ToastSeverity::Warning,
            Danger => ToastSeverity::Danger,
        }
    }
}

/// One toast action. A toast with an action never times out.
#[derive(Clone, Debug)]
pub struct ToastAction {
    pub label: String,
    pub on_click: Callback<()>,
}

/// A notification. Build it, then `use_toasts().push(toast)`.
///
/// Titles name the state (`saved`, `disconnected`); bodies are terse and literal.
#[derive(Clone, Debug)]
pub struct Toast {
    pub severity: ToastSeverity,
    pub title: String,
    pub body: Option<String>,
    /// Shown right of the title. Defaults to the current `hh:mm`.
    pub time: Option<String>,
    pub action: Option<ToastAction>,
    /// Milliseconds. `None` is the default (6s; never for danger or with an action); `Some(0)` keeps it
    /// until dismissed.
    pub timeout_ms: Option<u32>,
}

impl Toast {
    pub fn new(title: impl Into<String>) -> Self {
        Toast {
            severity: ToastSeverity::Info,
            title: title.into(),
            body: None,
            time: None,
            action: None,
            timeout_ms: None,
        }
    }

    pub fn success(title: impl Into<String>) -> Self {
        Self::new(title).severity(ToastSeverity::Success)
    }

    pub fn warning(title: impl Into<String>) -> Self {
        Self::new(title).severity(ToastSeverity::Warning)
    }

    pub fn danger(title: impl Into<String>) -> Self {
        Self::new(title).severity(ToastSeverity::Danger)
    }

    pub fn severity(mut self, s: impl Into<ToastSeverity>) -> Self {
        self.severity = s.into();
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn time(mut self, time: impl Into<String>) -> Self {
        self.time = Some(time.into());
        self
    }

    pub fn action(mut self, label: impl Into<String>, on_click: impl Fn() + Send + Sync + 'static) -> Self {
        self.action = Some(ToastAction { label: label.into(), on_click: Callback::new(move |()| on_click()) });
        self
    }

    pub fn timeout_ms(mut self, ms: u32) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
}

/// The toast queue. Get it with [`use_toasts`].
#[derive(Clone, Copy)]
pub struct Toasts {
    list: RwSignal<Vec<(u64, Toast)>>,
    next: StoredValue<u64>,
}

impl Toasts {
    /// Show a toast at the bottom of the stack. Returns an id for [`Toasts::dismiss`].
    pub fn push(&self, toast: Toast) -> u64 {
        let id = self.next.get_value();
        self.next.set_value(id + 1);
        self.list.update(|l| l.push((id, toast)));
        id
    }

    pub fn dismiss(&self, id: u64) {
        self.list.update(|l| l.retain(|(i, _)| *i != id));
    }

    pub fn clear(&self) {
        self.list.set(Vec::new());
    }
}

/// The toast queue from the nearest [`ToastProvider`] (or `DotsProvider`).
pub fn use_toasts() -> Toasts {
    expect_context::<Toasts>()
}

fn clock_text() -> String {
    let d = js_sys::Date::new_0();
    format!("{:02}:{:02}", d.get_hours(), d.get_minutes())
}

/// Renders the notification region (bottom right) and provides [`use_toasts`].
#[component]
pub fn ToastProvider(children: Children) -> impl IntoView {
    let toasts = Toasts { list: RwSignal::new(Vec::new()), next: StoredValue::new(0) };
    provide_context(toasts);
    view! {
        {children()}
        <div class="d-toasts" role="region" aria-label="notifications" aria-live="polite" aria-relevant="additions">
            <For each=move || toasts.list.get() key=|(id, _)| *id children=move |(id, toast)| view! { <ToastView id toast toasts/> }/>
        </div>
    }
}

#[component]
fn ToastView(id: u64, toast: Toast, toasts: Toasts) -> impl IntoView {
    let sev = toast.severity;
    let closing = RwSignal::new(false);
    let timer = StoredValue::new(None::<TimeoutHandle>);
    let has_action = toast.action.is_some();
    let timeout = toast
        .timeout_ms
        .unwrap_or(if sev == ToastSeverity::Danger || has_action { 0 } else { 6000 });

    let clear = move || {
        if let Some(h) = timer.get_value() {
            h.clear();
            timer.set_value(None);
        }
    };
    let dismiss = move || {
        if closing.get_untracked() {
            return;
        }
        clear();
        closing.set(true);
        let ms = if dom::reduced_motion() { 0 } else { 100 };
        set_timeout(move || toasts.dismiss(id), Duration::from_millis(ms));
    };
    let arm = move || {
        clear();
        if timeout > 0 {
            timer.set_value(set_timeout_with_handle(dismiss, Duration::from_millis(u64::from(timeout))).ok());
        }
    };
    Effect::new(move |_| arm());
    on_cleanup(clear);

    let title = toast.title.clone();
    let dismiss_label = format!("dismiss: {title}");
    let time = toast.time.clone().unwrap_or_else(clock_text);
    let action = toast.action.clone();

    view! {
        <div
            class="d-toast"
            role=if sev == ToastSeverity::Danger { "alert" } else { "status" }
            data-severity=sev.attr()
            data-closing=move || closing.get()
            on:mouseenter=move |_| clear()
            on:mouseleave=move |_| arm()
            on:focusin=move |_| clear()
            on:focusout=move |_| arm()
            on:keydown=move |e| { if e.key() == "Escape" { dismiss() } }
        >
            <span class="d-toast__icon"><Icon name=sev.icon()/></span>
            <span class="d-toast__title">{title}</span>
            <time class="d-toast__time">{time}</time>
            <button type="button" class="d-btn d-btn--ghost d-btn--icon d-btn--sm" aria-label=dismiss_label on:click=move |_| dismiss()>
                <Icon name="x" small=true/>
            </button>
            {toast.body.map(|b| view! { <div class="d-toast__body">{b}</div> })}
            {action.map(|a| view! {
                <div class="d-toast__actions">
                    <button type="button" class="d-btn d-btn--secondary d-btn--sm" on:click=move |_| { a.on_click.run(()); dismiss(); }>{a.label.clone()}</button>
                </div>
            })}
        </div>
    }
}
