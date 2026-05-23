//! RadzenText component — mirrors C# Radzen.Blazor.RadzenText.
//!
//! ## Blazor source reference: `Radzen.Blazor/RadzenText.cs`
//!
//! ### CSS class construction
//! Mirrors `BuildRenderTree` exactly:
//! ```csharp
//! var @class = ClassList.Create(className)   // style class is the root
//!     .Add(Attributes)                        // caller attrs["class"] merged
//!     .Add(alignClassName, TextAlign != TextAlign.Left)
//!     .ToString();
//! ```
//! `TextAlign.Left` resolves to `"rz-text-align-left"` internally but is
//! **never appended** (condition false) — `css_class()` returns `None` for
//! `Left` to reproduce this.
//!
//! ### Attribute emit order
//! Mirrors C# builder attribute order:
//!   `style` → spread `Attributes` (handled via `base.style` / `base.attrs`)
//!   → `class` → `id`
//!
//! ### `Anchor` prop semantics
//! In C# `Anchor` is the full path string e.g. `"typography#text-align"`.
//! `GetAnchor()` splits on `#` and takes the last fragment.
//! In our CSR-only port we simply accept a bare anchor id — the `href` is
//! always `#anchor` (no NavigationManager), and location-change scrolling is
//! omitted (no IJSRuntime injection in CSR Leptos).
//!
//! ### `RadzenTextAnchor`
//! Mirrors `BuildRenderTree` of the inner `RadzenTextAnchor` class:
//!   `<a id="{anchor}" href="{path}#{anchor}" class="rz-link">`
//!     `<RadzenIcon Icon="link" />`
//!   `</a>`
//! In CSR the href is `#{anchor}` (no NavigationManager.Uri).
//!
//! ### Visibility
//! Mirrors `if (Visible)` — element is fully omitted, not `display:none`.
//! Uses early-return `None::<AnyView>.into_any()` to avoid the
//! `Fn` vs `FnOnce` conflict that arises with `<Show>` + builder API.

