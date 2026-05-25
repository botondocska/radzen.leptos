//! RadzenCard component — mirrors C# Radzen.Blazor.RadzenCard.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `rz-card rz-variant-{v} [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! ClassList.Create("rz-card").AddVariant(Variant).ToString()
//! ```
//! `RadzenComponent.GetCssClass()` then appends any caller `class` attribute last.

use crate::components::{
    ClassList, Variant,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;

/// RadzenCard component.
///
/// A versatile styled container for displaying information, images, actions,
/// and other content in a structured format. Supports Filled, Flat, Outlined,
/// and Text visual variants.
#[component]
pub fn RadzenCard(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Visual variant (Filled, Flat, Outlined, Text). Default: `Variant::Filled`.
    #[prop(default = Variant::Filled)]
    variant: Variant,

    /// Child content — always required (mirrors `RadzenComponentWithChildren`).
    children: ChildrenFn,
) -> impl IntoView {
    // use_radzen_base with "" — full class built by ClassList below.
    let handle = use_radzen_base(&base, "");

    // ── CSS class ─────────────────────────────────────────────────────────────
    // Mirrors Blazor: ClassList.Create("rz-card").AddVariant(Variant)
    // then GetCssClass appends caller class last.
    let css_class = ClassList::create("rz-card")
        .add_variant(variant)
        .add_caller_class(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish();

    let style = base.style.clone().unwrap_or_default();
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();
    let handle_id = handle.id.clone();
    let visible = handle.visible;

    view! {
        <Show when=move || visible.get()>
            <div
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
                {children()}
            </div>
        </Show>
    }
}
