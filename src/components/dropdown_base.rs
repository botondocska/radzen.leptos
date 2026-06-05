//! DropdownBase — shared data types and composable for dropdown-family components.
//!
//! # What this replaces
//! C# inheritance chain: `RadzenDropDown<TValue>` → `DropDownBase<T>` →
//! `DataBoundFormComponent<T>` → `RadzenComponent`.
//!
//! In Rust there is no class inheritance. All shared state that Blazor kept on
//! those base classes is decomposed here:
//!
//! | C# concept                        | Rust equivalent                          |
//! |-----------------------------------|------------------------------------------|
//! | `Data`, `TextProperty`,`ValueProperty` | [`DropDownItem`] flat model           |
//! | `Value` / `selectedItem`          | `Option<RwSignal<String>>`               |
//! | `Value` / `selectedItems`         | `Option<RwSignal<Vec<String>>>`          |
//! | `searchText` / `View`             | `filter_text` signal + `filtered_items` Memo |
//! | `isOpen` / `isPopupOpen`          | `open: RwSignal<bool>`                   |
//! | `HasValue`                        | `has_value: Memo<bool>`                  |
//! | `GetClassList(root)`              | [`build_drop_down_root_class`]           |
//! | `DropDownBase` fields             | [`DropDownProps`] (nested-prop compat)   |
//!
//! # Filtering
//! Blazor's `View` / `FilterOperator` / `FilterCaseSensitivity` chain collapses
//! into a single reactive `Memo`: case-insensitive `contains` match on the item
//! label, matching `FilterCaseSensitivity.Default` + `StringFilterOperator.Contains`.
//!
//! # Selection / value model
//! Blazor used `TValue` generics + runtime reflection (`ValueProperty`/`TextProperty`).
//! We use flat `String` values: callers pass pre-resolved `(value, label)` pairs via
//! [`DropDownItem`]. This trades reflection flexibility for compile-time safety.

use crate::components::{
    ClassList,
    base_component::{ComponentProps, MouseEvent, RadzenBaseHandle, use_radzen_base},
};
use leptos::prelude::*;
use std::sync::Arc;

// ─────────────────────────────────────────────────────────────────────────────
// DropDownItem — universal flat item model
// ─────────────────────────────────────────────────────────────────────────────

/// A single selectable item in any dropdown-family component.
///
/// Mirrors Blazor's `Data` + `TextProperty` + `ValueProperty` + `DisabledProperty`
/// resolution — collapsed to explicit typed fields so callers need no runtime
/// reflection.
///
/// Both `value` (bound to signals) and `label` (displayed text) are `String`.
/// For enum / numeric sources convert to string before building the list.
#[derive(Clone, Debug, PartialEq)]
pub struct DropDownItem {
    /// Logical key stored in `Value` / `value_multiple`.
    pub value: String,
    /// Human-readable text shown in the list and selected-value display.
    pub label: String,
    /// When `true` the item cannot be selected — mirrors `DisabledProperty`.
    pub disabled: bool,
}

impl DropDownItem {
    /// Construct an enabled item.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    /// Builder — mark this item as disabled.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// Alias for callers that import `DropdownBaseOption`.
pub type DropdownBaseOption = DropDownItem;

// ─────────────────────────────────────────────────────────────────────────────
// DropDownProps — parameter surface of DropDownBase<T> as a nested struct
// ─────────────────────────────────────────────────────────────────────────────

/// All shared dropdown parameters in one struct.
///
/// `RadzenDropDown` takes this as a single `drop_down: DropDownProps` prop,
/// matching the Blazor pattern of `RadzenDropDown` inheriting all `DropDownBase`
/// parameters. New dropdown-family components should prefer the flat-prop API
/// and call `use_drop_down_base_flat` directly.
#[derive(Clone)]
pub struct DropDownProps {
    // ── DataBoundFormComponent ────────────────────────────────────────────────
    /// `name` attribute on the hidden input. Also used as element `id`.
    pub name: Option<String>,
    /// Placeholder shown when nothing is selected.
    pub placeholder: Option<String>,
    /// Whether the component is disabled.
    pub disabled: bool,
    /// Tab order. Mirrors `TabIndex`. Default: `0`.
    pub tab_index: i32,

