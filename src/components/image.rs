//! RadzenImage component — mirrors C# Radzen.Blazor.RadzenImage.
//!
//! # CSS class
//! No component-specific root class — `GetComponentCssClass()` is inherited
//! from `RadzenComponentWithChildren` which itself falls back to the base
//! `RadzenComponent` returning `""`. The caller's `attrs["class"]` is still
//! appended via `GetCssClass()` and surfaced through `base.attrs`.
//!
//! # Razor template structure
//! ```html
//! @if (Visible) {
//!     <img src="@Path"
//!          style="@Style"
//!          class="@GetCssClass()"
//!          id="@GetId()"
//!          alt="@GetAlternateText()"
//!          @onclick="OnClick"
//!          @onkeydown="OnKeyDown"
//!          role="button"      <!-- only when Click.HasDelegate -->
//!          tabindex="0"       <!-- only when Click.HasDelegate -->
//!          @attributes="imgAttributes" />
//! }
//! ```
//!
//! # Click / keyboard interaction
//! When `on_click` is `Some`, the image gains `role="button"` and `tabindex="0"`
//! (matching Blazor's `Click.HasDelegate` branch). The `onkeydown` handler fires
//! the click callback when `Enter` or `Space` is pressed — mirroring Blazor's
//! `OnKeyDown` which checks `args.Code` (falling back to `args.Key`) for
//! `"Enter"` or `"Space"`.
//!
//! # Alternate text
//! Blazor's `GetAlternateText()` appends `attrs["alt"]` when present:
//!   `"$"{AlternateText} {attrs["alt"]}"` (or just `AlternateText` when absent).
//! We mirror this by checking `base.attrs["alt"]`.
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted, not `display:none`.
//! Uses the same early-return pattern as `RadzenText` and `RadzenButton`.

use crate::components::base_component::*;
use leptos::prelude::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Boxed async click handler for `RadzenImage` — mirrors Blazor's
/// `EventCallback<MouseEventArgs>` (`async Task`).
pub type ImageClickFuture = Pin<Box<dyn Future<Output = ()>>>;
pub type ImageClickHandler = Arc<dyn Fn(web_sys::MouseEvent) -> ImageClickFuture + Send + Sync>;

