//! RadzenTextArea component — mirrors C# Radzen.Blazor.RadzenTextArea.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `rz-textarea [rz-state-disabled] [rz-state-empty] [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! protected override string GetComponentCssClass()
//! {
//!     return GetClassList("rz-textarea").ToString();
//! }
//! ```
//! Where `GetClassList(className)` (from `DataBoundFormComponent`) expands to:
//! ```csharp
//! ClassList.Create(className)
//!     .AddDisabled(Disabled)
//!     .Add(FieldIdentifier, EditContext)   // validation — omitted for now
//!     .Add("rz-state-empty", !HasValue)
//! ```
//! `RadzenComponent.GetCssClass()` then appends any caller `class` attribute last.
//!
//! # Two render modes — mirrors `Immediate` prop
//! ```
//! Immediate = true  → listens on `oninput`  (value updates as the user types)
//! Immediate = false → listens on `onchange` only (value updates on blur / Enter)
//! ```
//! Note: unlike `RadzenTextBox` which wires *both* handlers in Immediate mode,
//! the `RadzenTextArea` razor template only wires `oninput` when Immediate = true
//! and `onchange` when Immediate = false — one event per mode.
//!
//! # `id` resolution — mirrors `GetId()` override
//! Blazor overrides `GetId()` to return `Name` when set, falling back to the
//! auto-generated unique id. We reproduce this: when `name` is `Some`, the
//! `id` attribute equals `name`; otherwise `handle.id` is used.
//!
//! # `rows` and `cols` attributes
//! The textarea has default dimensions of `rows = 2` and `cols = 20` mirroring
//! the Blazor defaults. These set the initial/minimum visible size; the user may
//! resize via the browser handle depending on CSS.
//!
//! # `rz-state-empty` reactivity
//! The class is re-evaluated reactively via a `Memo` so it stays current as the
//! user types — mirrors Blazor's full re-render on each value change.
//!
//! # `placeholder` and `CurrentPlaceholder`
//! Blazor's `CurrentPlaceholder` suppresses the placeholder when the component
//! is inside a `RadzenFormField` with floating labels. For now we emit the
//! placeholder directly; the `RadzenFormField` integration is a future task.
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted, not `display:none`.
//!
//! # Validation CSS classes (future work)
//! Blazor adds `rz-state-error` / `rz-state-success` via
//! `ClassList.Add(FieldIdentifier, EditContext)`. These require a form
//! validation context not yet implemented. The slots are reserved in the
//! class-builder chain and will be wired in once `RadzenTemplateForm` /
//! `EditContext` are ported.

