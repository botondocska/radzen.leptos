//! RadzenTextBox component — mirrors C# Radzen.Blazor.RadzenTextBox.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `rz-textbox [rz-state-disabled] [rz-state-empty] [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! return GetClassList("rz-textbox").ToString();
//! ```
//!
//! # Two render modes — mirrors `Immediate` prop
//! ```
//! Immediate = true  → oninput AND onchange (Blazor: @bind:event="oninput" + @onchange)
//! Immediate = false → onchange only
//! ```
//!
//! # `@attributes` spread — mirrors Blazor's `@attributes="Attributes"`
//! Non-`"class"` entries in `base.attrs` are applied imperatively after mount
//! via `NodeRef` + `Effect`, avoiding compile-time type-parameter explosion
//! from folding `.attr()` calls.
//!
//! # `id` resolution
//! `name` wins over auto-generated id — mirrors Blazor's `GetId()` override.
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

/// RadzenTextBox component.
#[component]
pub fn RadzenTextBox(
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
    /// - `true`  → Blazor Immediate: `@bind:event="oninput"` + `@onchange`
    /// - `false` → Blazor default: `@onchange` only
    /// Default: `false`.
    #[prop(default = false)]
    immediate: bool,

    /// Whether to trim leading/trailing whitespace on change. Default: `false`.
    #[prop(default = false)]
    trim: bool,

    /// Browser autocomplete behaviour. Default: [`AutoCompleteType::On`].
    #[prop(default = AutoCompleteType::On)]
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

    // ── @attributes spread via NodeRef ────────────────────────────────────────
    // Folding .attr() at compile time produces a unique HtmlElement type per
    // combination. We apply extra attrs imperatively after mount instead.
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

    // ── Event handlers ────────────────────────────────────────────────────────
    let enter_cb = handle.on_mouse_enter;
    let leave_cb = handle.on_mouse_leave;
    let ctx_cb = handle.on_context_menu;

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

    // oninput — Immediate mode only (Blazor: @bind:event="oninput")
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

    // onchange — both modes (Blazor: @onchange present in both branches)
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
    Some(
        leptos::html::input()
            .node_ref(node_ref)
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
