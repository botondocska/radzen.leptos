//! RadzenText component — mirrors C# Radzen.Blazor.RadzenText.
//!
//! The C# implementation uses `BuildRenderTree` with a runtime-resolved tag
//! name string (e.g. `"h1"`, `"p"`, `"span"`) derived from `TextStyle` and
//! overridden by `TagName`.  In Leptos 0.8 we replicate this with
//! `leptos::html::custom(Custom { tag_name: Cow::Owned(tag) })` which
//! produces an `HtmlElement<Custom, …>` — the correct way to emit an element
//! whose tag name is only known at runtime.
//!
//! CSS class construction — mirrors Blazor's `BuildRenderTree`:
//!   ```csharp
//!   ClassList.Create(className)          // e.g. "rz-text-h3"
//!       .Add(Attributes)                 // caller attrs["class"]
//!       .Add(alignClassName, TextAlign != TextAlign.Left)
//!       .ToString()
//!   ```
//!   The style class is the *root* of the ClassList here — RadzenText does NOT
//!   use a fixed "rz-text" prefix like button/badge do.  We replicate this
//!   exactly.
//!
//! Content priority — mirrors Blazor's `if (!string.IsNullOrEmpty(Text))`:
//!   `text` prop takes precedence over `children` when both are provided.
//!
//! Anchor — mirrors the nested `RadzenTextAnchor` component:
//!   When `anchor` is set, an `<a id="{anchor}" href="#{anchor}"
//!   class="rz-link">` with a link icon is appended inside the element.
//!
//! Visibility — uses `<Show>` (full DOM removal), same pattern as badge/icon.

use crate::components::{
    TagName, TextAlign, TextStyle,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::html::Custom;
use leptos::prelude::*;
use std::borrow::Cow;

/// RadzenText component.
///
/// Renders text with consistent Material-Design-inspired typography.
/// The HTML tag is resolved automatically from [`TextStyle`] (e.g. `H3` →
/// `<h3>`) unless overridden by [`TagName`].  Alignment, anchors, and all
/// standard base props are supported.
///
/// # Content priority
/// `text` takes precedence over `children` when both are supplied, mirroring
/// Blazor's `if (!string.IsNullOrEmpty(Text))` guard.
///
/// # CSS classes
/// Mirrors Blazor exactly:
/// ```text
/// {style_class} [{caller_class}] [{align_class}]
/// ```
/// The alignment class is **omitted** when `text_align` is `TextAlign::Left`
/// (the default) — identical to Blazor's conditional.
#[component]
pub fn RadzenText(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Typography style controlling the CSS class and (when `tag_name` is
    /// `Auto`) the HTML element tag.  Default: [`TextStyle::Body1`] → `<p>`.
    #[prop(default = TextStyle::Body1)]
    text_style: TextStyle,

    /// Horizontal text alignment.
    ///
    /// `Left` (default) emits no alignment class — mirrors Blazor's
    /// `.Add(alignClassName, TextAlign != TextAlign.Left)`.
    #[prop(default = TextAlign::Left)]
    text_align: TextAlign,

    /// Explicit HTML tag override.  Default: [`TagName::Auto`] defers to
    /// `text_style`'s [`TextStyle::auto_tag`].
    #[prop(default = TagName::Auto)]
    tag_name: TagName,

    /// Plain-text content.
    ///
    /// Takes precedence over `children` when both are provided — mirrors
    /// Blazor's `if (!string.IsNullOrEmpty(Text)) AddContent(Text)`.
    #[prop(default = None, into)]
    text: Option<String>,

    /// Anchor identifier for linkable headings.
    ///
    /// When set, an `<a id="{anchor}" href="#{anchor}" class="rz-link">`
    /// containing a link icon is appended inside the element — mirrors the
    /// nested `RadzenTextAnchor` in Blazor.
    #[prop(default = None, into)]
    anchor: Option<String>,

    /// Optional child content — used when `text` is `None`.
    ///
    /// Mirrors Blazor's `RenderFragment? ChildContent`.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    // ── use_radzen_base ───────────────────────────────────────────────────────
    // RadzenText does NOT use a fixed "rz-text" component class like button
    // does — the style class IS the root (e.g. "rz-text-h3").  We pass ""
    // so we still get id, visible, locale, and event-handler wiring for free.
    let handle = use_radzen_base(&base, "");

    // ── Resolve HTML tag name ─────────────────────────────────────────────────
    // Mirrors Blazor's two-stage switch:
    //   1. TextStyle sets tagName and className
    //   2. TagName (if not Auto) overrides tagName
    let resolved_tag: String = tag_name
        .as_str()
        .unwrap_or_else(|| text_style.auto_tag())
        .to_string();

    // ── CSS class ─────────────────────────────────────────────────────────────
    // Mirrors Blazor:
    //   ClassList.Create(className)    ← style class is the root
    //       .Add(Attributes)           ← caller attrs["class"]
    //       .Add(alignClassName, TextAlign != TextAlign.Left)
    //       .ToString()
    let style_class = text_style.css_class();

    let caller_class = base
        .attrs
        .as_ref()
        .and_then(|a| a.get("class"))
        .cloned()
        .unwrap_or_default();

    let css_class = {
        let mut parts: Vec<&str> = vec![style_class];
        if !caller_class.is_empty() {
            parts.push(caller_class.as_str());
        }
        if let Some(ac) = text_align.css_class() {
            parts.push(ac);
        }
        parts.join(" ")
    };

    let style      = base.style.clone().unwrap_or_default();
    let handle_id  = handle.id.clone();
    let visible    = handle.visible;
    let enter_cb   = handle.on_mouse_enter.clone();
    let leave_cb   = handle.on_mouse_leave.clone();
    let ctx_cb     = handle.on_context_menu.clone();

    // ── Dynamic-tag rendering ─────────────────────────────────────────────────
    // `Custom { tag_name: Cow::Owned(…) }` is the correct Leptos 0.8 / tachys
    // way to construct a runtime-determined tag.  Everything is moved in a
    // single block — nothing is cloned after this point.
    view! {
        <Show when=move || visible.get()>
            {
                // Build anchor child from raw String data — no pre-built view
                // to avoid the non-Clone HtmlElement issue.
                let anchor_child: Option<AnyView> = anchor.as_deref().map(|anc| {
                    let href = format!("#{anc}");
                    view! {
                        <a id=anc.to_string() href=href class="rz-link">
                            <i class="notranslate rzi">"link"</i>
                        </a>
                    }
                    .into_any()
                });

                // Build main content child — text takes priority over children.
                let content_child: AnyView = match text {
                    Some(ref t) => t.clone().into_any(),
                    None => match children.as_ref() {
                        Some(c) => c().into_any(),
                        None    => "".into_any(),
                    },
                };

                leptos::html::custom(Custom { tag_name: Cow::Owned(resolved_tag) })
                    .attr("id",    handle_id)
                    .attr("class", css_class)
                    .attr("style", style)
                    .on(leptos::ev::mouseenter,  move |ev| enter_cb(ev))
                    .on(leptos::ev::mouseleave,  move |ev| leave_cb(ev))
                    .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
                    .child(content_child)
                    .child(anchor_child)
            }
        </Show>
    }
}