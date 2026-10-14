//! Form controls. Native elements underneath: the platform supplies keyboard and screen reader behaviour.

use leptos::prelude::*;

use crate::icon::Icon;
use crate::types::Size;
use crate::util::{uid, when_true};

/// Wrapper for a label, a control and its hint or error line.
#[component]
pub fn Field(children: Children) -> impl IntoView {
    view! { <div class="d-field">{children()}</div> }
}

/// A visible label bound to a control by id. Placeholder is not a label.
#[component]
pub fn Label(#[prop(into)] r#for: String, children: Children) -> impl IntoView {
    view! { <label class="d-label" for=r#for>{children()}</label> }
}

/// A hint under a control. Link it with `aria-describedby`.
#[component]
pub fn Hint(#[prop(into)] id: String, children: Children) -> impl IntoView {
    view! { <span class="d-hint" id=id>{children()}</span> }
}

/// An error line: an icon and a message that starts with `error:`.
#[component]
pub fn ErrorText(#[prop(into)] id: String, children: Children) -> impl IntoView {
    view! { <span class="d-error" id=id><Icon name="octagon-alert" small=true/>{children()}</span> }
}

fn error_message(msg: &str) -> String {
    if msg.starts_with("error:") { msg.to_string() } else { format!("error: {msg}") }
}

fn describedby(hint: Option<&str>, error: Option<&str>) -> Option<String> {
    let ids: Vec<&str> = [hint, error].into_iter().flatten().collect();
    (!ids.is_empty()).then(|| ids.join(" "))
}

/// Single-line input (or textarea), a faint `bg-field` tint with a 3:1 `border-strong` edge, with its label, hint and error.
///
/// `error: Some(..)` marks the field invalid and shows the message, prefixed with `error:` when it is not.
#[component]
pub fn TextField(
    #[prop(into)] label: String,
    value: RwSignal<String>,
    /// Appends `(optional)` to the label.
    #[prop(optional)]
    optional: bool,
    #[prop(optional, into)] hint: MaybeProp<String>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    /// `text`, `email`, `password`, `search`, … Default `text`.
    #[prop(optional)]
    input_type: Option<&'static str>,
    #[prop(optional)] size: Size,
    /// A textarea instead of an input.
    #[prop(optional)]
    multiline: bool,
    #[prop(default = 3)] rows: u32,
    /// Validating: dashes the border. Say what is happening in the hint.
    #[prop(optional, into)]
    busy: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] readonly: Signal<bool>,
    #[prop(optional, into)] autocomplete: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
) -> impl IntoView {
    let id = uid("field");
    let hint_id = format!("{id}-hint");
    let err_id = format!("{id}-err");
    let class = if size == Size::Lg { "d-input d-input--lg" } else { "d-input" };
    let describe = {
        let (hint_id, err_id) = (hint_id.clone(), err_id.clone());
        move || {
            describedby(
                hint.get().is_some().then_some(hint_id.as_str()),
                error.get().is_some().then_some(err_id.as_str()),
            )
        }
    };
    let invalid = move || when_true(error.get().is_some());
    let busy_attr = move || when_true(busy.get());

    let control = if multiline {
        view! {
            <textarea
                class=class id=id.clone() rows=rows
                prop:value=move || value.get()
                on:input=move |e| value.set(event_target_value(&e))
                placeholder=move || placeholder.get()
                aria-describedby=describe.clone()
                aria-invalid=invalid
                aria-busy=busy_attr
                disabled=move || disabled.get()
                readonly=move || readonly.get()
                name=move || name.get()
            ></textarea>
        }
        .into_any()
    } else {
        view! {
            <input
                class=class id=id.clone() type=input_type.unwrap_or("text")
                prop:value=move || value.get()
                on:input=move |e| value.set(event_target_value(&e))
                placeholder=move || placeholder.get()
                aria-describedby=describe
                aria-invalid=invalid
                aria-busy=busy_attr
                disabled=move || disabled.get()
                readonly=move || readonly.get()
                autocomplete=move || autocomplete.get()
                name=move || name.get()
            />
        }
        .into_any()
    };

    view! {
        <div class="d-field">
            <label class="d-label" for=id>{label}{optional.then(|| view! { " " <span>"(optional)"</span> })}</label>
            {control}
            {move || hint.get().map(|h| view! { <span class="d-hint" id=hint_id.clone()>{h}</span> })}
            {move || error.get().map(|e| view! {
                <span class="d-error" id=err_id.clone()><Icon name="octagon-alert" small=true/>{error_message(&e)}</span>
            })}
        </div>
    }
}

/// Native checkbox. Wrapped in `d-choice` so the label is part of the hit target.
///
/// A checkbox takes effect on submit; for an immediate setting use [`Switch`].
#[component]
pub fn Checkbox(
    checked: RwSignal<bool>,
    #[prop(optional, into)] indeterminate: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] name: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let input = NodeRef::<leptos::html::Input>::new();
    Effect::new(move |_| {
        let ind = indeterminate.get();
        if let Some(el) = input.get() {
            el.set_indeterminate(ind);
        }
    });
    view! {
        <label class="d-choice">
            <input
                node_ref=input type="checkbox" class="d-check"
                prop:checked=move || checked.get()
                on:change=move |e| checked.set(event_target_checked(&e))
                aria-invalid=move || when_true(invalid.get())
                disabled=move || disabled.get()
                name=move || name.get()
            />
            <span>{children()}</span>
        </label>
    }
}

