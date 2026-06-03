//! RadzenSelectBar component — mirrors C# Radzen.Blazor.RadzenSelectBar<TValue>.
//!
//! # CSS class order (mirrors Blazor exactly)
//! Root: `rz-selectbar rz-buttonset rz-selectbar-{horizontal|vertical} rz-buttonset-{N} [caller-class]`
//! Per-button: `rz-button rz-button-text-only rz-button-{xs|sm|md|lg} [rz-state-active] [rz-state-disabled]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! GetClassList("rz-selectbar rz-buttonset")
//!     .Add($"rz-selectbar-{orientation}")
//!     .Add($"rz-buttonset-{allItems.Count}")
//!     .ToString()
//! ```
//! Per-button: `ClassList.Create("rz-button rz-button-text-only").AddButtonSize(Size).Add("rz-state-active", IsSelected(item))...`
//!
//! # Usage — static items
//! ```rust,ignore
//! let view_mode = RwSignal::new("list".to_string());
//! <RadzenSelectBar value=view_mode>
//!     <RadzenSelectBarItem value="list" text="List" icon=Some("list") />
//!     <RadzenSelectBarItem value="grid" text="Grid" icon=Some("grid_view") />
//! </RadzenSelectBar>
//! ```
//!
//! # Value type
//! Uses `String` as the universal value type (simplest to use with the grid filter
//! operator toggle which passes `"And"` / `"Or"` strings). Callers can wrap signals
//! and convert as needed.
//!
//! For the DataGrid filter use-case, `TValue` in Blazor is `LogicalFilterOperator` (enum).
//! We represent this as a `String` value matching the enum name.
//!
//! # Multiple selection
//! When `multiple=true` the value signal holds a comma-separated list of selected values.
//! This mirrors Blazor's `IEnumerable<TValue>` — simplified here to a `Vec<String>` signal.
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted when invisible.

