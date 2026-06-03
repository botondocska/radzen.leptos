//! RadzenDropDown component — mirrors C# Radzen.Blazor.RadzenDropDown<TValue>.
//!
//! # CSS class order (mirrors Blazor exactly)
//! Root `<div>`: `rz-dropdown [rz-state-disabled] [rz-state-empty] [caller-class]`
//! When open: adds `rz-state-focused`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! GetClassList("rz-dropdown")
//!     .Add("rz-state-focused", focused)
//!     .ToString()
//! ```
//!
//! # HTML structure (simplified — mirrors core Blazor template)
//! ```html
//! <div class="rz-dropdown …" id="…" style="…"
//!      tabindex="0" role="listbox" aria-expanded="false"
//!      onclick=… onkeydown=… onblur=…>
//!
//!   <div class="rz-helper-hidden-accessible">
//!     <input type="text" readonly aria-haspopup="listbox" … />
//!   </div>
//!   <label class="rz-dropdown-label rz-inputtext">Selected text or placeholder</label>
//!   <span class="rz-dropdown-trigger rz-button rz-button-icon-only">
//!     <span class="notranslate rz-button-icon-left rzi rzi-chevron-down"></span>
//!   </span>
//!
//!   @if open {
//!     <div class="rz-dropdown-panel rz-shadow-1">
//!       @if allow_filtering {
//!         <div class="rz-dropdown-filter-container">
//!           <input class="rz-inputtext" type="text" placeholder=filter_placeholder … />
//!           <span class="notranslate rz-dropdown-filter-icon rzi rzi-search"></span>
//!         </div>
//!       }
//!       <div class="rz-dropdown-items-wrapper">
//!         <ul class="rz-dropdown-items rz-dropdown-list" role="listbox">
//!           @for item in filtered_items {
//!             <li class="rz-dropdown-item [rz-state-highlight] [rz-state-disabled]"
//!                 role="option" aria-selected="…" onclick=…>
//!               @item.label
//!             </li>
//!           }
//!         </ul>
//!       </div>
//!     </div>
//!   }
//! </div>
//! ```
//!
//! # Design decisions vs Blazor
//! Blazor uses JS interop (Radzen.openPopup) for the dropdown panel. In Leptos we
//! implement an inline panel toggled by a Leptos signal — no JS interop needed.
//!
//! Blazor's generic `TValue` becomes `String` here (the value of the selected item).
//! Items are `DropDownItem` structs with a `value: String` and a `label: String`.
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
/// let items = vec![
///     DropDownItem::new("1", "Orders"),
///     DropDownItem::new("2", "Employees"),
/// ];
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

    /// Text shown above the list for "Select All" in multi-select mode.
    #[prop(default = "Select All".to_string(), into)]
    select_all_text: String,

    /// Whether to show a "Select All" option in multi-select mode. Default: `true`.
    #[prop(default = true)]
    allow_select_all: bool,

    /// Called when the selection changes. Single: new value string. Multiple: comma-joined values.
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // Visibility.
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── Internal state ────────────────────────────────────────────────────────
    let open = RwSignal::new(false);
    let filter_text = RwSignal::new(String::new());
    let effective_tab = if disabled { -1 } else { tab_index };

    // ── CSS ───────────────────────────────────────────────────────────────────
    let caller_class = base
        .attrs
        .as_ref()
        .and_then(|a| a.get("class"))
        .cloned()
        .unwrap_or_default();

    let style = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();

    // ── selected_label ────────────────────────────────────────────────────────
    // Shared between the hidden-input .prop() and the visible label .child().
    // Wrapping in Arc<Fn> lets both closures clone it instead of moving it.
    let data_sv = StoredValue::new(data.clone());
    let selected_label: Arc<dyn Fn() -> String + Send + Sync> =
        Arc::new(move || -> String {
            let items = data_sv.get_value();
            if multiple {
                let selected = value_multiple.map(|s| s.get()).unwrap_or_default();
                if selected.is_empty() {
                    return String::new();
                }
                selected
                    .iter()
                    .filter_map(|v| items.iter().find(|i| &i.value == v))
                    .map(|i| i.label.clone())
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                let val = value.map(|s| s.get()).unwrap_or_default();
                items
                    .iter()
                    .find(|i| i.value == val)
                    .map(|i| i.label.clone())
                    .unwrap_or_default()
            }
        });

    // ── is_selected ───────────────────────────────────────────────────────────
    // Pure read; signals are Copy — no move issues.
    let is_selected = move |item_value: &str| -> bool {
        if multiple {
            value_multiple
                .map(|s| s.get().iter().any(|v| v == item_value))
                .unwrap_or(false)
        } else {
            value.map(|s| s.get() == item_value).unwrap_or(false)
        }
    };

    // ── select_item ───────────────────────────────────────────────────────────
    // Arc so the FnMut panel closure can clone it per item without moving it out.
    let on_change_cb = Arc::new(on_change);
    let on_change_cb2 = on_change_cb.clone(); 
    let select_item = Arc::new(move |item_value: String| {
        if disabled || read_only {
            return;
        }
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
            open.set(false);
            filter_text.set(String::new());
        }
    });

    // ── select_all ────────────────────────────────────────────────────────────
    let data_sv2 = StoredValue::new(data.clone());
    let select_all = Arc::new(move || {
        if let Some(multi_sig) = value_multiple {
            let all_vals: Vec<String> =
                data_sv2.get_value().iter().map(|i| i.value.clone()).collect();
            let currently_all = multi_sig.get().len() == all_vals.len();
            if currently_all {
                multi_sig.set(vec![]);
                if let Some(ref cb) = *on_change_cb2 {
                    cb(String::new());
                }
            } else {
                multi_sig.set(all_vals.clone());
                if let Some(ref cb) = *on_change_cb2 {
                    cb(all_vals.join(","));
                }
            }
        }
    });

    // ── Toggle open ───────────────────────────────────────────────────────────
    let toggle = move |_ev: web_sys::MouseEvent| {
        if disabled || read_only {
            return;
        }
        let currently_open = open.get_untracked();
        open.set(!currently_open);
        if currently_open {
            filter_text.set(String::new());
        }
    };

    // Close on blur (short delay so item clicks register first).
    let on_blur = move |_ev: web_sys::FocusEvent| {
        gloo_timers::callback::Timeout::new(150, move || {
            open.set(false);
            filter_text.set(String::new());
        })
        .forget();
    };

    // Keyboard navigation.
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        match ev.key().as_str() {
            "Escape" => {
                open.set(false);
                filter_text.set(String::new());
            }
            "Enter" | " " => {
                if !open.get_untracked() {
                    open.set(true);
                }
            }
            _ => {}
        }
    };

    // Base mouse event handlers.
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();

    // StoredValues for data used inside reactive closures.
    let data_panel_sv = StoredValue::new(data);
    let filter_placeholder_sv = StoredValue::new(filter_placeholder);
    let select_all_text_sv = StoredValue::new(select_all_text);

    // Clones of selected_label for each use site.
    let sl_hidden = selected_label.clone();
    let sl_label = selected_label.clone();

    Some(
        leptos::html::div()
            .attr("id", handle_id)
            .attr("style", style)
            .attr("tabindex", effective_tab.to_string())
            .attr("role", "listbox")
            .attr("aria-expanded", move || {
                if open.get() { "true" } else { "false" }
            })
            .attr("aria-disabled", if disabled { "true" } else { "false" })
            // Root CSS class — reactive so open/empty state is reflected.
            .attr("class", move || {
                let is_empty = if multiple {
                    value_multiple.map(|s| s.get().is_empty()).unwrap_or(true)
                } else {
                    value.map(|s| s.get().is_empty()).unwrap_or(true)
                };
                ClassList::create("rz-dropdown")
                    .add_class(if open.get() { "rz-state-focused" } else { "" })
                    .add_disabled(disabled)
                    .add("rz-state-empty", is_empty)
                    .add_caller_class(if caller_class.is_empty() {
                        None
                    } else {
                        Some(caller_class.as_str())
                    })
                    .finish()
            })
            .on(leptos::ev::click, toggle)
            .on(leptos::ev::blur, on_blur)
            .on(leptos::ev::keydown, on_keydown)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            // ── Hidden accessible input ───────────────────────────────────────
            .child(
                leptos::html::div()
                    .attr("class", "rz-helper-hidden-accessible")
                    .child(
                        leptos::html::input()
                            .attr("type", "text")
                            .attr("name", name)
                            .attr("readonly", true)
                            .attr("aria-haspopup", "listbox")
                            .attr("aria-expanded", move || {
                                if open.get() { "true" } else { "false" }
                            })
                            // Clone of Arc — does not move.
                            .prop("value", move || sl_hidden()),
                    ),
            )
            // ── Selected-value label ──────────────────────────────────────────
            .child(move || {
                let lbl = sl_label();
                let display_text = if lbl.is_empty() {
                    placeholder.clone().unwrap_or_default()
                } else {
                    lbl
                };
                leptos::html::label()
                    .attr("class", "rz-dropdown-label rz-inputtext")
                    .child(display_text)
            })
            // ── Trigger chevron ───────────────────────────────────────────────
            .child(
                leptos::html::span()
                    .attr("class", "rz-dropdown-trigger rz-button rz-button-icon-only")
                    .child(
                        leptos::html::span().attr(
                            "class",
                            "notranslate rz-button-icon-left rzi rzi-chevron-down",
                        ),
                    ),
            )
            // ── Panel (shown when open) ───────────────────────────────────────
            // FnMut: every capture is Copy, RwSignal (Copy), StoredValue, or Arc (cloned).
            .child(move || {
                if !open.get() {
                    return None::<AnyView>.into_any();
                }

                let items = data_panel_sv.get_value();
                let filter = filter_text.get();
                let filter_lower = filter.to_lowercase();

                // Apply filter.
                let filtered: Vec<DropDownItem> = if filter.is_empty() {
                    items.clone()
                } else {
                    items
                        .iter()
                        .filter(|i| i.label.to_lowercase().contains(&filter_lower))
                        .cloned()
                        .collect()
                };

                // Select-all row (multiple mode only).
                let select_all_row: Option<AnyView> =
                    (multiple && allow_select_all && !allow_filtering).then(|| {
                        let sa = select_all.clone();
                        let is_all = value_multiple
                            .map(|s| {
                                let sel = s.get();
                                !sel.is_empty() && sel.len() == items.len()
                            })
                            .unwrap_or(false);
                        leptos::html::div()
                            .attr("class", "rz-multiselect-header rz-selectall")
                            .child(
                                leptos::html::div()
                                    .attr(
                                        "class",
                                        if is_all {
                                            "notranslate rz-chkbox-box rz-state-active"
                                        } else {
                                            "notranslate rz-chkbox-box"
                                        },
                                    )
                                    .on(leptos::ev::click, move |_| sa()),
                            )
                            .child(
                                leptos::html::label()
                                    .attr("class", "rz-chkbox-label")
                                    .child(select_all_text_sv.get_value()),
                            )
                            .into_any()
                    });

                Some(
                    leptos::html::div()
                        .attr("class", "rz-dropdown-panel rz-shadow-1")
                        .on(leptos::ev::mousedown, move |ev: web_sys::MouseEvent| {
                            ev.prevent_default();
                        })
                        // Filter input.
                        .child(allow_filtering.then(|| {
                            leptos::html::div()
                                .attr("class", "rz-dropdown-filter-container")
                                .child(
                                    leptos::html::input()
                                        .attr("type", "text")
                                        .attr("class", "rz-inputtext")
                                        .attr(
                                            "placeholder",
                                            filter_placeholder_sv.get_value(),
                                        )
                                        .prop("value", move || filter_text.get())
                                        .on(leptos::ev::input, move |ev: web_sys::Event| {
                                            use web_sys::wasm_bindgen::JsCast;
                                            if let Some(input) = ev.target().and_then(|t| {
                                                t.dyn_into::<web_sys::HtmlInputElement>().ok()
                                            }) {
                                                filter_text.set(input.value());
                                            }
                                        })
                                        .on(leptos::ev::click, |ev: web_sys::MouseEvent| {
                                            ev.stop_propagation();
                                        }),
                                )
                                .child(
                                    leptos::html::span().attr(
                                        "class",
                                        "notranslate rz-dropdown-filter-icon rzi rzi-search",
                                    ),
                                )
                        }))
                        .child(select_all_row)
                        // Items list.
                        .child(
                            leptos::html::div()
                                .attr("class", "rz-dropdown-items-wrapper")
                                .child(
                                    leptos::html::ul()
                                        .attr("class", "rz-dropdown-items rz-dropdown-list")
                                        .attr("role", "listbox")
                                        .child(
                                            filtered
                                                .into_iter()
                                                .map(|item| {
                                                    let item_selected = is_selected(&item.value);
                                                    let li_class =
                                                        ClassList::create("rz-dropdown-item")
                                                            .add("rz-state-highlight", item_selected)
                                                            .add_disabled(item.disabled)
                                                            .finish();

                                                    let iv = item.value.clone();
                                                    let item_disabled = item.disabled;
                                                    // Clone Arc — keeps outer closure FnMut.
                                                    let si = select_item.clone();
                                                    let on_item_click =
                                                        move |ev: web_sys::MouseEvent| {
                                                            ev.stop_propagation();
                                                            if !item_disabled {
                                                                si(iv.clone());
                                                            }
                                                        };

                                                    // Checkbox tick for multiple mode.
                                                    let chk: Option<AnyView> =
                                                        multiple.then(|| {
                                                            leptos::html::div()
                                                                .attr(
                                                                    "class",
                                                                    if item_selected {
                                                                        "notranslate rz-chkbox-box rz-state-active"
                                                                    } else {
                                                                        "notranslate rz-chkbox-box"
                                                                    },
                                                                )
                                                                .child(
                                                                    leptos::html::span().attr(
                                                                        "class",
                                                                        if item_selected {
                                                                            "notranslate rz-chkbox-icon rzi rzi-check"
                                                                        } else {
                                                                            "notranslate rz-chkbox-icon"
                                                                        },
                                                                    ),
                                                                )
                                                                .into_any()
                                                        });

                                                    leptos::html::li()
                                                        .attr("class", li_class)
                                                        .attr("role", "option")
                                                        .attr(
                                                            "aria-selected",
                                                            if item_selected { "true" } else { "false" },
                                                        )
                                                        .on(leptos::ev::click, on_item_click)
                                                        .child(chk)
                                                        .child(
                                                            leptos::html::span()
                                                                .child(item.label.clone()),
                                                        )
                                                        .into_any()
                                                })
                                                .collect_view(),
                                        ),
                                ),
                        ),
                )
                .into_any()
            }),
    )
    .into_any()
}