use crate::components::{
    RadzenIcon, TagName, TextAlign, TextStyle,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;

/// RadzenText component.
///
/// Renders text with consistent Material Design typography.  The HTML tag is
/// resolved automatically from [`TextStyle`] unless overridden by [`TagName`].
///
/// # Content priority
/// `text` takes precedence over `children` — mirrors Blazor's
/// `if (!string.IsNullOrEmpty(Text))`.
///
/// # CSS class order
/// `{style_class} [{caller_class}] [{align_class}]`
/// The alignment class is omitted when `text_align` is `TextAlign::Left`.
///
/// # Anchor
/// `anchor` accepts a bare id string (e.g. `"my-heading"`).  A
/// `<a id="my-heading" href="#my-heading" class="rz-link"><RadzenIcon … /></a>`
/// is appended inside the element.  In C# `Anchor` accepts a full path string
/// (`"page#my-heading"`); in our CSR-only port the bare id is sufficient.
#[component]
pub fn RadzenText(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Typography style controlling the CSS class and (when `tag_name` is
    /// `Auto`) the HTML element tag.  Default: [`TextStyle::Body1`] → `<p>`.
    #[prop(default = TextStyle::Body1)]
    text_style: TextStyle,

    /// Horizontal text alignment. Default: [`TextAlign::Left`] (no class emitted).
    ///
    /// Mirrors Blazor: `.Add(alignClassName, TextAlign != TextAlign.Left)`.
    #[prop(default = TextAlign::Left)]
    text_align: TextAlign,

    /// Explicit HTML tag override. Default: [`TagName::Auto`] defers to
    /// [`TextStyle::auto_tag`].
    #[prop(default = TagName::Auto)]
    tag_name: TagName,

    /// Plain-text content. Takes precedence over `children` when both are set.
    ///
    /// Mirrors C#: `if (!string.IsNullOrEmpty(Text)) AddContent(Text)`.
    #[prop(default = None, into)]
    text: Option<String>,

    /// Anchor identifier for linkable headings.
    ///
    /// When set, appends:
    /// `<a id="{anchor}" href="#{anchor}" class="rz-link"><RadzenIcon icon="link"/></a>`
    ///
    /// Mirrors C# `RadzenTextAnchor` inner component.
    #[prop(default = None, into)]
    anchor: Option<String>,

    /// Optional child content — used when `text` is `None`.
    ///
    /// Mirrors C#: `RenderFragment? ChildContent`.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    // ── use_radzen_base ────────────────────────────────────────────────────────
    // RadzenText uses the style class as the root (e.g. "rz-text-h3"), not a
    // fixed "rz-text" prefix.  Pass "" so we get id/visible/locale/events for
    // free without an extra class being prepended.
    let handle = use_radzen_base(&base, "");

    // Visibility — mirrors `if (Visible)` in BuildRenderTree.
    // Early-return avoids FnOnce conflict with the builder API inside <Show>.
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── Resolve tag name ───────────────────────────────────────────────────────
    // Stage 1: TextStyle sets tagName.
    // Stage 2: TagName (if not Auto) overrides.
    // Mirrors the two switch blocks in BuildRenderTree.
    let resolved_tag: String = tag_name
        .as_str()
        .unwrap_or_else(|| text_style.auto_tag())
        .to_string();

    // ── CSS class ──────────────────────────────────────────────────────────────
    // Mirrors:
    //   ClassList.Create(className)
    //       .Add(Attributes)                          ← attrs["class"] key
    //       .Add(alignClassName, TextAlign != Left)   ← conditional
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

    // ── Attribute values ───────────────────────────────────────────────────────
    // C# attribute emit order: style → spread Attributes → class → id.
    let style     = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();
    let enter_cb  = handle.on_mouse_enter.clone();
    let leave_cb  = handle.on_mouse_leave.clone();
    let ctx_cb    = handle.on_context_menu.clone();

    // ── Anchor child ───────────────────────────────────────────────────────────
    // Mirrors RadzenTextAnchor.BuildRenderTree:
    //   builder.OpenElement(1, "a");
    //   builder.AddAttribute(2, "id", GetAnchor());
    //   builder.AddAttribute(3, "href", GetPath());   // path#anchor in C#
    //   builder.AddAttribute(4, "class", "rz-link");
    //   builder.OpenComponent<RadzenIcon>(7);
    //   builder.AddAttribute(8, "Icon", "link");
    //
    // CSR simplification: href = "#anchor" (no NavigationManager.Uri).
    // GetAnchor() in C# splits "page#fragment" on '#' — we accept bare ids.
    let anchor_child: Option<AnyView> = anchor.as_deref().map(|anc| {
        let href = format!("#{anc}");
        view! {
            <a id=anc.to_string() href=href class="rz-link">
                <RadzenIcon icon=Some("link".to_string()) />
            </a>
        }
        .into_any()
    });

    // ── Content child ──────────────────────────────────────────────────────────
    // Mirrors:
    //   if (!string.IsNullOrEmpty(Text)) AddContent(5, Text)
    //   else AddContent(5, ChildContent)
    let content_child: AnyView = match text {
        Some(ref t) => t.clone().into_any(),
        None => match children.as_ref() {
            Some(c) => c().into_any(),
            None    => "".into_any(),
        },
    };

    // ── Render ─────────────────────────────────────────────────────────────────
    // `leptos::html::custom(tag_str)` = Blazor's `builder.OpenElement(0, tagName)`.
    // Attribute order mirrors C#: style → class → id.
    Some(
        leptos::html::custom(resolved_tag)
            .attr("style", style)
            .attr("class", css_class)
            .attr("id",    handle_id)
            .on(leptos::ev::mouseenter,  move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave,  move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            .child(content_child)
            .child(anchor_child)
    )
    .into_any()
}