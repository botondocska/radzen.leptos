//! RadzenIcon component — mirrors C# Radzen.Blazor.RadzenIcon.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `notranslate rzi [rzi-{style}] [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! $"notranslate rzi{(IconStyle.HasValue ? $" rzi-{IconStyle.Value.ToString().ToLowerInvariant()}" : "")}"
//! ```
//! `RadzenComponent.GetCssClass()` then appends any caller `class` attribute last.
//!
//! Style — mirrors `getStyle()`:
//! ```csharp
//! $"{(!string.IsNullOrEmpty(IconColor) ? $"color:{IconColor};" : null)}{Style}"
//! ```

use crate::components::{
    IconStyle,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;

/// RadzenIcon component.
#[component]
pub fn RadzenIcon(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Material Symbols icon name, e.g. `"home"`, `"settings"`, `"check_circle"`.
    #[prop(default = None, into)]
    icon: Option<String>,

    /// Custom CSS color for the icon, e.g. `"#FF0000"`, `"var(--rz-primary)"`.
    #[prop(default = None, into)]
    icon_color: Option<String>,

    /// Visual style variant of the icon. Default: `None` (Outlined, no class added).
    #[prop(default = None)]
    icon_style: Option<IconStyle>,
) -> impl IntoView {
    // use_radzen_base with "" — full class built by ClassList below so that
    // the rzi-{style} suffix always comes before any caller attrs["class"].
    let handle = use_radzen_base(&base, "");

    // ── CSS class ─────────────────────────────────────────────────────────────
    // Mirrors Blazor GetComponentCssClass():
    //   $"notranslate rzi{(IconStyle.HasValue ? $" rzi-{...ToLowerInvariant()}" : "")}"
    // then GetCssClass appends caller class last.
    let css_class = {
        let mut cl = crate::components::ClassList::create("notranslate rzi");
        if let Some(style) = icon_style {
            cl = cl.add_class(format!("rzi-{}", style.as_str()));
        }
        cl.add_caller_class(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish()
    };

    // ── Style ─────────────────────────────────────────────────────────────────
    // Mirrors getStyle(): color:{IconColor}; prepended before base Style.
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
