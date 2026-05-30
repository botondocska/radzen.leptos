//! RadzenAlert component — mirrors C# Radzen.Blazor.RadzenAlert.
//!
//! # CSS class order (mirrors Blazor exactly)
//! `rz-alert rz-alert-{size} rz-variant-{v} rz-{style} rz-shade-{s} [caller-class]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! ClassList.Create("rz-alert")
//!     .Add($"rz-alert-{GetAlertSize()}")   // xs|sm|md|lg
//!     .AddVariant(Variant)
//!     .Add($"rz-{AlertStyle.ToString().ToLowerInvariant()}")
//!     .AddShade(Shade)
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
//!
//! # Why Shade / Variant / ButtonStyle derive Copy
//! All three are simple C-like enums with no heap data.  Blazor accesses them
//! as C# properties (reference semantics) so they can be read multiple times
//! without moving.  Rust requires explicit `Copy` to allow the same pattern.
//! Deriving `Copy` is idiomatic for fieldless enums and is the right solution
//! here — it avoids `.clone()` noise and matches the intent of the Blazor code.

use crate::components::{
    AlertSize, AlertStyle, ButtonSize, ButtonStyle, ClassList, RadzenButton, RadzenIcon, Shade,
    Variant,
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
    // Mirrors GetComponentCssClass() exactly:
    //   ClassList.Create("rz-alert")
    //       .Add($"rz-alert-{GetAlertSize()}")
    //       .AddVariant(Variant)
    //       .AddShade(Shade)
    //       .Add($"rz-{AlertStyle.ToString().ToLowerInvariant()}")
    // then GetCssClass() appends caller class last.
    //
    // `shade` and `variant` are Copy so they can be used here and again in the
    // close-button helpers below without needing .clone().
    let css_class = ClassList::create("rz-alert")
        .add_class(format!("rz-alert-{}", size.css_suffix()))
        .add_variant(variant)
        .add_class(format!("rz-{}", alert_style.as_str()))
        .add_shade(shade)
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

    // ── Resolved icon — mirrors GetIcon() ────────────────────────────────────
    // Blazor: !string.IsNullOrEmpty(Icon) ? Icon : AlertStyle switch { … }
    let icon_name = icon.unwrap_or_else(|| alert_style.default_icon().to_string());

    // ── Close button helpers — mirrors C# private methods ────────────────────
    // `shade` and `alert_style` are Copy so multiple reads are fine.
    //
    // GetCloseButtonSize(): ExtraSmall alert → ExtraSmall button, else Small.
    let close_button_size = if size == AlertSize::ExtraSmall {
        ButtonSize::ExtraSmall
    } else {
        ButtonSize::Small
    };

    // GetCloseButtonShade(): Light/Lighter shade → Darker, else Default.
    let close_button_shade = if shade == Shade::Light || shade == Shade::Lighter {
        Shade::Darker
    } else {
        Shade::Default
    };

    // GetCloseButtonStyle(): depends on both shade and alert_style.
    let close_button_style = if shade == Shade::Light || shade == Shade::Lighter {
        match alert_style {
            AlertStyle::Success => ButtonStyle::Success,
            AlertStyle::Danger => ButtonStyle::Danger,
            AlertStyle::Warning => ButtonStyle::Warning,
            AlertStyle::Info => ButtonStyle::Info,
            AlertStyle::Primary => ButtonStyle::Primary,
            AlertStyle::Secondary => ButtonStyle::Secondary,
            AlertStyle::Light | AlertStyle::Base => ButtonStyle::Dark,
            AlertStyle::Dark => ButtonStyle::Light,
        }
    } else {
        match alert_style {
            AlertStyle::Light | AlertStyle::Base => ButtonStyle::Dark,
            _ => ButtonStyle::Light,
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
                if let Some(cb) = cb_close {
                    cb();
                }
                if let Some(cb) = cb_vis {
                    cb(false);
                }
            })
        });

    // ── Build all static children eagerly before the element builder ─────────
    //
    // Leptos requires closures passed to .child() to be FnMut (callable many
    // times). A closure that moves a non-Copy value (like an Arc) out of its
    // environment is FnOnce — it can only run once. The solution is to build
    // static content (guarded by non-reactive bools like allow_close /
    // show_icon) as Option<AnyView> values *before* the builder chain, then
    // pass them as plain values (not closures) to .child(). Only content that
    // genuinely needs to re-run on reactive signal changes stays as a closure.

    // @if (AllowClose) { <RadzenButton … /> }
    let close_button_child: Option<AnyView> = allow_close.then(|| {
        view! {
            <RadzenButton
                icon=Some("close".to_string())
                variant=Variant::Text
                button_style=close_button_style
                shade=close_button_shade
                size=close_button_size
                on_click=Some(on_close_handler)
            />
        }
        .into_any()
    });

    // @if (ShowIcon) { <RadzenIcon … class="rz-alert-icon" /> }
    let icon_child: Option<AnyView> = show_icon.then(|| {
        let mut icon_base = ComponentProps::default();
        let mut attrs = std::collections::HashMap::new();
        attrs.insert("class".to_string(), "rz-alert-icon".to_string());
        icon_base.attrs = Some(attrs);
        view! {
            <RadzenIcon
                base=icon_base
                icon=Some(icon_name)
                icon_color=icon_color
            />
        }
        .into_any()
    });

    // @if (!string.IsNullOrEmpty(Title)) { <div class="rz-alert-title">@Title</div> }
    let title_child: Option<AnyView> = title.map(|t| {
        leptos::html::div()
            .attr("class", "rz-alert-title")
            .child(t)
            .into_any()
    });

    // ── StoredValues only for data used inside reactive (FnMut) closures ──────
    // `children` and `text` live inside the rz-alert-content closure which
    // Leptos may re-invoke reactively. Everything else was consumed above.
    let text_sv = StoredValue::new(text);
    let css_class_sv = StoredValue::new(css_class);
    let handle_id_sv = StoredValue::new(handle_id);

    Some(
        leptos::html::div()
            .attr("id", move || handle_id_sv.get_value())
            .attr("class", move || css_class_sv.get_value())
            .attr("style", style)
            .attr("aria-live", "polite")
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            .child(
                // rz-alert-item
                leptos::html::div()
                    .attr("class", "rz-alert-item")
                    .child(icon_child)
                    .child(
                        // rz-alert-message
                        leptos::html::div()
                            .attr("class", "rz-alert-message")
                            .child(title_child)
                            .child(
                                // rz-alert-content — @(ChildContent ?? @Text)
                                leptos::html::div().attr("class", "rz-alert-content").child(
                                    move || match children.as_ref() {
                                        Some(c) => c().into_any(),
                                        None => text_sv.get_value().unwrap_or_default().into_any(),
                                    },
                                ),
                            ),
                    ),
            )
            .child(close_button_child),
    )
    .into_any()
}
