//! RadzenAlert component — mirrors C# Radzen.Blazor.RadzenAlert.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `rz-alert rz-alert-{size} rz-variant-{v} rz-shade-{s} rz-{style} [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! ClassList.Create("rz-alert")
//!     .Add($"rz-alert-{GetAlertSize()}")   // xs|sm|md|lg
//!     .AddVariant(Variant)
//!     .AddShade(Shade)
//!     .Add($"rz-{AlertStyle.ToString().ToLowerInvariant()}")
//!     .ToString()
//! ```
//! `RadzenComponent.GetCssClass()` then appends any caller `class` attribute last.
//!
//! # Visibility / dismissal
//! Blazor keeps a `bool visible` field toggled to `false` by the close button.
//! We mirror this with `display:none` toggling via an `RwSignal<bool>`.
//!
//! # Close button helpers  (mirrors C# private methods)
//! | Method                   | Rule                                                           |
//! |--------------------------|----------------------------------------------------------------|
//! | `GetCloseButtonSize()`   | `ExtraSmall` alert → `ButtonSize::ExtraSmall`, else `Small`    |
//! | `GetCloseButtonShade()`  | Light/Lighter shade → `Shade::Darker`, else `Shade::Default`   |
//! | `GetCloseButtonStyle()`  | Light/Lighter shade: each AlertStyle → matching ButtonStyle     |
//! |                          | Other shades: Light/Base alert → `Dark`, else `Light` button   |

