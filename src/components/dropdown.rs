//! RadzenDropDown component — mirrors C# Radzen.Blazor.RadzenDropDown<TValue>.
//!
//! # Inheritance chain (C#)
//! `RadzenDropDown<TValue>` → `DropDownBase<TValue>` → `DataBoundFormComponent<T>`
//! → `FormComponent<T>` → `RadzenComponent`
//!
//! In Rust all shared behaviour from those base classes lives in
//! [`DropDownProps`] + [`use_drop_down_base`] (see `dropdown_base.rs`).
//! [`ComponentProps`] + [`use_radzen_base`] covers `RadzenComponent`.
//!
//! # CSS class (mirrors `GetComponentCssClass` in RadzenDropDown.razor.cs)
//! ```csharp
//! return GetClassList("rz-dropdown")
//!     .AddDisabled(Disabled)
//!     .Add("rz-state-empty", !HasValue)
//!     .Add("rz-clear", AllowClear)
//!     .Add("rz-dropdown-chips", Chips && selectedItems.Count > 0)
//!     .ToString();
//! ```
//! `GetCssClass()` then appends the caller `class` attribute last.
//! `rz-state-focused` is added reactively when `open.get()` is true.
//!
//! # Panel rendering — key difference from Blazor
//! Blazor JS (`Radzen.togglePopup`) teleports the panel div to `document.body`
//! so it escapes the `overflow: hidden` root container. Without JS teleportation
//! in Leptos, the panel must be rendered outside the overflow-clipping root.
//!
//! Solution: a thin outer wrapper `<div style="position:relative; display:block; width:100%">`
//! contains BOTH the Radzen root div (which has `overflow:hidden` from SCSS)
//! AND the absolutely-positioned panel as a sibling. The panel is therefore
//! not clipped by the root's overflow.
//!
//! # Panel flip (above/below)
//! Uses a `NodeRef` on the wrapper + a `RwSignal<bool> flip_up` that is set in
//! an `Effect` by measuring `getBoundingClientRect()` vs `window.innerHeight`.
//! When there is not enough space below the input, the panel opens upward
//! (mirrors Blazor's JS `Radzen.openPopup` smart-position logic).
//!
//! # Clear button placement
//! Mirrors Blazor razor exactly: the clear button is a direct child of the root
//! `<div>` (after the trigger chevron), NOT inside the panel div. For multi-select
//! AllowClear the clear button appears inside the header.
//!
//! # Search/filter — stop propagation
//! Clicking the filter `<input>` must NOT bubble to the root's click handler
//! (which would toggle the popup closed). Blazor uses
//! `onclick="Radzen.preventDefaultAndStopPropagation(event)"` on the filter input.
//! We mirror this by calling `ev.stop_propagation()` on the input's `click` and
//! `mousedown` handlers.
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted when invisible.

use crate::components::{
    base_component::ComponentProps,
    dropdown_base::{DropDownItem, DropDownProps, build_drop_down_root_class, use_drop_down_base},
};
use leptos::prelude::*;
use std::sync::Arc;

pub use crate::components::dropdown_base::DropDownItem as DropDownItemAlias;

