//! RadzenTextArea component — mirrors C# Radzen.Blazor.RadzenTextArea.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `rz-textarea [rz-state-disabled] [rz-state-empty] [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! return GetClassList("rz-textarea").ToString();
//! ```
//!
//! # Two render modes — mirrors `Immediate` prop
//! ```
//! Immediate = true  → oninput ONLY  (Blazor: @bind:event="oninput", no @onchange)
//! Immediate = false → onchange ONLY (Blazor: value="@Value" @onchange="@OnChange")
//! ```
//!
//! # `@attributes` spread — mirrors Blazor's `@attributes="Attributes"`
//! Non-`"class"` entries in `base.attrs` are applied imperatively after mount
//! via `NodeRef` + `Effect`.
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted, not `display:none`.

use crate::components::{
    ClassList,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;
use std::sync::Arc;

/// RadzenTextArea component.
#[component]
pub fn RadzenTextArea(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    /// Non-`"class"` entries in `attrs` are spread onto the `<textarea>` element.
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Two-way bound string value.
    #[prop(optional)]
    value: Option<RwSignal<String>>,

    /// Placeholder text shown when the textarea is empty.
    #[prop(default = None, into)]
    placeholder: Option<String>,

    /// Whether the textarea is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Whether the textarea is read-only.
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

    /// Number of visible text rows. Default: `2`.
    #[prop(default = 2)]
    rows: u32,

    /// Number of visible text columns. Default: `20`.
    /// Note: has no visual effect when the Radzen theme sets `width: 100%`.
    #[prop(default = 20)]
    cols: u32,

    /// Whether to update on every keystroke (`oninput`) instead of `onchange`.
    /// - `true`  → Blazor Immediate: `oninput` only
    /// - `false` → Blazor default: `onchange` only
    /// Default: `false`.
    #[prop(default = false)]
    immediate: bool,

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

    // ── Attribute values ───────────────────────────────────────────────────────
    let style = base.style.clone().unwrap_or_default();
    let textarea_id = name.clone().unwrap_or_else(|| handle.id);
    let effective_tab_index = if disabled { -1 } else { tab_index };

    // ── CSS class — reactive via Memo ─────────────────────────────────────────
    let static_class_prefix = ClassList::create("rz-textarea")
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

    let node_ref = NodeRef::<leptos::html::Textarea>::new();

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

    // ── Event handlers ─────────────────────────────────────────────────────────
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

    // oninput — Immediate = true ONLY
    let commit_input = commit.clone();
    let on_input = move |ev: web_sys::Event| {
        use web_sys::wasm_bindgen::JsCast;
        if let Some(textarea) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
        {
            commit_input(textarea.value());
        }
    };

    // onchange — Immediate = false ONLY
    let commit_change = commit.clone();
    let on_change_ev = move |ev: web_sys::Event| {
        use web_sys::wasm_bindgen::JsCast;
        if let Some(textarea) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
        {
            commit_change(textarea.value());
        }
    };

    // ── Render ─────────────────────────────────────────────────────────────────
    Some(
        leptos::html::textarea()
            .node_ref(node_ref)
            .attr("id", textarea_id)
            .attr("name", name)
            .attr("class", move || css_class.get())
            .attr("style", style)
            .attr("placeholder", placeholder)
            .attr("disabled", disabled)
            .attr("readonly", read_only)
            .attr("rows", rows.to_string())
            .attr("cols", cols.to_string())
            .attr("tabindex", effective_tab_index.to_string())
            .attr("maxlength", max_length.map(|n| n.to_string()))
            .prop("value", move || value_signal.get())
            .on(leptos::ev::input, move |ev| {
                if immediate {
                    on_input(ev.into());
                }
            })
            .on(leptos::ev::change, move |ev| {
                if !immediate {
                    on_change_ev(ev.into());
                }
            })
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev)),
    )
    .into_any()
}