    // ── DropDownBase ─────────────────────────────────────────────────────────
    /// Items shown in the list.
    pub data: Vec<DropDownItem>,
    /// Single-selection value signal.
    pub value: Option<RwSignal<String>>,
    /// Multiple-selection value signal. Only used when `multiple = true`.
    pub value_multiple: Option<RwSignal<Vec<String>>>,
    /// Enable multiple item selection. Mirrors `Multiple`. Default: `false`.
    pub multiple: bool,
    /// Show a filter input inside the popup. Mirrors `AllowFiltering`. Default: `false`.
    pub allow_filtering: bool,
    /// Show a "select all" checkbox in multi-select header. Mirrors `AllowSelectAll`. Default: `true`.
    pub allow_select_all: bool,
    /// Show a clear button. Mirrors `AllowClear`. Default: `false`.
    pub allow_clear: bool,

    // ── RadzenDropDown-specific ───────────────────────────────────────────────
    /// Placeholder text in the filter input box. Mirrors `FilterPlaceholder`.
    pub filter_placeholder: String,
    /// Max number of selected labels shown before switching to "{n} items selected".
    /// Mirrors `MaxSelectedLabels`. Default: `4`.
    pub max_selected_labels: usize,
    /// Text appended when `MaxSelectedLabels` is exceeded. Default: `"items selected"`.
    pub selected_items_text: String,
    /// CSS style for the popup panel. Mirrors `PopupStyle`.
    /// Default: `"max-height:200px;overflow-x:hidden"`.
    pub popup_style: String,