/// RadzenImage component.
///
/// Renders an HTML `<img>` element with optional click/keyboard interaction,
/// alternate text for accessibility, and full base-component support (id,
/// style, visibility, extra attrs, mouse-enter/leave/context-menu events).
///
/// # Sources
/// `path` accepts any value valid as an HTML `src` attribute:
/// - Relative or absolute file paths (`"images/logo.png"`)
/// - External URLs (`"https://example.com/photo.jpg"`)
/// - Base64 data URLs (`"data:image/jpeg;base64,..."`)
///
/// # Clickable images
/// Supply `on_click` to make the image interactive. When set the component
/// automatically adds `role="button"` and `tabindex="0"`, and the keyboard
/// handler fires the callback on `Enter` or `Space` — matching Blazor exactly.
///
/// # Alternate text
/// Set `alternate_text` for accessibility (screen readers and broken-image
/// fallback). If `base.attrs` contains an `"alt"` key its value is appended
/// to `alternate_text`, mirroring `GetAlternateText()` in the C# code-behind.
#[component]
pub fn RadzenImage(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Image source — any value valid as HTML `src`: relative path, absolute
    /// URL, or base64 data URL. Mirrors C# `Path`.
    #[prop(default = None, into)]
    path: Option<String>,

    /// Alternate text for accessibility and broken-image fallback.
    /// Default: `"image"` — mirrors C# `AlternateText = "image"`.
    ///
    /// When `base.attrs` contains an `"alt"` key its value is appended:
    /// `"{alternate_text} {attrs["alt"]}"` — mirrors `GetAlternateText()`.
    #[prop(default = "image".to_string(), into)]
    alternate_text: String,

    /// Async click callback — mirrors Blazor's `EventCallback<MouseEventArgs>`.
    ///
    /// When `Some`, the image gains `role="button"` and `tabindex="0"`, and
    /// keyboard `Enter`/`Space` also fires the callback.
    #[prop(default = None)]
    on_click: Option<ImageClickHandler>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // ── Visibility — mirrors `@if (Visible)` ──────────────────────────────────
    // Same early-return pattern as RadzenText / RadzenButton.
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── CSS class ─────────────────────────────────────────────────────────────
    // GetComponentCssClass() on RadzenImage returns "" (inherited from base).
    // GetCssClass() appends caller attrs["class"] — we reproduce that here.
    let css_class = base
        .attrs
        .as_ref()
        .and_then(|a| a.get("class"))
        .cloned()
        .unwrap_or_default();

    // ── Alternate text — mirrors GetAlternateText() ───────────────────────────
    // Blazor: if attrs["alt"] is set → return $"{AlternateText} {attrs["alt"]}"
    //         otherwise              → return AlternateText
    let alt_text = {
        let extra = base
            .attrs
            .as_ref()
            .and_then(|a| a.get("alt"))
            .map(String::as_str)
            .unwrap_or("");
        if extra.is_empty() {
            alternate_text.clone()
        } else {
            format!("{} {}", alternate_text, extra)
        }
    };

    // ── Style ─────────────────────────────────────────────────────────────────
    let style = base.style.clone().unwrap_or_default();

    // ── Click delegate flag — mirrors Click.HasDelegate ───────────────────────
    // When a click handler is provided the image is given role="button" and
    // tabindex="0", and keyboard Enter/Space fires the handler.
    let has_click = on_click.is_some();

    // ── Click handler ─────────────────────────────────────────────────────────
    // Arc-wrapped so the closure is Fn (not FnOnce) — same pattern as
    // RadzenButton's on_button_click.
    let on_click_cb = Arc::new(on_click);
    let on_click_cb_kb = on_click_cb.clone(); // clone for keydown handler

    let on_img_click = {
        let cb = on_click_cb.clone();
        move |ev: web_sys::MouseEvent| {
            if let Some(ref handler) = *cb {
                let fut = handler(ev);
                wasm_bindgen_futures::spawn_local(async move {
                    fut.await;
                });
            }
        }
    };

    // ── Keyboard handler — mirrors OnKeyDown ──────────────────────────────────
    // Blazor checks args.Code first, falling back to args.Key, for "Enter" or
    // "Space". We use the web_sys KeyboardEvent API to reproduce this.
    let on_key_down = move |ev: web_sys::KeyboardEvent| {
        if !has_click {
            return;
        }
        // Mirrors: var key = args.Code != null ? args.Code : args.Key;
        let key = {
            let code = ev.code();
            if code.is_empty() { ev.key() } else { code }
        };
        if key == "Enter" || key == "Space" {
            if let Some(ref handler) = *on_click_cb_kb {
                let fut = handler(web_sys::MouseEvent::new("click").unwrap_or_else(|_| {
                    // Fallback: construct a minimal MouseEvent — the handler
                    // receives it only for API compatibility; it carries no
                    // coordinates (mirrors `new MouseEventArgs()` in Blazor).
                    web_sys::MouseEvent::new("click").expect("MouseEvent::new")
                }));
                wasm_bindgen_futures::spawn_local(async move {
                    fut.await;
                });
            }
        }
    };

    // ── Base event handlers ───────────────────────────────────────────────────
    let handle_mouse_enter  = handle.on_mouse_enter.clone();
    let handle_mouse_leave  = handle.on_mouse_leave.clone();
    let handle_context_menu = handle.on_context_menu.clone();
    let handle_id           = handle.id;

    // ── Render ────────────────────────────────────────────────────────────────
    // Mirrors Blazor's single `<img>` element with conditional role/tabindex.
    Some(
        leptos::html::img()
            .attr("id",    handle_id)
            .attr("src",   path.unwrap_or_default())
            .attr("alt",   alt_text)
            .attr("class", css_class)
            .attr("style", style)
            // role="button" and tabindex="0" — only when click delegate present.
            // Mirrors: if (Click.HasDelegate) { imgAttributes["role"] = "button"; imgAttributes["tabindex"] = "0"; }
            .attr("role",     has_click.then_some("button"))
            .attr("tabindex", has_click.then_some("0"))
            .on(leptos::ev::click,       on_img_click)
            .on(leptos::ev::keydown,     on_key_down)
            .on(leptos::ev::mouseenter,  move |ev| handle_mouse_enter(ev))
            .on(leptos::ev::mouseleave,  move |ev| handle_mouse_leave(ev))
            .on(leptos::ev::contextmenu, move |ev| handle_context_menu(ev))
    )
    .into_any()
}