use crate::components::{
    ClassList,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;
use std::sync::Arc;

/// RadzenTextArea component.
///
/// A styled multi-line `<textarea>` that supports two-way signal binding,
/// optional immediate updates, and all standard HTML textarea attributes
/// (`disabled`, `readonly`, `placeholder`, `maxlength`, `rows`, `cols`,
/// `name`, `tabindex`).
///
/// # Two-way binding
/// Pass an `RwSignal<String>` via `value`. The component reads from and
/// writes back to the signal whenever the value changes.
///
/// # Immediate vs deferred updates
/// - `immediate = false` (default): value is committed on blur / Enter
///   (`onchange` event). Good for expensive reactions.
/// - `immediate = true`: value is committed on every keystroke (`oninput`
///   event). Use for live character counters, search-as-you-type, etc.
///
/// # Dimensions
/// `rows` (default 2) and `cols` (default 20) set the initial textarea size,
/// matching the Blazor defaults. Users may resize via the browser handle
/// depending on the active CSS.
///
/// # Example
/// ```rust,ignore
/// let description = RwSignal::new(String::new());
///
/// view! {
///     <RadzenTextArea
///         value=description
///         rows=5
///         placeholder=Some("Enter description...".to_string())
///         max_length=Some(500)
///     />
/// }
/// ```
#[component]
pub fn RadzenTextArea(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Two-way bound string value.
    ///
    /// Pass an `RwSignal<String>`. The component reads the current value
    /// from the signal and writes back to it whenever the textarea changes.
    /// Default: a fresh empty signal.
    #[prop(optional)]
    value: Option<RwSignal<String>>,

    /// Placeholder text shown when the textarea is empty.
    #[prop(default = None, into)]
    placeholder: Option<String>,

    /// Whether the textarea is disabled. Adds `rz-state-disabled` CSS class,
    /// sets the HTML `disabled` attribute, and forces `tabindex="-1"`.
    #[prop(default = false)]
    disabled: bool,

    /// Whether the textarea is read-only. Displays the value but prevents
    /// editing — the user can still select and copy the text.
    #[prop(default = false)]
    read_only: bool,

    /// `name` attribute for the `<textarea>` element. Also used as the `id`
    /// when set — mirrors Blazor's `GetId()` override in `RadzenTextArea`.
    #[prop(default = None, into)]
    name: Option<String>,

    /// Tab order index. Forced to `"-1"` when `disabled = true` so the element
    /// is skipped during keyboard navigation.
    #[prop(default = 0)]
    tab_index: i32,

    /// Maximum number of characters the user may enter.
    /// Maps to the HTML `maxlength` attribute.
    #[prop(default = None)]
    max_length: Option<u64>,

    /// Number of visible text rows (height). Default: `2`.
    ///
    /// Sets the initial height of the textarea. Users may resize it via the
    /// browser handle depending on CSS. Mirrors Blazor's `Rows` parameter.
    #[prop(default = 2)]
    rows: u32,

    /// Number of visible text columns (width). Default: `20`.
    ///
    /// Sets the initial width of the textarea based on average character width.
    /// In modern CSS layouts, setting an explicit width via `base.style` is
    /// often preferred. Mirrors Blazor's `Cols` parameter.
    #[prop(default = 20)]
    cols: u32,

    /// Whether to update the bound value on every keystroke (`oninput`)
    /// instead of waiting for the field to lose focus (`onchange`).
    /// Default: `false`.
    ///
    /// Mirrors Blazor's `Immediate` parameter. When `true` the textarea
    /// binds with `oninput`; when `false` it binds with `onchange`.
    #[prop(default = false)]
    immediate: bool,

    /// Called after the value has been committed.
    /// Mirrors Blazor's `Change` `EventCallback<string>`.
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(String) + Send + Sync>>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // ── Visibility — mirrors `@if (Visible)` ──────────────────────────────────
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── Internal signal ────────────────────────────────────────────────────────
    // If the caller didn't pass a signal we create a private one so the
    // component still works in uncontrolled mode.
    let value_signal = value.unwrap_or_else(|| RwSignal::new(String::new()));

    // ── Attribute values ───────────────────────────────────────────────────────
    let style = base.style.unwrap_or_default();
    // id: Name wins over the auto-generated id — mirrors GetId() override.
    let textarea_id = name.clone().unwrap_or_else(|| handle.id);
    let effective_tab_index = if disabled { -1 } else { tab_index };

    // ── CSS class — reactive via Memo ──────────────────────────────────────────
    // `rz-state-empty` must track the live signal so it updates as the user
    // types — mirrors Blazor's full re-render which recomputes HasValue on
    // every change. A Memo re-runs only when value_signal changes, which is
    // exactly what we want.
    //
    // The other classes (disabled, caller) are static for the component
    // lifetime so we compute them once and move into the Memo closure.
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

    // ── Event handlers ─────────────────────────────────────────────────────────
    let enter_cb = handle.on_mouse_enter;
    let leave_cb = handle.on_mouse_leave;
    let ctx_cb = handle.on_context_menu;

    // Shared commit logic — mirrors SetValue():
    //   Value = value;
    //   ValueChanged.InvokeAsync(Value);   ← writes signal
    //   Change.InvokeAsync(Value);         ← fires on_change callback
    let on_change_cb = on_change;
    let commit = Arc::new(move |raw: String| {
        value_signal.set(raw.clone());
        if let Some(ref cb) = on_change_cb {
            cb(raw);
        }
    });

    // oninput handler — used when Immediate = true.
    // Mirrors Blazor Immediate branch: @bind:event="oninput" (only oninput).
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

    // onchange handler — used when Immediate = false.
    // Mirrors Blazor non-Immediate branch: @onchange="@OnChange".
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
    // Mirrors Blazor's single `<textarea>` element. Mode toggles which DOM
    // event commits the value: `oninput` (Immediate) or `onchange` (!Immediate).
    //
    // `prop:value` (DOM property, not HTML attribute) keeps the cursor position
    // stable on re-render — same as Blazor's @bind:get / @bind:set pattern.
    Some(
        leptos::html::textarea()
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
            // Controlled value — always reflects the signal.
            .prop("value", move || value_signal.get())
            // Wire oninput — commits immediately when Immediate = true.
            // Mirrors Blazor Immediate branch: @bind:event="oninput"
            .on(leptos::ev::input, move |ev| {
                if immediate {
                    on_input(ev.into());
                }
            })
            // Wire onchange — commits when Immediate = false.
            // Mirrors Blazor: @onchange="@OnChange" in the non-Immediate branch.
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