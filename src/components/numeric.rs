//! RadzenNumeric component — mirrors C# Radzen.Blazor.RadzenNumeric<TValue>.
//!
//! # CSS class order (mirrors Blazor exactly)
//! Root `<span>`: `rz-numeric [rz-state-disabled] [caller-class]`
//! Input `<input>`: `rz-numeric-input rz-inputtext rz-text-align-{left|center|right} [rz-state-disabled] [rz-state-empty]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! GetClassList("rz-numeric").ToString()
//! ```
//! Input CSS: `GetClassList("rz-numeric-input").Add("rz-inputtext").Add($"rz-text-align-{textAlignName}").ToString()`
//!
//! # HTML structure (mirrors Blazor)
//! ```html
//! <span class="rz-numeric …" id="…" style="…">
//!   <input class="rz-numeric-input rz-inputtext rz-text-align-left …"
//!          type="text" inputmode="decimal"
//!          value="…" placeholder="…" disabled readonly tabindex="…"
//!          onkeydown=… oninput=… onchange=… onblur=… />
//!   <span class="rz-numeric-buttons">
//!     <button type="button" class="rz-numeric-button rz-numeric-up rz-button" tabindex="-1">
//!       <span class="notranslate rz-numeric-button-icon rzi rzi-caret-up"></span>
//!     </button>
//!     <button type="button" class="rz-numeric-button rz-numeric-down rz-button" tabindex="-1">
//!       <span class="notranslate rz-numeric-button-icon rzi rzi-caret-down"></span>
//!     </button>
//!   </span>
//! </span>
//! ```
//!
//! # Value type
//! We use `f64` internally with an `Option<f64>` signal to support nullable numerics.
//! The display string is formatted according to `format` (e.g. `"N0"`, `"C2"`) or
//! falls back to plain decimal formatting.
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted when invisible.