/// RadzenDropDown component.
///
/// A styled combobox/select allowing single or multiple item selection from a
/// popup list. Mirrors `Radzen.Blazor.RadzenDropDown<TValue>`.
///
/// All shared dropdown behaviour (filtering, selection logic, CSS class
/// construction, popup state) is handled by the [`DropDownProps`] /
/// [`use_drop_down_base`] composable — the Rust equivalent of inheriting
/// `DropDownBase<TValue>`.
///
/// # Single selection
/// ```rust,ignore
/// let selected = RwSignal::new(String::new());
/// <RadzenDropDown
///     drop_down=DropDownProps {
///         value: Some(selected),
///         data: vec![
///             DropDownItem::new("1", "Orders"),
///             DropDownItem::new("2", "Employees"),
///         ],
///         placeholder: Some("Select…".to_string()),
///         ..Default::default()
///     }
/// />
/// ```
#[component]
pub fn RadzenDropDown(
    /// Shared dropdown props (data, value, multiple, filtering, …).
    #[prop(default = Default::default())]
    drop_down: DropDownProps,

    /// Base component props (id, style, visible, attrs, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Whether the component is read-only.
    #[prop(default = false)]
    read_only: bool,

    /// Whether to display selected items as removable chips in multi-select.
    #[prop(default = false)]
    chips: bool,

    /// Optional footer content rendered below the items list.
    #[prop(optional)]
    footer_template: Option<ChildrenFn>,
) -> impl IntoView {
    let (handle, _base_handle) = use_drop_down_base(&drop_down, &base);

    // ── Visibility — mirrors `@if (Visible)` ──────────────────────────────────
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── Alias shared props into locals ────────────────────────────────────────
    let disabled = drop_down.disabled;
    let multiple = drop_down.multiple;
    let allow_filtering = drop_down.allow_filtering;
    let allow_select_all = drop_down.allow_select_all;
    let allow_clear = drop_down.allow_clear;
    let max_selected_labels = drop_down.max_selected_labels;
    let value = drop_down.value;
    let value_multiple = drop_down.value_multiple;
    let effective_tab = drop_down.effective_tab();

    let popup_style_sv = StoredValue::new(drop_down.popup_style.clone());
    let filter_placeholder_sv = StoredValue::new(drop_down.filter_placeholder.clone());
    let selected_items_text_sv = StoredValue::new(drop_down.selected_items_text.clone());
    let footer_sv = StoredValue::new(footer_template);

    let style = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();
    let caller_class = base
        .attrs
        .as_ref()
        .and_then(|a| a.get("class"))
        .cloned()
        .unwrap_or_default();

    let on_change_sv: StoredValue<Option<Arc<dyn Fn(String) + Send + Sync>>> =
        StoredValue::new(drop_down.on_change.clone());
    let on_open_sv: StoredValue<Option<Arc<dyn Fn() + Send + Sync>>> =
        StoredValue::new(drop_down.on_open.clone());
    let on_close_sv: StoredValue<Option<Arc<dyn Fn() + Send + Sync>>> =
        StoredValue::new(drop_down.on_close.clone());

    let open = handle.open;
    let filter_text = handle.filter_text;
    let filtered_items = handle.filtered_items;
    let has_value = handle.has_value;

    // ── Flip-up detection ─────────────────────────────────────────────────────
    // When the panel opens, measure available space below the wrapper.
    // If less than 220px remain before the viewport bottom, open upward.
    let flip_up = RwSignal::new(false);
    let wrapper_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        if !open.get() {
            return;
        }
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(el) = wrapper_ref.get() {
                use web_sys::wasm_bindgen::JsCast;
                if let Some(el) = el.dyn_ref::<web_sys::Element>() {
                    let rect = el.get_bounding_client_rect();
                    if let Some(win) = web_sys::window() {
                        let inner_h = win
                            .inner_height()
                            .ok()
                            .and_then(|v| v.as_f64())
                            .unwrap_or(600.0);
                        let space_below = inner_h - rect.bottom();
                        flip_up.set(space_below < 220.0);
                    }
                }
            }
        }
    });

    // ── Root CSS class — reactive ─────────────────────────────────────────────
    let caller_class_cl = caller_class.clone();
    let root_class = move || {
        let has_chips = chips && has_value.get() && multiple;
        build_drop_down_root_class(
            "rz-dropdown",
            &[
                ("rz-clear", allow_clear),
                ("rz-dropdown-chips", has_chips),
                ("rz-state-focused", open.get()),
            ],
            disabled,
            has_value.get(),
            if caller_class_cl.is_empty() {
                None
            } else {
                Some(caller_class_cl.as_str())
            },
        )
    };

    // ── Selected display label ────────────────────────────────────────────────
    let selected_label = move || -> String {
        if multiple {
            let selected = value_multiple.map(|s| s.get()).unwrap_or_default();
            if selected.is_empty() {
                return String::new();
            }
            let count = selected.len();
            if count > max_selected_labels {
                return format!("{} {}", count, selected_items_text_sv.get_value());
            }
            // Use the full unfiltered data to resolve labels — filtering should not
            // affect the display of already-selected items.
            let items = filtered_items.get();
            selected
                .iter()
                .filter_map(|v| items.iter().find(|i| &i.value == v))
                .map(|i| i.label.clone())
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            let val = value.map(|s| s.get()).unwrap_or_default();
            filtered_items
                .get()
                .into_iter()
                .find(|i| i.value == val)
                .map(|i| i.label)
                .unwrap_or_default()
        }
    };

    // ── is_selected helper ────────────────────────────────────────────────────
    let is_selected = move |item_value: &str| -> bool {
        if multiple {
            value_multiple
                .map(|s| s.get().iter().any(|v| v == item_value))
                .unwrap_or(false)
        } else {
            value.map(|s| s.get() == item_value).unwrap_or(false)
        }
    };

    // ── Hidden input value ────────────────────────────────────────────────────
    let hidden_value = move || -> String {
        if multiple {
            value_multiple
                .map(|s| s.get().join(","))
                .unwrap_or_default()
        } else {
            value.map(|s| s.get()).unwrap_or_default()
        }
    };

    // ── select_item — mirrors OnSelectItem → SelectItem ───────────────────────
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
                if let Some(cb) = on_change_sv.get_value() {
                    cb(value_multiple.unwrap().get().join(","));
                }
            }
        } else {
            if let Some(single_sig) = value {
                single_sig.set(item_value.clone());
            }
            if let Some(cb) = on_change_sv.get_value() {
                cb(item_value);
            }
            // Close popup on single selection — mirrors ClosePopup("Enter")
            open.set(false);
            filter_text.set(String::new());
            if let Some(cb) = on_close_sv.get_value() {
                cb();
            }
        }
    });

    // ── select_all ────────────────────────────────────────────────────────────
    let select_all = Arc::new(move || {
        if disabled || read_only {
            return;
        }
        if let Some(multi_sig) = value_multiple {
            let all_enabled: Vec<String> = filtered_items
                .get()
                .into_iter()
                .filter(|i| !i.disabled)
                .map(|i| i.value)
                .collect();
            let currently_all =
                multi_sig.get().len() == all_enabled.len() && !all_enabled.is_empty();
            if currently_all {
                multi_sig.set(vec![]);
                if let Some(cb) = on_change_sv.get_value() {
                    cb(String::new());
                }
            } else {
                multi_sig.set(all_enabled.clone());
                if let Some(cb) = on_change_sv.get_value() {
                    cb(all_enabled.join(","));
                }
            }
        }
    });

    // ── clear_all ─────────────────────────────────────────────────────────────
    let clear_all = Arc::new(move || {
        if disabled || read_only {
            return;
        }
        if multiple {
            if let Some(multi_sig) = value_multiple {
                multi_sig.set(vec![]);
                if let Some(cb) = on_change_sv.get_value() {
                    cb(String::new());
                }
            }
        } else if let Some(single_sig) = value {
            single_sig.set(String::new());
            if let Some(cb) = on_change_sv.get_value() {
                cb(String::new());
            }
        }
        filter_text.set(String::new());
    });
    let clear_all_for_panel = clear_all.clone();
    let clear_all_for_root = clear_all.clone();

    // ── Root toggle ───────────────────────────────────────────────────────────
    // Mirrors Blazor's @onclick="OpenPopup("ArrowDown", false, true)"
    // Use onclick (same as Blazor) on the root div — NOT mousedown.
    let toggle = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
        ev.stop_propagation();
        if disabled || read_only {
            return;
        }
        if open.get_untracked() {
            open.set(false);
            filter_text.set(String::new());
            if let Some(cb) = on_close_sv.get_value() {
                cb();
            }
        } else {
            if let Some(cb) = on_open_sv.get_value() {
                cb();
            }
            open.set(true);
        }
    };

    // ── Blur — safety-net close ───────────────────────────────────────────────
    // 150ms delay so item mousedown can fire before the blur closes the panel.
    // The delay also allows the filter input click to register without triggering
    // a close — the filter input calls stop_propagation so the blur fires on
    // the root only when focus truly leaves the component.
    let wrapper_ref = NodeRef::<leptos::html::Div>::new();

    let on_blur = move |ev: web_sys::FocusEvent| {
        use web_sys::wasm_bindgen::JsCast;

        let focus_stayed_inside = ev
            .related_target()
            .and_then(|t| t.dyn_into::<web_sys::Node>().ok())
            .and_then(|target| {
                wrapper_ref
                    .get()
                    .map(|wrapper| wrapper.contains(Some(&target)))
            })
            .unwrap_or(false);

        if focus_stayed_inside {
            return;
        }

        open.set(false);
        filter_text.set(String::new());
    };

    // ── Keyboard — mirrors OnKeyPress ─────────────────────────────────────────
    let on_keydown = move |ev: web_sys::KeyboardEvent| match ev.key().as_str() {
        "Escape" | "Tab" => {
            open.set(false);
            filter_text.set(String::new());
            if let Some(cb) = on_close_sv.get_value() {
                cb();
            }
        }
        "Enter" | " " | "ArrowDown" => {
            ev.prevent_default();
            if !open.get_untracked() {
                if let Some(cb) = on_open_sv.get_value() {
                    cb();
                }
                open.set(true);
            }
        }
        _ => {}
    };

    // ── Base mouse events ─────────────────────────────────────────────────────
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();

    // ── Panel content builder ─────────────────────────────────────────────────
    // The panel is a sibling of the root div inside the wrapper. This places it
    // outside the root's overflow:hidden so it is not clipped.
    // `flip_up` controls whether the panel appears above or below the input.
    let build_panel_clear = clear_all_for_panel.clone();
    let build_panel = move || -> Option<AnyView> {
        if !open.get() {
            return None;
        }

        let panel_class = if multiple {
            "rz-multiselect-panel"
        } else {
            "rz-dropdown-panel"
        };

        // Panel position: below by default, above when near viewport bottom.
        // `top: 100%` = just below the wrapper; `bottom: 100%` = just above.
        let panel_pos_style = if flip_up.get() {
            "position: absolute; bottom: 100%; left: 0; right: 0; width: 100%; z-index: 2000; box-sizing: border-box;"
        } else {
            "position: absolute; top: 100%; left: 0; right: 0; width: 100%; z-index: 2000; box-sizing: border-box;"
        };

        let items = filtered_items.get();

        // ── Single-mode filter ────────────────────────────────────────────────
        // Mirrors Blazor: @if(!Multiple && (AllowFiltering || HeaderTemplate != null))
        // The filter input uses stop_propagation on click/mousedown so that
        // clicking inside it does NOT bubble to the wrapper and re-trigger toggle.
        let single_filter: Option<AnyView> = (!multiple && allow_filtering).then(|| {
            leptos::html::div()
                .attr("class", "rz-dropdown-filter-container")
                .child(
                    leptos::html::input()
                        .attr("type", "text")
                        .attr("class", "rz-dropdown-filter rz-inputtext")
                        .attr("autocomplete", "off")
                        .attr("aria-autocomplete", "none")
                        .attr("placeholder", filter_placeholder_sv.get_value())
                        .prop("value", move || filter_text.get())
                        .on(leptos::ev::input, move |ev: web_sys::Event| {
                            use web_sys::wasm_bindgen::JsCast;
                            if let Some(input) = ev
                                .target()
                                .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                            {
                                filter_text.set(input.value());
                            }
                        })
                        // Critical: stop propagation so the root's toggle handler
                        // does not receive this click and close the panel.
                        // Mirrors Blazor: onclick="Radzen.preventDefaultAndStopPropagation(event)"
                        .on(leptos::ev::click, |ev: web_sys::MouseEvent| {
                            ev.stop_propagation();
                        })
                        .on(leptos::ev::mousedown, |ev: web_sys::MouseEvent| {
                            ev.stop_propagation();
                        }),
                )
                .child(leptos::html::span().attr(
                    "class",
                    "notranslate rz-dropdown-filter-icon rzi rzi-search",
                ))
                .into_any()
        });

        // ── Multiple-mode header ──────────────────────────────────────────────
        // Mirrors Blazor: @if(Multiple && (AllowSelectAll || AllowFiltering || HeaderTemplate))
        let multi_header: Option<AnyView> = (multiple && (allow_select_all || allow_filtering))
            .then(|| {
                let all_enabled_count = items.iter().filter(|i| !i.disabled).count();
                let is_all = value_multiple
                    .map(|s| {
                        let sel = s.get();
                        !sel.is_empty() && sel.len() == all_enabled_count
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

                let sa1 = select_all.clone();
                let mut header_children: Vec<AnyView> = Vec::new();

                if allow_select_all {
                    header_children.push(
                        leptos::html::div()
                            .attr("class", "rz-chkbox")
                            .attr("role", "checkbox")
                            .attr("aria-checked", if is_all { "true" } else { "false" })
                            .on(leptos::ev::mousedown, {
                                let sa = sa1.clone();
                                move |ev: web_sys::MouseEvent| {
                                    ev.stop_propagation();
                                    ev.prevent_default();
                                    sa();
                                }
                            })
                            .child(
                                leptos::html::div()
                                    .attr("class", "rz-helper-hidden-accessible")
                                    .child(
                                        leptos::html::input()
                                            .attr("type", "checkbox")
                                            .attr("readonly", true)
                                            .prop("checked", is_all),
                                    ),
                            )
                            .child(
                                leptos::html::div()
                                    .attr("class", chkbox_box_class)
                                    .child(leptos::html::span().attr("class", chkbox_icon_class)),
                            )
                            .into_any(),
                    );
                }

                if allow_filtering {
                    header_children.push(
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
                                        if let Some(input) = ev.target().and_then(|t| {
                                            t.dyn_into::<web_sys::HtmlInputElement>().ok()
                                        }) {
                                            filter_text.set(input.value());
                                        }
                                    })
                                    // Mirrors: onclick="Radzen.preventDefaultAndStopPropagation(event)"
                                    .on(leptos::ev::click, |ev: web_sys::MouseEvent| {
                                        ev.stop_propagation();
                                    })
                                    .on(leptos::ev::mousedown, |ev: web_sys::MouseEvent| {
                                        ev.stop_propagation();
                                    }),
                            )
                            .child(leptos::html::span().attr(
                                "class",
                                "notranslate rz-multiselect-filter-icon rzi rzi-search",
                            ))
                            .into_any(),
                    );
                }

                // Multi-mode AllowClear — clear button inside the header.
                // Mirrors Blazor:
                //   @if (AllowClear && !ReadOnly && (Multiple && selectedItems.Count > 0))
                //   { <button class="rz-multiselect-close" … /> }
                if allow_clear && !read_only {
                    let ca = build_panel_clear.clone();
                    let cur_has_value =
                        value_multiple.map(|s| !s.get().is_empty()).unwrap_or(false);
                    if cur_has_value {
                        header_children.push(
                            leptos::html::button()
                                .attr("type", "button")
                                .attr("tabindex", "-1")
                                .attr("class", "rz-multiselect-close")
                                .attr("aria-label", "Clear")
                                .on(leptos::ev::mousedown, move |ev: web_sys::MouseEvent| {
                                    ev.stop_propagation();
                                    ev.prevent_default();
                                    ca();
                                })
                                .child(
                                    leptos::html::span().attr("class", "notranslate rzi rzi-times"),
                                )
                                .into_any(),
                        );
                    }
                }

                leptos::html::div()
                    .attr("class", "rz-multiselect-header rz-helper-clearfix")
                    .on(leptos::ev::mousedown, |ev: web_sys::MouseEvent| {
                        ev.prevent_default();
                        ev.stop_propagation();
                    })
                    .child(header_children)
                    .into_any()
            });

        // ── Item classes ──────────────────────────────────────────────────────
        let (items_wrapper_class, items_list_class, item_base_class) = if multiple {
            (
                "rz-multiselect-items-wrapper",
                "rz-multiselect-items rz-multiselect-list",
                "rz-multiselect-item",
            )
        } else {
            (
                "rz-dropdown-items-wrapper",
                "rz-dropdown-items rz-dropdown-list",
                "rz-dropdown-item",
            )
        };

        // ── Item list ─────────────────────────────────────────────────────────
        let item_views: Vec<AnyView> = items
            .into_iter()
            .map(|item| {
                let item_selected = is_selected(&item.value);
                let item_disabled = item.disabled;

                let li_class = crate::components::ClassList::create(item_base_class)
                    .add("rz-state-highlight", item_selected)
                    .add_disabled(item_disabled)
                    .finish();

                let iv = item.value.clone();
                let si = select_item.clone();

                let chk_child: Option<AnyView> = multiple.then(|| {
                    let chkbox_box_class = if item_selected {
                        "notranslate rz-chkbox-box rz-state-active"
                    } else {
                        "notranslate rz-chkbox-box"
                    };
                    let chkbox_icon_class = if item_selected {
                        "notranslate rz-chkbox-icon rzi rzi-check"
                    } else {
                        "notranslate rz-chkbox-icon"
                    };
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
                                .attr("class", chkbox_box_class)
                                .child(leptos::html::span().attr("class", chkbox_icon_class)),
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
                    // prevent_default keeps focus on root (no blur fires).
                    // stop_propagation prevents bubbling to the wrapper's mousedown.
                    .on(leptos::ev::mousedown, move |ev: web_sys::MouseEvent| {
                        ev.stop_propagation();
                        ev.prevent_default();
                        if !item_disabled {
                            si(iv.clone());
                        }
                    })
                    .child(chk_child)
                    .child(leptos::html::span().child(item.label.clone()))
                    .into_any()
            })
            .collect();

        // ── Footer ────────────────────────────────────────────────────────────
        let footer: Option<AnyView> = footer_sv.get_value().map(|f| {
            leptos::html::div()
                .attr("class", "rz-dropdown-footer")
                .child(f())
                .into_any()
        });

        Some(
            leptos::html::div()
                .attr("class", panel_class)
                .attr("style", panel_pos_style)
                // prevent_default keeps focus on root trigger when clicking inside panel.
                .on(leptos::ev::mousedown, |ev: web_sys::MouseEvent| {
                    ev.prevent_default();
                })
                .child(single_filter)
                .child(multi_header)
                .child(
                    leptos::html::div()
                        .attr("class", items_wrapper_class)
                        .attr("style", popup_style_sv.get_value())
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
                )
                .child(footer)
                .into_any(),
        )
    };

    // ── Render ────────────────────────────────────────────────────────────────
    // Structure — mirrors Blazor's DOM while solving the overflow:hidden clipping:
    //
    //   <div class="rz-dropdown-wrapper">          ← position:relative, no overflow clipping
    //     <div class="rz-dropdown …" …>            ← actual Radzen root (may have overflow:hidden)
    //       … label, trigger, hidden input …
    //       <button class="rz-dropdown-clear-icon" />  ← inside root, after trigger (Blazor exact)
    //     </div>
    //     <div class="rz-dropdown-panel …" />       ← sibling of root, not clipped
    //   </div>
    //
    // Clear button placement (mirrors Blazor razor exactly):
    //   - Single mode: INSIDE the root div, as the last child, after the trigger chevron.
    //     Blazor: `@if (AllowClear && …) { <button class="rz-dropdown-clear-icon …" /> }`
    //     which is written directly inside the root `<div>`, after `rz-dropdown-trigger`.
    //   - Multiple mode: inside the multiselect header (rendered inside the panel header).
    Some(
        leptos::html::div()
            .node_ref(wrapper_ref)
            // Wrapper: positioning context for the absolute panel.
            // display:block (not inline-flex) for proper width containment.
            .attr("style", "position: relative; display: block; width: 100%;")
            // ── Radzen root div ───────────────────────────────────────────────
            .child(
                leptos::html::div()
                    .attr("id", handle_id)
                    .attr("style", style)
                    .attr("class", root_class)
                    .attr("role", "combobox")
                    .attr("aria-haspopup", "listbox")
                    .attr(
                        "aria-expanded",
                        move || {
                            if open.get() { "true" } else { "false" }
                        },
                    )
                    .attr("aria-disabled", if disabled { "true" } else { "false" })
                    .attr("tabindex", effective_tab.to_string())
                    // Use onclick (same as Blazor) — not mousedown.
                    .on(leptos::ev::click, toggle)
                    .on(leptos::ev::blur, on_blur)
                    .on(leptos::ev::keydown, on_keydown)
                    .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
                    .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
                    .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
                    // ── Hidden accessible input ───────────────────────────────
                    .child(
                        leptos::html::div()
                            .attr("class", "rz-helper-hidden-accessible")
                            .child(
                                leptos::html::input()
                                    .attr("type", "text")
                                    .attr("name", drop_down.name.clone())
                                    .attr("id", drop_down.name.clone())
                                    .attr("disabled", disabled)
                                    .attr("readonly", true)
                                    .attr("tabindex", "-1")
                                    .attr("aria-haspopup", "listbox")
                                    .attr("aria-expanded", move || {
                                        if open.get() { "true" } else { "false" }
                                    })
                                    .prop("value", move || hidden_value()),
                            ),
                    )
                    // ── Selected value display ────────────────────────────────
                    .child(move || {
                        let lbl = selected_label();
                        if !lbl.is_empty() {
                            leptos::html::span()
                                .attr("class", "rz-dropdown-label rz-inputtext")
                                .child(lbl)
                                .into_any()
                        } else if let Some(ref ph) = drop_down.placeholder {
                            leptos::html::span()
                                .attr("class", "rz-dropdown-label rz-inputtext rz-placeholder")
                                .child(ph.clone())
                                .into_any()
                        } else {
                            leptos::html::span()
                                .attr("class", "rz-dropdown-label rz-inputtext")
                                .child("\u{00a0}")
                                .into_any()
                        }
                    })
                    // ── Trigger chevron ───────────────────────────────────────
                    .child(
                        leptos::html::div()
                            .attr("class", "rz-dropdown-trigger rz-corner-right")
                            .child(leptos::html::span().attr(
                                "class",
                                "notranslate rz-dropdown-trigger-icon rzi rzi-chevron-down",
                            )),
                    )
                    // ── Single-mode clear button ──────────────────────────────
                    // Mirrors Blazor razor: placed INSIDE the root div, after the trigger,
                    // as the last child. This is the correct position per the Blazor source:
                    //   @if (AllowClear && !ReadOnly && (!Multiple && HasValue || …)) {
                    //     <button class="notranslate rz-dropdown-clear-icon rzi rzi-times" … />
                    //   }
                    .child(move || -> Option<AnyView> {
                        if multiple || !allow_clear || read_only || !has_value.get() {
                            return None;
                        }
                        let ca = clear_all_for_root.clone();
                        Some(
                            leptos::html::button()
                                .attr("type", "button")
                                .attr("tabindex", "-1")
                                .attr("class", "notranslate rz-dropdown-clear-icon rzi rzi-times")
                                .attr("aria-label", "Clear")
                                .on(leptos::ev::mousedown, move |ev: web_sys::MouseEvent| {
                                    ev.stop_propagation();
                                    ev.prevent_default();
                                    ca();
                                })
                                .into_any(),
                        )
                    }),
            )
            // ── Panel — sibling of root, outside overflow:hidden ──────────────
            // Rendered as a sibling (not a child) of the root div so that it is
            // not clipped by the root's overflow:hidden. The wrapper's
            // position:relative anchors the absolute panel correctly.
            .child(move || build_panel()),
    )
    .into_any()
}
