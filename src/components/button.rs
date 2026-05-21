//! RadzenButton component — mirrors C# Radzen.Blazor.RadzenButton.
//!
//! A clickable button with support for multiple visual styles, icons, images,
//! and loading states. Supports variants (Filled, Flat, Outlined, Text), color
//! styles (Primary, Secondary, Success, etc.), and sizes (ExtraSmall, Small, Medium, Large).

use crate::components::base_component::*;
use crate::components::renderer::ClassList;
use crate::components::{ButtonSize, ButtonStyle, ButtonType, Shade, Variant};
use leptos::prelude::*;

/// Render a RadzenButton component.
///
/// # Example
/// ```rust,ignore
/// use radzen_leptos::components::{RadzenButton, ButtonStyle};
///
/// view! {
///     <RadzenButton
///         text="Click me".to_string()
///         button_style=ButtonStyle::Primary
///     />
/// }
/// ```
#[component]
pub fn RadzenButton(
    /// Base component properties (id, class, style, etc.)
    #[prop(default = Default::default())]
    base: ComponentProps,
    /// Button text label. If both text and icon are set, both are displayed.
    /// Ignored when `children` is provided.
    #[prop(default = String::new())]
    text: String,
    /// Material icon name (e.g., "save", "delete", "add").
    /// Rendered using the `rzi` icon font. Ignored when `children` is provided.
    #[prop(default = None)]
    icon: Option<String>,
    /// Custom color for the icon (CSS value, e.g., "#FF0000", "var(--my-color)").
    /// Overrides the button style and variant.
    #[prop(default = None)]
    icon_color: Option<String>,
    /// URL or path to an image to display in the button.
    /// For icon fonts, use `icon` instead. Ignored when `children` is provided.
    #[prop(default = None)]
    image: Option<String>,
    /// Alt text for the image (defaults to "button").
    #[prop(default = "button".to_string())]
    image_alt_text: String,
    /// Semantic color style of the button.
    #[prop(default = ButtonStyle::Primary)]
    button_style: ButtonStyle,
    /// HTML `type` attribute (`button`, `submit`, `reset`).
    #[prop(default = ButtonType::Button)]
    button_type: ButtonType,
    /// Visual appearance variant (Filled, Flat, Outlined, Text).
    #[prop(default = Variant::Filled)]
    variant: Variant,
    /// Color intensity shade (Default, Light, Dark, Lighter, Darker).
    #[prop(default = Shade::Default)]
    shade: Shade,
    /// Button size (ExtraSmall, Small, Medium, Large).
    #[prop(default = ButtonSize::Medium)]
    size: ButtonSize,
    /// Whether the button is disabled and cannot be clicked.
    #[prop(default = false)]
    disabled: bool,
    /// Callback invoked when the button is clicked.
    /// Only fires if the button is not disabled and not busy.
    #[prop(default = None)]
    on_click: Option<std::sync::Arc<dyn Fn(web_sys::MouseEvent) + Send + Sync>>,
    /// Whether the button is in a loading/busy state.
    /// When true, shows a loading indicator, displays `busy_text`, and becomes disabled.
    #[prop(default = false)]
    is_busy: bool,
    /// Text displayed when `is_busy` is true.
    #[prop(default = String::new())]
    busy_text: String,
    /// Tab index for keyboard navigation.
    #[prop(default = 0)]
    tab_index: i32,
    /// Optional child content. When provided, replaces Text / Icon / Image entirely —
    /// mirrors Blazor's `ChildContent` RenderFragment on RadzenButton.
    /// `ChildrenFn` implements `Fn` (unlike `Children` which is `FnOnce`), so it
    /// can be called directly inside the view! closure with no wrapper needed.
    #[prop(optional)]
    children: Option<ChildrenFn>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "rz-button");

    // ── Compute effective disabled state ─────────────────────────────────────
    let is_disabled = disabled || is_busy;

    let has_children = children.is_some();

    // ── CSS class — mirrors Blazor GetComponentCssClass exactly ──────────────
    // Order: rz-button → size → variant → style → disabled → shade → icon-only
    //
    // Fix #1: `rz-button-icon-only` condition — Blazor checks only
    // `string.IsNullOrEmpty(Text) && !string.IsNullOrEmpty(Icon)`,
    // with no ChildContent guard. Match that exactly.
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
        // Merge any caller-supplied class from base.attrs
        .add_option(
            base.attrs
                .as_ref()
                .and_then(|a| a.get("class"))
                .map(String::as_str),
        )
        .finish();

    // ── Button type ──────────────────────────────────────────────────────────
    let button_type_str = button_type.as_str();

    // ── Prepare rendering values as signals ──────────────────────────────────
    let text_sig = RwSignal::new(text);
    let icon_sig = RwSignal::new(icon);
    let icon_color_sig = RwSignal::new(icon_color);
    let image_sig = RwSignal::new(image);
    let image_alt_text_sig = RwSignal::new(image_alt_text);
    let busy_text_sig = RwSignal::new(busy_text);
    let is_busy_sig = RwSignal::new(is_busy);

    // ── Re-entrancy guard — mirrors Blazor's `_clicking` boolean flag ────────
    // Fix #3: Prevents simultaneous/re-entrant clicks (e.g. rapid double-clicks
    // before an async handler resolves). In Blazor this is `private bool _clicking`.
    let clicking = RwSignal::new(false);

    // ── Event handler ────────────────────────────────────────────────────────
    // Fix #4: Explicitly guard against disabled state at the Rust level,
    // matching Blazor's `if (IsDisabled) { return; }` check inside OnClick.
    // The HTML `disabled` attribute prevents browser events, but this guard
    // ensures correctness even if the attribute is bypassed programmatically.
    let on_click_cb = on_click.clone();
    let on_button_click = move |ev: web_sys::MouseEvent| {
        // Disabled guard — mirrors Blazor: `if (IsDisabled) { return; }`
        if is_disabled {
            return;
        }
        // Re-entrancy guard — mirrors Blazor: `if (_clicking) { return; }`
        if clicking.get_untracked() {
            return;
        }
        clicking.set(true);
        if let Some(cb) = &on_click_cb {
            cb(ev);
        }
        clicking.set(false);
    };

    // ── Visibility / display style ───────────────────────────────────────────
    let display_style = if !handle.visible.get_untracked() {
        if let Some(s) = base.style.clone() {
            Some(format!("{}; display: none", s))
        } else {
            Some("display: none".to_string())
        }
    } else {
        base.style.clone()
    };

    // ── Clone handle event handlers for use in the view ──────────────────────
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
            tabindex=if is_disabled { -1 } else { tab_index }
            on:click=on_button_click
            on:mouseenter=move |ev| handle_mouse_enter(ev)
            on:mouseleave=move |ev| handle_mouse_leave(ev)
            on:contextmenu=move |ev| handle_context_menu(ev)
        >
            <span class="rz-button-box">
                // ── Mirrors Blazor structure exactly: ────────────────────────
                // if ChildContent != null  →  render it (bypasses busy state)
                // else if IsBusy          →  spinner + busy text
                // else                    →  icon / image / text
                {children.as_ref().map(|c| c())}

                <Show when=move || !has_children && is_busy_sig.get()>
                    // Busy spinner — mirrors Blazor's <RadzenIcon> output:
                    // <i class="notranslate rzi"> (no rz-button-icon-left)
                    // animation: rotation — space after colon matches Blazor exactly
                    <i class="notranslate rzi" style="animation: rotation 700ms linear infinite">
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
                    // Icon
                    {move || {
                        icon_sig.get().map(|icon_val| {
                            let icon_style = icon_color_sig
                                .get()
                                .as_ref()
                                // Matches Blazor: style="color:{IconColor}" — no space after colon
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

                    // Fix #2: Image — mirrors Blazor exactly:
                    // class="notranslate rz-button-icon-left rzi"
                    // The previous port was missing `notranslate`.
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

                    // Text label
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