use crate::components::{
    ClassList, TextAlign,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;
use std::sync::Arc;

/// RadzenNumeric component.
///
/// A numeric input with up/down stepper buttons, optional min/max clamping,
/// configurable step, and text alignment.
///
/// # Example
/// ```rust,ignore
/// let qty = RwSignal::new(Some(1.0_f64));
/// <RadzenNumeric value=qty min=Some(0.0) max=Some(100.0) step=1.0 />
/// ```
#[component]
pub fn RadzenNumeric(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Current numeric value. `None` represents an empty/null numeric field.
    #[prop(optional)]
    value: Option<RwSignal<Option<f64>>>,

    /// Placeholder shown when the field is empty.
    #[prop(default = None, into)]
    placeholder: Option<String>,

    /// Whether the input is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Whether the input is read-only.
    #[prop(default = false)]
    read_only: bool,

    /// `name` attribute. Also used as the element `id` when set.
    #[prop(default = None, into)]
    name: Option<String>,

    /// Tab order. Forced to `-1` when disabled.
    #[prop(default = 0)]
    tab_index: i32,

    /// Step size for the up/down buttons and arrow keys. Default: `1.0`.
    #[prop(default = 1.0_f64)]
    step: f64,

    /// Minimum allowed value (inclusive). `None` = no minimum.
    #[prop(default = None)]
    min: Option<f64>,

    /// Maximum allowed value (inclusive). `None` = no maximum.
    #[prop(default = None)]
    max: Option<f64>,

    /// Number of decimal places shown. `None` = auto (shows up to 10 significant digits).
    #[prop(default = None)]
    decimals: Option<usize>,

    /// Text alignment of the input value. Default: `TextAlign::Left`.
    #[prop(default = TextAlign::Left)]
    text_align: TextAlign,

    /// Whether to show the up/down stepper buttons. Default: `true`.
    #[prop(default = true)]
    show_updown: bool,

    /// Called when the committed value changes.
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(Option<f64>) + Send + Sync>>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // Visibility.
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    let value_signal = value.unwrap_or_else(|| RwSignal::new(None));
    let input_id = name.clone().unwrap_or_else(|| handle.id.clone());
    let effective_tab = if disabled { -1 } else { tab_index };

    // ── CSS classes ───────────────────────────────────────────────────────────
    let root_class = ClassList::create("rz-numeric")
        .add_disabled(disabled)
        .add_caller_class(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish();

    let align_suffix = match text_align {
        TextAlign::Center => "center",
        TextAlign::Right => "right",
        TextAlign::Start => "start",
        TextAlign::End => "end",
        TextAlign::Justify | TextAlign::JustifyAll => "justify",
        TextAlign::Left => "left",
    };
    let input_class_base = format!("rz-numeric-input rz-inputtext rz-text-align-{align_suffix}");
    let input_class_disabled = if disabled {
        format!("{input_class_base} rz-state-disabled")
    } else {
        input_class_base.clone()
    };

    let style = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();

    // ── Formatting helpers ────────────────────────────────────────────────────
    let format_value = move |v: f64| -> String {
        match decimals {
            Some(d) => format!("{:.prec$}", v, prec = d),
            None => {
                // Strip trailing zeros after decimal point.
                let s = format!("{}", v);
                s
            }
        }
    };

    // ── Clamp helper ──────────────────────────────────────────────────────────
    let clamp = move |v: f64| -> f64 {
        let v = if let Some(mn) = min { v.max(mn) } else { v };
        let v = if let Some(mx) = max { v.min(mx) } else { v };
        v
    };

    // ── Commit a new numeric value ────────────────────────────────────────────
    let on_change_cb = on_change.clone();
    let commit = Arc::new(move |new_val: Option<f64>| {
        let clamped = new_val.map(clamp);
        value_signal.set(clamped);
        if let Some(ref cb) = on_change_cb {
            cb(clamped);
        }
    });

    // ── Parse input text and commit ───────────────────────────────────────────
    let commit_parse = commit.clone();
    let parse_and_commit = Arc::new(move |text: &str| {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            commit_parse(None);
        } else if let Ok(v) = trimmed.parse::<f64>() {
            commit_parse(Some(v));
        }
        // Invalid parse → ignore (keep previous value).
    });

    // onchange handler.
    let parse_change = parse_and_commit.clone();
    let on_change_ev = move |ev: web_sys::Event| {
        use web_sys::wasm_bindgen::JsCast;
        if let Some(input) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        {
            parse_change(&input.value());
        }
    };

    // onblur — same as onchange to normalise the displayed value.
    let parse_blur = parse_and_commit.clone();
    let on_blur = move |ev: web_sys::FocusEvent| {
        use web_sys::wasm_bindgen::JsCast;
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                parse_blur(&input.value());
            }
        }
    };

    // onkeydown — Arrow keys step the value.
    let commit_key = commit.clone();
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if disabled || read_only {
            return;
        }
        let delta = match ev.key().as_str() {
            "ArrowUp" => Some(step),
            "ArrowDown" => Some(-step),
            _ => None,
        };
        if let Some(d) = delta {
            ev.prevent_default();
            let current = value_signal.get_untracked().unwrap_or(0.0);
            commit_key(Some(clamp(current + d)));
        }
    };

    // ── Step buttons ──────────────────────────────────────────────────────────
    let commit_up = commit.clone();
    let step_up = move |_ev: web_sys::MouseEvent| {
        if disabled || read_only {
            return;
        }
        let current = value_signal.get_untracked().unwrap_or(0.0);
        commit_up(Some(clamp(current + step)));
    };

    let commit_down = commit.clone();
    let step_down = move |_ev: web_sys::MouseEvent| {
        if disabled || read_only {
            return;
        }
        let current = value_signal.get_untracked().unwrap_or(0.0);
        commit_down(Some(clamp(current - step)));
    };

    // ── Base events ───────────────────────────────────────────────────────────
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();

    // Reactive empty class for input.
    let css_empty_input = {
        let base_cls = input_class_disabled.clone();
        move || {
            if value_signal.get().is_none() {
                format!("{base_cls} rz-state-empty")
            } else {
                base_cls.clone()
            }
        }
    };

    Some(
        leptos::html::span()
            .attr("id", handle_id)
            .attr("class", root_class)
            .attr("style", style)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            .child(
                leptos::html::input()
                    .attr("id", input_id)
                    .attr("name", name)
                    .attr("type", "text")
                    .attr("inputmode", "decimal")
                    .attr("class", move || css_empty_input())
                    .attr("placeholder", placeholder)
                    .attr("disabled", disabled)
                    .attr("readonly", read_only)
                    .attr("tabindex", effective_tab.to_string())
                    // Display the formatted value reactively.
                    .prop("value", move || {
                        value_signal
                            .get()
                            .map(|v| format_value(v))
                            .unwrap_or_default()
                    })
                    .on(leptos::ev::change, on_change_ev)
                    .on(leptos::ev::blur, on_blur)
                    .on(leptos::ev::keydown, on_keydown),
            )
            // Up/down buttons.
            .child(show_updown.then(|| {
                leptos::html::span()
                    .attr("class", "rz-numeric-buttons")
                    .child(
                        leptos::html::button()
                            .attr("type", "button")
                            .attr(
                                "class",
                                "rz-numeric-button rz-numeric-up rz-button",
                            )
                            .attr("tabindex", "-1")
                            .attr("disabled", disabled)
                            .on(leptos::ev::click, step_up)
                            .child(
                                leptos::html::span().attr(
                                    "class",
                                    "notranslate rz-numeric-button-icon rzi rzi-caret-up",
                                ),
                            ),
                    )
                    .child(
                        leptos::html::button()
                            .attr("type", "button")
                            .attr(
                                "class",
                                "rz-numeric-button rz-numeric-down rz-button",
                            )
                            .attr("tabindex", "-1")
                            .attr("disabled", disabled)
                            .on(leptos::ev::click, step_down)
                            .child(
                                leptos::html::span().attr(
                                    "class",
                                    "notranslate rz-numeric-button-icon rzi rzi-caret-down",
                                ),
                            ),
                    )
            })),
    )
    .into_any()
}