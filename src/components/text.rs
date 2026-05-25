//! RadzenText component — mirrors C# Radzen.Blazor.RadzenText.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `{style-class} [caller-class] [align-class]`
//!
//! Blazor `BuildRenderTree`:
//! ```csharp
//! var @class = ClassList.Create(className)   // style class is the root
//!     .Add(Attributes)                        // caller attrs["class"] merged
//!     .Add(alignClassName, TextAlign != TextAlign.Left)
//!     .ToString();
//! ```
//! Note: for RadzenText, caller class comes **before** the align class (unlike
//! other components where caller class is strictly last). This mirrors Blazor's
//! class-building order exactly.
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted, not `display:none`.

use crate::components::{
    ClassList, RadzenIcon, TagName, TextAlign, TextStyle,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;

/// RadzenText component.
#[component]
pub fn RadzenText(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Typography style. Default: [`TextStyle::Body1`] → `<p>`.
    #[prop(default = TextStyle::Body1)]
    text_style: TextStyle,

    /// Horizontal text alignment. Default: [`TextAlign::Left`] (no class emitted).
    #[prop(default = TextAlign::Left)]
    text_align: TextAlign,

    /// Explicit HTML tag override. Default: [`TagName::Auto`].
    #[prop(default = TagName::Auto)]
    tag_name: TagName,

    /// Plain-text content. Takes precedence over `children`.
    #[prop(default = None, into)]
    text: Option<String>,

    /// Anchor identifier for linkable headings.
    #[prop(default = None, into)]
    anchor: Option<String>,

    /// Optional child content — used when `text` is `None`.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // Visibility — mirrors `@if (Visible)`.
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── Resolve tag name ───────────────────────────────────────────────────────
    let resolved_tag: String = tag_name
        .as_str()
        .unwrap_or_else(|| text_style.auto_tag())
        .to_string();

    // ── CSS class ─────────────────────────────────────────────────────────────
    // Mirrors Blazor BuildRenderTree:
    //   ClassList.Create(className)     ← style class is the root
    //       .Add(Attributes)            ← caller attrs["class"] — before align
    //       .Add(alignClassName, ...)   ← align class comes last
    //
    // This is the one component where caller class comes before the align class,
    // matching Blazor's literal class-building order.
    let css_class = ClassList::create(text_style.css_class())
        .add_caller_class(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .add_text_align(text_align)
        .finish();

    // ── Attribute values ───────────────────────────────────────────────────────
    let style = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();

    // ── Anchor child ───────────────────────────────────────────────────────────
    let anchor_child: Option<AnyView> = anchor.as_deref().map(|anc| {
        let href = format!("#{anc}");
        view! {
            <a id=anc.to_string() href=href class="rz-link" target="_top">
                <RadzenIcon icon=Some("link".to_string()) />
            </a>
        }
        .into_any()
    });

    // ── Content child ──────────────────────────────────────────────────────────
    let content_child: AnyView = match text {
        Some(ref t) => t.clone().into_any(),
        None => match children.as_ref() {
            Some(c) => c().into_any(),
            None => "".into_any(),
        },
    };

    // ── Render ─────────────────────────────────────────────────────────────────
    Some(
        leptos::html::custom(resolved_tag)
            .attr("style", style)
            .attr("class", css_class)
            .attr("id", handle_id)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            .child(content_child)
            .child(anchor_child),
    )
    .into_any()
}
