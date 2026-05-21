//! RadzenCard component — mirrors C# Radzen.Blazor.RadzenCard.
//!
//! Blazor source:
//!   `ClassList.Create("rz-card").AddVariant(Variant).ToString()`
//!
//! `use_radzen_base(&base, "rz-card")` already puts `"rz-card"` into
//! `handle.css_class` (and appends any caller `attrs["class"]`), so the
//! `ClassList` here only appends the variant — no duplication.
//!
//! Mirrors `RadzenComponentWithChildren` — `children` is always required.
//! `ChildrenFn` (not `Children`) is used because the `<Show>` wrapper
//! requires its body closure to be `Fn` (potentially called multiple
//! times), while `Children` / `FnOnce` can only be called once.
//! Visibility guard and all three mouse-event handlers are wired identically
//! to `RadzenBadge` and `RadzenButton`.

use crate::components::{
    ClassList, Variant,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;

/// RadzenCard component.
///
/// A versatile styled container for displaying information, images, actions,
/// and other content in a structured format. Supports Filled, Flat, Outlined,
/// and Text visual variants that affect the card's appearance.
#[component]
pub fn RadzenCard(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,
    /// Visual variant (Filled, Flat, Outlined, Text). Default: `Variant::Filled`.
    #[prop(default = Variant::Filled)]
    variant: Variant,
    /// Child content — always required (mirrors `RadzenComponentWithChildren`).
    /// `ChildrenFn` rather than `Children` because the `<Show>` closure is `Fn`.
    children: ChildrenFn,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "rz-card");

    // CSS class — mirrors Blazor:
    //   ClassList.Create("rz-card").AddVariant(Variant).ToString()
    //
    // `handle.css_class` already contains "rz-card" (+ any caller attrs class),
    // so ClassList only appends the variant class.
    let variant_class = ClassList::new().add_variant(variant).finish();

    let combined_class = if variant_class.is_empty() {
        handle.css_class.clone()
    } else {
        format!("{} {}", handle.css_class, variant_class)
    };

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
                {children()}
            </div>
        </Show>
    }
}