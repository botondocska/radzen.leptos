//! RadzenStack component — mirrors C# Radzen.Blazor.RadzenStack.
//!
//! # CSS class
//! Always `"rz-stack"` (plus caller `attrs["class"]` appended last).
//! The `rz-stack` CSS class owns `display: flex` in the Radzen theme SCSS.
//! We also emit `display: flex` in the inline style so the component works
//! regardless of whether the compiled theme includes that rule.
//!
//! # Inline style — `GetComponentStyle()`
//! Mirrors `RadzenFlexComponent.GetComponentStyle()` which emits:
//!   ```
//!   display: flex
//!   flex-direction: {row|column}[-reverse]
//!   gap: {Gap}                      (only when Gap is non-empty)
//!   align-items: {value}            (only when AlignItems != Normal)
//!   justify-content: {value}        (only when JustifyContent != Normal)
//!   flex-wrap: {value}              (only when Wrap != NoWrap)
//!   ```
//! `display: flex` is emitted by the component, not the theme — the theme's
//! `.rz-stack` rule only sets `box-sizing` and `gap` (the CSS variable).
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element is fully absent when invisible.

use crate::components::{
    AlignItems, ClassList, FlexWrap, JustifyContent, Orientation,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;

/// RadzenStack component.
///
/// A flexbox container that arranges children in a vertical or horizontal
/// stack with configurable gap, alignment, justification, and wrapping.
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
    /// When `None` or empty, no `gap` is emitted.
    #[prop(default = None, into)]
    gap: Option<String>,

    /// Reverse child order. Appends `-reverse` to `flex-direction`.
    #[prop(default = false)]
    reverse: bool,

    /// Child content.
    children: ChildrenFn,
) -> impl IntoView {
    // use_radzen_base with "" — full class built by ClassList below.
    let handle = use_radzen_base(&base, "");

    // ── Visibility — mirrors `@if (Visible)` ──────────────────────────────────
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── CSS class ─────────────────────────────────────────────────────────────
    // GetComponentCssClass() always returns "rz-stack".
    // GetCssClass() appends caller attrs["class"] last.
    let css_class = ClassList::create("rz-stack")
        .add_caller_class(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish();

    // ── Inline style — mirrors RadzenFlexComponent.GetComponentStyle() ────────
    // display: flex is emitted by the component itself, not by the theme CSS.
    // The theme's .rz-stack rule only provides box-sizing and the gap variable.
    let flex_dir = match (&orientation, reverse) {
        (Orientation::Horizontal, false) => "row",
        (Orientation::Horizontal, true) => "row-reverse",
        (Orientation::Vertical, false) => "column",
        (Orientation::Vertical, true) => "column-reverse",
    };

    let mut component_style = format!("display: flex; flex-direction: {};", flex_dir);

    // gap — only when non-empty
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

    // GetStyle() = GetComponentStyle() + caller Style
    let caller_style = base.style.clone().unwrap_or_default();
    let style = if caller_style.is_empty() {
        component_style
    } else {
        format!("{} {}", component_style, caller_style)
    };

    // ── Event handlers ────────────────────────────────────────────────────────
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();
    let handle_id = handle.id.clone();

    Some(
        leptos::html::div()
            .attr("style", style)
            .attr("class", css_class)
            .attr("id", handle_id)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            .child(children()),
    )
    .into_any()
}
