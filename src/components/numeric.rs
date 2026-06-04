//! RadzenNumeric component — mirrors C# Radzen.Blazor.RadzenNumeric<TValue>.
//!
//! # CSS class order (mirrors Blazor exactly)
//! Root `<span>`: `rz-numeric [rz-state-disabled] [caller-class]`
//! Input `<input>`: `rz-numeric-input rz-inputtext rz-text-align-{left|center|right|start|end|justify}`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! GetClassList("rz-numeric").ToString()
//! ```
//! Input CSS — `GetInputCssClass()`:
//! ```csharp
//! GetClassList("rz-numeric-input")
//!     .Add("rz-inputtext")
//!     .Add($"rz-text-align-{Enum.GetName<TextAlign>(TextAlign).ToLowerInvariant()}")
//!     .ToString()
//! ```
//! Note: `rz-state-disabled` and `rz-state-empty` are NOT part of the input CSS class
//! in Blazor — they belong on the root span via `GetClassList`.
//!
//! # HTML structure (mirrors Blazor razor template exactly)
//! ```html
//! <span class="rz-numeric …" id="…" style="…">
//!   <input class="rz-numeric-input rz-inputtext rz-text-align-left"
//!          type="text" inputmode="decimal"
//!          value="…" placeholder="…" disabled readonly tabindex="…" />
//!   @if (ShowUpDown) {
//!     <button type="button" class="rz-numeric-button rz-numeric-up rz-button" tabindex="-1">
//!       <span class="notranslate rz-numeric-button-icon rzi rzi-caret-up"></span>
//!     </button>
//!     <button type="button" class="rz-numeric-button rz-numeric-down rz-button" tabindex="-1">
//!       <span class="notranslate rz-numeric-button-icon rzi rzi-caret-down"></span>
//!     </button>
//!   }
//! </span>
//! ```
//! Important: Blazor renders the up/down buttons as DIRECT children of the root `<span>`,
//! NOT wrapped in an extra `<span class="rz-numeric-buttons">`. The previous version
//! incorrectly added a wrapper span.
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

    /// Number of decimal places shown. `None` = auto (plain f64 Display).
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
    // Root span: GetClassList("rz-numeric") — includes rz-state-disabled when disabled.
    let root_class = ClassList::create("rz-numeric")
        .add_disabled(disabled)
        .add_caller_class(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish();

    // Input: GetInputCssClass() = "rz-numeric-input rz-inputtext rz-text-align-{name}"
    // Mirrors: Enum.GetName<TextAlign>(TextAlign).ToLowerInvariant()
    let align_name = match text_align {
        TextAlign::Left => "left",
        TextAlign::Center => "center",
        TextAlign::Right => "right",
        TextAlign::Start => "start",
        TextAlign::End => "end",
        TextAlign::Justify | TextAlign::JustifyAll => "justify",
    };
    // Input CSS is STATIC — no reactive empty/disabled classes on input in Blazor.
    let input_class = format!("rz-numeric-input rz-inputtext rz-text-align-{align_name}");

    let style = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();

    // ── Formatting helper ─────────────────────────────────────────────────────
    let format_value = move |v: f64| -> String {
        match decimals {
            Some(d) => format!("{:.prec$}", v, prec = d),
            None => format!("{}", v),
        }
    };

    // ── Clamp helper ──────────────────────────────────────────────────────────
    let clamp = move |v: f64| -> f64 {
        let v = if let Some(mn) = min { v.max(mn) } else { v };
        if let Some(mx) = max { v.min(mx) } else { v }
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
        // Invalid parse → keep previous value (no-op).
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

    // onblur — normalise the displayed value to the committed value.
    let parse_blur = parse_and_commit.clone();
    let on_blur = move |ev: web_sys::FocusEvent| {
        use web_sys::wasm_bindgen::JsCast;
        if let Some(target) = ev.target() {
            if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
                parse_blur(&input.value());
            }
        }
    };

    // onkeydown — ArrowUp/ArrowDown step the value (mirrors Blazor OnKeyPress).
    let commit_key = commit.clone();
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if disabled || read_only {
            return;
        }
        let key = if ev.code().is_empty() {
            ev.key()
        } else {
            ev.code()
        };
        let delta = match key.as_str() {
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

    // Reactive prop value for the input — shows formatted or empty.
    let input_class_clone = input_class.clone();

    Some(
        leptos::html::span()
            .attr("id", handle_id)
            .attr("class", root_class)
            .attr("style", style)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            // ── Input ──────────────────────────────────────────────────────────
            // Mirrors Blazor's <input class="@GetInputCssClass()" type="text" inputmode="decimal" …>
            .child(
                leptos::html::input()
                    .attr("id", input_id)
                    .attr("name", name)
                    .attr("type", "text")
                    .attr("inputmode", "decimal")
                    .attr("class", input_class_clone)
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
            // ── Up/down buttons — direct children of root span (NOT wrapped) ──
            // Mirrors Blazor razor: @if (ShowUpDown) { <button>…</button> <button>…</button> }
            // No intermediate wrapper span — that was wrong in the previous version.
            .child(show_updown.then(|| {
                view! {
                    <>
                        <button
                            type="button"
                            class="rz-numeric-button rz-numeric-up rz-button"
                            tabindex="-1"
                            disabled=disabled
                            on:click=step_up
                        >
                            <span class="notranslate rz-numeric-button-icon rzi rzi-caret-up"></span>
                        </button>
                        <button
                            type="button"
                            class="rz-numeric-button rz-numeric-down rz-button"
                            tabindex="-1"
                            disabled=disabled
                            on:click=step_down
                        >
                            <span class="notranslate rz-numeric-button-icon rzi rzi-caret-down"></span>
                        </button>
                    </>
                }
            })),
    )
    .into_any()
}
