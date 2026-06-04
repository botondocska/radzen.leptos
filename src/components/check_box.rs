//! RadzenCheckBox component — mirrors C# Radzen.Blazor.RadzenCheckBox<TValue>.
//!
//! # CSS class order (mirrors Blazor exactly)
//! Root `<div>`: `rz-chkbox [rz-state-disabled] [caller-class]`
//! Inner box `<div>`: `notranslate rz-chkbox-box [rz-state-active] [rz-state-disabled]`
//! Icon `<span>`: `notranslate rz-chkbox-icon [rzi rzi-check | rzi rzi-times]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! GetClassList("rz-chkbox").ToString()
//! ```
//!
//! # Value semantics (mirrors Blazor exactly)
//! - `Some(true)`  → checked   (`rz-state-active` on box + `rzi rzi-check` icon)
//! - `Some(false)` → unchecked (no active class, no icon content)
//! - `None`        → indeterminate (`rz-state-active` on box + `rzi rzi-times` icon)
//!
//! # Toggle cycle (mirrors Blazor exactly)
//! Two-state: `false → true → false`
//! Tri-state: `false → null → true → false`
//!
//! Blazor C# `Toggle()`:
//! ```csharp
//! if (object.Equals(Value, false)) {
//!     Value = TriState ? default(TValue) : (TValue)(object)true;
//! } else if (Value == null) {
//!     Value = (TValue)(object)true;
//! } else if (object.Equals(Value, true)) {
//!     Value = (TValue)(object)false;
//! }
//! ```
//!
//! # BoxClass (mirrors Blazor exactly)
//! ```csharp
//! ClassList.Create("rz-chkbox-box")
//!     .Add("rz-state-active", !object.Equals(Value, false))  // active when true OR null
//!     .AddDisabled(Disabled)
//! ```
//! Note: `notranslate` prefix is added in the razor template, not in BoxClass.
//!
//! # IconClass (mirrors Blazor exactly)
//! ```csharp
//! ClassList.Create("notranslate rz-chkbox-icon")
//!     .Add("rzi rzi-check", object.Equals(Value, true))
//!     .Add("rzi rzi-times", object.Equals(Value, null))
//! ```
//!
//! # Keyboard
//! Only `Space` triggers toggle — mirrors Blazor's `OnKeyPress` which checks `args.Code == "Space"`.
//! Enter is NOT included in the Blazor source.
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted when invisible.