    // ── Event callbacks ───────────────────────────────────────────────────────
    /// Called when the committed value changes (single: new value; multiple: comma-joined).
    pub on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,
    /// Called when the popup opens. Mirrors `Open` event.
    pub on_open: Option<Arc<dyn Fn() + Send + Sync>>,
    /// Called when the popup closes. Mirrors `Close` event.
    pub on_close: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl Default for DropDownProps {
    fn default() -> Self {
        Self {
            name: None,
            placeholder: None,
            disabled: false,
            tab_index: 0,
            data: Vec::new(),
            value: None,
            value_multiple: None,
            multiple: false,
            allow_filtering: false,
            allow_select_all: true,
            allow_clear: false,
            filter_placeholder: String::new(),
            max_selected_labels: 4,
            selected_items_text: "items selected".to_string(),
            popup_style: "max-height:200px;overflow-x:hidden".to_string(),
            on_change: None,
            on_open: None,
            on_close: None,
        }
    }
}

impl DropDownProps {
    /// Returns `tab_index` clamped to `-1` when disabled — mirrors `GetId()` guard.
    pub fn effective_tab(&self) -> i32 {
        if self.disabled { -1 } else { self.tab_index }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DropDownHandle — reactive state returned by use_drop_down_base
// ─────────────────────────────────────────────────────────────────────────────

/// Reactive handle returned by [`use_drop_down_base`].
///
/// Mirrors the instance state Blazor maintained on `DropDownBase<T>`:
/// `selectedItem`, `selectedItems`, `searchText`, `isOpen` / `isPopupOpen`, `HasValue`.
pub struct DropDownHandle {
    /// Stable DOM `id` — from `use_radzen_base`.
    pub id: String,
    /// Visibility signal — from `use_radzen_base`.
    pub visible: RwSignal<bool>,
    /// Whether the popup panel is open. Mirrors `isOpen` in `RadzenDropDown`.
    pub open: RwSignal<bool>,
    /// Current filter text. Mirrors `searchText` in `DataBoundFormComponent`.
    pub filter_text: RwSignal<String>,
    /// Reactive filtered list. Mirrors Blazor's `View` property.
    pub filtered_items: Memo<Vec<DropDownItem>>,
    /// Whether a value is selected. Mirrors `HasValue` override in `DropDownBase`.
    pub has_value: Memo<bool>,
    // ── Forwarded base mouse events ───────────────────────────────────────────
    pub on_mouse_enter: Arc<dyn Fn(MouseEvent) + Send + Sync>,
    pub on_mouse_leave: Arc<dyn Fn(MouseEvent) + Send + Sync>,
    pub on_context_menu: Arc<dyn Fn(MouseEvent) + Send + Sync>,
}

// ─────────────────────────────────────────────────────────────────────────────
// use_drop_down_base — primary entry point (DropDownProps + ComponentProps)
// ─────────────────────────────────────────────────────────────────────────────

/// Call at the top of every dropdown-family component function.
///
/// This overload accepts the [`DropDownProps`] + [`ComponentProps`] pair used
/// by `RadzenDropDown` and any other component that bundles all shared params
/// into a single nested struct.
///
/// Returns `(DropDownHandle, RadzenBaseHandle)`.
pub fn use_drop_down_base(
    props: &DropDownProps,
    base: &ComponentProps,
) -> (DropDownHandle, RadzenBaseHandle) {
    use_drop_down_base_flat(
        base,
        props.data.clone(),
        props.value,
        props.value_multiple,
        props.multiple,
        props.disabled,
        props.allow_filtering,
        false, // case-insensitive (Blazor FilterCaseSensitivity.Default)
        props.tab_index,
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// use_drop_down_base_flat — flat-arg version for new components
// ─────────────────────────────────────────────────────────────────────────────

/// Flat-argument variant of `use_drop_down_base`.
///
/// Preferred for new dropdown-family components that take individual `#[prop]`
/// items rather than a nested `DropDownProps` struct.
#[allow(clippy::too_many_arguments)]
pub fn use_drop_down_base_flat(
    base: &ComponentProps,
    data: Vec<DropDownItem>,
    value: Option<RwSignal<String>>,
    value_multiple: Option<RwSignal<Vec<String>>>,
    multiple: bool,
    disabled: bool,
    allow_filtering: bool,
    filter_case_sensitive: bool,
    tab_index: i32,
) -> (DropDownHandle, RadzenBaseHandle) {
    let base_handle = use_radzen_base(base, "");

    // ── Popup state ───────────────────────────────────────────────────────────
    let open = RwSignal::new(false);

    // ── Filter text ───────────────────────────────────────────────────────────
    let filter_text: RwSignal<String> = RwSignal::new(String::new());

    // ── Filtered items — reactive Memo ────────────────────────────────────────
    // Mirrors Blazor's `View` property (filtered, case-insensitive contains by default).
    // Data is stored in a StoredValue so it is accessible from the reactive closure
    // without requiring Clone to fire on every signal update.
    let data_sv = StoredValue::new(data);
    let filtered_items = Memo::new(move |_| {
        let items = data_sv.get_value();
        let filter = filter_text.get();
        if !allow_filtering || filter.trim().is_empty() {
            return items;
        }
        let needle = if filter_case_sensitive {
            filter.clone()
        } else {
            filter.to_lowercase()
        };
        items
            .into_iter()
            .filter(|item| {
                let haystack = if filter_case_sensitive {
                    item.label.clone()
                } else {
                    item.label.to_lowercase()
                };
                haystack.contains(&needle)
            })
            .collect()
    });

    // ── has_value — reactive Memo ─────────────────────────────────────────────
    // Mirrors `HasValue` override in `DropDownBase<T>`:
    //   single:   !string.IsNullOrEmpty(internalValue)
    //   multiple: internalValue != null && collection.Any()
    let has_value = Memo::new(move |_| {
        if multiple {
            value_multiple.map(|s| !s.get().is_empty()).unwrap_or(false)
        } else {
            value.map(|s| !s.get().is_empty()).unwrap_or(false)
        }
    });

    let handle = DropDownHandle {
        id: base_handle.id.clone(),
        visible: base_handle.visible,
        open,
        filter_text,
        filtered_items,
        has_value,
        on_mouse_enter: base_handle.on_mouse_enter.clone(),
        on_mouse_leave: base_handle.on_mouse_leave.clone(),
        on_context_menu: base_handle.on_context_menu.clone(),
    };

    (handle, base_handle)
}

// ─────────────────────────────────────────────────────────────────────────────
// build_drop_down_root_class — shared CSS class builder
// ─────────────────────────────────────────────────────────────────────────────

/// Build the root element CSS class for any dropdown-family component.
///
/// Mirrors `GetClassList(rootClass)` from `DataBoundFormComponent`:
/// ```csharp
/// ClassList.Create(className)
///     .AddDisabled(Disabled)
///     .Add("rz-state-empty", !HasValue)
///     // component-specific extras …
///     // caller class last
/// ```
///
/// `extras` — `(class, condition)` pairs inserted after the disabled/empty
/// classes and before the caller class.  `RadzenDropDown` uses this to add
/// `rz-clear`, `rz-dropdown-chips`, and `rz-state-focused`.
pub fn build_drop_down_root_class(
    root: &str,
    extras: &[(&str, bool)],
    disabled: bool,
    has_value: bool,
    caller_class: Option<&str>,
) -> String {
    let mut cl = ClassList::create(root)
        .add_disabled(disabled)
        .add("rz-state-empty", !has_value);

    for (class, condition) in extras {
        cl = cl.add(*class, *condition);
    }

    cl.add_caller_class(caller_class).finish()
}

// ─────────────────────────────────────────────────────────────────────────────
// Backward-compat aliases
// ─────────────────────────────────────────────────────────────────────────────

/// Backward-compat alias — `DropdownBase` was used by some early code.
pub type DropdownBase = DropDownHandle;

/// Backward-compat alias.
pub type DropdownBaseProps = DropDownProps;
