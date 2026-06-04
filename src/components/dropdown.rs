//! RadzenDropDown component — mirrors C# Radzen.Blazor.RadzenDropDown<TValue>.
//!
//! # CSS class order (mirrors Blazor exactly)
//! Root `<div>`: `rz-dropdown [rz-state-focused] [rz-state-disabled] [rz-state-empty] [caller-class]`
//!
//! Blazor `GetComponentCssClass()` (from DropDownBase):
//! ```csharp
//! GetClassList("rz-dropdown")
//!     .Add("rz-state-focused", focused)
//!     .ToString()
//! ```
//!
//! # Key implementation notes vs Blazor
//! Blazor uses JS interop (`Radzen.openPopup` / `Radzen.togglePopup`) which renders the panel
//! with `display:none` always in the DOM. We replicate by toggling an `open` signal and
//! conditionally rendering — but we must be very careful about the click/blur event ordering.
//!
//! ## Click handling — the critical fix
//! Blazor attaches `@onclick:preventDefault @onclick:stopPropagation` on the root `<div>`,
//! and calls `OpenPopup("ArrowDown", false, true)`. Item clicks inside the panel call
//! `ev.stop_propagation()` so they do NOT bubble up to the root toggle.
//!
//! In Leptos we mirror this by:
//! 1. Using `on:mousedown` (not `on:click`) on the root to toggle open/closed. `mousedown`
//!    fires before `blur`, ensuring we can prevent the blur from closing the panel.
//! 2. Using `on:mousedown:prevent_default` on the panel to stop the root from losing focus.
//! 3. Item clicks call `ev.stop_propagation()` so the root `mousedown` toggle does not fire.
//!
//! This eliminates the race condition where `blur` closes the panel before an item click
//! can register, and also eliminates the double-toggle caused by item clicks bubbling up.
//!
//! # Trigger element
//! Blazor: `<div class="rz-dropdown-trigger rz-corner-right">` (not a `<button>`).
//! The trigger is NOT a separate interactive element — the entire root div is clickable.
//!
//! # Selected value display
//! Blazor uses `<span class="rz-dropdown-label rz-inputtext">` (not `<label>`).
//! Using `<label>` would cause extra click events through the browser's label→input routing.
//!
//! # Panel visibility
//! Always rendered when `open` is true, removed when false. Panel has
//! `on:mousedown = prevent_default` to stop blur from closing it during item selection.
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

    /// Text shown for "Select All" in multi-select mode.
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

    // Visibility — mirrors `@if (Visible)`.
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── Internal state ────────────────────────────────────────────────────────
    let open = RwSignal::new(false);
    let filter_text = RwSignal::new(String::new());
    let effective_tab = if disabled { -1 } else { tab_index };

    // ── Static props ──────────────────────────────────────────────────────────
    let caller_class = base
        .attrs
        .as_ref()
        .and_then(|a| a.get("class"))
        .cloned()
        .unwrap_or_default();
    let style = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();

    // ── StoredValues for data used in reactive closures ───────────────────────
    let data_sv = StoredValue::new(data.clone());
    let filter_placeholder_sv = StoredValue::new(filter_placeholder);
    let select_all_text_sv = StoredValue::new(select_all_text);

    // ── selected_label — reactive closure ─────────────────────────────────────
    // Returns the display text for the selected value(s).
    // Mirrors Blazor's label display logic.
    let selected_label = move || -> String {
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
    };

    // ── is_selected — reads reactive signals ──────────────────────────────────
    let is_selected = move |item_value: &str| -> bool {
        if multiple {
            value_multiple
                .map(|s| s.get().iter().any(|v| v == item_value))
                .unwrap_or(false)
        } else {
            value.map(|s| s.get() == item_value).unwrap_or(false)
        }
    };

    // ── on_change callback wrapped in Arc ─────────────────────────────────────
    let on_change_cb = Arc::new(on_change);

    // ── select_item — called when a user clicks an item ───────────────────────
    // For single: sets value, closes panel.
    // For multiple: toggles item in/out of selection, keeps panel open.
    let on_change_cb_si = on_change_cb.clone();
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
                if let Some(ref cb) = **on_change_cb_si {
                    cb(value_multiple.unwrap().get().join(","));
                }
            }
        } else {
            if let Some(single_sig) = value {
                single_sig.set(item_value.clone());
            }
            if let Some(ref cb) = **on_change_cb_si {
                cb(item_value);
            }
            open.set(false);
            filter_text.set(String::new());
        }
    });

    // ── select_all ────────────────────────────────────────────────────────────
    let on_change_cb_sa = on_change_cb.clone();
    let data_sv_sa = StoredValue::new(data.clone());
    let select_all = Arc::new(move || {
        if let Some(multi_sig) = value_multiple {
            let all_vals: Vec<String> = data_sv_sa
                .get_value()
                .iter()
                .filter(|i| !i.disabled)
                .map(|i| i.value.clone())
                .collect();
            let currently_all = multi_sig.get().len() == all_vals.len();
            if currently_all {
                multi_sig.set(vec![]);
                if let Some(ref cb) = **on_change_cb_sa {
                    cb(String::new());
                }
            } else {
                multi_sig.set(all_vals.clone());
                if let Some(ref cb) = **on_change_cb_sa {
                    cb(all_vals.join(","));
                }
            }
        }
    });

    // ── Toggle open/close ─────────────────────────────────────────────────────
    // Uses `mousedown` on the root so it fires BEFORE `blur`.
    // This prevents the blur handler from closing the panel before this fires.
    // Mirrors Blazor's `@onclick="@(args => OpenPopup("ArrowDown", false, true))"`.
    let toggle = move |ev: web_sys::MouseEvent| {
        ev.prevent_default(); // prevent focus shift that would cause blur
        if disabled || read_only {
            return;
        }
        let currently_open = open.get_untracked();
        open.set(!currently_open);
        if currently_open {
            filter_text.set(String::new());
        }
    };

    // Close on blur — fires when focus moves completely outside the component.
    // We use a short delay so that a mousedown on an item can register first.
    // The panel's own `prevent_default` on mousedown prevents blur in most cases,
    // so this is a safety net for clicks outside the component.
    let on_blur = move |_ev: web_sys::FocusEvent| {
        if !inline_mode_is_always_open() {
            gloo_timers::callback::Timeout::new(150, move || {
                open.set(false);
                filter_text.set(String::new());
            })
            .forget();
        }
    };

    // ── Keyboard navigation — mirrors Blazor OnKeyPress ───────────────────────
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
            "ArrowDown" => {
                if !open.get_untracked() {
                    open.set(true);
                }
            }
            _ => {}
        }
    };

    // Base mouse event handlers from use_radzen_base.
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();

    Some(
        leptos::html::div()
            .attr("id", handle_id)
            .attr("style", style)
            .attr("tabindex", effective_tab.to_string())
            .attr("role", "combobox")
            .attr("aria-haspopup", "listbox")
            .attr("aria-expanded", move || {
                if open.get() { "true" } else { "false" }
            })
            .attr("aria-disabled", if disabled { "true" } else { "false" })
            // Root CSS class — reactive so open/empty/disabled state updates.
            .attr("class", move || {
                let is_empty = if multiple {
                    value_multiple.map(|s| s.get().is_empty()).unwrap_or(true)
                } else {
                    value.map(|s| s.get().is_empty()).unwrap_or(true)
                };
                ClassList::create("rz-dropdown")
                    .add("rz-state-focused", open.get())
                    .add_disabled(disabled)
                    .add("rz-state-empty", is_empty)
                    .add_caller_class(if caller_class.is_empty() {
                        None
                    } else {
                        Some(caller_class.as_str())
                    })
                    .finish()
            })
            // Use mousedown on the root to toggle — fires before blur.
            .on(leptos::ev::mousedown, toggle)
            .on(leptos::ev::blur, on_blur)
            .on(leptos::ev::keydown, on_keydown)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            // ── Hidden accessible input ───────────────────────────────────────
            // Mirrors Blazor's <div class="rz-helper-hidden-accessible"><input …/>
            .child(
                leptos::html::div()
                    .attr("class", "rz-helper-hidden-accessible")
                    .child(
                        leptos::html::input()
                            .attr("type", "text")
                            .attr("name", name.clone())
                            .attr("readonly", true)
                            .attr("tabindex", "-1")
                            .attr("aria-haspopup", "listbox")
                            .attr("aria-expanded", move || {
                                if open.get() { "true" } else { "false" }
                            })
                            .prop("value", move || selected_label()),
                    ),
            )
            // ── Selected value label — mirrors Blazor's <span class="rz-dropdown-label rz-inputtext"> ──
            // NOTE: Blazor uses <span>, NOT <label>. Using <label> would cause click routing to
            // an associated <input> element, producing spurious events.
            .child(move || {
                let lbl = selected_label();
                if lbl.is_empty() {
                    // Placeholder — mirrors: <span class="rz-dropdown-label rz-inputtext rz-placeholder">
                    if let Some(ref ph) = placeholder {
                        leptos::html::span()
                            .attr("class", "rz-dropdown-label rz-inputtext rz-placeholder")
                            .child(ph.clone())
                            .into_any()
                    } else {
                        leptos::html::span()
                            .attr("class", "rz-dropdown-label rz-inputtext")
                            .child("\u{00a0}") // &nbsp;
                            .into_any()
                    }
                } else {
                    leptos::html::span()
                        .attr("class", "rz-dropdown-label rz-inputtext")
                        .child(lbl)
                        .into_any()
                }
            })
            // ── Trigger chevron — mirrors Blazor's <div class="rz-dropdown-trigger rz-corner-right"> ──
            // This is a <div>, NOT a <button>. The entire root div is the click target.
            .child(
                leptos::html::div()
                    .attr("class", "rz-dropdown-trigger rz-corner-right")
                    .child(
                        leptos::html::span().attr(
                            "class",
                            "notranslate rz-dropdown-trigger-icon rzi rzi-chevron-down",
                        ),
                    ),
            )
            // ── Panel ─────────────────────────────────────────────────────────
            // Rendered when open=true. `prevent_default` on mousedown prevents
            // the root from losing focus (which would trigger blur and close the panel)
            // when the user clicks inside the panel.
            //
            // Blazor panel class:
            //   single: "rz-dropdown-panel"
            //   multiple: "rz-multiselect-panel"
            .child(move || {
                if !open.get() {
                    return None::<AnyView>.into_any();
                }

                let panel_class = if multiple {
                    "rz-multiselect-panel"
                } else {
                    "rz-dropdown-panel"
                };

                let items = data_sv.get_value();
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

                // ── Header row (multiple mode: AllowSelectAll / AllowFiltering) ──
                // Mirrors Blazor's `@if (Multiple && (AllowSelectAll || AllowFiltering))` block.
                // The header contains the select-all checkbox and/or the filter input.
                let header_child: Option<AnyView> = multiple.then(|| {
                    let sa = select_all.clone();
                    let all_item_count = items.iter().filter(|i| !i.disabled).count();
                    let is_all = value_multiple
                        .map(|s| {
                            let sel = s.get();
                            !sel.is_empty() && sel.len() == all_item_count
                        })
                        .unwrap_or(false);

                    let chkbox_box_class = if is_all {
                        "notranslate rz-chkbox-box rz-state-active"
                    } else {
                        "notranslate rz-chkbox-box"
                    };
                    let chkbox_icon_class = if is_all {
                        "notranslate rz-chkbox-icon rzi rzi-check"
                    } else {
                        "notranslate rz-chkbox-icon"
                    };

                    // Mirrors Blazor's `<div class="rz-multiselect-header rz-helper-clearfix">` block.
                    let mut header = leptos::html::div()
                        .attr("class", "rz-multiselect-header rz-helper-clearfix");

                    if allow_select_all && !allow_filtering {
                        header = header
                            // Checkbox
                            .child(
                                leptos::html::div()
                                    .attr("class", "rz-chkbox")
                                    .attr("role", "checkbox")
                                    .attr("aria-checked", if is_all { "true" } else { "false" })
                                    .on(leptos::ev::click, {
                                        let sa2 = sa.clone();
                                        move |ev: web_sys::MouseEvent| {
                                            ev.stop_propagation();
                                            sa2();
                                        }
                                    })
                                    .child(
                                        leptos::html::div()
                                            .attr("class", chkbox_box_class)
                                            .child(
                                                leptos::html::span()
                                                    .attr("class", chkbox_icon_class),
                                            ),
                                    ),
                            )
                            // "Select All" button label
                            .child(
                                leptos::html::button()
                                    .attr("type", "button")
                                    .attr("class", "rz-multiselect-selectall")
                                    .attr("disabled", disabled || read_only)
                                    .on(leptos::ev::click, move |ev: web_sys::MouseEvent| {
                                        ev.stop_propagation();
                                        sa();
                                    })
                                    .child(select_all_text_sv.get_value()),
                            );
                    }

                    if allow_filtering {
                        header = header.child(
                            leptos::html::div()
                                .attr("class", "rz-multiselect-filter-container")
                                .child(
                                    leptos::html::input()
                                        .attr("type", "text")
                                        .attr("class", "rz-inputtext")
                                        .attr("placeholder", filter_placeholder_sv.get_value())
                                        .prop("value", move || filter_text.get())
                                        .on(
                                            leptos::ev::input,
                                            move |ev: web_sys::Event| {
                                                use web_sys::wasm_bindgen::JsCast;
                                                if let Some(input) = ev.target().and_then(|t| {
                                                    t.dyn_into::<web_sys::HtmlInputElement>().ok()
                                                }) {
                                                    filter_text.set(input.value());
                                                }
                                            },
                                        )
                                        .on(leptos::ev::click, |ev: web_sys::MouseEvent| {
                                            ev.stop_propagation();
                                        })
                                        .on(leptos::ev::mousedown, |ev: web_sys::MouseEvent| {
                                            ev.stop_propagation();
                                        }),
                                )
                                .child(
                                    leptos::html::span().attr(
                                        "class",
                                        "notranslate rz-multiselect-filter-icon rzi rzi-search",
                                    ),
                                ),
                        );
                    }

                    header.into_any()
                });

                // ── Single-mode filter header ──────────────────────────────────
                // Mirrors Blazor: `@if(!Multiple && AllowFiltering)`.
                let single_filter_child: Option<AnyView> = (!multiple && allow_filtering).then(|| {
                    leptos::html::div()
                        .attr("class", "rz-dropdown-filter-container")
                        .child(
                            leptos::html::input()
                                .attr("type", "text")
                                .attr("class", "rz-dropdown-filter rz-inputtext")
                                .attr("placeholder", filter_placeholder_sv.get_value())
                                .attr("autocomplete", "off")
                                .attr("aria-autocomplete", "none")
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
                                })
                                .on(leptos::ev::mousedown, |ev: web_sys::MouseEvent| {
                                    ev.stop_propagation();
                                }),
                        )
                        .child(
                            leptos::html::span()
                                .attr("class", "notranslate rz-dropdown-filter-icon rzi rzi-search"),
                        )
                        .into_any()
                });

                // ── Items list ────────────────────────────────────────────────
                // Mirrors Blazor:
                //   single: <div class="rz-dropdown-items-wrapper"> <ul class="rz-dropdown-items rz-dropdown-list">
                //   multiple: <div class="rz-multiselect-items-wrapper"> <ul class="rz-multiselect-items rz-multiselect-list">
                let (items_wrapper_class, items_list_class) = if multiple {
                    ("rz-multiselect-items-wrapper", "rz-multiselect-items rz-multiselect-list")
                } else {
                    ("rz-dropdown-items-wrapper", "rz-dropdown-items rz-dropdown-list")
                };

                let item_views: Vec<AnyView> = filtered
                    .into_iter()
                    .map(|item| {
                        let item_selected = is_selected(&item.value);
                        let item_disabled = item.disabled;

                        // Item CSS — mirrors Blazor's GetItemCssClass():
                        //   single:   "rz-dropdown-item [rz-state-highlight] [rz-state-disabled]"
                        //   multiple: "rz-multiselect-item [rz-state-highlight] [rz-state-disabled]"
                        let li_class = if multiple {
                            ClassList::create("rz-multiselect-item")
                                .add("rz-state-highlight", item_selected)
                                .add_disabled(item_disabled)
                                .finish()
                        } else {
                            ClassList::create("rz-dropdown-item")
                                .add("rz-state-highlight", item_selected)
                                .add_disabled(item_disabled)
                                .finish()
                        };

                        let iv = item.value.clone();
                        let si = select_item.clone();

                        // Checkbox for multiple mode — mirrors Blazor's
                        // <div class="rz-chkbox-box [rz-state-active]"> …
                        let chk_child: Option<AnyView> = multiple.then(|| {
                            leptos::html::div()
                                .attr("class", "rz-chkbox rz-nofilter")
                                .child(
                                    leptos::html::div()
                                        .attr("class", "rz-helper-hidden-accessible")
                                        .child(
                                            leptos::html::input()
                                                .attr("type", "checkbox")
                                                .attr("readonly", true)
                                                .prop("checked", item_selected),
                                        ),
                                )
                                .child(
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
                                        ),
                                )
                                .into_any()
                        });

                        leptos::html::li()
                            .attr("class", li_class)
                            .attr("role", "option")
                            .attr("aria-selected", if item_selected { "true" } else { "false" })
                            // CRITICAL: stop_propagation prevents the click from bubbling to the
                            // root div's mousedown handler, which would toggle the panel closed.
                            .on(leptos::ev::mousedown, move |ev: web_sys::MouseEvent| {
                                ev.stop_propagation(); // don't bubble to root toggle
                                ev.prevent_default();  // don't trigger blur on root
                                if !item_disabled {
                                    si(iv.clone());
                                }
                            })
                            .child(chk_child)
                            .child(
                                leptos::html::span().child(item.label.clone()),
                            )
                            .into_any()
                    })
                    .collect();

                Some(
                    leptos::html::div()
                        .attr("class", panel_class)
                        // CRITICAL: prevent_default on mousedown so the root div does NOT lose
                        // focus and does NOT trigger `blur` while the user is interacting with
                        // the panel. This is the equivalent of Blazor's JS popup which keeps
                        // the panel alive regardless of focus changes.
                        .on(leptos::ev::mousedown, |ev: web_sys::MouseEvent| {
                            ev.prevent_default();
                        })
                        .child(single_filter_child)
                        .child(header_child)
                        .child(
                            leptos::html::div()
                                .attr("class", items_wrapper_class)
                                .child(
                                    leptos::html::ul()
                                        .attr("class", items_list_class)
                                        .attr("role", "listbox")
                                        .attr(
                                            "aria-multiselectable",
                                            if multiple { "true" } else { "false" },
                                        )
                                        .child(item_views),
                                ),
                        ),
                )
                .into_any()
            }),
    )
    .into_any()
}

// Helper — not a real inline mode, just a placeholder to silence the compiler.
// Inline mode doesn't exist on dropdown (only on DatePicker).
#[inline(always)]
fn inline_mode_is_always_open() -> bool {
    false
}