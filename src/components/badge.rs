//! RadzenBadge component — mirrors C# Radzen.Blazor.RadzenBadge.
//!
//! Content priority (mirrors Blazor): `children` (ChildContent) takes precedence
//! over `text` when both are supplied, matching the C# razor template behaviour
//! confirmed by `Badge_Renders_ChildContent` in BadgeTests.cs.
//!
//! Visibility (mirrors Blazor `Badge_NotVisible_DoesNotRender`): when
//! `base.visible` is `false` the component renders **nothing** — no element,
//! no `display:none` wrapper. This is identical to Blazor's `@if(Visible)` guard.

use crate::components::{
    BadgeStyle, ClassList, Shade, Variant,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;

/// RadzenBadge component.
///
/// A compact visual indicator for notification counts, statuses, or short text
/// labels. Content can be simple text via `text` or custom markup via `children`
/// (children takes precedence when both are provided).
///
/// Can be absolutely positioned to overlay other elements (e.g. a notification
/// icon with a count badge).
#[component]
pub fn RadzenBadge(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Text content displayed in the badge.
    ///
    /// Ignored when `children` is also provided — mirrors Blazor's
    /// `@(ChildContent ?? (RenderFragment)(_ => builder.AddContent(0, Text)))`.
    #[prop(default = None, into)]
    text: Option<String>,

    /// Semantic color style of the badge. Default: `BadgeStyle::Primary`.
    #[prop(default = BadgeStyle::Primary)]
    badge_style: BadgeStyle,

    /// Visual variant (Filled, Flat, Outlined, Text). Default: `Variant::Filled`.
    #[prop(default = Variant::Filled)]
    variant: Variant,

    /// Color shade intensity. Default: `Shade::Default`.
    #[prop(default = Shade::Default)]
    shade: Shade,

    /// Render as pill-shaped (rounded ends) instead of rectangular.
    /// Default: `false`.
    #[prop(default = false)]
    is_pill: bool,

    /// Optional child content — overrides `text` when provided.
    ///
    /// Mirrors Blazor's `RenderFragment? ChildContent`.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "rz-badge");

    // ── CSS class ─────────────────────────────────────────────────────────────
    // Mirrors Blazor GetComponentCssClass():
    //   ClassList.Create("rz-badge")
    //       .Add($"rz-badge-{BadgeStyle.ToString().ToLowerInvariant()}")
    //       .AddVariant(Variant)
    //       .AddShade(Shade)
    //       .Add("rz-badge-pill", IsPill)
    //       .ToString()
    //
    // `use_radzen_base` already merges any caller `attrs["class"]` into
    // `handle.css_class`, so we build the full class here via ClassList and
    // do NOT re-add "rz-badge" a second time from the ClassList itself —
    // the base handle provides it via the `component_class` argument above.
    // Additional caller classes from attrs are appended by use_radzen_base.
    let css_class = ClassList::new()
        .add_badge_style(badge_style)
        .add_variant(variant)
        .add_shade(shade)
        .add("rz-badge-pill", is_pill)
        .finish();

    let combined_class = if css_class.is_empty() {
        handle.css_class.clone()
    } else {
        format!("{} {}", handle.css_class, css_class)
    };

    // ── Style ─────────────────────────────────────────────────────────────────
    let style = base.style.clone().unwrap_or_default();

    // ── Event handlers ────────────────────────────────────────────────────────
    // Each Arc is cloned once here.  Inside `<Show>` the children closure is
    // `Fn` (called on every render), so the `move |ev|` closures must not
    // *consume* the Arc — they capture a further clone made at closure-build
    // time.  Wrapping in another Arc::clone-capturing closure keeps the outer
    // closure `Fn`.  This is the same pattern button.rs uses.
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();
    let handle_id = handle.id.clone();

    // ── Visibility guard ──────────────────────────────────────────────────────
    // Mirrors Blazor's `@if(Visible)` — when false, renders nothing at all.
    // Confirmed by Badge_NotVisible_DoesNotRender in BadgeTests.cs.
    let visible = handle.visible;

    view! {
        <Show when=move || visible.get()>
            <span
                id=handle_id.clone()
                class=combined_class.clone()
                style=style.clone()
                on:mouseenter={
                    let cb = enter_cb.clone();
                    move |ev| cb(ev)
                }
                on:mouseleave={
                    let cb = leave_cb.clone();
                    move |ev| cb(ev)
                }
                on:contextmenu={
                    let cb = ctx_cb.clone();
                    move |ev| cb(ev)
                }
            >
                // ChildContent takes priority over Text — mirrors Blazor's
                // `ChildContent ?? (_ => builder.AddContent(0, Text))`.
                {match children.as_ref() {
                    Some(c) => view! { {c()} }.into_any(),
                    None => view! {
                        {text.clone().unwrap_or_default()}
                    }.into_any(),
                }}
            </span>
        </Show>
    }
}
