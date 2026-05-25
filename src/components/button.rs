//! RadzenButton component — mirrors C# Radzen.Blazor.RadzenButton.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `rz-button rz-button-{size} rz-variant-{v} rz-{style} [rz-state-disabled] rz-shade-{s} [rz-button-icon-only] [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! ClassList.Create("rz-button")
//!     .AddButtonSize(Size)
//!     .AddVariant(Variant)
//!     .AddButtonStyle(ButtonStyle)
//!     .AddDisabled(IsDisabled)
//!     .AddShade(Shade)
//!     .Add("rz-button-icon-only", string.IsNullOrEmpty(Text) && !string.IsNullOrEmpty(Icon))
//!     .ToString()
//! ```
//!
//! # Visibility
//! Uses `<Show>` (full DOM removal when invisible), matching Blazor's `@if (Visible)`.
//! Previously the component used `display: none` inline style — that was wrong.
//!
//! # Busy spinner
//! Blazor razor: `<i class="notranslate rzi rz-spin">refresh</i>`
//! The `rz-spin` class is defined in Radzen's SCSS and must be used rather than
//! an inline `animation:` style so the theme system controls the animation.
//!
//! # Async click handler
//! `on_click` is a boxed `Future<Output = ()>`, mirroring Blazor's
//! `EventCallback<MouseEventArgs>` (`async Task`). The `_clicking` re-entrancy
//! guard stays `true` for the full duration of the future.

use crate::components::base_component::*;
use crate::components::renderer::ClassList;
use crate::components::{ButtonSize, ButtonStyle, ButtonType, Shade, Variant};
use leptos::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Boxed async click handler.
pub type AsyncClickFuture = Pin<Box<dyn Future<Output = ()>>>;
pub type AsyncClickHandler = Arc<dyn Fn(web_sys::MouseEvent) -> AsyncClickFuture + Send + Sync>;

