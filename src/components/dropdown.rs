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
//! return GetClassList("rz-dropdown")   // from DataBoundFormComponent:
//!     //   .AddDisabled(Disabled)
//!     //   .Add("rz-state-empty", !HasValue)
//!     .Add("rz-clear", AllowClear)
//!     .Add("rz-dropdown-chips", Chips && selectedItems.Count > 0)
//!     .ToString();
//! ```
//! `GetCssClass()` then appends the caller `class` attribute last.
//! `rz-state-focused` is appended by `GetClassList` when `isPopupOpen` — we
//! mirror this by adding it reactively when `open.get()` is true.
//!
//! # HTML structure (mirrors `RadzenDropDown.razor`)
//! ```html
//! @if (Visible) {
//! <div role="combobox" class="rz-dropdown [rz-clear] [rz-state-focused]"
//!      @onclick @onclick:preventDefault @onclick:stopPropagation>
//!
//!   <div class="rz-helper-hidden-accessible">
//!     <input type="text" readonly name="@Name" value="@internalValue" />
//!   </div>
//!
//!   <!-- selected display: one of the following spans -->
//!   <span class="rz-dropdown-label rz-inputtext">…selected label…</span>
//!   <!-- OR: rz-placeholder when nothing selected and Placeholder set -->
//!   <!-- OR: &nbsp; -->
//!
//!   <div class="rz-dropdown-trigger rz-corner-right">
//!     <span class="notranslate rz-dropdown-trigger-icon rzi rzi-chevron-down" />
//!   </div>
//!
//!   <!-- @if (AllowClear && !ReadOnly && HasValue) — OUTSIDE the panel -->
//!   <button class="notranslate rz-dropdown-clear-icon rzi rzi-times" … />
//!
//!   <div class="rz-dropdown-panel" style="display:none">
//!     <!-- @if (!Multiple && AllowFiltering) filter input -->
//!     <!-- @if (Multiple && (AllowSelectAll || AllowFiltering)) header -->
//!     <div class="rz-dropdown-items-wrapper" style="@PopupStyle">
//!       <ul class="rz-dropdown-items rz-dropdown-list" role="listbox">
//!         <li class="rz-dropdown-item [rz-state-highlight] [rz-state-disabled]">
//!           <span>…label…</span>
//!         </li>
//!       </ul>
//!     </div>
//!     <!-- @FooterTemplate -->
//!   </div>
//!
//! </div>
//! }
//! ```
//!
//! # Popup strategy
//! Blazor uses JS `Radzen.togglePopup` to show/hide the panel (always in DOM,
//! toggled via `display:none`). We use a reactive `open: RwSignal<bool>` and
//! conditionally render the panel — the Leptos idiomatic equivalent. The
//! focus/blur race is handled via `mousedown` + `prevent_default` (fires before
//! `blur`, prevents root focus loss).
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted when invisible.