use crate::components::{
    AlertSize, AlertStyle, ButtonSize, ButtonStyle, ClassList, RadzenButton, Shade, Variant,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;
use std::sync::Arc;

/// RadzenAlert component.
///
/// An alert/notification box for displaying contextual feedback messages with
/// semantic colors, optional close functionality, and automatic icons.
#[component]
pub fn RadzenAlert(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Semantic style / severity. Default: [`AlertStyle::Base`].
    #[prop(default = AlertStyle::Base)]
    alert_style: AlertStyle,

    /// Visual variant. Default: [`Variant::Filled`].
    #[prop(default = Variant::Filled)]
    variant: Variant,

    /// Color shade intensity. Default: [`Shade::Default`].
    #[prop(default = Shade::Default)]
    shade: Shade,

    /// Alert size. Default: [`AlertSize::Medium`].
    #[prop(default = AlertSize::Medium)]
    size: AlertSize,

    /// Whether to show the close button. Default: `true`.
    #[prop(default = true)]
    allow_close: bool,

    /// Whether to show the contextual icon. Default: `true`.
    #[prop(default = true)]
    show_icon: bool,

    /// Optional title shown prominently above the alert content.
    #[prop(default = None, into)]
    title: Option<String>,

    /// Body text. Ignored when `children` is provided.
    #[prop(default = None, into)]
    text: Option<String>,

    /// Custom Material icon name — overrides the automatic style-based icon.
    #[prop(default = None, into)]
    icon: Option<String>,

    /// CSS color for the icon.
    #[prop(default = None, into)]
    icon_color: Option<String>,

    /// Fired when the alert is closed by the user.
    #[prop(default = None)]
    on_close: Option<Arc<dyn Fn() + Send + Sync>>,

    /// Fired with the new visibility value when the alert shows or hides.
    #[prop(default = None)]
    on_visible_changed: Option<Arc<dyn Fn(bool) + Send + Sync>>,

    /// Optional rich child content — overrides `text` when provided.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // ── Visibility — mirrors Blazor's `bool visible` field ────────────────────
    let visible = RwSignal::new(handle.visible.get_untracked());

    if !visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── CSS class ─────────────────────────────────────────────────────────────
    let css_class = ClassList::create("rz-alert")
        .add_class(format!("rz-alert-{}", size.css_suffix()))
        .add_variant(variant)
        .add_shade(shade)
        .add_class(format!("rz-{}", alert_style.as_str()))
        .add_caller_class(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish();

    // ── Reactive style — display:none when dismissed ──────────────────────────
    let caller_style = base.style.clone().unwrap_or_default();
    let style = move || {
        if visible.get() {
            caller_style.clone()
        } else if caller_style.is_empty() {
            "display:none".to_string()
        } else {
            format!("display:none; {}", caller_style)
        }
    };

    let handle_id = handle.id.clone();
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();

    // ── Resolved icon name — mirrors GetIcon() ────────────────────────────────
    let icon_name = icon.unwrap_or_else(|| alert_style.default_icon().to_string());
    let icon_style = icon_color
        .as_deref()
        .map(|c| format!("color:{};", c))
        .unwrap_or_default();

    // ── Close button helpers ──────────────────────────────────────────────────
    let close_button_size = if size == AlertSize::ExtraSmall {
        ButtonSize::ExtraSmall
    } else {
        ButtonSize::Small
    };
    let close_button_shade = if shade.clone() == Shade::Light || shade == Shade::Lighter {
        Shade::Darker
    } else {
        Shade::Default
    };
    let close_button_style = if shade == Shade::Light || shade == Shade::Lighter {
        match alert_style {
            AlertStyle::Success   => ButtonStyle::Success,
            AlertStyle::Danger    => ButtonStyle::Danger,
            AlertStyle::Warning   => ButtonStyle::Warning,
            AlertStyle::Info      => ButtonStyle::Info,
            AlertStyle::Primary   => ButtonStyle::Primary,
            AlertStyle::Secondary => ButtonStyle::Secondary,
            AlertStyle::Light | AlertStyle::Base => ButtonStyle::Dark,
            AlertStyle::Dark      => ButtonStyle::Light,
        }
    } else {
        match alert_style {
            AlertStyle::Light | AlertStyle::Base => ButtonStyle::Dark,
            _                                    => ButtonStyle::Light,
        }
    };

    // ── Close handler ─────────────────────────────────────────────────────────
    let on_close_cb = on_close.clone();
    let on_visible_changed_cb = on_visible_changed.clone();
    let on_close_handler: crate::components::AsyncClickHandler =
        Arc::new(move |_ev: web_sys::MouseEvent| {
            let cb_close = on_close_cb.clone();
            let cb_vis = on_visible_changed_cb.clone();
            Box::pin(async move {
                visible.set(false);
                if let Some(cb) = cb_close { cb(); }
                if let Some(cb) = cb_vis { cb(false); }
            })
        });

    // ── Store close-button props in signals so view! closures stay Fn ─────────
    // The view! macro turns each `{...}` expression into a closure that must be
    // Fn + Send + Sync.  Non-Copy enum values (ButtonStyle, ButtonSize, Shade)
    // captured by move would make those closures FnOnce.  Wrapping them in
    // StoredValue gives us a Copy handle that reads them back each time.
    let close_style_sv   = StoredValue::new(close_button_style);
    let close_shade_sv   = StoredValue::new(close_button_shade);
    let close_size_sv    = StoredValue::new(close_button_size);
    let handler_sv       = StoredValue::new(on_close_handler);
    let icon_name_sv     = StoredValue::new(icon_name);
    let icon_style_sv    = StoredValue::new(icon_style);
    let title_sv         = StoredValue::new(title);
    let text_sv          = StoredValue::new(text);
    let css_class_sv     = StoredValue::new(css_class);
    let handle_id_sv     = StoredValue::new(handle_id);

    Some(
        leptos::html::div()
            .attr("id",         move || handle_id_sv.get_value())
            .attr("class",      move || css_class_sv.get_value())
            .attr("style",      style)
            .attr("aria-live",  "polite")
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            .child(
                // rz-alert-item
                leptos::html::div()
                    .attr("class", "rz-alert-item")
                    .child(move || {
                        // Icon — mirrors @if (ShowIcon)
                        show_icon.then(|| {
                            leptos::html::i()
                                .attr("class", "notranslate rzi rz-alert-icon")
                                .attr("style", icon_style_sv.get_value())
                                .child(icon_name_sv.get_value())
                        })
                    })
                    .child(
                        // rz-alert-message
                        leptos::html::div()
                            .attr("class", "rz-alert-message")
                            .child(move || {
                                // Title — mirrors @if (!string.IsNullOrEmpty(Title))
                                title_sv.get_value().map(|t| {
                                    leptos::html::div()
                                        .attr("class", "rz-alert-title")
                                        .child(t)
                                })
                            })
                            .child(
                                // rz-alert-content — ChildContent ?? Text
                                leptos::html::div()
                                    .attr("class", "rz-alert-content")
                                    .child(move || {
                                        match children.as_ref() {
                                            Some(c) => c().into_any(),
                                            None    => text_sv
                                                        .get_value()
                                                        .unwrap_or_default()
                                                        .into_any(),
                                        }
                                    }),
                            ),
                    ),
            )
            .child(move || {
                // Close button — mirrors @if (AllowClose)
                allow_close.then(|| {
                    let handler = handler_sv.get_value();
                    view! {
                        <RadzenButton
                            icon=Some("close".to_string())
                            variant=Variant::Text
                            button_style=close_style_sv.get_value()
                            shade=close_shade_sv.get_value()
                            size=close_size_sv.get_value()
                            on_click=Some(handler)
                        />
                    }
                })
            }),
    )
    .into_any()
}