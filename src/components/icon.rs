//! RadzenIcon component — mirrors C# Radzen.Blazor.RadzenIcon.
//!
//! Blazor razor template (abbreviated):
//!   ```razor
//!   @if (Visible) {
//!       <i @ref="@Element" @attributes="Attributes"
//!          class="@GetCssClass()" style="@getStyle()" id="@GetId()">@Icon</i>
//!   }
//!   ```
//!
//! CSS class — mirrors `GetComponentCssClass()`:
//!   `$"notranslate rzi{(IconStyle.HasValue ? $" rzi-{IconStyle.Value.ToString().ToLowerInvariant()}" : "")}"`
//!
//! Style — mirrors `getStyle()`:
//!   `$"{(!string.IsNullOrEmpty(IconColor) ? $"color:{IconColor};" : null)}{Style}"`
//!
//! Visibility: uses `@if (Visible)` — same as RadzenBadge, component is fully
//! removed from the DOM when invisible (not display:none like RadzenCard).

use crate::components::{
    IconStyle,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;

/// RadzenIcon component.
///
/// Displays a Material Symbols icon glyph (2,500+ icons) using the embedded
/// variable font. Icon names use underscores, e.g. `"home"`, `"account_circle"`.
///
/// Color is controlled via `icon_color` (inline `color:` style) or CSS inheritance.
/// The visual variant (Outlined / Filled / Rounded / Sharp) is set via `icon_style`.
#[component]
pub fn RadzenIcon(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Material Symbols icon name, e.g. `"home"`, `"settings"`, `"check_circle"`.
    ///
    /// When `None` the `<i>` renders empty (matching Blazor's `string? Icon`).
    #[prop(default = None, into)]
    icon: Option<String>,

    /// Custom CSS color for the icon, e.g. `"#FF0000"`, `"var(--rz-primary)"`.
    ///
    /// When `Some`, prepended as `color:{value};` in the style attribute —
    /// mirrors Blazor's `!string.IsNullOrEmpty(IconColor) ? $"color:{IconColor};" : null`.
    /// When `None`, the icon inherits the current text color.
    #[prop(default = None, into)]
    icon_color: Option<String>,

    /// Visual style variant of the icon. Default: `None` (Outlined, no class added).
    ///
    /// `None` mirrors Blazor's nullable `IconStyle? IconStyle` — no `rzi-*` class
    /// is emitted, and the font renders in its default Outlined appearance.
    /// Set to `Some(IconStyle::Filled)` etc. to add `rzi-filled` and so on.
    #[prop(default = None)]
    icon_style: Option<IconStyle>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "notranslate rzi");

    // ── CSS class ──────────────────────────────────────────────────────────────
    // Mirrors GetComponentCssClass():
    //   $"notranslate rzi{(IconStyle.HasValue ? $" rzi-{...ToLowerInvariant()}" : "")}"
    //
    // use_radzen_base was called with "notranslate rzi" as the component class,
    // so handle.css_class already contains that (+ any caller attrs["class"]).
    // We only append the optional rzi-* variant class.
    let css_class = match icon_style {
        Some(style) => format!("{} rzi-{}", handle.css_class, style.as_str()),
        None => handle.css_class.clone(),
    };

    // ── Style ──────────────────────────────────────────────────────────────────
    // Mirrors getStyle():
    //   $"{(IconColor != null ? $"color:{IconColor};" : null)}{Style}"
    //
    // IconColor is prepended, then the base Style string follows.
    let style = {
        let color_part = icon_color
            .as_deref()
            .map(|c| format!("color:{};", c))
            .unwrap_or_default();
        let base_style = base.style.clone().unwrap_or_default();
        format!("{}{}", color_part, base_style)
    };

    // ── Event handlers ─────────────────────────────────────────────────────────
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();
    let handle_id = handle.id.clone();

    // ── Visibility — @if (Visible), same pattern as RadzenBadge ───────────────
    // Component is fully removed from the DOM when invisible.
    let visible = handle.visible;

    view! {
        <Show when=move || visible.get()>
            <i
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
                {icon.clone().unwrap_or_default()}
            </i>
        </Show>
    }
}