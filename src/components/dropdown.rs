//! RadzenDropDown component — mirrors C# Radzen.Blazor.RadzenDropDown<TValue>.
//!
//! # CSS class order (mirrors Blazor exactly)
//! Root `<div>`: `rz-dropdown [rz-state-disabled] [rz-state-empty] [rz-state-focused] [caller-class]`
//!
//! Blazor `GetComponentCssClass()` (via DropDownBase → GetClassList):
//! ```csharp
//! GetClassList("rz-dropdown")
//!     .Add("rz-state-focused", focused)
//!     .ToString()
//! ```
//!
//! # HTML structure (mirrors Blazor RadzenDropDown.razor exactly)
//! ```html
//! <div class="rz-dropdown …" role="combobox" aria-haspopup="listbox" aria-expanded="…"
//!      tabindex="0" id="…" style="…">
//!
//!   <!-- hidden accessible input -->
//!   <div class="rz-helper-hidden-accessible">
//!     <input disabled readonly type="text" tabindex="-1" name="…" value="…" />
//!   </div>
//!
//!   <!-- selected label OR placeholder -->
//!   <span class="rz-dropdown-label rz-inputtext [rz-placeholder]">…</span>
//!
//!   <!-- chevron trigger -->
//!   <div class="rz-dropdown-trigger rz-corner-right">
//!     <span class="notranslate rz-dropdown-trigger-icon rzi rzi-chevron-down"></span>
//!   </div>
//!
//!   <!-- clear button (when AllowClear && HasValue) -->
//!   <button class="notranslate rz-dropdown-clear-icon rzi rzi-times" …></button>
//!
//!   <!-- popup panel — inside root div, absolutely positioned -->
//!   <div id="…-popup" class="rz-dropdown-panel" style="display:none | block; position:absolute; …">
//!     <!-- filter input (when AllowFiltering) -->
//!     <div class="rz-dropdown-filter-container"> … </div>
//!     <!-- multiple header with select-all -->
//!     <div class="rz-multiselect-header rz-helper-clearfix"> … </div>
//!     <!-- items list -->
//!     <div class="rz-dropdown-items-wrapper">
//!       <ul class="rz-dropdown-items rz-dropdown-list" role="listbox">
//!         <li class="rz-dropdown-item [rz-state-highlight] [rz-state-disabled]" role="option">…</li>
//!       </ul>
//!     </div>
//!   </div>
//! </div>
//! ```
//!
//! # Popup positioning
//! Blazor uses JS (`Radzen.openPopup`) to absolutely position the panel relative to the
//! root element, allowing it to overflow parent containers. In Leptos we use
//! `position: absolute` on the panel and `position: relative` on the root div —
//! a pure-CSS equivalent that works for most layouts. The panel opens downward and
//! has a high z-index so it overlays sibling content.
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted when invisible.

