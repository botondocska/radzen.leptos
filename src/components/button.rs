//! RadzenButton component — mirrors C# Radzen.Blazor.RadzenButton.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `rz-button rz-button-{size} rz-variant-{v} rz-{style} [rz-state-disabled] rz-shade-{s} [rz-button-icon-only] [caller-class]`
//!
//! # Changes from the old implementation
//! - `base: ComponentProps` is gone.
//! - All base props (`style`, `visible`, `id`, `attrs`, `on_mouse_enter`, …)
//!   are now flat props injected automatically by `#[radzen_component]`.
//! - `handle` is still available exactly as before — the macro wires it.
//! - Caller attrs are accessed via the injected `attrs` binding instead of
//!   `base.attrs`.

use crate::components::renderer::ClassList;
use crate::components::{ButtonSize, ButtonStyle, ButtonType, Shade, Variant};
use radzen_macros::radzen_component;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Boxed async click handler.
pub type AsyncClickFuture = Pin<Box<dyn Future<Output = ()>>>;
pub type AsyncClickHandler = Arc<dyn Fn(web_sys::MouseEvent) -> AsyncClickFuture + Send + Sync>;

// ─────────────────────────────────────────────────────────────────────────────
// Component
// ─────────────────────────────────────────────────────────────────────────────

/// RadzenButton component.
///
/// # Base props (injected automatically — do not declare them yourself)
///
/// | Prop             | Type                        | Default  |
/// |------------------|-----------------------------|----------|
/// | `style`          | `Option<String>`            | `None`   |
/// | `visible`        | `bool`                      | `true`   |
/// | `id`             | `Option<String>`            | `None`   |
/// | `attrs`          | `Option<HashMap<…>>`        | `None`   |
/// | `locale`         | `Option<String>`            | `None`   |
/// | `on_mouse_enter` | `Option<Arc<dyn Fn(…)>>`   | `None`   |
/// | `on_mouse_leave` | `Option<Arc<dyn Fn(…)>>`   | `None`   |
/// | `on_context_menu`| `Option<Arc<dyn Fn(…)>>`   | `None`   |
#[radzen_component]
pub fn RadzenButton(
    // ── Button-specific props ─────────────────────────────────────────────────
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

    /// Alt text for the image.
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

    /// Async click callback.
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

    /// Optional child content.
    #[prop(optional)]
    children: Option<leptos::children::ChildrenFn>,
) -> impl leptos::prelude::IntoView {
    // `handle` is already in scope — wired by #[radzen_component].
    // `attrs`  is already in scope — injected as a flat prop.
    // `style`  is already in scope — injected as a flat prop.
    // `visible` is already in scope — handle.visible mirrors it.

    let is_disabled = disabled || is_busy;
    let has_children = children.is_some();

    // ── CSS class ─────────────────────────────────────────────────────────────
    // `attrs` is the injected HashMap prop — same pattern as before, just
    // without the `base.` prefix.
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
            attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish();

    let button_type_str = button_type.as_str();
    // `style` is the injected flat prop — no more `base.style`.
    let style_str = style.unwrap_or_default();

    let text_sig      = leptos::prelude::RwSignal::new(text);
    let icon_sig      = leptos::prelude::RwSignal::new(icon);
    let icon_color_sig = leptos::prelude::RwSignal::new(icon_color);
    let image_sig     = leptos::prelude::RwSignal::new(image);
    let image_alt_sig = leptos::prelude::RwSignal::new(image_alt_text);
    let busy_text_sig = leptos::prelude::RwSignal::new(busy_text);
    let is_busy_sig   = leptos::prelude::RwSignal::new(is_busy);

    // ── Re-entrancy guard ─────────────────────────────────────────────────────
    let clicking = leptos::prelude::RwSignal::new(false);

    // ── Async click handler ───────────────────────────────────────────────────
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

    // `handle` is in scope from the macro prelude.
    let handle_mouse_enter  = handle.on_mouse_enter.clone();
    let handle_mouse_leave  = handle.on_mouse_leave.clone();
    let handle_context_menu = handle.on_context_menu.clone();
    let handle_id           = handle.id;

    // ── Visibility — mirrors `@if (Visible)` ──────────────────────────────────
    // `handle.visible` is an RwSignal seeded from the `visible` flat prop.
    if !handle.visible.get_untracked() {
        return None::<leptos::prelude::AnyView>.into_any();
    }

    Some(
        leptos::html::button()
            .attr("id",       handle_id)
            .attr("type",     button_type_str)
            .attr("class",    css_class)
            .attr("style",    style_str)
            .attr("disabled", is_disabled)
            .attr("tabindex", if disabled { -1 } else { tab_index })
            .on(leptos::ev::click,        on_button_click)
            .on(leptos::ev::mouseenter,   move |ev| handle_mouse_enter(ev))
            .on(leptos::ev::mouseleave,   move |ev| handle_mouse_leave(ev))
            .on(leptos::ev::contextmenu,  move |ev| handle_context_menu(ev))
            .child(
                leptos::html::span()
                    .attr("class", "rz-button-box")
                    .child(children.as_ref().map(|c| c()))
                    // ── Busy state ──────────────────────────────────────────
                    .child(leptos::prelude::view! {
                        <leptos::prelude::Show when=move || !has_children && is_busy_sig.get()>
                            <i class="notranslate rzi rz-spin">"refresh"</i>
                            {move || {
                                let busy = busy_text_sig.get();
                                (!busy.is_empty()).then(|| {
                                    leptos::prelude::view! {
                                        <span class="rz-button-text">{busy}</span>
                                    }
                                })
                            }}
                        </leptos::prelude::Show>
                    })
                    // ── Normal state ────────────────────────────────────────
                    .child(leptos::prelude::view! {
                        <leptos::prelude::Show when=move || !has_children && !is_busy_sig.get()>
                            {move || {
                                icon_sig.get().map(|icon_val| {
                                    let icon_style = icon_color_sig
                                        .get()
                                        .as_ref()
                                        .map(|c| format!("color:{}", c));
                                    leptos::prelude::view! {
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
                                    leptos::prelude::view! {
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
                                    leptos::prelude::view! {
                                        <span class="rz-button-text">{txt}</span>
                                    }
                                })
                            }}
                        </leptos::prelude::Show>
                    }),
            ),
    )
    .into_any()
}