use crate::components::{
    ClassList,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;
use std::sync::Arc;

/// RadzenCheckBox component.
///
/// A styled checkbox supporting two-state (`bool`) or tri-state (`Option<bool>`) modes.
/// Uses `Option<bool>` as the universal value type:
/// - `Some(true)`  → checked
/// - `Some(false)` → unchecked
/// - `None`        → indeterminate (only produced in tri-state mode)
///
/// # Toggle cycle
/// Two-state: `false → true → false`
/// Tri-state: `false → null (None) → true → false`
///
/// # Two-state usage
/// ```rust,ignore
/// let checked = RwSignal::new(Some(false));
/// <RadzenCheckBox value=checked />
/// ```
///
/// # Tri-state usage
/// ```rust,ignore
/// let tri = RwSignal::new(None::<bool>);
/// <RadzenCheckBox value=tri tri_state=true />
/// ```
#[component]
pub fn RadzenCheckBox(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Current value. `Some(true)` = checked, `Some(false)` = unchecked, `None` = indeterminate.
    #[prop(optional)]
    value: Option<RwSignal<Option<bool>>>,

    /// `name` attribute forwarded to the hidden `<input>`. Also used as element `id`
    /// when set — mirrors Blazor's `GetId()` override.
    #[prop(default = None, into)]
    name: Option<String>,

    /// Tab order. Forced to `-1` when disabled.
    #[prop(default = 0)]
    tab_index: i32,

    /// Whether the checkbox is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Whether the checkbox is read-only (renders but ignores clicks/keyboard).
    #[prop(default = false)]
    read_only: bool,

    /// Enable tri-state cycling: `false → None (null) → true → false`.
    /// When `false` (default) clicking only toggles between `Some(true)` and `Some(false)`.
    #[prop(default = false)]
    tri_state: bool,

    /// Called after the value changes, with the new `Option<bool>`.
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(Option<bool>) + Send + Sync>>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // Visibility — mirrors `@if (Visible)`.
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // Internal signal — use provided or create local.
    let value_signal = value.unwrap_or_else(|| RwSignal::new(Some(false)));

    // Unique id for the hidden input.
    let input_id = name.clone().unwrap_or_else(|| handle.id.clone());
    let effective_tab = if disabled { -1 } else { tab_index };

    // ── Root CSS class ────────────────────────────────────────────────────────
    // GetComponentCssClass() → GetClassList("rz-chkbox")
    let root_class = ClassList::create("rz-chkbox")
        .add_disabled(disabled)
        .add_caller_class(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish();

    let style = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();

    // ── Toggle logic — mirrors Blazor `Toggle()` exactly ──────────────────────
    // Two-state:  false → true → false
    // Tri-state:  false → null → true → false
    //
    // Blazor:
    //   if (Equals(Value, false)) Value = TriState ? null : true
    //   else if (Value == null)   Value = true
    //   else if (Equals(Value, true)) Value = false
    let on_change_cb = on_change.clone();
    let toggle = Arc::new(move || {
        if disabled || read_only {
            return;
        }
        let next = match value_signal.get_untracked() {
            Some(false) => {
                if tri_state {
                    None // false → null (tri-state only)
                } else {
                    Some(true) // false → true (two-state)
                }
            }
            None => Some(true),        // null → true
            Some(true) => Some(false), // true → false
        };
        value_signal.set(next);
        if let Some(ref cb) = on_change_cb {
            cb(next);
        }
    });

    // Click handler.
    let toggle_click = toggle.clone();
    let on_click = move |_ev: web_sys::MouseEvent| toggle_click();

    // Keydown — Space ONLY triggers toggle (mirrors Blazor's OnKeyPress checking args.Code == "Space").
    let toggle_key = toggle.clone();
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.code() == "Space" {
            ev.prevent_default();
            toggle_key();
        }
    };

    // Base mouse event handlers.
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();

    Some(
        leptos::html::div()
            .attr("id", handle_id)
            .attr("class", root_class)
            .attr("style", style)
            .attr("tabindex", effective_tab.to_string())
            .attr("role", "checkbox")
            // aria-checked: "true" | "false" | "mixed" (for indeterminate/null)
            // Mirrors: CheckBoxAriaChecked => true ? "true" : null ? "mixed" : "false"
            .attr("aria-checked", move || match value_signal.get() {
                Some(true) => "true",
                Some(false) => "false",
                None => "mixed",
            })
            .attr("aria-disabled", if disabled { "true" } else { "false" })
            .on(leptos::ev::click, on_click)
            .on(leptos::ev::keydown, on_keydown)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            // Hidden accessible input — mirrors Blazor's rz-helper-hidden-accessible wrapper.
            .child(
                leptos::html::div()
                    .attr("class", "rz-helper-hidden-accessible")
                    .child(
                        leptos::html::input()
                            .attr("type", "checkbox")
                            .attr("id", input_id.clone())
                            .attr("name", name)
                            .attr("readonly", true)
                            // value attr reflects the checked state as a string.
                            .attr("value", move || match value_signal.get() {
                                Some(true) => "true",
                                _ => "false",
                            })
                            // The actual checked property for the DOM element.
                            .prop("checked", move || matches!(value_signal.get(), Some(true))),
                    ),
            )
            // Visible box + icon — rendered reactively.
            // BoxClass mirrors: ClassList.Create("rz-chkbox-box")
            //     .Add("rz-state-active", !object.Equals(Value, false))  // active when true OR null
            //     .AddDisabled(Disabled)
            // Note: "notranslate" is prepended in the razor template separately.
            .child(move || {
                let box_class = ClassList::create("notranslate rz-chkbox-box")
                    // Active when NOT false — i.e. when Some(true) OR None(indeterminate)
                    .add(
                        "rz-state-active",
                        !matches!(value_signal.get(), Some(false)),
                    )
                    .add_disabled(disabled)
                    .finish();

                // IconClass mirrors:
                // ClassList.Create("notranslate rz-chkbox-icon")
                //     .Add("rzi rzi-check", Equals(Value, true))
                //     .Add("rzi rzi-times", Value == null)
                let icon_class = match value_signal.get() {
                    Some(true) => "notranslate rz-chkbox-icon rzi rzi-check",
                    None => "notranslate rz-chkbox-icon rzi rzi-times",
                    Some(false) => "notranslate rz-chkbox-icon",
                };

                leptos::html::div()
                    .attr("class", box_class)
                    .child(leptos::html::span().attr("class", icon_class))
            }),
    )
    .into_any()
}