use crate::components::{
    ButtonSize, ClassList,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;
use std::sync::Arc;

// ─────────────────────────────────────────────────────────────────────────────
// RadzenSelectBarItem
// ─────────────────────────────────────────────────────────────────────────────

/// A single item inside a [`RadzenSelectBar`].
///
/// Mirrors `RadzenSelectBarItem` in Blazor. Registers itself with the parent bar
/// via the `SelectBarContext` provided by `RadzenSelectBar`.
///
/// # Example
/// ```rust,ignore
/// <RadzenSelectBarItem value="list" text="List" icon=Some("list") />
/// ```
#[component]
pub fn RadzenSelectBarItem(
    /// The value this item represents. Must match the type used in the parent bar's signal.
    #[prop(into)]
    value: String,

    /// Display label.
    #[prop(default = String::new(), into)]
    text: String,

    /// Optional Material icon name.
    #[prop(default = None, into)]
    icon: Option<String>,

    /// Optional image URL shown instead of an icon.
    #[prop(default = None, into)]
    image_url: Option<String>,

    /// Whether this item is disabled individually.
    #[prop(default = false)]
    disabled: bool,

    /// Whether this item is visible. Default: `true`.
    #[prop(default = true)]
    visible: bool,
) -> impl IntoView {
    // Register with parent context.
    if let Some(ctx) = use_context::<SelectBarContext>() {
        ctx.register.update(|items| {
            // Only add if not already registered (StrictMode / double-render guard).
            if !items.iter().any(|i: &SelectBarItemData| i.value == value) {
                items.push(SelectBarItemData {
                    value: value.clone(),
                    text: text.clone(),
                    icon: icon.clone(),
                    image_url: image_url.clone(),
                    disabled,
                    visible,
                });
            }
        });
    }

    // Items themselves don't render; they only register data.
    ().into_any()
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal data model
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SelectBarItemData {
    pub value: String,
    pub text: String,
    pub icon: Option<String>,
    pub image_url: Option<String>,
    pub disabled: bool,
    pub visible: bool,
}

/// Context provided by `RadzenSelectBar` so child items can register themselves.
#[derive(Clone)]
pub struct SelectBarContext {
    pub register: RwSignal<Vec<SelectBarItemData>>,
}

// ─────────────────────────────────────────────────────────────────────────────
// RadzenSelectBar
// ─────────────────────────────────────────────────────────────────────────────

/// RadzenSelectBar component.
///
/// A segmented button control where each button represents a selectable option.
/// Commonly used for filter operator toggles (And/Or) and view mode selectors (list/grid).
///
/// # Single selection (default)
/// ```rust,ignore
/// let mode = RwSignal::new("list".to_string());
/// <RadzenSelectBar value=mode>
///     <RadzenSelectBarItem value="list" text="List" />
///     <RadzenSelectBarItem value="grid" text="Grid" />
/// </RadzenSelectBar>
/// ```
///
/// # Multiple selection
/// ```rust,ignore
/// let selected = RwSignal::new(vec!["orders".to_string()]);
/// <RadzenSelectBar value_multiple=selected multiple=true>
///     <RadzenSelectBarItem value="orders"    text="Orders" />
///     <RadzenSelectBarItem value="employees" text="Employees" />
/// </RadzenSelectBar>
/// ```
#[component]
pub fn RadzenSelectBar(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Single-selection value signal. The value that matches a `RadzenSelectBarItem.value`
    /// is the selected item. Ignored when `multiple=true`.
    #[prop(optional)]
    value: Option<RwSignal<String>>,

    /// Multi-selection value signal. Each element matches a `RadzenSelectBarItem.value`.
    /// Only used when `multiple=true`.
    #[prop(optional)]
    value_multiple: Option<RwSignal<Vec<String>>>,

    /// Enable multiple-item selection. Default: `false`.
    #[prop(default = false)]
    multiple: bool,

    /// Layout direction. Default: `Orientation::Horizontal`.
    #[prop(default = crate::components::Orientation::Horizontal)]
    orientation: crate::components::Orientation,

    /// Size of each button. Default: `ButtonSize::Medium`.
    #[prop(default = ButtonSize::Medium)]
    size: ButtonSize,

    /// Whether the entire bar is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Called when selection changes. Receives the new selected value (or comma-separated values for multiple).
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,

    /// Child `<RadzenSelectBarItem>` elements.
    children: ChildrenFn,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // Visibility.
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── Item registry ─────────────────────────────────────────────────────────
    // Items register themselves into this signal when rendered as children.
    let registry: RwSignal<Vec<SelectBarItemData>> = RwSignal::new(Vec::new());
    provide_context(SelectBarContext {
        register: registry,
    });

    // Render children so they can register.
    let _rendered_children = children();

    // ── Root CSS ──────────────────────────────────────────────────────────────
    let orient_suffix = match orientation {
        crate::components::Orientation::Vertical => "vertical",
        crate::components::Orientation::Horizontal => "horizontal",
    };

    // We need the item count for rz-buttonset-{N}. Use a derived value.
    let style = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();
    let caller_class = base
        .attrs
        .as_ref()
        .and_then(|a| a.get("class"))
        .cloned()
        .unwrap_or_default();

    // ── Selection helpers ─────────────────────────────────────────────────────
    let is_selected = move |item_value: &str| -> bool {
        if multiple {
            value_multiple
                .map(|s| s.get().iter().any(|v| v == item_value))
                .unwrap_or(false)
        } else {
            value.map(|s| s.get() == item_value).unwrap_or(false)
        }
    };

    let on_change_cb = on_change.clone();
    let select_item = move |item_value: String| {
        if disabled {
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
                if let Some(ref cb) = on_change_cb {
                    cb(multi_sig.get().join(","));
                }
            }
        } else {
            if let Some(single_sig) = value {
                single_sig.set(item_value.clone());
            }
            if let Some(ref cb) = on_change_cb {
                cb(item_value);
            }
        }
    };

    Some(
        leptos::html::div()
            .attr("id", handle_id)
            .attr("style", style)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            // Class computed reactively so item count is always current.
            .attr("class", move || {
                let items = registry.get();
                let count = items.len();
                let base_cls = format!(
                    "rz-selectbar rz-buttonset rz-selectbar-{orient_suffix} rz-buttonset-{count}"
                );
                if caller_class.is_empty() {
                    base_cls
                } else {
                    format!("{base_cls} {caller_class}")
                }
            })
            // Render buttons reactively from the registry.
            .child(move || {
                registry
                    .get()
                    .into_iter()
                    .filter(|item| item.visible)
                    .map(|item| {
                        let item_value = item.value.clone();
                        let item_disabled = item.disabled || disabled;

                        let btn_class = ClassList::create("rz-button rz-button-text-only")
                            .add_button_size(size)
                            .add("rz-state-active", is_selected(&item_value))
                            .add_disabled(item_disabled)
                            .finish();

                        let iv = item_value.clone();
                        let on_click = move |_ev: web_sys::MouseEvent| {
                            if !item_disabled {
                                select_item(iv.clone());
                            }
                        };

                        leptos::html::button()
                            .attr("type", "button")
                            .attr("class", btn_class)
                            .attr("disabled", item_disabled)
                            // Icon.
                            .child(item.icon.as_ref().map(|icon_name| {
                                leptos::html::i()
                                    .attr("class", "notranslate rz-button-icon-left rzi")
                                    .child(icon_name.clone())
                            }))
                            // Image.
                            .child(item.image_url.as_ref().map(|img| {
                                leptos::html::img()
                                    .attr("class", "notranslate rz-button-icon-left rzi")
                                    .attr("src", img.clone())
                                    .attr("alt", item.text.clone())
                            }))
                            // Text label.
                            .child(
                                (!item.text.is_empty()).then(|| {
                                    leptos::html::span()
                                        .attr("class", "rz-button-text")
                                        .child(item.text.clone())
                                }),
                            )
                            .on(leptos::ev::click, on_click)
                            .into_any()
                    })
                    .collect_view()
            }),
    )
    .into_any()
}