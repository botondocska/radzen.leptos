//! RadzenLabel component — mirrors C# Radzen.Blazor.RadzenLabel.
//!
//! # CSS class
//! `rz-label [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! protected override string GetComponentCssClass() => "rz-label";
//! ```
//! `RadzenComponent.GetCssClass()` then appends any caller `class` attribute last.
//!
//! # Razor template structure
//! ```html
//! @if (Visible) {
//!     <label id="@GetId()"
//!            for="@Component"
//!            class="@GetCssClass()"
//!            style="@Style"
//!            @attributes="Attributes">
//!         @(ChildContent ?? (RenderFragment)(_ => builder.AddContent(0, Text)))
//!     </label>
//! }
//! ```
//!
//! # Properties
//! | C# Parameter    | Rust prop     | Notes                                              |
//! |-----------------|---------------|----------------------------------------------------|
//! | `Text`          | `text`        | Plain-text label content                           |
//! | `Component`     | `component`   | Maps to HTML `for` attr; matches input `Name`      |
//! | `ChildContent`  | `children`    | Rich markup — overrides `text` when provided       |
//!
//! # Content priority
//! `children` (ChildContent) takes precedence over `text` when both are
//! supplied — mirrors Blazor's `@(ChildContent ?? ...)` pattern.
//!
//! # Component / for attribute
//! When `component` is `None` the `for` attribute is omitted entirely —
//! mirrors Blazor's nullable `string? Component` param (`null` → no attr).
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted when invisible (same
//! early-return pattern as `RadzenText`, `RadzenStack`, `RadzenLink`).

use crate::components::{
    ClassList,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;

/// RadzenLabel component.
///
/// A label element for associating descriptive text with form input components.
/// Creates the HTML `for`/`id` relationship that allows clicking the label to
/// focus the associated input, improving accessibility and usability.
///
/// # Association
/// Set `component` to the `Name` prop of the target Radzen input component.
/// This produces `<label for="{name}" …>` which browsers use to route clicks
/// to the correct input element.
///
/// # Content
/// Provide either `text` (plain string) or `children` (rich markup). When
/// both are given, `children` wins — mirroring Blazor's `ChildContent ?? Text`
/// precedence.
///
/// # Examples
/// ```rust,ignore
/// // Simple text label associated with a named input
/// <RadzenLabel text="Email address" component=Some("email_input") />
/// <RadzenTextBox name="email_input" … />
///
/// // Rich content label with required indicator
/// <RadzenLabel component=Some("password_input")>
///     "Password " <span style="color: red;">"*"</span>
/// </RadzenLabel>
/// ```
#[component]
pub fn RadzenLabel(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Plain-text label content.
    ///
    /// Ignored when `children` is also provided — mirrors Blazor's
    /// `@(ChildContent ?? (RenderFragment)(_ => builder.AddContent(0, Text)))`.
    #[prop(default = None, into)]
    text: Option<String>,

    /// Name of the associated input component.
    ///
    /// Must match the `Name` prop of the target Radzen input to create the
    /// proper `<label for="…">` relationship. When `None` the `for` attribute
    /// is omitted entirely — mirrors C# `public string? Component { get; set; }`.
    #[prop(default = None, into)]
    component: Option<String>,

    /// Optional child content — overrides `text` when provided.
    ///
    /// Mirrors Blazor's `RenderFragment? ChildContent`.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // ── Visibility — mirrors `@if (Visible)` ──────────────────────────────────
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── CSS class ─────────────────────────────────────────────────────────────
    // GetComponentCssClass() returns "rz-label".
    // GetCssClass() appends caller attrs["class"] last.
    let css_class = ClassList::create("rz-label")
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

    // ── Content — mirrors ChildContent ?? Text ────────────────────────────────
    let content: AnyView = match children.as_ref() {
        Some(c) => c().into_any(),
        None => text.clone().unwrap_or_default().into_any(),
    };

    // ── Render ────────────────────────────────────────────────────────────────
    // The `for` attribute is only emitted when `component` is `Some` — mirrors
    // Blazor's nullable Component property (null → no for attribute).
    Some(
        leptos::html::label()
            .attr("id", handle_id)
            .attr("class", css_class)
            .attr("style", style)
            // `for` is a reserved Rust keyword; leptos accepts it as a string key.
            .attr("for", component)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            .child(content),
    )
    .into_any()
}