#[component]
pub fn RadzenButton(
    /// Base component properties (id, class, style, etc.)
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Button text label.
    #[prop(default = String::new(), into)]
    text: String,

    /// Material icon name (e.g., "save", "delete").
    #[prop(default = None)]
    icon: Option<String>,

    /// Custom CSS color for the icon.
    #[prop(default = None)]
    icon_color: Option<String>,

    /// URL/path to an image to display in the button.
    #[prop(default = None)]
    image: Option<String>,

    /// Alt text for the image (mirrors Blazor `ImageAlternateText`).
    #[prop(default = "image".to_string(), into)]
    image_alt_text: String,

    /// Semantic color style.
    #[prop(default = ButtonStyle::Primary)]
    button_style: ButtonStyle,

    /// HTML `type` attribute.
    #[prop(default = ButtonType::Button)]
    button_type: ButtonType,

    /// Visual variant.
    #[prop(default = Variant::Filled)]
    variant: Variant,

    /// Color intensity shade.
    #[prop(default = Shade::Default)]
    shade: Shade,

    /// Button size.
    #[prop(default = ButtonSize::Medium)]
    size: ButtonSize,

    /// Whether the button is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Async click callback — mirrors Blazor's `EventCallback<MouseEventArgs>`.
    #[prop(default = None)]
    on_click: Option<AsyncClickHandler>,

    /// Whether the button is in a loading/busy state.
    #[prop(default = false)]
    is_busy: bool,

    /// Text shown while `is_busy` is true.
    #[prop(default = String::new())]
    busy_text: String,

    /// Tab index for keyboard navigation.
    #[prop(default = 0)]
    tab_index: i32,

    /// Optional child content — replaces Text/Icon/Image when provided.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    let is_disabled = disabled || is_busy;
    let has_children = children.is_some();

    // ── CSS class ─────────────────────────────────────────────────────────────
    // Mirrors Blazor GetComponentCssClass() exactly:
    //   ClassList.Create("rz-button")
    //       .AddButtonSize(Size)
    //       .AddVariant(Variant)
    //       .AddButtonStyle(ButtonStyle)
    //       .AddDisabled(IsDisabled)
    //       .AddShade(Shade)
    //       .Add("rz-button-icon-only", ...)
    // then GetCssClass appends caller class last.
    let css_class = ClassList::create("rz-button")
        .add_button_size(size)
        .add_variant(variant)
        .add_button_style(button_style)
        .add_disabled(is_disabled)
        .add_shade(shade)
        .add(
            "rz-button-icon-only",
            text.trim().is_empty() && icon.is_some(),
        )
        .add_caller_class(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish();

    let button_type_str = button_type.as_str();
    let style = base.style.clone().unwrap_or_default();

    let text_sig = RwSignal::new(text);
    let icon_sig = RwSignal::new(icon);
    let icon_color_sig = RwSignal::new(icon_color);
    let image_sig = RwSignal::new(image);
    let image_alt_sig = RwSignal::new(image_alt_text);
    let busy_text_sig = RwSignal::new(busy_text);
    let is_busy_sig = RwSignal::new(is_busy);

    // ── Re-entrancy guard ─────────────────────────────────────────────────────
    let clicking = RwSignal::new(false);

    // ── Async click handler ───────────────────────────────────────────────────
    // Wrap in Arc so the closure captures an Arc<Option<...>> and can clone it
    // on every invocation — making the closure `Fn` rather than `FnOnce`.
    let on_click_cb = Arc::new(on_click);
    let on_button_click = move |ev: web_sys::MouseEvent| {
        if is_disabled || clicking.get_untracked() {
            return;
        }
        clicking.set(true);

        if let Some(ref cb) = *on_click_cb.clone() {
            let fut = cb(ev);
            wasm_bindgen_futures::spawn_local(async move {
                fut.await;
                clicking.set(false);
            });
        } else {
            clicking.set(false);
        }
    };

    let handle_mouse_enter = handle.on_mouse_enter.clone();
    let handle_mouse_leave = handle.on_mouse_leave.clone();
    let handle_context_menu = handle.on_context_menu.clone();
    let handle_id = handle.id;

    // ── Visibility — mirrors `@if (Visible)` ──────────────────────────────────
    // Early-return (same pattern as RadzenStack / RadzenText) avoids the
    // FnOnce problem: on_button_click captures non-Clone state so it cannot
    // live inside a <Show> children Fn closure.
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    Some(
        leptos::html::button()
            .attr("id", handle_id)
            .attr("type", button_type_str)
            .attr("class", css_class)
            .attr("style", style)
            .attr("disabled", is_disabled)
            .attr("tabindex", if disabled { -1 } else { tab_index })
            .on(leptos::ev::click, on_button_click)
            .on(leptos::ev::mouseenter, move |ev| handle_mouse_enter(ev))
            .on(leptos::ev::mouseleave, move |ev| handle_mouse_leave(ev))
            .on(leptos::ev::contextmenu, move |ev| handle_context_menu(ev))
            .child(
                leptos::html::span()
                    .attr("class", "rz-button-box")
                    .child(children.as_ref().map(|c| c()))
                    // ── Busy state ─────────────────────────────────────────
                    // Blazor razor: <i class="notranslate rzi rz-spin">refresh</i>
                    // `rz-spin` is the Radzen SCSS class for the spin animation.
                    .child(view! {
                        <Show when=move || !has_children && is_busy_sig.get()>
                            <i class="notranslate rzi rz-spin">"refresh"</i>
                            {move || {
                                let busy = busy_text_sig.get();
                                (!busy.is_empty()).then(|| {
                                    view! { <span class="rz-button-text">{busy}</span> }
                                })
                            }}
                        </Show>
                    })
                    // ── Normal state ────────────────────────────────────────
                    .child(view! {
                        <Show when=move || !has_children && !is_busy_sig.get()>
                            {move || {
                                icon_sig.get().map(|icon_val| {
                                    let icon_style = icon_color_sig
                                        .get()
                                        .as_ref()
                                        .map(|c| format!("color:{}", c));
                                    view! {
                                        <i
                                            class="notranslate rz-button-icon-left rzi"
                                            style=icon_style
                                        >
                                            {icon_val}
                                        </i>
                                    }
                                })
                            }}
                            {move || {
                                image_sig.get().map(|img_src| {
                                    let alt_text = image_alt_sig.get();
                                    view! {
                                        <img
                                            class="notranslate rz-button-icon-left rzi"
                                            src=img_src
                                            alt=alt_text
                                        />
                                    }
                                })
                            }}
                            {move || {
                                let txt = text_sig.get();
                                (!txt.trim().is_empty()).then(|| {
                                    view! { <span class="rz-button-text">{txt}</span> }
                                })
                            }}
                        </Show>
                    }),
            ),
    )
    .into_any()
}