use crate::components::{
    ClassList,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;
use std::sync::Arc;

// ─────────────────────────────────────────────────────────────────────────────
// Data types
// ─────────────────────────────────────────────────────────────────────────────

/// A single item in a [`RadzenDropDown`].
#[derive(Clone, Debug, PartialEq)]
pub struct DropDownItem {
    /// The underlying value (matched against the selected `value` signal).
    pub value: String,
    /// Display label shown in the list and selected label.
    pub label: String,
    /// Whether this item is disabled.
    pub disabled: bool,
}

impl DropDownItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RadzenDropDown
// ─────────────────────────────────────────────────────────────────────────────

/// RadzenDropDown component.
///
/// A styled select/dropdown with optional search filtering and multiple-selection support.
///
/// # Single selection
/// ```rust,ignore
/// let selected = RwSignal::new(String::new());
/// <RadzenDropDown value=selected data=items placeholder=Some("Select…") />
/// ```
///
/// # Multiple selection
/// ```rust,ignore
/// let selected = RwSignal::new(Vec::<String>::new());
/// <RadzenDropDown value_multiple=selected data=items multiple=true />
/// ```
#[component]
pub fn RadzenDropDown(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Items to display in the list.
    #[prop(default = vec![])]
    data: Vec<DropDownItem>,

    /// Single-selection value signal. Holds the `value` of the selected item.
    /// Ignored when `multiple=true`.
    #[prop(optional)]
    value: Option<RwSignal<String>>,

    /// Multi-selection value signal. Holds the `value`s of all selected items.
    /// Only used when `multiple=true`.
    #[prop(optional)]
    value_multiple: Option<RwSignal<Vec<String>>>,

    /// Enable multi-select mode. Default: `false`.
    #[prop(default = false)]
    multiple: bool,

    /// Placeholder text shown when nothing is selected.
    #[prop(default = None, into)]
    placeholder: Option<String>,

    /// Whether to show a text filter box inside the dropdown panel. Default: `false`.
    #[prop(default = false)]
    allow_filtering: bool,

    /// Placeholder inside the filter input. Default: `"Search…"`.
    #[prop(default = "Search…".to_string(), into)]
    filter_placeholder: String,

    /// Whether the component is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Whether the component is read-only.
    #[prop(default = false)]
    read_only: bool,

    /// `name` attribute forwarded to the hidden input.
    #[prop(default = None, into)]
    name: Option<String>,

    /// Tab order. Forced to `-1` when disabled.
    #[prop(default = 0)]
    tab_index: i32,

    /// Text shown for "Select All" in multi-select mode. Default: `"Select All"`.
    #[prop(default = "Select All".to_string(), into)]
    select_all_text: String,

    /// Whether to show a "Select All" checkbox in multi-select mode. Default: `true`.
    #[prop(default = true)]
    allow_select_all: bool,

    /// Whether to show a clear (×) button when a value is selected. Default: `false`.
    #[prop(default = false)]
    allow_clear: bool,

    /// Called when the selection changes (single: new value; multiple: comma-joined values).
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // Visibility.
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── Internal state ────────────────────────────────────────────────────────
    let open       = RwSignal::new(false);
    let focused    = RwSignal::new(false);
    let filter_text = RwSignal::new(String::new());

    let effective_tab = if disabled { -1 } else { tab_index };

    // ── Static values ─────────────────────────────────────────────────────────
    let style     = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();
    let popup_id  = format!("{}-popup", handle_id);

    let caller_class = base
        .attrs
        .as_ref()
        .and_then(|a| a.get("class"))
        .cloned()
        .unwrap_or_default();

    // ── selected_label ────────────────────────────────────────────────────────
    // Single StoredValue for data — shared by all closures.
    let data_panel_sv = StoredValue::new(data.clone());

    let selected_label = {
        move || -> String {
            let items = data_panel_sv.get_value();
            if multiple {
                let selected = value_multiple.map(|s| s.get()).unwrap_or_default();
                if selected.is_empty() { return String::new(); }
                selected
                    .iter()
                    .filter_map(|v| items.iter().find(|i| &i.value == v))
                    .map(|i| i.label.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                let val = value.map(|s| s.get()).unwrap_or_default();
                items.iter().find(|i| i.value == val)
                    .map(|i| i.label.clone())
                    .unwrap_or_default()
            }
        }
    };

    // ── has_value ─────────────────────────────────────────────────────────────
    let has_value = move || -> bool {
        if multiple {
            value_multiple.map(|s| !s.get().is_empty()).unwrap_or(false)
        } else {
            value.map(|s| !s.get().is_empty()).unwrap_or(false)
        }
    };

    // ── is_selected ───────────────────────────────────────────────────────────
    let is_selected = move |item_value: &str| -> bool {
        if multiple {
            value_multiple.map(|s| s.get().iter().any(|v| v == item_value)).unwrap_or(false)
        } else {
            value.map(|s| s.get() == item_value).unwrap_or(false)
        }
    };

    // ── select_item ───────────────────────────────────────────────────────────
    let on_change_cb = Arc::new(on_change);
    let on_change_cb2 = on_change_cb.clone();

    let select_item = Arc::new(move |item_value: String| {
        if disabled || read_only { return; }
        if multiple {
            if let Some(multi_sig) = value_multiple {
                multi_sig.update(|v| {
                    if let Some(pos) = v.iter().position(|x| *x == item_value) {
                        v.remove(pos);
                    } else {
                        v.push(item_value.clone());
                    }
                });
                if let Some(ref cb) = *on_change_cb {
                    cb(value_multiple.unwrap().get().join(","));
                }
            }
        } else {
            if let Some(single_sig) = value {
                single_sig.set(item_value.clone());
            }
            if let Some(ref cb) = *on_change_cb {
                cb(item_value);
            }
            // Close and reset filter on single select.
            open.set(false);
            filter_text.set(String::new());
        }
    });

    // ── select_all ────────────────────────────────────────────────────────────
    let select_all = Arc::new(move || {
        if let Some(multi_sig) = value_multiple {
            let all_vals: Vec<String> = data_panel_sv.get_value().iter()
                .filter(|i| !i.disabled)
                .map(|i| i.value.clone())
                .collect();
            let currently_all = {
                let sel = multi_sig.get();
                !all_vals.is_empty() && all_vals.iter().all(|v| sel.contains(v))
            };
            if currently_all {
                multi_sig.set(vec![]);
                if let Some(ref cb) = *on_change_cb2 { cb(String::new()); }
            } else {
                multi_sig.set(all_vals.clone());
                if let Some(ref cb) = *on_change_cb2 { cb(all_vals.join(",")); }
            }
        }
    });

    // ── clear ─────────────────────────────────────────────────────────────────
    let clear = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        if let Some(s) = value { s.set(String::new()); }
        if let Some(s) = value_multiple { s.set(vec![]); }
        open.set(false);
        filter_text.set(String::new());
    };

    // ── Toggle open ───────────────────────────────────────────────────────────
    let toggle = move |_ev: web_sys::MouseEvent| {
        if disabled || read_only { return; }
        let is_open = open.get_untracked();
        open.set(!is_open);
        if is_open { filter_text.set(String::new()); }
    };

    // Close on blur — short delay so item clicks register first.
    let on_blur = move |_ev: web_sys::FocusEvent| {
        focused.set(false);
        gloo_timers::callback::Timeout::new(150, move || {
            open.set(false);
            filter_text.set(String::new());
        }).forget();
    };

    let on_focus = move |_ev: web_sys::FocusEvent| { focused.set(true); };

    // Keyboard navigation.
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        let key = if ev.code().is_empty() { ev.key() } else { ev.code() };
        match key.as_str() {
            "Escape" => {
                open.set(false);
                filter_text.set(String::new());
            }
            "Enter" | "Space" => {
                if !open.get_untracked() { open.set(true); }
            }
            _ => {}
        }
    };

    // Base mouse event handlers.
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb   = handle.on_context_menu.clone();

    // StoredValues for data used inside reactive (FnMut) closures.
    let filter_placeholder_sv = StoredValue::new(filter_placeholder);
    let select_all_text_sv   = StoredValue::new(select_all_text);
    let popup_id_sv          = StoredValue::new(popup_id.clone());

    Some(
        leptos::html::div()
            .attr("id", handle_id)
            // position:relative so the absolutely-positioned panel stays anchored to this element.
            .attr("style", {
                let base_style = if style.is_empty() {
                    "position: relative;".to_string()
                } else {
                    format!("position: relative; {}", style)
                };
                base_style
            })
            .attr("tabindex", effective_tab.to_string())
            .attr("role", "combobox")
            .attr("aria-haspopup", "listbox")
            .attr("aria-expanded", move || if open.get() { "true" } else { "false" })
            .attr("aria-disabled", if disabled { "true" } else { "false" })
            // Root CSS class — reactive: includes rz-state-focused, rz-state-empty.
            .attr("class", move || {
                let is_empty = !has_value();
                ClassList::create("rz-dropdown")
                    .add("rz-state-focused", focused.get())
                    .add_disabled(disabled)
                    .add("rz-state-empty", is_empty)
                    .add_caller_class(if caller_class.is_empty() { None } else { Some(caller_class.as_str()) })
                    .finish()
            })
            .on(leptos::ev::click,      toggle)
            .on(leptos::ev::blur,       on_blur)
            .on(leptos::ev::focus,      on_focus)
            .on(leptos::ev::keydown,    on_keydown)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu,move |ev| ctx_cb(ev))

            // ── Hidden accessible input ───────────────────────────────────────
            // Mirrors Blazor: <div class="rz-helper-hidden-accessible"><input …/></div>
            .child(
                leptos::html::div()
                    .attr("class", "rz-helper-hidden-accessible")
                    .child(
                        leptos::html::input()
                            .attr("type", "text")
                            .attr("name", name.clone())
                            .attr("readonly", true)
                            .attr("aria-haspopup", "listbox")
                            .attr("aria-expanded", move || if open.get() { "true" } else { "false" })
                            .attr("tabindex", "-1")
                            .attr("disabled", disabled)
                            .prop("value", move || selected_label()),
                    ),
            )

            // ── Selected label / placeholder ──────────────────────────────────
            // Mirrors Blazor's span.rz-dropdown-label.rz-inputtext branch logic.
            .child(move || {
                let lbl = if multiple {
                    value_multiple.map(|s| s.get()).unwrap_or_default()
                        .iter()
                        .filter_map(|v| {
                            data_panel_sv.get_value().iter().find(|i| &i.value == v).map(|i| i.label.clone())
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                } else {
                    let val = value.map(|s| s.get()).unwrap_or_default();
                    data_panel_sv.get_value().iter()
                        .find(|i| i.value == val)
                        .map(|i| i.label.clone())
                        .unwrap_or_default()
                };

                if lbl.is_empty() {
                    // Placeholder.
                    leptos::html::span()
                        .attr("class", "rz-dropdown-label rz-inputtext rz-placeholder")
                        .child(placeholder.clone().unwrap_or_default())
                        .into_any()
                } else {
                    leptos::html::span()
                        .attr("class", "rz-dropdown-label rz-inputtext")
                        .child(lbl)
                        .into_any()
                }
            })

            // ── Chevron trigger ───────────────────────────────────────────────
            // Mirrors Blazor: <div class="rz-dropdown-trigger rz-corner-right">…</div>
            .child(
                leptos::html::div()
                    .attr("class", "rz-dropdown-trigger rz-corner-right")
                    .child(
                        leptos::html::span()
                            .attr("class", "notranslate rz-dropdown-trigger-icon rzi rzi-chevron-down"),
                    ),
            )

            // ── Clear button ─────────────────────────────────────────────────
            // Mirrors: @if (AllowClear && !ReadOnly && HasValue) { <button …/> }
            .child(move || {
                if !allow_clear || read_only || !has_value() {
                    return None::<AnyView>.into_any();
                }
                Some(
                    leptos::html::button()
                        .attr("type", "button")
                        .attr("tabindex", "-1")
                        .attr("class", "notranslate rz-dropdown-clear-icon rzi rzi-times")
                        .attr("aria-label", "Clear")
                        .on(leptos::ev::click, clear)
                        .into_any()
                ).into_any()
            })

            // ── Popup panel ───────────────────────────────────────────────────
            // Rendered inside the root div and absolutely positioned over sibling content.
            // Mirrors Blazor's <div id="@PopupID" class="rz-dropdown-panel" style="display:none">
            .child(move || {
                let panel_class = if multiple { "rz-multiselect-panel" } else { "rz-dropdown-panel" };
                let display_style = if open.get() { "display:block" } else { "display:none" };
                // Absolute positioning anchored to root div; high z-index overrides siblings.
                let panel_style = format!(
                    "{}; position:absolute; left:0; top:100%; min-width:100%; z-index:1000; box-sizing:border-box;",
                    display_style
                );

                let items = data_panel_sv.get_value();
                let filter = filter_text.get();
                let filter_lower = filter.to_lowercase();

                // Apply filter.
                let filtered: Vec<DropDownItem> = if filter.is_empty() {
                    items.clone()
                } else {
                    items.iter()
                        .filter(|i| i.label.to_lowercase().contains(&filter_lower))
                        .cloned()
                        .collect()
                };

                // ── Filter input ──────────────────────────────────────────────
                // Single: rz-dropdown-filter-container
                // Multiple: rz-multiselect-filter-container (inside header)
                let single_filter: Option<AnyView> = (!multiple && allow_filtering).then(|| {
                    leptos::html::div()
                        .attr("class", "rz-dropdown-filter-container")
                        .child(
                            leptos::html::input()
                                .attr("type", "text")
                                .attr("class", "rz-dropdown-filter rz-inputtext")
                                .attr("placeholder", filter_placeholder_sv.get_value())
                                .attr("autocomplete", "off")
                                .prop("value", move || filter_text.get())
                                .on(leptos::ev::input, move |ev: web_sys::Event| {
                                    use web_sys::wasm_bindgen::JsCast;
                                    if let Some(inp) = ev.target()
                                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                                    {
                                        filter_text.set(inp.value());
                                    }
                                })
                                // Stop propagation so clicks inside filter don't toggle the panel.
                                .on(leptos::ev::click, |ev: web_sys::MouseEvent| ev.stop_propagation()),
                        )
                        .child(
                            leptos::html::span()
                                .attr("class", "notranslate rz-dropdown-filter-icon rzi rzi-search"),
                        )
                        .into_any()
                });

                // ── Multiple header (select-all + optional filter) ────────────
                // Mirrors Blazor: <div class="rz-multiselect-header rz-helper-clearfix">
                // Each conditional child is built as Option<AnyView> and collected, avoiding
                // the Leptos HtmlElement monomorphisation problem with mut reassignment.
                let multi_header: Option<AnyView> = multiple.then(|| {
                    let sa = select_all.clone();
                    let is_all = value_multiple.map(|s| {
                        let sel = s.get();
                        let total_enabled = items.iter().filter(|i| !i.disabled).count();
                        !sel.is_empty() && total_enabled > 0 && sel.len() >= total_enabled
                    }).unwrap_or(false);

                    let chkbox_class = if is_all {
                        "notranslate rz-chkbox-box rz-state-active"
                    } else {
                        "notranslate rz-chkbox-box"
                    };
                    let chk_icon_class = if is_all {
                        "notranslate rz-chkbox-icon rzi rzi-check"
                    } else {
                        "notranslate rz-chkbox-icon"
                    };

                    let selectall_chkbox: Option<AnyView> = allow_select_all.then(|| {
                        leptos::html::div()
                            .attr("class", "rz-chkbox")
                            .attr("role", "checkbox")
                            .attr("aria-checked", if is_all { "true" } else { "false" })
                            .attr("aria-disabled", if disabled { "true" } else { "false" })
                            .on(leptos::ev::click, {
                                let sa2 = sa.clone();
                                move |ev: web_sys::MouseEvent| {
                                    ev.stop_propagation();
                                    if !disabled && !read_only {
                                        sa2();
                                    }
                                }
                            })
                            .child(
                                leptos::html::div()
                                    .attr("class", "rz-helper-hidden-accessible")
                                    .child(
                                        leptos::html::input()
                                            .attr("type", "checkbox")
                                            .attr("readonly", true)
                                    )
                            )
                            .child(
                                leptos::html::div()
                                    .attr("class", chkbox_class)
                                    .child(
                                        leptos::html::span()
                                            .attr("class", chk_icon_class)
                                    )
                            )
                            .into_any()
                    });

                    let selectall_btn: Option<AnyView> =
                        (allow_select_all && !allow_filtering).then(|| {
                            leptos::html::button()
                                .attr("type", "button")
                                .attr("class", "rz-multiselect-selectall")
                                .attr("disabled", disabled || read_only)
                                .on(leptos::ev::click, {
                                    let sa3 = sa.clone();
                                    move |ev: web_sys::MouseEvent| {
                                        ev.stop_propagation();
                                        if !disabled && !read_only {
                                            sa3();
                                        }
                                    }
                                })
                                .child(select_all_text_sv.get_value())
                                .into_any()
                        });

                    let multi_filter: Option<AnyView> = allow_filtering.then(|| {
                        leptos::html::div()
                            .attr("class", "rz-multiselect-filter-container")
                            .child(
                                leptos::html::input()
                                    .attr("type", "text")
                                    .attr("class", "rz-inputtext")
                                    .attr("placeholder", filter_placeholder_sv.get_value())
                                    .prop("value", move || filter_text.get())
                                    .on(leptos::ev::input, move |ev: web_sys::Event| {
                                        use web_sys::wasm_bindgen::JsCast;
                                        if let Some(inp) = ev.target()
                                            .and_then(|t| {
                                                t.dyn_into::<web_sys::HtmlInputElement>().ok()
                                            })
                                        {
                                            filter_text.set(inp.value());
                                        }
                                    })
                                    .on(
                                        leptos::ev::click,
                                        |ev: web_sys::MouseEvent| ev.stop_propagation(),
                                    ),
                            )
                            .child(
                                leptos::html::span()
                                    .attr(
                                        "class",
                                        "notranslate rz-multiselect-filter-icon rzi rzi-search",
                                    ),
                            )
                            .into_any()
                    });

                    leptos::html::div()
                        .attr(
                            "class",
                            "rz-multiselect-header rz-helper-clearfix",
                        )
                        .child(selectall_chkbox)
                        .child(selectall_btn)
                        .child(multi_filter)
                        .into_any()
                });

                // ── Items list ────────────────────────────────────────────────
                let list_class = if multiple {
                    "rz-multiselect-items rz-multiselect-list"
                } else {
                    "rz-dropdown-items rz-dropdown-list"
                };
                let wrapper_class = if multiple {
                    "rz-multiselect-items-wrapper"
                } else {
                    "rz-dropdown-items-wrapper"
                };

                let items_view = filtered.into_iter().map(|item| {
                    let item_selected = is_selected(&item.value);
                    let li_class = ClassList::create("rz-dropdown-item")
                        .add("rz-state-highlight", item_selected)
                        .add_disabled(item.disabled)
                        .finish();

                    let iv = item.value.clone();
                    let si = select_item.clone();
                    let item_disabled = item.disabled;

                    let on_item_click = move |ev: web_sys::MouseEvent| {
                        ev.stop_propagation();
                        if !item_disabled { si(iv.clone()); }
                    };

                    // Checkbox for multiple mode.
                    let chk: Option<AnyView> = multiple.then(|| {
                        let box_cls = if item_selected {
                            "notranslate rz-chkbox-box rz-state-active"
                        } else {
                            "notranslate rz-chkbox-box"
                        };
                        let icon_cls = if item_selected {
                            "notranslate rz-chkbox-icon rzi rzi-check"
                        } else {
                            "notranslate rz-chkbox-icon"
                        };
                        leptos::html::div()
                            .attr("class", box_cls)
                            .child(leptos::html::span().attr("class", icon_cls))
                            .into_any()
                    });

                    leptos::html::li()
                        .attr("class", li_class)
                        .attr("role", "option")
                        .attr("aria-selected", if item_selected { "true" } else { "false" })
                        .on(leptos::ev::click, on_item_click)
                        .child(chk)
                        .child(leptos::html::span().child(item.label.clone()))
                        .into_any()
                }).collect_view();

                leptos::html::div()
                    .attr("id", popup_id_sv.get_value())
                    .attr("class", panel_class)
                    .attr("style", panel_style)
                    // Prevent mousedown from blurring the root and closing the panel
                    // before click events on items can fire.
                    .on(leptos::ev::mousedown, |ev: web_sys::MouseEvent| ev.prevent_default())
                    .child(single_filter)
                    .child(multi_header)
                    .child(
                        leptos::html::div()
                            .attr("class", wrapper_class)
                            .child(
                                leptos::html::ul()
                                    .attr("class", list_class)
                                    .attr("role", "listbox")
                                    .attr("aria-multiselectable", if multiple { "true" } else { "false" })
                                    .child(items_view),
                            ),
                    )
                    .into_any()
            }),
    )
    .into_any()
}