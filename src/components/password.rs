//! RadzenPassword component — mirrors C# Radzen.Blazor.RadzenPassword.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `rz-textbox [rz-state-disabled] [rz-state-empty] [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! protected override string GetComponentCssClass()
//! {
//!     return GetClassList("rz-textbox").ToString();
//! }
//! ```
//! `RadzenPassword` reuses the same CSS class as `RadzenTextBox` — the only
//! visual difference is the HTML `type="password"` attribute, which the browser
//! uses to mask the input as dots.
//!
//! # HTML `type="password"`
//! The single key difference from `RadzenTextBox`: the `<input>` is rendered
//! with `type="password"` instead of `type="text"`. The browser handles all
//! masking automatically.
//!
//! # Autocomplete
//! Default is `AutoCompleteType::NewPassword` — mirrors Blazor's
//! `DefaultAutoCompleteAttribute = "new-password"` which helps password managers
//! recognise this as a new-password field (e.g. on registration forms).
//! Use `AutoCompleteType::CurrentPassword` for login forms.
//!
//! # Two render modes — mirrors `Immediate` prop
//! Same as `RadzenTextBox`:
//! ```
//! Immediate = true  → listens on both `oninput` AND `onchange`
//! Immediate = false → listens on `onchange` only
//! ```
//!
//! # `@attributes` spread
//! All non-`"class"` entries in `base.attrs` are forwarded to the `<input>`.
//!
//! # `id` resolution
//! `name` wins over the auto-generated id — mirrors Blazor's `GetId()` override.
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted, not `display:none`.

use crate::components::{
    AutoCompleteType, ClassList,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;
use std::sync::Arc;

fn aria_autocomplete_value(ac: &AutoCompleteType) -> &'static str {
    match ac {
        AutoCompleteType::Off => "none",
        _ => "both",
    }
}

/// RadzenPassword component.
///
/// A styled `<input type="password">` that masks characters as dots.
/// Shares the same CSS class (`rz-textbox`) and all behaviour with
/// [`RadzenTextBox`], differing only in the `type` attribute.
///
/// # Autocomplete
/// Defaults to `AutoCompleteType::NewPassword` (`autocomplete="new-password"`),
/// which is correct for registration / change-password forms and helps password
/// managers identify the field. For login forms, pass
/// `auto_complete=AutoCompleteType::CurrentPassword`.
///
/// # Example
/// ```rust,ignore
/// let password = RwSignal::new(String::new());
///
/// view! {
///     <RadzenLabel text=Some("Password".to_string()) component=Some("pwd".to_string()) />
///     <RadzenPassword
///         name=Some("pwd".to_string())
///         value=password
///         placeholder=Some("Enter password".to_string())
///     />
/// }
/// ```
#[component]
pub fn RadzenPassword(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    /// Non-`"class"` entries in `attrs` are spread onto the `<input>` element.
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Two-way bound string value.
    #[prop(optional)]
    value: Option<RwSignal<String>>,

    /// Placeholder text shown when the input is empty.
    #[prop(default = None, into)]
    placeholder: Option<String>,

    /// Whether the input is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Whether the input is read-only.
    #[prop(default = false)]
    read_only: bool,

    /// `name` attribute. Also used as `id` when set — mirrors `GetId()` override.
    #[prop(default = None, into)]
    name: Option<String>,

    /// Tab order index. Forced to `-1` when disabled.
    #[prop(default = 0)]
    tab_index: i32,

    /// Maximum number of characters the user may enter.
    #[prop(default = None)]
    max_length: Option<u64>,

    /// Whether to update on every keystroke (`oninput`) in addition to `onchange`.
    /// Mirrors Blazor's `Immediate` parameter. Default: `false`.
    #[prop(default = false)]
    immediate: bool,

    /// Browser autocomplete behaviour.
    ///
    /// Default: [`AutoCompleteType::NewPassword`] — mirrors Blazor's
    /// `DefaultAutoCompleteAttribute = "new-password"`.
    /// Use [`AutoCompleteType::CurrentPassword`] for login forms.
    #[prop(default = AutoCompleteType::NewPassword)]
    auto_complete: AutoCompleteType,

    /// Called after the value has been committed.
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // ── Visibility ────────────────────────────────────────────────────────────
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── Internal signal ───────────────────────────────────────────────────────
    let value_signal = value.unwrap_or_else(|| RwSignal::new(String::new()));

    // ── Attribute strings ─────────────────────────────────────────────────────
    let autocomplete_str = auto_complete.as_str().to_string();
    let aria_autocomplete_str = aria_autocomplete_value(&auto_complete).to_string();
    let style = base.style.clone().unwrap_or_default();
    let input_id = name.clone().unwrap_or_else(|| handle.id);
    let effective_tab_index = if disabled { -1 } else { tab_index };

    // ── CSS class — reactive via Memo ─────────────────────────────────────────
    // Mirrors GetClassList("rz-textbox"):
    //   .AddDisabled(Disabled)
    //   .Add("rz-state-empty", !HasValue)
    // then caller class appended last.
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

    // ── @attributes spread ────────────────────────────────────────────────────
    let extra_attrs: Vec<(String, String)> = base
        .attrs
        .as_ref()
        .map(|a| {
            a.iter()
                .filter(|(k, _)| k.as_str() != "class")
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        })
        .unwrap_or_default();

    // ── Event handlers ────────────────────────────────────────────────────────
    let enter_cb = handle.on_mouse_enter;
    let leave_cb = handle.on_mouse_leave;
    let ctx_cb = handle.on_context_menu;

    let on_change_cb = on_change;
    let commit = Arc::new(move |raw: String| {
        value_signal.set(raw.clone());
        if let Some(ref cb) = on_change_cb {
            cb(raw);
        }
    });

    // oninput — Immediate mode only.
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

    // onchange — both modes (same as RadzenTextBox).
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
    let node_ref = NodeRef::<leptos::html::Input>::new();

    if !extra_attrs.is_empty() {
        let attrs_clone = extra_attrs.clone();
        Effect::new(move |_| {
            if let Some(el) = node_ref.get() {
                use web_sys::wasm_bindgen::JsCast;
                if let Some(el) = el.dyn_ref::<web_sys::HtmlElement>() {
                    for (k, v) in &attrs_clone {
                        el.set_attribute(k, v).ok();
                    }
                }
            }
        });
    }

    Some(
        leptos::html::input()
            .node_ref(node_ref)
            .attr("id", input_id)
            .attr("type", "password")
            .attr("name", name)
            .attr("class", move || css_class.get())
            .attr("style", style)
            .attr("placeholder", placeholder)
            .attr("disabled", disabled)
            .attr("readonly", read_only)
            .attr("tabindex", effective_tab_index.to_string())
            .attr("maxlength", max_length.map(|n| n.to_string()))
            .attr("autocomplete", autocomplete_str)
            .attr("aria-autocomplete", aria_autocomplete_str)
            .prop("value", move || value_signal.get())
            .on(leptos::ev::input, move |ev| {
                if immediate {
                    on_input(ev.into());
                }
            })
            .on(leptos::ev::change, move |ev| {
                on_change_ev(ev.into());
            })
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev)),
    )
    .into_any()
}