#[derive(Clone)]
struct RadioCtx {
    name: String,
    value: RwSignal<String>,
    invalid: Signal<bool>,
}

/// A fieldset of [`Radio`]s sharing one value. Two to five options; more is a [`Select`].
#[component]
pub fn RadioGroup(
    #[prop(into)] label: String,
    value: RwSignal<String>,
    #[prop(optional, into)] invalid: Signal<bool>,
    children: Children,
) -> impl IntoView {
    provide_context(RadioCtx { name: uid("radio"), value, invalid });
    view! {
        <fieldset>
            <legend class="d-label">{label}</legend>
            <div class="d-row">{children()}</div>
        </fieldset>
    }
}

/// One option of a [`RadioGroup`]. Arrow keys move the selection (native).
#[component]
pub fn Radio(
    #[prop(into)] value: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<RadioCtx>();
    let v = value.clone();
    let v2 = value.clone();
    view! {
        <label class="d-choice">
            <input
                type="radio" class="d-radio" name=ctx.name.clone() value=value
                prop:checked=move || ctx.value.get() == v
                on:change=move |_| ctx.value.set(v2.clone())
                aria-invalid=move || when_true(ctx.invalid.get())
                disabled=move || disabled.get()
            />
            <span>{children()}</span>
        </label>
    }
}

/// A checkbox with `role="switch"` for settings that apply immediately. The label names the setting, not the state.
#[component]
pub fn Switch(
    checked: RwSignal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <label class="d-choice">
            <input
                type="checkbox" role="switch" class="d-switch"
                prop:checked=move || checked.get()
                on:change=move |e| checked.set(event_target_checked(&e))
                disabled=move || disabled.get()
            />
            <span>{children()}</span>
        </label>
    }
}

/// An entry of a [`Select`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        SelectOption { value: value.into(), label: label.into(), disabled: false }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl<V: Into<String> + Clone> From<V> for SelectOption {
    fn from(v: V) -> Self {
        let s: String = v.into();
        SelectOption::new(s.clone(), s)
    }
}

/// Native select with the same field and edge as the text field. Up to about twelve options; more is a menu or the palette.
#[component]
pub fn Select(
    #[prop(into)] label: String,
    value: RwSignal<String>,
    #[prop(into)] options: Signal<Vec<SelectOption>>,
    #[prop(optional, into)] hint: MaybeProp<String>,
    #[prop(optional, into)] error: MaybeProp<String>,
    #[prop(optional)] size: Size,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let id = uid("select");
    let hint_id = format!("{id}-hint");
    let err_id = format!("{id}-err");
    let class = if size == Size::Lg { "d-input d-input--lg" } else { "d-input" };
    let select = NodeRef::<leptos::html::Select>::new();
    // set the value once the options exist and whenever either changes
    Effect::new(move |_| {
        let v = value.get();
        let _ = options.get();
        if let Some(el) = select.get() {
            el.set_value(&v);
        }
    });
    let describe = {
        let (hint_id, err_id) = (hint_id.clone(), err_id.clone());
        move || {
            describedby(
                hint.get().is_some().then_some(hint_id.as_str()),
                error.get().is_some().then_some(err_id.as_str()),
            )
        }
    };
    view! {
        <div class="d-field">
            <label class="d-label" for=id.clone()>{label}</label>
            <span class="d-select">
                <select
                    node_ref=select class=class id=id
                    aria-describedby=describe
                    aria-invalid=move || when_true(error.get().is_some())
                    disabled=move || disabled.get()
                    on:change=move |e| value.set(event_target_value(&e))
                >
                    {move || options.get().into_iter().map(|o| {
                        let selected = value.get_untracked() == o.value;
                        view! { <option value=o.value.clone() disabled=o.disabled selected=selected>{o.label.clone()}</option> }
                    }).collect_view()}
                </select>
            </span>
            {move || hint.get().map(|h| view! { <span class="d-hint" id=hint_id.clone()>{h}</span> })}
            {move || error.get().map(|e| view! {
                <span class="d-error" id=err_id.clone()><Icon name="octagon-alert" small=true/>{error_message(&e)}</span>
            })}
        </div>
    }
}

/// Native range input with its value shown in an `<output>` (the fill alone is colour; the number is the cue).
#[component]
pub fn Slider(
    value: RwSignal<f64>,
    /// `aria-label`; a slider always needs a name.
    #[prop(into)]
    label: String,
    #[prop(default = 0.0)] min: f64,
    #[prop(default = 100.0)] max: f64,
    #[prop(default = 1.0)] step: f64,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let id = uid("slider");
    let fill = move || {
        let pct = if max > min { (value.get() - min) / (max - min) * 100.0 } else { 0.0 };
        format!("--d-fill:{pct}%")
    };
    view! {
        <div class="d-slider-row">
            <input
                type="range" class="d-slider" id=id.clone() aria-label=label
                min=min max=max step=step
                prop:value=move || value.get().to_string()
                on:input=move |e| {
                    if let Ok(v) = event_target_value(&e).parse::<f64>() { value.set(v) }
                }
                style=fill
                disabled=move || disabled.get()
            />
            <output r#for=id>{move || value.get().to_string()}</output>
        </div>
    }
}

