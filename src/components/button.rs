//! RadzenButton component — mirrors C# Radzen.Blazor.RadzenButton.
//!
//! `on_click` is an async handler (boxed `Future<Output = ()>`), mirroring
//! Blazor's `EventCallback<MouseEventArgs>` which is `async Task`.
//! The `_clicking` re-entrancy guard remains `true` for the full duration
//! of the async task before resetting — identical to Blazor's await behaviour.
//!
//! Sync callers just return `async {}`:
//! ```rust,ignore
//! on_click=Some(Arc::new(|_ev| Box::pin(async { do_sync_thing(); })))
//! ```

use crate::components::base_component::*;
use crate::components::renderer::ClassList;
use crate::components::{ButtonSize, ButtonStyle, ButtonType, Shade, Variant};
use leptos::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Boxed async click handler — mirrors Blazor's `async Task OnClick(MouseEventArgs)`.
pub type AsyncClickFuture = Pin<Box<dyn Future<Output = ()>>>;
pub type AsyncClickHandler = Arc<dyn Fn(web_sys::MouseEvent) -> AsyncClickFuture + Send + Sync>;

#[component]
pub fn RadzenButton(
    /// Base component properties (id, class, style, etc.)
    #[prop(default = Default::default())]
    base: ComponentProps,
    /// Button text label.
    #[prop(default = String::new())]
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
    #[prop(default = "image".to_string())]
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
    /// Async click callback — mirrors Blazor's `EventCallback<MouseEventArgs>`
    /// (`async Task`). The `_clicking` guard stays true for the full duration
    /// of the future, preventing re-entrant clicks during async work.
    ///
    /// For sync work: `Some(Arc::new(|_ev| Box::pin(async { … })))`
    /// For async work: `Some(Arc::new(|ev| Box::pin(async move { fetch().await; })))`
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
    let handle = use_radzen_base(&base, "rz-button");

    let is_disabled = disabled || is_busy;
    let has_children = children.is_some();

    let css_class = ClassList::new()
        .add_class("rz-button")
        .add_button_size(size)
        .add_variant(variant)
        .add_button_style(button_style)
        .add_disabled(is_disabled)
        .add_shade(shade)
        .add(
            "rz-button-icon-only",
            text.trim().is_empty() && icon.is_some(),
        )
        .add_option(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish();

    let button_type_str = button_type.as_str();

    let text_sig = RwSignal::new(text);
    let icon_sig = RwSignal::new(icon);
    let icon_color_sig = RwSignal::new(icon_color);
    let image_sig = RwSignal::new(image);
    let image_alt_text_sig = RwSignal::new(image_alt_text);
    let busy_text_sig = RwSignal::new(busy_text);
    let is_busy_sig = RwSignal::new(is_busy);

    // ── Re-entrancy guard — mirrors Blazor's `private bool _clicking` ────────
    // Stays `true` for the entire duration of the async future, so rapid
    // double-clicks are dropped until the previous handler resolves.
    let clicking = RwSignal::new(false);

    // ── Async click handler ───────────────────────────────────────────────────
    // `spawn_local` drives the future on the WASM microtask queue.
    // The guard is set before spawn and cleared inside the spawned task,
    // so it remains true for the full async duration — identical to Blazor's:
    //   _clicking = true;
    //   await OnClick.InvokeAsync(args);
    //   _clicking = false;
    let on_click_cb = on_click.clone();
    let on_button_click = move |ev: web_sys::MouseEvent| {
        if is_disabled || clicking.get_untracked() {
            return;
        }
        clicking.set(true);

        if let Some(cb) = on_click_cb.clone() {
            let fut = cb(ev);
            wasm_bindgen_futures::spawn_local(async move {
                fut.await;
                clicking.set(false);
            });
        } else {
            clicking.set(false);
        }
    };

    let display_style = if !handle.visible.get_untracked() {
        if let Some(s) = base.style.clone() {
            Some(format!("{}; display: none", s))
        } else {
            Some("display: none".to_string())
        }
    } else {
        base.style.clone()
    };

    let handle_mouse_enter = handle.on_mouse_enter.clone();
    let handle_mouse_leave = handle.on_mouse_leave.clone();
    let handle_context_menu = handle.on_context_menu.clone();
    let handle_id = handle.id;

    view! {
        <button
            id=handle_id
            type=button_type_str
            class=css_class
            style=display_style
            disabled=is_disabled
            tabindex=if disabled { -1 } else { tab_index }
            on:click=on_button_click
            on:mouseenter=move |ev| handle_mouse_enter(ev)
            on:mouseleave=move |ev| handle_mouse_leave(ev)
            on:contextmenu=move |ev| handle_context_menu(ev)
        >
            <span class="rz-button-box">
                {children.as_ref().map(|c| c())}

                <Show when=move || !has_children && is_busy_sig.get()>
                    <i class="notranslate rzi" style="animation: button-icon-spin 700ms linear infinite">
                        "refresh"
                    </i>
                    {move || {
                        let busy = busy_text_sig.get();
                        (!busy.is_empty()).then(|| {
                            view! { <span class="rz-button-text">{busy}</span> }
                        })
                    }}
                </Show>

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
                            let alt_text = image_alt_text_sig.get();
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
            </span>
        </button>
    }
}
