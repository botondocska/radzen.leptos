//! RadzenTextBox component — mirrors C# Radzen.Blazor.RadzenTextBox.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `rz-textbox [rz-state-disabled] [rz-state-empty] [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! GetClassList("rz-textbox")   // → ClassList.Create("rz-textbox")
//!     .AddDisabled(Disabled)   //   .Add("rz-state-disabled", Disabled)
//!     .Add(FieldIdentifier, EditContext) // validation — omitted for now
//!     .Add("rz-state-empty", !HasValue) // when value is empty/whitespace
//!     .ToString()
//! ```
//! `RadzenComponent.GetCssClass()` then appends any caller `class` attribute last.
//!
//! # Two render modes — mirrors `Immediate` prop
//! ```
//! Immediate = true  → listens on `oninput`  AND `onchange` (value updates as the user types)
//! Immediate = false → listens on `onchange` only (value updates on blur / Enter)
//! ```
//!
//! # `id` resolution — mirrors `GetId()` override
//! Blazor overrides `GetId()` to return `Name` when set, falling back to the
//! auto-generated unique id. We reproduce this: when `name` is `Some`, the
//! `id` attribute equals `name`; otherwise `handle.id` is used.
//!
//! # `Trim` behaviour
//! When enabled, leading and trailing whitespace is stripped from the value
//! whenever it changes — mirrors Blazor's `SetValue()`.
//!
//! # `aria-autocomplete` attribute
//! Mirrors Blazor's `aria-autocomplete="@AriaAutoCompleteAttribute"`.
//! The value is derived from `AutoCompleteType` via `aria_autocomplete_value()`.
//!
//! # `rz-state-empty` reactivity
//! The class is re-evaluated reactively via a `Memo` so it stays current
//! as the user types — mirrors Blazor's full re-render on each value change.
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted, not `display:none`.
//!
//! # Validation CSS classes (future work)
//! Blazor adds `rz-state-error` / `rz-state-success` via
//! `ClassList.Add(FieldIdentifier, EditContext)`. These require a form
//! validation context that is not yet implemented. The slots are reserved
//! in the class-builder chain; they will be wired in once
//! `RadzenTemplateForm` / `EditContext` are ported.

use crate::components::{
    AutoCompleteType, ClassList,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;
use std::sync::Arc;

/// Returns the ARIA autocomplete attribute value for a given `AutoCompleteType`.
///
/// Mirrors Blazor's `AriaAutoCompleteAttribute` on `FormComponentWithAutoComplete<T>`.
/// The ARIA attribute conveys the autocomplete behaviour to assistive technologies:
/// - `"none"` when autocomplete is off
/// - `"inline"` / `"list"` / `"both"` for specific semantic types
/// - `"both"` as the general default when autocomplete is on
fn aria_autocomplete_value(ac: &AutoCompleteType) -> &'static str {
    match ac {
        AutoCompleteType::Off => "none",
        AutoCompleteType::On => "both",
        // All named semantic types (email, username, tel, …) behave like "both"
        // — the browser's autofill popup both completes inline and shows a list.
        _ => "both",
    }
}

