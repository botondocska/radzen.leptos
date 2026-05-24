//! RadzenStack component — mirrors C# Radzen.Blazor.RadzenStack.
//!
//! ## Blazor source reference
//!
//! Inherits from `RadzenFlexComponent` which itself extends
//! `RadzenComponentWithChildren`. The full class hierarchy:
//!   RadzenStack : RadzenFlexComponent : RadzenComponentWithChildren : RadzenComponent
//!
//! ### CSS class — `GetComponentCssClass()`
//! Always returns `"rz-stack"`. Caller `attrs["class"]` is appended by
//! `use_radzen_base` (same pattern as every other component).
//!
//! ### Inline style — `GetComponentStyle()` / `GetStyle()`
//! Blazor emits all layout as **inline style**, not CSS classes:
//!   ```
//!   flex-direction: {row|column}[-reverse]
//!   gap: {Gap}                      (only when Gap is non-empty)
//!   align-items: {value}            (only when AlignItems != Normal)
//!   justify-content: {value}        (only when JustifyContent != Normal)
//!   flex-wrap: {value}              (only when Wrap != NoWrap)
//!   ```
//! `GetStyle()` then concatenates `GetComponentStyle()` with the caller's
//! `Style` attribute — identical to how `RadzenComponent.GetStyle()` works.
//!
//! ### Visibility
//! Mirrors `@if (Visible)` — element is fully absent when invisible.
//! Uses the same early-return `None::<AnyView>.into_any()` pattern as
//! `RadzenText`, avoiding `Fn` / `FnOnce` issues with the builder API.
//!
//! ### Razor template
//! ```razor
//! @if (Visible) {
//!     <div @ref="Element" style="@GetStyle()" @attributes="Attributes"
//!          class="@GetCssClass()" id="@GetId()">
//!         @ChildContent
//!     </div>
//! }
//! ```
//! Attribute order in Blazor: style → spread Attributes → class → id.
//! We mirror this in the Leptos builder chain.

use crate::components::{
    AlignItems, FlexWrap, JustifyContent, Orientation,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;

/// RadzenStack component.
///
/// A flexbox container that arranges children in a vertical or horizontal
/// stack with configurable gap, alignment, justification, and wrapping.
/// Simpler alternative to `RadzenRow`/`RadzenColumn` for linear layouts.
///
/// # CSS
/// The component class is always `"rz-stack"`.  All layout behaviour is
/// expressed as **inline style** (`flex-direction`, `gap`, `align-items`,
/// `justify-content`, `flex-wrap`) — no layout utility classes are added.
///
/// # Defaults
/// - `orientation` — `Vertical` (`flex-direction: column`)
/// - `align_items` — `Normal` (not emitted)
/// - `justify_content` — `Normal` (not emitted)
/// - `wrap` — `NoWrap` (not emitted)
/// - `gap` — `None` (not emitted)
/// - `reverse` — `false`
#[component]
pub fn RadzenStack(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Stack direction. Default: [`Orientation::Vertical`] → `flex-direction: column`.
    #[prop(default = Orientation::Vertical)]
    orientation: Orientation,

    /// Cross-axis alignment. Default: [`AlignItems::Normal`] (not emitted).
    #[prop(default = AlignItems::Normal)]
    align_items: AlignItems,

    /// Main-axis justification. Default: [`JustifyContent::Normal`] (not emitted).
    #[prop(default = JustifyContent::Normal)]
    justify_content: JustifyContent,

    /// Flex-wrap behaviour. Default: [`FlexWrap::NoWrap`] (not emitted).
    #[prop(default = FlexWrap::NoWrap)]
    wrap: FlexWrap,

    /// Gap between children. Accepts any CSS length, e.g. `"1rem"`, `"16px"`.
    /// When `None` or empty, no `gap` is emitted. Default: `None`.
    #[prop(default = None, into)]
    gap: Option<String>,

    /// Reverse child order. When `true`, appends `-reverse` to `flex-direction`.
    /// Default: `false`.
    #[prop(default = false)]
    reverse: bool,

    /// Child content — always required (mirrors `RadzenComponentWithChildren`).
    /// `ChildrenFn` rather than `Children` to satisfy Leptos `Fn` bounds.
    children: ChildrenFn,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "rz-stack");

    // ── Visibility — mirrors `@if (Visible)` ──────────────────────────────────
    // Early return produces no DOM node when invisible — same pattern as
    // RadzenText (avoids Fn/FnOnce conflict with the custom-element builder).
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── CSS class ──────────────────────────────────────────────────────────────
    // GetComponentCssClass() always returns "rz-stack".
    // use_radzen_base already put "rz-stack" in handle.css_class and appended
    // any caller attrs["class"] — nothing else to add here.
    let css_class = handle.css_class.clone();

    // ── Inline style — GetComponentStyle() ───────────────────────────────────
    // flex-direction: {row|column}[-reverse]
    let flex_dir = match (&orientation, reverse) {
        (Orientation::Horizontal, false) => "row",
        (Orientation::Horizontal, true)  => "row-reverse",
        (Orientation::Vertical,   false) => "column",
        (Orientation::Vertical,   true)  => "column-reverse",
    };

    let mut component_style = format!("display: flex; flex-direction: {};", flex_dir);


    // gap — only when a non-empty value is provided
    if let Some(ref g) = gap {
        if !g.trim().is_empty() {
            component_style.push_str(&format!(" gap: {};", g.trim()));
        }
    }

    // align-items — only when not Normal
    if let Some(ai) = align_items.css_value() {
        component_style.push_str(&format!(" align-items: {};", ai));
    }

    // justify-content — only when not Normal
    if let Some(jc) = justify_content.css_value() {
        component_style.push_str(&format!(" justify-content: {};", jc));
    }

    // flex-wrap — only when not NoWrap
    if let Some(fw) = wrap.css_value() {
        component_style.push_str(&format!(" flex-wrap: {};", fw));
    }

    // GetStyle() = GetComponentStyle() + caller Style (attribute order mirrors C#)
    let caller_style = base.style.clone().unwrap_or_default();
    let style = if caller_style.is_empty() {
        component_style
    } else {
        format!("{} {}", component_style, caller_style)
    };

    // ── Event handlers ────────────────────────────────────────────────────────
    let enter_cb  = handle.on_mouse_enter.clone();
    let leave_cb  = handle.on_mouse_leave.clone();
    let ctx_cb    = handle.on_context_menu.clone();
    let handle_id = handle.id.clone();

    // ── Render ─────────────────────────────────────────────────────────────────
    // Blazor attribute order: style → spread Attributes → class → id.
    Some(
        leptos::html::div()
            .attr("style",  style)
            .attr("class",  css_class)
            .attr("id",     handle_id)
            .on(leptos::ev::mouseenter,  move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave,  move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            .child(children())
    )
    .into_any()
}