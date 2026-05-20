use leptos::prelude::*;
use crate::components::{
    base_component::{ComponentProps, use_radzen_base},
    alert_style::AlertStyle,
    alert_size::AlertSize,
    variant::Variant,
    shade::Shade,
    ClassList,
};

/// RadzenAlert component
/// A notification/alert box for displaying contextual messages
#[component]
pub fn RadzenAlert(
    /// Base component properties (id, class, style, etc.)
    #[prop(default = Default::default())]
    base: ComponentProps,
    /// Semantic style (Success, Warning, Danger, Info, etc.)
    #[prop(default = AlertStyle::Base)]
    alert_style: AlertStyle,
    /// Visual variant (Filled, Flat, Outlined, Text)
    #[prop(default = Variant::Filled)]
    variant: Variant,
    /// Color shade intensity
    #[prop(default = Shade::Default)]
    shade: Shade,
    /// Size of the alert
    #[prop(default = AlertSize::Medium)]
    size: AlertSize,
    /// Alert title
    #[prop(default = String::new(), into = true)]
    title: String,
    /// Alert text content
    #[prop(default = String::new(), into = true)]
    text: String,
    /// Whether to show the contextual icon
    #[prop(default = true)]
    show_icon: bool,
    /// Whether to allow closing the alert
    #[prop(default = true)]
    allow_close: bool,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "rz-alert");
    let show_alert = RwSignal::new(true);

    let handle_close = move || {
        show_alert.set(false);
    };

    let classes = ClassList::default()
        .add("rz-alert", true)
        .add_alert_style(alert_style)
        .add_alert_size(size)
        .add_variant(variant)
        .add_shade(shade);

    let combined_class = format!("{} {}", handle.css_class, classes.finish());

    let default_icon = match alert_style {
        AlertStyle::Success => "check_circle",
        AlertStyle::Warning => "warning_amber",
        AlertStyle::Danger => "error",
        AlertStyle::Info => "info",
        _ => "info",
    };

    // Clone non-reactive values before view! macro
    let title_cloned = title.clone();
    let title_cloned2 = title.clone();
    let text_cloned = text.clone();
    let text_cloned2 = text.clone();
    let handle_id = handle.id.clone();
    let handle_visible = handle.visible;
    let handle_mouse_enter = handle.on_mouse_enter.clone();
    let handle_mouse_leave = handle.on_mouse_leave.clone();
    let handle_context_menu = handle.on_context_menu.clone();

    let base_style = base.style.clone().unwrap_or_default();

    view! {
        <div
            id=handle_id
            class=combined_class
            style=move || {
                let mut style = base_style.clone();
                if !handle_visible.get() || !show_alert.get() {
                    if !style.is_empty() && !style.ends_with(';') {
                        style.push(';');
                    }
                    style.push_str("display: none;");
                }
                style
            }
            role="alert"
            aria_live="polite"
            on:mouseenter=move |ev| handle_mouse_enter(ev)
            on:mouseleave=move |ev| handle_mouse_leave(ev)
            on:contextmenu=move |ev| handle_context_menu(ev)
        >
            <div class="rz-alert-item">
                <Show when=move || show_icon>
                    <span class="rzi rz-alert-icon">
                        {default_icon}
                    </span>
                </Show>

                <div class="rz-alert-message">
                    <Show when=move || !title_cloned.is_empty()>
                        <div class="rz-alert-title">{title_cloned2.clone()}</div>
                    </Show>

                    <Show when=move || !text_cloned.is_empty()>
                        <div>{text_cloned2.clone()}</div>
                    </Show>
                </div>
            </div>

            <Show when=move || allow_close>
                <button
                    class="rz-button rz-variant-text rz-button-sm"
                    on:click=move |_| handle_close()
                    aria_label="Close alert"
                    style="flex-shrink: 0; align-self: flex-start; padding: 0; min-width: auto; height: auto;">
                    <span class="rzi">"close"</span>
                </button>
            </Show>
        </div>
    }
}