/// RadzenTextBox component.
///
/// A styled single-line text input that supports two-way signal binding,
/// optional immediate updates, whitespace trimming, and all standard
/// HTML input attributes (`disabled`, `readonly`, `placeholder`,
/// `maxlength`, `autocomplete`, `aria-autocomplete`, `name`, `tabindex`).
///
/// # Two-way binding
/// Pass an `RwSignal<String>` via `value`. The component reads from and
/// writes back to the signal whenever the value changes.
///
/// # Immediate vs deferred updates
/// - `immediate = false` (default): value is committed on blur / Enter
///   (`onchange` event). Good for expensive reactions.
/// - `immediate = true`: value is committed on every keystroke (`oninput`
///   event). Use for live search, character counters, etc.
///
/// # Example
/// ```rust,ignore
/// let username = RwSignal::new(String::new());
///
/// view! {
///     <RadzenTextBox
///         value=username
///         placeholder=Some("Enter username".to_string())
///         max_length=Some(50)
///         immediate=true
///         trim=true
///     />
/// }
/// ```
#[component]
pub fn RadzenTextBox(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Two-way bound string value.
    ///
    /// Pass an `RwSignal<String>`. The component reads the current value
    /// from the signal and writes back to it whenever the input changes.
    /// Default: a fresh empty signal.
    #[prop(optional)]
    value: Option<RwSignal<String>>,

    /// Placeholder text shown when the input is empty.
    #[prop(default = None, into)]
    placeholder: Option<String>,

    /// Whether the input is disabled.  Adds `rz-state-disabled` CSS class,
    /// sets the HTML `disabled` attribute, and forces `tabindex="-1"`.
    #[prop(default = false)]
    disabled: bool,

    /// Whether the input is read-only. Displays the value but prevents
    /// editing — the user can still select and copy the text.
    #[prop(default = false)]
    read_only: bool,

    /// `name` attribute for the `<input>` element.  Also used as the `id`
    /// when set — mirrors Blazor's `GetId()` override in `RadzenTextBox`.
    #[prop(default = None, into)]
    name: Option<String>,

    /// Tab order index.  Forced to `"-1"` when `disabled = true` so the
    /// element is skipped during keyboard navigation.
    #[prop(default = 0)]
    tab_index: i32,

    /// Maximum number of characters the user may enter.
    /// Maps to the HTML `maxlength` attribute.
    #[prop(default = None)]
    max_length: Option<u64>,

    /// Whether to update the bound value on every keystroke (`oninput`)
    /// instead of waiting for the field to lose focus (`onchange`).
    /// Default: `false`.
    #[prop(default = false)]
    immediate: bool,

    /// Whether to trim leading and trailing whitespace from the value
    /// whenever it changes.  Default: `false`.
    #[prop(default = false)]
    trim: bool,

    /// Browser autocomplete behaviour. Default: [`AutoCompleteType::On`].
    #[prop(default = AutoCompleteType::On)]
    auto_complete: AutoCompleteType,

    /// Called after the value has been committed (and trimmed, if enabled).
    /// Mirrors Blazor's `Change` `EventCallback<string>`.
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // ── Visibility — mirrors `@if (Visible)` ─────────────────────────────────
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── Internal signal ───────────────────────────────────────────────────────
    // If the caller didn't pass a signal we create a private one so the
    // component still works in uncontrolled mode.
    let value_signal = value.unwrap_or_else(|| RwSignal::new(String::new()));

    // ── Derived ARIA / autocomplete attribute strings ─────────────────────────
    let autocomplete_str = auto_complete.as_str().to_string();
    let aria_autocomplete_str = aria_autocomplete_value(&auto_complete).to_string();

    // ── Attribute values ───────────────────────────────────────────────────────
    let style = base.style.unwrap_or_default();
    // id: Name wins over the auto-generated id — mirrors GetId() override.
    let input_id = name.clone().unwrap_or_else(|| handle.id);
    let effective_tab_index = if disabled { -1 } else { tab_index };

    // ── CSS class — reactive via Memo ─────────────────────────────────────────
    // `rz-state-empty` must track the live signal so it updates as the user
    // types — mirrors Blazor's full re-render which recomputes HasValue on
    // every change. A Memo re-runs only when value_signal changes, which is
    // exactly what we want.
    //
    // The other classes (disabled, caller) are static for the component
    // lifetime, so we compute them once and move into the Memo closure.
    let static_class_prefix = ClassList::create("rz-textbox")
        .add_disabled(disabled)
        .finish();

    let caller_class = base
        .attrs
        .as_ref()
        .and_then(|a| a.get("class"))
        .cloned()
        .unwrap_or_default();

    let css_class = Memo::new(move |_| {
        let is_empty = value_signal.get().is_empty();
        let empty_class = if is_empty { " rz-state-empty" } else { "" };
        let caller = if caller_class.is_empty() {
            String::new()
        } else {
            format!(" {}", caller_class)
        };
        format!("{}{}{}", static_class_prefix, empty_class, caller)
    });

    // ── Event handlers ────────────────────────────────────────────────────────
    let enter_cb = handle.on_mouse_enter;
    let leave_cb = handle.on_mouse_leave;
    let ctx_cb = handle.on_context_menu;

    // Shared change logic — mirrors SetValue():
    //   Value = value; if (Trim) Value = Value.Trim();
    //   ValueChanged.InvokeAsync(Value);   ← writes signal
    //   Change.InvokeAsync(Value);         ← fires on_change callback
    let on_change_cb = on_change;
    let commit = Arc::new(move |raw: String| {
        let mut v = raw;
        if trim {
            v = v.trim().to_string();
        }
        value_signal.set(v.clone());
        if let Some(ref cb) = on_change_cb {
            cb(v);
        }
    });

    // oninput handler (Immediate = true)
    // Blazor Immediate branch: @bind:event="oninput" @onchange="@OnChange"
    // Both oninput and onchange fire SetValue when Immediate = true.
    let commit_input = commit.clone();
    let on_input = move |ev: web_sys::Event| {
        use web_sys::wasm_bindgen::JsCast;
        if let Some(input) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        {
            commit_input(input.value());
        }
    };

    // onchange handler — fires in both modes, mirrors Blazor's `@onchange="@OnChange"`
    // which is present in both the Immediate and non-Immediate razor branches.
    let commit_change = commit.clone();
    let on_change_ev = move |ev: web_sys::Event| {
        use web_sys::wasm_bindgen::JsCast;
        if let Some(input) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        {
            commit_change(input.value());
        }
    };

    // ── Render ────────────────────────────────────────────────────────────────
    // Mirrors Blazor's single `<input>` element. Both modes share the same
    // attributes; the difference is whether oninput also commits (Immediate).
    //
    // `prop:value` (DOM property, not HTML attribute) keeps the cursor position
    // stable on re-render — same as Blazor's @bind:get / @bind:set pattern.
    Some(
        leptos::html::input()
            .attr("id", input_id)
            .attr("type", "text")
            .attr("name", name)
            .attr("class", move || css_class.get())
            .attr("style", style)
            .attr("placeholder", placeholder)
            .attr("disabled", disabled)
            .attr("readonly", read_only)
            .attr("tabindex", effective_tab_index.to_string())
            .attr("maxlength", max_length.map(|n| n.to_string()))
            .attr("autocomplete", autocomplete_str)
            // Mirrors: aria-autocomplete="@AriaAutoCompleteAttribute"
            .attr("aria-autocomplete", aria_autocomplete_str)
            // Controlled value — always reflects the signal.
            .prop("value", move || value_signal.get())
            // Wire oninput — commits immediately when Immediate = true.
            // Mirrors Blazor Immediate branch: @bind:event="oninput"
            .on(leptos::ev::input, move |ev| {
                if immediate {
                    on_input(ev.into());
                }
            })
            // Wire onchange — commits in both modes.
            // Mirrors Blazor's @onchange="@OnChange" present in both branches.
            .on(leptos::ev::change, move |ev| {
                on_change_ev(ev.into());
            })
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
    )
    .into_any()
}