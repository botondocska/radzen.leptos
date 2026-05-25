//! RadzenLink component — mirrors C# Radzen.Blazor.RadzenLink.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `rz-link [rz-link-disabled] [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! return Disabled ? "rz-link rz-link-disabled" : "rz-link";
//! ```
//! `RadzenComponent.GetCssClass()` then appends any caller `class` attribute last.
//!
//! # Razor template structure
//! ```html
//! <NavLink href="@GetPath()" class="@GetCssClass()" target="@GetTarget()" ...>
//!     <i class="notranslate rzi" style="color:{IconColor}">@Icon</i>   <!-- when Icon set -->
//!     <img class="notranslate rzi" src="@Image" alt="@ImageAlternateText" /> <!-- when Image set -->
//!     <span class="rz-link-text">@ChildContent ?? @Text</span>
//! </NavLink>
//! ```
//!
//! # Active state
//! Blazor's `<NavLink>` adds `class="active"` when the URL matches.
//! Leptos' `<A>` sets `aria-current="page"` instead — the Radzen theme's
//! CSS uses `[aria-current="page"]` as its active-link selector.
//!
//! # Disabled behaviour
//! Blazor's `GetPath()` returns `null` when disabled, so the rendered `<a>`
//! has **no `href` attribute** — clicking does nothing.
//! We mirror this by branching: disabled → plain `<a>` with no href,
//! enabled → `<A>` (router-enhanced anchor with href).
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted when invisible.

use crate::components::{
    ClassList,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;
use leptos_router::components::A;

/// How the active state of the link is determined.
///
/// Mirrors `Microsoft.AspNetCore.Components.Routing.NavLinkMatch`.
#[derive(Clone, PartialEq, Default, Debug)]
pub enum NavLinkMatch {
    /// Active when the current URL **starts with** the link path. Default.
    #[default]
    Prefix,
    /// Active only when the current URL matches the link path exactly.
    All,
}

/// RadzenLink component.
///
/// A hyperlink styled according to the Radzen theme, with optional icon/image,
/// disabled state, and active-URL highlighting via `aria-current`.
///
/// # Disabled links
/// When `disabled=true` the link renders as `<a>` with **no `href`** — exactly
/// as Blazor's `GetPath()` returning `null` does — so clicking has no effect.
/// The CSS class `rz-link-disabled` is also added.
///
/// # Active state
/// Active styling is driven by `aria-current="page"` (set automatically by
/// leptos_router's `<A>`). Use `[aria-current="page"]` in CSS to style it,
/// matching the Radzen theme's selector.
///
/// # Match mode
/// `NavLinkMatch::Prefix` (default) marks the link active when the current URL
/// starts with `path`. `NavLinkMatch::All` requires an exact match.
#[component]
pub fn RadzenLink(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// URL path for navigation (relative or absolute). Default: `""`.
    #[prop(default = String::new(), into)]
    path: String,

    /// Link text. Ignored when `children` is provided. Default: `""`.
    #[prop(default = String::new(), into)]
    text: String,

    /// Target window/frame (`"_blank"`, `"_self"`, …). Omitted when disabled.
    #[prop(default = None, into)]
    target: Option<String>,

    /// Material icon name (e.g. `"home"`, `"open_in_new"`).
    #[prop(default = None, into)]
    icon: Option<String>,

    /// CSS color for the icon (e.g. `"var(--rz-primary)"`, `"#FF0000"`).
    #[prop(default = None, into)]
    icon_color: Option<String>,

    /// Image URL shown instead of an icon.
    #[prop(default = None, into)]
    image: Option<String>,

    /// Alt text for the image. Default: `"image"`.
    #[prop(default = "image".to_string(), into)]
    image_alt_text: String,

    /// Whether the link is disabled.
    /// Disabled links render with no `href` (clicking does nothing) and gain
    /// the `rz-link-disabled` CSS class. `target` is also omitted.
    #[prop(default = false)]
    disabled: bool,

    /// Active-state match mode. Default: [`NavLinkMatch::Prefix`].
    #[prop(default = NavLinkMatch::Prefix)]
    match_: NavLinkMatch,

    /// Optional child content — overrides `text` when provided.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // Visibility — mirrors `@if (Visible)`.
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── CSS class ─────────────────────────────────────────────────────────────
    // Mirrors GetComponentCssClass():
    //   return Disabled ? "rz-link rz-link-disabled" : "rz-link";
    // then GetCssClass() appends caller class last.
    let css_class = ClassList::create("rz-link")
        .add("rz-link-disabled", disabled)
        .add_caller_class(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish();

    let style = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();

    // ── exact flag ────────────────────────────────────────────────────────────
    let exact = matches!(match_, NavLinkMatch::All);

    // ── Shared inner content ──────────────────────────────────────────────────
    // Icon — mirrors: @if (!string.IsNullOrEmpty(Icon)) { <i ...>@Icon</i> }
    let icon_child: Option<AnyView> = icon.as_ref().map(|icon_name| {
        let icon_style = icon_color
            .as_deref()
            .map(|c| format!("color:{}", c))
            .unwrap_or_default();
        view! {
            <i class="notranslate rzi" style=icon_style>
                {icon_name.clone()}
            </i>
        }
        .into_any()
    });

    // Image — mirrors: @if (!string.IsNullOrEmpty(Image)) { <img ...> }
    let image_child: Option<AnyView> = image.as_ref().map(|img_src| {
        view! {
            <img class="notranslate rzi" src=img_src.clone() alt=image_alt_text.clone() />
        }
        .into_any()
    });

    // Text / children — mirrors: <span class="rz-link-text">@(ChildContent ?? Text)</span>
    let text_child: AnyView = match children.as_ref() {
        Some(c) => c().into_any(),
        None => text.clone().into_any(),
    };

    // ── Render ────────────────────────────────────────────────────────────────
    // Blazor: GetPath() returns null when disabled → <a> has no href attribute.
    // We mirror this by branching:
    //   disabled → plain <a> with no href (browser ignores clicks)
    //   enabled  → <A> (leptos_router anchor, sets aria-current when active)
    if disabled {
        // Plain <a> with no href — clicking does nothing, matches Blazor behaviour.
        Some(view! {
            <a
                id=handle_id
                class=css_class
                style=style
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
                {icon_child}
                {image_child}
                <span class="rz-link-text">{text_child}</span>
            </a>
        })
        .into_any()
    } else {
        // Router-enhanced <A>: resolves relative routes, sets aria-current="page".
        Some(view! {
            <A
                href=path
                target=target.unwrap_or_default()
                exact=exact
                attr:id=handle_id
                attr:class=css_class
                attr:style=style
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
                {icon_child}
                {image_child}
                <span class="rz-link-text">{text_child}</span>
            </A>
        })
        .into_any()
    }
}