use crate::components::{
    base_component::ComponentProps,
    dropdown_base::{
        build_drop_down_root_class, use_drop_down_base, DropDownItem, DropDownProps,
    },
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
///
/// # Multiple selection with filtering
/// ```rust,ignore
/// let selected = RwSignal::new(Vec::<String>::new());
/// <RadzenDropDown
///     drop_down=DropDownProps {
///         value_multiple: Some(selected),
///         multiple: true,
///         allow_filtering: true,
///         allow_select_all: true,
///         data: items,
///         ..Default::default()
///     }
/// />
/// ```
#[component]
pub fn RadzenDropDown(
    /// Shared dropdown props (data, value, multiple, filtering, …).
    /// Contains the full parameter surface of `DropDownBase<TValue>`.
    #[prop(default = Default::default())]
    drop_down: DropDownProps,

    /// Base component props (id, style, visible, attrs, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    // ── RadzenDropDown-specific params ─────────────────────────────────────────
    /// Whether the component is read-only (displays but prevents changes).
    #[prop(default = false)]
    read_only: bool,

    /// Whether to display selected items as removable chips in multi-select.
    /// Mirrors `Chips` on `RadzenDropDown`. Default: `false`.
    #[prop(default = false)]
    chips: bool,

    /// Optional footer content rendered below the items list.
    /// Mirrors `FooterTemplate` on `RadzenDropDown`.
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

    // ── Root CSS class — reactive ─────────────────────────────────────────────
    // Mirrors GetComponentCssClass():
    //   GetClassList("rz-dropdown")
    //       .AddDisabled(Disabled)
    //       .Add("rz-state-empty", !HasValue)
    //       .Add("rz-clear", AllowClear)
    //       .Add("rz-dropdown-chips", Chips && selectedItems.Count > 0)
    // + GetCssClass appends caller class last.
    // rz-state-focused added when popup is open (mirrors isPopupOpen in Blazor).
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
    // Mirrors Blazor's cascade of display branches.
    let selected_label = move || -> String {
        if multiple {
            let selected = value_multiple.map(|s| s.get()).unwrap_or_default();
            if selected.is_empty() {
                return String::new();
            }
            let count = selected.len();
            if count > max_selected_labels {
                // Mirrors: $"{selectedItems.Count} {SelectedItemsText}"
                return format!("{} {}", count, selected_items_text_sv.get_value());
            }
            // Mirrors: string.Join(Separator, selectedItems.Select(label))
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
            value_multiple.map(|s| s.get().join(",")).unwrap_or_default()
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
            // Close popup — mirrors `ClosePopup("Enter")` in OnSelectItem.
            open.set(false);
            filter_text.set(String::new());
            if let Some(cb) = on_close_sv.get_value() {
                cb();
            }
        }
    });

    // ── select_all — mirrors SelectAll() ─────────────────────────────────────
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

    // ── clear_all — mirrors ClearAll() ───────────────────────────────────────
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

    // ── Root toggle — mirrors Blazor's onclick + stopPropagation + preventDefault ──
    // mousedown fires before blur, so we use it to toggle without losing focus.
    let toggle = move |ev: web_sys::MouseEvent| {
        ev.prevent_default();
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
    // 150 ms delay so item mousedown can register before blur fires.
    let on_blur = move |_ev: web_sys::FocusEvent| {
        gloo_timers::callback::Timeout::new(150, move || {
            open.set(false);
            filter_text.set(String::new());
        })
        .forget();
    };

    // ── Keyboard — mirrors OnKeyPress ─────────────────────────────────────────
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        match ev.key().as_str() {
            "Escape" | "Tab" => {
                open.set(false);
                filter_text.set(String::new());
                if let Some(cb) = on_close_sv.get_value() {
                    cb();
                }
            }
            "Enter" | " " | "ArrowDown" => {
                if !open.get_untracked() {
                    if let Some(cb) = on_open_sv.get_value() {
                        cb();
                    }
                    open.set(true);
                }
            }
            _ => {}
        }
    };

    // ── Base mouse events ─────────────────────────────────────────────────────
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();

    Some(
        leptos::html::div()
            .attr("id", handle_id)
            .attr("style", style)
            .attr("class", root_class)
            .attr("role", "combobox")
            .attr("aria-haspopup", "listbox")
            .attr("aria-expanded", move || {
                if open.get() { "true" } else { "false" }
            })
            .attr("aria-disabled", if disabled { "true" } else { "false" })
            .attr("tabindex", effective_tab.to_string())
            .on(leptos::ev::mousedown, toggle)
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
                            .attr("name", drop_down.name.clone())
                            .attr("id", drop_down.name.clone())
                            .attr("readonly", true)
                            .attr("tabindex", "-1")
                            .attr("aria-haspopup", "listbox")
                            .attr("aria-expanded", move || {
                                if open.get() { "true" } else { "false" }
                            })
                            .prop("value", move || hidden_value()),
                    ),
            )
            // ── Selected value display ────────────────────────────────────────
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
            // ── Trigger chevron ───────────────────────────────────────────────
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
            // ── Clear button — OUTSIDE the popup panel ────────────────────────
            .child(move || -> Option<AnyView> {
                if !allow_clear || read_only || !has_value.get() {
                    return None;
                }
                let ca = clear_all.clone();
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
            })
            // ── Popup panel ───────────────────────────────────────────────────
            .child(move || -> Option<AnyView> {
                if !open.get() {
                    return None;
                }

                let panel_class = if multiple {
                    "rz-multiselect-panel"
                } else {
                    "rz-dropdown-panel"
                };

                let items = filtered_items.get();

                // ── Single-mode filter header ──────────────────────────────────
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
                                .on(leptos::ev::mousedown, |ev: web_sys::MouseEvent| {
                                    ev.stop_propagation();
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
                        .into_any()
                });

                // ── Multiple-mode header ───────────────────────────────────────
                // Leptos HtmlElement builder types change with every .child() /
                // .on() call — we cannot `let mut header = builder; header =
                // header.child(...)` because that would change the type.
                // Solution: collect conditional children as Vec<AnyView> and
                // assemble them with a single .child(vec) call on the wrapper.
                let multi_header: Option<AnyView> =
                    (multiple && (allow_select_all || allow_filtering)).then(|| {
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

                        // Build conditional children as AnyView so the wrapper
                        // div receives a single homogeneous Vec<AnyView>.
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
                                            .child(
                                                leptos::html::span()
                                                    .attr("class", chkbox_icon_class),
                                            ),
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
                                            .on(leptos::ev::mousedown, |ev: web_sys::MouseEvent| {
                                                ev.stop_propagation();
                                            })
                                            .on(leptos::ev::click, |ev: web_sys::MouseEvent| {
                                                ev.stop_propagation();
                                            }),
                                    )
                                    .child(
                                        leptos::html::span().attr(
                                            "class",
                                            "notranslate rz-multiselect-filter-icon rzi rzi-search",
                                        ),
                                    )
                                    .into_any(),
                            );
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

                // ── Item list ─────────────────────────────────────────────────
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

                let item_views: Vec<AnyView> = items
                    .into_iter()
                    .map(|item| {
                        let item_selected = is_selected(&item.value);
                        let item_disabled = item.disabled;

                        // Mirrors GetItemCssClass():
                        //   ClassList.Create("rz-dropdown-item")
                        //       .Add("rz-state-highlight", IsSelected(item))
                        //       .AddDisabled(IsDisabled(item))
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
                                        .child(
                                            leptos::html::span()
                                                .attr("class", chkbox_icon_class),
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
                            // stop_propagation → does not bubble to root toggle.
                            // prevent_default → root does not lose focus.
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

                // ── Footer — mirrors @FooterTemplate ──────────────────────────
                let footer: Option<AnyView> = footer_sv.get_value().map(|f| {
                    leptos::html::div()
                        .attr("class", "rz-dropdown-footer")
                        .child(f())
                        .into_any()
                });

                Some(
                    leptos::html::div()
                        .attr("class", panel_class)
                        // prevent_default on panel: keeps focus on root trigger.
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
            }),
    )
    .into_any()
}