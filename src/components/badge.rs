//! RadzenBadge component — mirrors C# Radzen.Blazor.RadzenBadge.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `rz-badge rz-badge-{style} rz-variant-{v} rz-shade-{s} [rz-badge-pill] [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! ClassList.Create("rz-badge")
//!     .Add($"rz-badge-{BadgeStyle.ToString().ToLowerInvariant()}")
//!     .AddVariant(Variant)
//!     .AddShade(Shade)
//!     .Add("rz-badge-pill", IsPill)
//!     .ToString()
//! ```
//! `RadzenComponent.GetCssClass()` then appends any caller `class` attribute last.
//!
//! Content priority (mirrors Blazor): `children` (ChildContent) takes precedence
//! over `text` when both are supplied.
//!
//! Visibility: when `base.visible` is `false` the component renders **nothing**.

use crate::components::{
    BadgeStyle, ClassList, Shade, Variant,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;

/// RadzenBadge component.
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
    #[prop(default = false)]
    is_pill: bool,

    /// Optional child content — overrides `text` when provided.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    // use_radzen_base is called with "" — the full class is built by ClassList
    // below so that class ordering is fully controlled here, matching Blazor's
    // GetComponentCssClass() → GetCssClass() pipeline.
    let handle = use_radzen_base(&base, "");

    // ── CSS class ─────────────────────────────────────────────────────────────
    // Mirrors Blazor exactly:
    //   ClassList.Create("rz-badge")
    //       .Add($"rz-badge-{BadgeStyle.ToString().ToLowerInvariant()}")
    //       .AddVariant(Variant)
    //       .AddShade(Shade)
    //       .Add("rz-badge-pill", IsPill)
    //       [GetCssClass appends caller class last]
    let css_class = ClassList::create("rz-badge")
        .add_badge_style(badge_style)
        .add_variant(variant)
        .add_shade(shade)
        .add("rz-badge-pill", is_pill)
        .add_caller_class(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish();

    // ── Style ─────────────────────────────────────────────────────────────────
    let style = base.style.clone().unwrap_or_default();

    // ── Event handlers ────────────────────────────────────────────────────────
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();
    let handle_id = handle.id.clone();
    let visible = handle.visible;

    view! {
        <Show when=move || visible.get()>
            <span
                id=handle_id.clone()
                class=css_class.clone()
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
