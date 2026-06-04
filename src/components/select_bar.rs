//! RadzenSelectBar component — mirrors C# Radzen.Blazor.RadzenSelectBar<TValue>.
//!
//! # CSS class order (mirrors Blazor exactly)
//! Root: `rz-selectbar rz-buttonset rz-selectbar-{horizontal|vertical} rz-buttonset-{N} [caller-class]`
//! Per-button: `rz-button rz-button-text-only rz-button-{xs|sm|md|lg} [rz-state-active] [rz-state-focused] [rz-state-disabled]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! GetClassList("rz-selectbar rz-buttonset")
//!     .Add($"rz-selectbar-{(Orientation == Orientation.Vertical ? "vertical" : "horizontal")}")
//!     .Add($"rz-buttonset-{allItems.Count}")
//!     .ToString()
//! ```
//!
//! Per-button `ButtonClass()`:
//! ```csharp
//! ClassList.Create("rz-button rz-button-text-only")
//!     .AddButtonSize(Size)
//!     .Add("rz-state-active", IsSelected(item))
//!     .Add("rz-state-focused", IsFocused(item) && focused)
//!     .AddDisabled(Disabled || item.Disabled)
//! ```
//!
//! # Keyboard navigation (mirrors Blazor OnKeyPress)
//! ArrowLeft/ArrowRight: move focus between items
//! Space/Enter: select focused item
//! Home/End: move to first/last item
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
/// Registers itself with the parent bar via [`SelectBarContext`].
#[component]
pub fn RadzenSelectBarItem(
    /// The value this item represents.
    #[prop(into)]
    value: String,

    /// Display label.
    #[prop(default = String::new(), into)]
    text: String,

    /// Optional Material icon name (e.g. `"list"`, `"grid_view"`).
    #[prop(default = None, into)]
    icon: Option<String>,

    /// Optional image URL shown instead of an icon.
    #[prop(default = None, into)]
    image_url: Option<String>,

    /// Whether this item is individually disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Whether this item is visible. Default: `true`.
    #[prop(default = true)]
    visible: bool,
) -> impl IntoView {
    if let Some(ctx) = use_context::<SelectBarContext>() {
        ctx.register.update(|items| {
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
    // Items themselves render nothing — they only register data.
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
/// Supports single and multiple selection, horizontal/vertical orientation,
/// configurable button sizes, and full keyboard navigation.
///
/// # Single selection (default)
/// ```rust,ignore
/// let mode = RwSignal::new("list".to_string());
/// <RadzenSelectBar value=mode>
///     <RadzenSelectBarItem value="list" text="List" icon=Some("list") />
///     <RadzenSelectBarItem value="grid" text="Grid" icon=Some("grid_view") />
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

    /// Single-selection value signal. Ignored when `multiple=true`.
    #[prop(optional)]
    value: Option<RwSignal<String>>,

    /// Multi-selection value signal. Only used when `multiple=true`.
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

    /// Called when selection changes.
    /// Single: new selected value string.
    /// Multiple: comma-separated selected values.
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,

    /// Child `<RadzenSelectBarItem>` elements.
    children: ChildrenFn,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── Item registry ─────────────────────────────────────────────────────────
    let registry: RwSignal<Vec<SelectBarItemData>> = RwSignal::new(Vec::new());
    provide_context(SelectBarContext { register: registry });

    // Render children so they register.
    let _rendered_children = children();

    // ── Keyboard navigation state ─────────────────────────────────────────────
    // focused_index: which item has keyboard focus (-1 = none)
    // focused: whether the bar itself has focus (drives rz-state-focused on button)
    let focused_index: RwSignal<i32> = RwSignal::new(-1);
    let focused = RwSignal::new(false);

    // ── Static props for closures ─────────────────────────────────────────────
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

    let orient_suffix = match orientation {
        crate::components::Orientation::Vertical => "vertical",
        crate::components::Orientation::Horizontal => "horizontal",
    };

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
    let select_item = Arc::new(move |item_value: String| {
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
    });

    // ── Keyboard handler (mirrors Blazor OnKeyPress) ───────────────────────────
    // ArrowLeft/ArrowRight: navigate focus
    // Home/End: jump to first/last
    // Space/Enter: select focused item
    let select_item_key = select_item.clone();
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        let key = if ev.code().is_empty() {
            ev.key()
        } else {
            ev.code()
        };
        let items = registry.get();
        let visible_items: Vec<&SelectBarItemData> = items.iter().filter(|i| i.visible).collect();
        let count = visible_items.len() as i32;
        if count == 0 {
            return;
        }

        match key.as_str() {
            "ArrowLeft" | "ArrowRight" => {
                ev.prevent_default();
                focused.set(true);
                let dir: i32 = if key == "ArrowLeft" { -1 } else { 1 };
                let current = focused_index.get_untracked();
                let start = if current < 0 { 0 } else { current };
                let mut next = (start + dir).clamp(0, count - 1);
                // Skip disabled items.
                let max_tries = count;
                let mut tries = 0;
                while tries < max_tries
                    && visible_items
                        .get(next as usize)
                        .map_or(false, |i| i.disabled)
                {
                    next = (next + dir).clamp(0, count - 1);
                    tries += 1;
                }
                focused_index.set(next);
            }
            "Home" => {
                ev.prevent_default();
                focused.set(true);
                focused_index.set(0);
            }
            "End" => {
                ev.prevent_default();
                focused.set(true);
                focused_index.set(count - 1);
            }
            "Space" | "Enter" => {
                ev.prevent_default();
                let idx = focused_index.get_untracked();
                if idx >= 0 && idx < count {
                    if let Some(item) = visible_items.get(idx as usize) {
                        if !item.disabled && !disabled {
                            select_item_key(item.value.clone());
                        }
                    }
                }
            }
            _ => {}
        }
    };

    let on_focus = move |_ev: web_sys::FocusEvent| {
        focused.set(true);
        if focused_index.get_untracked() < 0 {
            focused_index.set(0);
        }
    };

    let on_blur = move |_ev: web_sys::FocusEvent| {
        focused.set(false);
    };

    Some(
        leptos::html::div()
            .attr("id", handle_id)
            .attr("style", style)
            .attr("tabindex", if disabled { "-1" } else { "0" })
            .on(leptos::ev::keydown, on_keydown)
            .on(leptos::ev::focus, on_focus)
            .on(leptos::ev::blur, on_blur)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            // Root class — reactive so item count (rz-buttonset-{N}) is always current.
            .attr("class", move || {
                let count = registry.get().iter().filter(|i| i.visible).count();
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
                let items = registry.get();
                let visible_items: Vec<SelectBarItemData> =
                    items.into_iter().filter(|i| i.visible).collect();

                visible_items
                    .into_iter()
                    .enumerate()
                    .map(|(idx, item)| {
                        let item_value = item.value.clone();
                        let item_disabled = item.disabled || disabled;
                        let item_selected = is_selected(&item_value);
                        // rz-state-focused: item is at focused_index AND bar has focus
                        let item_focused = focused.get() && focused_index.get() == idx as i32;

                        // ButtonClass mirrors Blazor exactly:
                        // ClassList.Create("rz-button rz-button-text-only")
                        //     .AddButtonSize(Size)
                        //     .Add("rz-state-active", IsSelected(item))
                        //     .Add("rz-state-focused", IsFocused(item) && focused)
                        //     .AddDisabled(Disabled || item.Disabled)
                        let btn_class = ClassList::create("rz-button rz-button-text-only")
                            .add_button_size(size)
                            .add("rz-state-active", item_selected)
                            .add("rz-state-focused", item_focused)
                            .add_disabled(item_disabled)
                            .finish();

                        let iv = item_value.clone();
                        let si = select_item.clone();
                        let on_click = move |_ev: web_sys::MouseEvent| {
                            if !item_disabled {
                                focused_index.set(idx as i32);
                                si(iv.clone());
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
                            .child((!item.text.is_empty()).then(|| {
                                leptos::html::span()
                                    .attr("class", "rz-button-text")
                                    .child(item.text.clone())
                            }))
                            .on(leptos::ev::click, on_click)
                            .into_any()
                    })
                    .collect_view()
            }),
    )
    .into_any()
}
