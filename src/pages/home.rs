use leptos::prelude::*;
use std::collections::HashMap;

use crate::components::{
    AlertSize, AlertStyle, RadzenLabel, AlignItems, BadgeStyle, ButtonSize, ButtonStyle, ComponentProps,
    FlexWrap, IconStyle, ImageClickFuture, ImageClickHandler, JustifyContent, NavLinkMatch,
    Orientation, RadzenAlert, RadzenBadge, RadzenButton, RadzenCard, RadzenIcon, RadzenImage,
    RadzenLink, RadzenStack, RadzenText, Shade, TagName, TextAlign, TextStyle, Variant,
};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    // ── Timed-alert demo state ────────────────────────────────────────────────
    // Each signal drives one alert's visibility. Clicking the matching button
    // sets it to `true`; a gloo timeout flips it back to `false` after 3 s.
    let show_success = RwSignal::new(false);
    let show_danger = RwSignal::new(false);
    let show_info = RwSignal::new(false);

    /// Trigger an alert for `ms` milliseconds then auto-dismiss it.
    fn trigger(signal: RwSignal<bool>, ms: u32) {
        signal.set(true);
        gloo_timers::callback::Timeout::new(ms, move || {
            signal.set(false);
        })
        .forget(); // hand ownership to the browser event loop
    }

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <p>"Errors: "</p>
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}
                </ul>
            }
        }>
            <div class="container">

                // ── Timed Alert Demo ──────────────────────────────────────────
                <h2>"Timed Alert Demo"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Click a button — the matching alert appears for 3 seconds then dismisses itself."
                </p>

                // Trigger buttons
                <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center; margin-bottom: 1rem;">
                    <RadzenButton
                        text="Show Success".to_string()
                        button_style=ButtonStyle::Success
                        icon=Some("check_circle".to_string())
                        on_click=Some(std::sync::Arc::new(move |_ev| {
                            Box::pin(async move { trigger(show_success, 3000); })
                        }))
                    />
                    <RadzenButton
                        text="Show Error".to_string()
                        button_style=ButtonStyle::Danger
                        icon=Some("error".to_string())
                        on_click=Some(std::sync::Arc::new(move |_ev| {
                            Box::pin(async move { trigger(show_danger, 3000); })
                        }))
                    />
                    <RadzenButton
                        text="Show Info".to_string()
                        button_style=ButtonStyle::Info
                        icon=Some("info".to_string())
                        on_click=Some(std::sync::Arc::new(move |_ev| {
                            Box::pin(async move { trigger(show_info, 3000); })
                        }))
                    />
                </div>

                // Alerts — rendered conditionally via Show
                <div style="display: flex; flex-direction: column; gap: 0.5rem; min-height: 3rem;">
                    <Show when=move || show_success.get()>
                        <RadzenAlert
                            alert_style=AlertStyle::Success
                            title=Some("Success!".to_string())
                            text=Some("The operation completed successfully.".to_string())
                            on_close=Some(std::sync::Arc::new(move || show_success.set(false)))
                        />
                    </Show>
                    <Show when=move || show_danger.get()>
                        <RadzenAlert
                            alert_style=AlertStyle::Danger
                            title=Some("Error!".to_string())
                            text=Some("Something went wrong. Please try again.".to_string())
                            on_close=Some(std::sync::Arc::new(move || show_danger.set(false)))
                        />
                    </Show>
                    <Show when=move || show_info.get()>
                        <RadzenAlert
                            alert_style=AlertStyle::Info
                            variant=Variant::Flat
                            shade=Shade::Lighter
                            title=Some("Did you know?".to_string())
                            text=Some("This alert will dismiss itself after 3 seconds.".to_string())
                            on_close=Some(std::sync::Arc::new(move || show_info.set(false)))
                        />
                    </Show>
                </div>

                // ── Buttons ───────────────────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Buttons"</h2>
                <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                    <RadzenButton
                        text="Primary".to_string()
                        button_style=ButtonStyle::Primary
                        size=ButtonSize::ExtraSmall
                        variant=Variant::Filled
                        shade=Shade::Dark
                    />
                    <RadzenButton
                        text="Save".to_string()
                        button_style=ButtonStyle::Success
                        size=ButtonSize::Small
                        variant=Variant::Flat
                        shade=Shade::Light
                    />
                    <RadzenButton
                        text="Delete".to_string()
                        button_style=ButtonStyle::Danger
                        size=ButtonSize::Medium
                        variant=Variant::Outlined
                        shade=Shade::Darker
                        base=ComponentProps {
                            attrs: Some(HashMap::from([
                                ("class".to_string(), "rz-border-radius-10".to_string())
                            ])),
                            ..Default::default()
                        }
                    />
                    <RadzenButton
                        text="Cancel".to_string()
                        button_style=ButtonStyle::Secondary
                        size=ButtonSize::Large
                        variant=Variant::Text
                        shade=Shade::Lighter
                    />
                </div>

                // ── Badges — Styles ───────────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Badges — Styles"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "All nine badge styles with Filled variant and Default shade."
                </p>
                <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center;">
                    <RadzenBadge text=Some("Primary".to_string())   badge_style=BadgeStyle::Primary />
                    <RadzenBadge text=Some("Secondary".to_string()) badge_style=BadgeStyle::Secondary />
                    <RadzenBadge text=Some("Success".to_string())   badge_style=BadgeStyle::Success />
                    <RadzenBadge text=Some("Danger".to_string())    badge_style=BadgeStyle::Danger />
                    <RadzenBadge text=Some("Warning".to_string())   badge_style=BadgeStyle::Warning />
                    <RadzenBadge text=Some("Info".to_string())      badge_style=BadgeStyle::Info />
                    <RadzenBadge text=Some("Light".to_string())     badge_style=BadgeStyle::Light />
                    <RadzenBadge text=Some("Dark".to_string())      badge_style=BadgeStyle::Dark />
                    <RadzenBadge text=Some("Base".to_string())      badge_style=BadgeStyle::Base />
                </div>

                // ── Badges — Variants ─────────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Badges — Variants"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Same Danger style across all four variants."
                </p>
                <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center;">
                    <RadzenBadge text=Some("Filled".to_string())   badge_style=BadgeStyle::Danger variant=Variant::Filled />
                    <RadzenBadge text=Some("Flat".to_string())     badge_style=BadgeStyle::Danger variant=Variant::Flat />
                    <RadzenBadge text=Some("Outlined".to_string()) badge_style=BadgeStyle::Danger variant=Variant::Outlined />
                    <RadzenBadge text=Some("Text".to_string())     badge_style=BadgeStyle::Danger variant=Variant::Text />
                </div>

                // ── Badges — Shades ───────────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Badges — Shades"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Same Info style across all five shades."
                </p>
                <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center;">
                    <RadzenBadge text=Some("Lighter".to_string()) badge_style=BadgeStyle::Info shade=Shade::Lighter />
                    <RadzenBadge text=Some("Light".to_string())   badge_style=BadgeStyle::Info shade=Shade::Light />
                    <RadzenBadge text=Some("Default".to_string()) badge_style=BadgeStyle::Info shade=Shade::Default />
                    <RadzenBadge text=Some("Dark".to_string())    badge_style=BadgeStyle::Info shade=Shade::Dark />
                    <RadzenBadge text=Some("Darker".to_string())  badge_style=BadgeStyle::Info shade=Shade::Darker />
                </div>

                // ── Badges — Pills ────────────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Badges — Pill Shape"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Rectangular vs pill, and pill with counts."
                </p>
                <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center;">
                    <RadzenBadge text=Some("Rectangular".to_string()) badge_style=BadgeStyle::Primary />
                    <RadzenBadge text=Some("Pill".to_string())        badge_style=BadgeStyle::Primary is_pill=true />
                    <RadzenBadge text=Some("3".to_string())           badge_style=BadgeStyle::Danger  is_pill=true />
                    <RadzenBadge text=Some("12".to_string())          badge_style=BadgeStyle::Warning is_pill=true />
                    <RadzenBadge text=Some("99+".to_string())         badge_style=BadgeStyle::Success is_pill=true />
                    <RadzenBadge
                        text=Some("Outlined pill".to_string())
                        badge_style=BadgeStyle::Secondary
                        variant=Variant::Outlined
                        is_pill=true
                    />
                    <RadzenBadge
                        text=Some("Flat pill".to_string())
                        badge_style=BadgeStyle::Info
                        variant=Variant::Flat
                        is_pill=true
                        shade=Shade::Dark
                    />
                </div>

                // ── Badges — ChildContent ─────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Badges — Custom Content"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Using the children slot instead of the text prop."
                </p>
                <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center;">
                    <RadzenBadge badge_style=BadgeStyle::Success is_pill=true>
                        <span style="display: flex; align-items: center; gap: 0.25rem;">
                            <i class="notranslate rzi" style="font-size: 0.85rem;">"check"</i>
                            " Active"
                        </span>
                    </RadzenBadge>
                    <RadzenBadge badge_style=BadgeStyle::Danger variant=Variant::Outlined is_pill=true>
                        <span style="display: flex; align-items: center; gap: 0.25rem;">
                            <i class="notranslate rzi" style="font-size: 0.85rem;">"error"</i>
                            " Failed"
                        </span>
                    </RadzenBadge>
                    <RadzenBadge badge_style=BadgeStyle::Warning variant=Variant::Flat>
                        <strong>"⚠ Pending review"</strong>
                    </RadzenBadge>
                </div>

                // ── Badges — Visibility ───────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Badges — Visibility"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "The second badge below has Visible=false — it renders nothing."
                </p>
                <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center;">
                    <RadzenBadge text=Some("Visible".to_string()) badge_style=BadgeStyle::Success />
                    <RadzenBadge
                        text=Some("Hidden (not rendered)".to_string())
                        badge_style=BadgeStyle::Danger
                        base=ComponentProps { visible: Some(false), ..Default::default() }
                    />
                    <RadzenBadge text=Some("Also visible".to_string()) badge_style=BadgeStyle::Info />
                </div>

                // ── Cards ─────────────────────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Cards"</h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; margin-bottom: 2rem;">
                    <RadzenCard variant=Variant::Filled>
                        <div style="padding: 1rem;">
                            <h3>"Card Title"</h3>
                            <p>"This is a filled card with sample content."</p>
                        </div>
                    </RadzenCard>
                    <RadzenCard variant=Variant::Outlined>
                        <div style="padding: 1rem;">
                            <h3>"Outlined Card"</h3>
                            <p>"This is an outlined card variant."</p>
                        </div>
                    </RadzenCard>
                    <RadzenCard variant=Variant::Flat>
                        <div style="padding: 1rem;">
                            <h3>"Flat Card"</h3>
                            <p>"This is a flat card variant with minimal styling."</p>
                        </div>
                    </RadzenCard>
                </div>

                // ── Icons — Basic ─────────────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Icons — Basic"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Default Outlined style, inheriting text color."
                </p>
                <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                    <RadzenIcon icon=Some("home".to_string()) />
                    <RadzenIcon icon=Some("settings".to_string()) />
                    <RadzenIcon icon=Some("account_circle".to_string()) />
                    <RadzenIcon icon=Some("check_circle".to_string()) />
                    <RadzenIcon icon=Some("notifications".to_string()) />
                    <RadzenIcon icon=Some("favorite".to_string()) />
                    <RadzenIcon icon=Some("delete".to_string()) />
                    <RadzenIcon icon=Some("search".to_string()) />
                </div>

                // ── Icons — Colors ────────────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Icons — Colors"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Using icon_color to set explicit CSS color values."
                </p>
                <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                    <RadzenIcon icon=Some("home".to_string())         icon_color=Some("var(--rz-primary)".to_string()) />
                    <RadzenIcon icon=Some("check_circle".to_string()) icon_color=Some("var(--rz-success)".to_string()) />
                    <RadzenIcon icon=Some("warning".to_string())      icon_color=Some("var(--rz-warning)".to_string()) />
                    <RadzenIcon icon=Some("error".to_string())        icon_color=Some("var(--rz-danger)".to_string()) />
                    <RadzenIcon icon=Some("info".to_string())         icon_color=Some("var(--rz-info)".to_string()) />
                    <RadzenIcon icon=Some("favorite".to_string())     icon_color=Some("#FF0000".to_string()) />
                </div>

                // ── Icons — Style Variants ────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Icons — Style Variants"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Same icon across all four IconStyle variants (plus None = default Outlined)."
                </p>
                <div style="display: flex; gap: 1.5rem; flex-wrap: wrap; align-items: center;">
                    <div style="display: flex; flex-direction: column; align-items: center; gap: 0.25rem;">
                        <RadzenIcon icon=Some("favorite".to_string()) />
                        <span style="font-size: 0.75rem; color: var(--rz-base-600);">"None"</span>
                    </div>
                    <div style="display: flex; flex-direction: column; align-items: center; gap: 0.25rem;">
                        <RadzenIcon icon=Some("favorite".to_string()) icon_style=Some(IconStyle::Outlined) />
                        <span style="font-size: 0.75rem; color: var(--rz-base-600);">"Outlined"</span>
                    </div>
                    <div style="display: flex; flex-direction: column; align-items: center; gap: 0.25rem;">
                        <RadzenIcon icon=Some("favorite".to_string()) icon_style=Some(IconStyle::Filled) />
                        <span style="font-size: 0.75rem; color: var(--rz-base-600);">"Filled"</span>
                    </div>
                    <div style="display: flex; flex-direction: column; align-items: center; gap: 0.25rem;">
                        <RadzenIcon icon=Some("favorite".to_string()) icon_style=Some(IconStyle::Rounded) />
                        <span style="font-size: 0.75rem; color: var(--rz-base-600);">"Rounded"</span>
                    </div>
                    <div style="display: flex; flex-direction: column; align-items: center; gap: 0.25rem;">
                        <RadzenIcon icon=Some("favorite".to_string()) icon_style=Some(IconStyle::Sharp) />
                        <span style="font-size: 0.75rem; color: var(--rz-base-600);">"Sharp"</span>
                    </div>
                </div>

                // ── Icons — Sizes ─────────────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Icons — Sizes"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Scaled via the style prop (font-size)."
                </p>
                <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                    <RadzenIcon icon=Some("star".to_string())
                        base=ComponentProps { style: Some("font-size: 1rem;".to_string()),   ..Default::default() } />
                    <RadzenIcon icon=Some("star".to_string())
                        base=ComponentProps { style: Some("font-size: 1.5rem;".to_string()), ..Default::default() } />
                    <RadzenIcon icon=Some("star".to_string())
                        base=ComponentProps { style: Some("font-size: 2rem;".to_string()),   ..Default::default() } />
                    <RadzenIcon icon=Some("star".to_string())
                        base=ComponentProps { style: Some("font-size: 3rem;".to_string()),   ..Default::default() } />
                    <RadzenIcon icon=Some("star".to_string())
                        base=ComponentProps { style: Some("font-size: 4rem;".to_string()),   ..Default::default() } />
                </div>

                // ── Icons — Visibility ────────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Icons — Visibility"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "The middle icon has Visible=false — renders nothing."
                </p>
                <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                    <RadzenIcon icon=Some("check".to_string()) icon_color=Some("var(--rz-success)".to_string()) />
                    <RadzenIcon icon=Some("block".to_string())
                        base=ComponentProps { visible: Some(false), ..Default::default() } />
                    <RadzenIcon icon=Some("check".to_string()) icon_color=Some("var(--rz-success)".to_string()) />
                </div>

                // ══════════════════════════════════════════════════════════════
                // RadzenText examples
                // ══════════════════════════════════════════════════════════════

                <RadzenText
                    text_style=TextStyle::H2
                    tag_name=TagName::H1
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-pt-8".to_string())])),
                        ..Default::default()
                    }
                >
                    "Text"
                </RadzenText>
                <RadzenText
                    text_style=TextStyle::Subtitle1
                    tag_name=TagName::P
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-pb-4".to_string())])),
                        ..Default::default()
                    }
                >
                    "Format and style text in your application with predefined text styles."
                </RadzenText>

                <RadzenText
                    anchor="text-style".to_string()
                    text_style=TextStyle::H5
                    tag_name=TagName::H2
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-pt-8".to_string())])),
                        ..Default::default()
                    }
                >
                    "Text Style"
                </RadzenText>
                <RadzenText
                    text_style=TextStyle::Body1
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-mb-8".to_string())])),
                        ..Default::default()
                    }
                >
                    "Use the TextStyle property to apply a predefined text style."
                </RadzenText>

                <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                    <RadzenText text_style=TextStyle::H1      text=Some("H1 – Heading 1".to_string()) />
                    <RadzenText text_style=TextStyle::H2      text=Some("H2 – Heading 2".to_string()) />
                    <RadzenText text_style=TextStyle::H3      text=Some("H3 – Heading 3".to_string()) />
                    <RadzenText text_style=TextStyle::H4      text=Some("H4 – Heading 4".to_string()) />
                    <RadzenText text_style=TextStyle::H5      text=Some("H5 – Heading 5".to_string()) />
                    <RadzenText text_style=TextStyle::H6      text=Some("H6 – Heading 6".to_string()) />
                    <RadzenText text_style=TextStyle::Subtitle1 text=Some("Subtitle1".to_string()) />
                    <RadzenText text_style=TextStyle::Subtitle2 text=Some("Subtitle2".to_string()) />
                    <RadzenText text_style=TextStyle::Body1   text=Some("Body1 – default paragraph style.".to_string()) />
                    <RadzenText text_style=TextStyle::Body2   text=Some("Body2 – smaller paragraph style.".to_string()) />
                    <RadzenText text_style=TextStyle::Caption text=Some("Caption – small descriptive text.".to_string()) />
                    <RadzenText text_style=TextStyle::Overline text=Some("OVERLINE – label above content.".to_string()) />
                    <RadzenText text_style=TextStyle::Button  text=Some("Button – button label style.".to_string()) />
                </div>

                <RadzenText
                    anchor="text-tag-name".to_string()
                    text_style=TextStyle::H5
                    tag_name=TagName::H2
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-pt-12".to_string())])),
                        ..Default::default()
                    }
                >
                    "Text Style and Tag Name"
                </RadzenText>
                <RadzenText
                    text_style=TextStyle::Body1
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-mb-8".to_string())])),
                        ..Default::default()
                    }
                >
                    "Use TextStyle together with TagName to apply different styling while keeping the code semantically correct."
                </RadzenText>

                <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                    <RadzenText text_style=TextStyle::H5      tag_name=TagName::H2 text=Some("H5 style on an <h2> element".to_string()) />
                    <RadzenText text_style=TextStyle::Subtitle1 tag_name=TagName::P text=Some("Subtitle1 style on a <p> element".to_string()) />
                    <RadzenText text_style=TextStyle::Body1   tag_name=TagName::Div text=Some("Body1 style on a <div> element".to_string()) />
                    <RadzenText text_style=TextStyle::Caption tag_name=TagName::Span text=Some("Caption style on a <span> element".to_string()) />
                </div>

                <RadzenText
                    anchor="text-display-headings".to_string()
                    text_style=TextStyle::H5
                    tag_name=TagName::H2
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-pt-12".to_string())])),
                        ..Default::default()
                    }
                >
                    "Display Headings"
                </RadzenText>
                <RadzenText
                    text_style=TextStyle::Body1
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-mb-8".to_string())])),
                        ..Default::default()
                    }
                >
                    "Use display headings to emphasise a text or page title."
                </RadzenText>

                <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                    <RadzenText text_style=TextStyle::DisplayH1 text=Some("Display H1".to_string()) />
                    <RadzenText text_style=TextStyle::DisplayH2 text=Some("Display H2".to_string()) />
                    <RadzenText text_style=TextStyle::DisplayH3 text=Some("Display H3".to_string()) />
                    <RadzenText text_style=TextStyle::DisplayH4 text=Some("Display H4".to_string()) />
                    <RadzenText text_style=TextStyle::DisplayH5 text=Some("Display H5".to_string()) />
                    <RadzenText text_style=TextStyle::DisplayH6 text=Some("Display H6".to_string()) />
                </div>

                <RadzenText
                    anchor="text-align".to_string()
                    text_style=TextStyle::H5
                    tag_name=TagName::H2
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-pt-12".to_string())])),
                        ..Default::default()
                    }
                >
                    "Text Align"
                </RadzenText>
                <RadzenText
                    text_style=TextStyle::Body1
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-mb-8".to_string())])),
                        ..Default::default()
                    }
                >
                    "Use TextAlign to align your text."
                </RadzenText>

                <div style="display: flex; flex-direction: column; gap: 0.5rem; border: 1px solid var(--rz-base-300); border-radius: 4px; padding: 1rem;">
                    <RadzenText text_style=TextStyle::Body1 text_align=TextAlign::Left    text=Some("TextAlign.Left — Radzen Leptos Components are open source and free for commercial use.".to_string()) />
                    <RadzenText text_style=TextStyle::Body1 text_align=TextAlign::Center  text=Some("TextAlign.Center — Radzen Leptos Components are open source and free for commercial use.".to_string()) />
                    <RadzenText text_style=TextStyle::Body1 text_align=TextAlign::Right   text=Some("TextAlign.Right — Radzen Leptos Components are open source and free for commercial use.".to_string()) />
                    <RadzenText text_style=TextStyle::Body1 text_align=TextAlign::Justify text=Some("TextAlign.Justify — Radzen Leptos Components are open source and free for commercial use. You can install them from crates.io or build your own copy from source.".to_string()) />
                    <RadzenText text_style=TextStyle::Body1 text_align=TextAlign::Start   text=Some("TextAlign.Start — logical inline-start (same as Left in LTR documents).".to_string()) />
                    <RadzenText text_style=TextStyle::Body1 text_align=TextAlign::End     text=Some("TextAlign.End — logical inline-end (same as Right in LTR documents).".to_string()) />
                </div>

                <RadzenText
                    anchor="text-children".to_string()
                    text_style=TextStyle::H5
                    tag_name=TagName::H2
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-pt-12".to_string())])),
                        ..Default::default()
                    }
                >
                    "Children (Rich Content)"
                </RadzenText>
                <RadzenText text_style=TextStyle::Body1>
                    "When "
                    <code>"text"</code>
                    " is not set, child content is rendered instead."
                </RadzenText>

                <RadzenText
                    anchor="text-visibility".to_string()
                    text_style=TextStyle::H5
                    tag_name=TagName::H2
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-pt-12".to_string())])),
                        ..Default::default()
                    }
                >
                    "Visibility"
                </RadzenText>
                <RadzenText text_style=TextStyle::Body1 text=Some("I am visible.".to_string()) />
                <RadzenText
                    text_style=TextStyle::Body1
                    text=Some("I am hidden — you should not see this.".to_string())
                    base=ComponentProps { visible: Some(false), ..Default::default() }
                />
                <RadzenText text_style=TextStyle::Body1 text=Some("I am also visible.".to_string()) />

                // ── Stack ─────────────────────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Stack"</h2>

                <h3 style="margin-top: 1rem;">"Stack — Vertical (default)"</h3>
                <RadzenStack gap=Some("0.5rem".to_string())>
                    <RadzenBadge text=Some("First".to_string())  badge_style=BadgeStyle::Primary />
                    <RadzenBadge text=Some("Second".to_string()) badge_style=BadgeStyle::Secondary />
                    <RadzenBadge text=Some("Third".to_string())  badge_style=BadgeStyle::Info />
                </RadzenStack>

                <h3 style="margin-top: 1rem;">"Stack — Horizontal"</h3>
                <RadzenStack orientation=Orientation::Horizontal gap=Some("1rem".to_string())>
                    <RadzenBadge text=Some("First".to_string())  badge_style=BadgeStyle::Primary />
                    <RadzenBadge text=Some("Second".to_string()) badge_style=BadgeStyle::Secondary />
                    <RadzenBadge text=Some("Third".to_string())  badge_style=BadgeStyle::Info />
                </RadzenStack>

                <h3 style="margin-top: 1rem;">"Stack — AlignItems Center"</h3>
                <RadzenStack
                    orientation=Orientation::Horizontal
                    align_items=AlignItems::Center
                    gap=Some("1rem".to_string())
                    base=ComponentProps { style: Some("border: 1px solid var(--rz-base-300); padding: 0.5rem;".to_string()), ..Default::default() }
                >
                    <RadzenBadge text=Some("A".to_string()) badge_style=BadgeStyle::Primary />
                    <RadzenBadge text=Some("B".to_string()) badge_style=BadgeStyle::Success
                        base=ComponentProps { style: Some("font-size: 1.5rem; padding: 0.5rem 1rem;".to_string()), ..Default::default() } />
                    <RadzenBadge text=Some("C".to_string()) badge_style=BadgeStyle::Danger />
                </RadzenStack>

                <h3 style="margin-top: 1rem;">"Stack — JustifyContent SpaceBetween"</h3>
                <RadzenStack
                    orientation=Orientation::Horizontal
                    justify_content=JustifyContent::SpaceBetween
                    base=ComponentProps { style: Some("width: 100%;".to_string()), ..Default::default() }
                >
                    <RadzenButton text="Cancel".to_string() button_style=ButtonStyle::Secondary />
                    <RadzenButton text="Cancel".to_string() button_style=ButtonStyle::Secondary />
                    <RadzenButton text="Save".to_string()   button_style=ButtonStyle::Primary />
                </RadzenStack>

                <h3 style="margin-top: 1rem;">"Stack — Wrap"</h3>
                <RadzenStack
                    orientation=Orientation::Horizontal
                    wrap=FlexWrap::Wrap
                    gap=Some("0.5rem".to_string())
                    base=ComponentProps { style: Some("max-width: 300px; border: 1px solid var(--rz-base-300); padding: 0.5rem;".to_string()), ..Default::default() }
                >
                    <RadzenBadge text=Some("Alpha".to_string())   badge_style=BadgeStyle::Primary />
                    <RadzenBadge text=Some("Beta".to_string())    badge_style=BadgeStyle::Secondary />
                    <RadzenBadge text=Some("Gamma".to_string())   badge_style=BadgeStyle::Success />
                    <RadzenBadge text=Some("Delta".to_string())   badge_style=BadgeStyle::Danger />
                    <RadzenBadge text=Some("Epsilon".to_string()) badge_style=BadgeStyle::Warning />
                </RadzenStack>

                <h3 style="margin-top: 1rem;">"Stack — Reverse"</h3>
                <RadzenStack
                    orientation=Orientation::Horizontal
                    gap=Some("0.5rem".to_string())
                    reverse=true
                    base=ComponentProps { style: Some("width: 100%;".to_string()), ..Default::default() }
                >
                    <RadzenBadge text=Some("1".to_string()) badge_style=BadgeStyle::Primary />
                    <RadzenBadge text=Some("2".to_string()) badge_style=BadgeStyle::Secondary />
                    <RadzenBadge text=Some("3".to_string()) badge_style=BadgeStyle::Info />
                </RadzenStack>

                <h3 style="margin-top: 1rem;">"Stack — Visibility"</h3>
                <RadzenStack gap=Some("0.5rem".to_string())>
                    <RadzenBadge text=Some("Visible".to_string())              badge_style=BadgeStyle::Success />
                    <RadzenStack
                        orientation=Orientation::Horizontal
                        base=ComponentProps { visible: Some(false), ..Default::default() }
                    >
                        <RadzenBadge text=Some("Hidden stack".to_string()) badge_style=BadgeStyle::Danger />
                    </RadzenStack>
                    <RadzenBadge text=Some("Also visible".to_string()) badge_style=BadgeStyle::Info />
                </RadzenStack>
            </div>

            // ── Links ─────────────────────────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Links — Basic"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                <RadzenLink path="/" text="Home" />
                <RadzenLink path="/about" text="About" />
                <RadzenLink path="https://radzen.com" text="Radzen" target=Some("_blank".to_string()) />
            </div>

            <h2 style="margin-top: 2rem;">"Links — With Icon"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                <RadzenLink path="/" text="Home" icon=Some("home".to_string()) />
                <RadzenLink path="https://radzen.com" text="Visit Radzen" icon=Some("open_in_new".to_string()) target=Some("_blank".to_string()) />
                <RadzenLink path="/settings" text="Settings" icon=Some("settings".to_string()) icon_color=Some("var(--rz-primary)".to_string()) />
            </div>

            <h2 style="margin-top: 2rem;">"Links — Disabled"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                <RadzenLink path="/active"        text="Active link"         />
                <RadzenLink path="/disabled"      text="Disabled link"       disabled=true />
                <RadzenLink path="/disabled-icon" text="Disabled with icon"  icon=Some("lock".to_string()) disabled=true />
            </div>

            <h2 style="margin-top: 2rem;">"Links — Match Mode"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                <RadzenLink path="/" text="Home (Prefix)" match_=NavLinkMatch::Prefix />
                <RadzenLink path="/" text="Home (Exact)"  match_=NavLinkMatch::All />
            </div>

            <h2 style="margin-top: 2rem;">"Links — Children (Rich Content)"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                <RadzenLink path="/"><strong>"🏠 Go Home"</strong></RadzenLink>
            </div>

            <h2 style="margin-top: 2rem;">"Links — Visibility"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                <RadzenLink path="/" text="Visible" />
                <RadzenLink path="/" text="Hidden (not rendered)" base=ComponentProps { visible: Some(false), ..Default::default() } />
                <RadzenLink path="/" text="Also visible" />
            </div>

            // ── Image examples ────────────────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Image — Basic"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenImage path=Some("https://picsum.photos/seed/basic/120/80".to_string()) />
            </div>

            <h2 style="margin-top: 2rem;">"Image — Alternate Text"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenImage path=Some("https://picsum.photos/seed/alttext/120/80".to_string()) alternate_text="A scenic mountain landscape".to_string() />
            </div>

            <h2 style="margin-top: 2rem;">"Image — Styling"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenImage
                    path=Some("https://picsum.photos/seed/circle/100/100".to_string())
                    alternate_text="Circular image".to_string()
                    base=ComponentProps {
                        style: Some("width: 100px; height: 100px; border-radius: 50%; border: 3px solid var(--rz-primary); object-fit: cover;".to_string()),
                        ..Default::default()
                    }
                />
                <RadzenImage
                    path=Some("https://picsum.photos/seed/rounded/150/100".to_string())
                    alternate_text="Rounded image".to_string()
                    base=ComponentProps {
                        style: Some("width: 150px; height: 100px; border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.2); object-fit: cover;".to_string()),
                        ..Default::default()
                    }
                />
            </div>

            <h2 style="margin-top: 2rem;">"Image — Clickable"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenImage
                    path=Some("https://picsum.photos/seed/clickable/160/100".to_string())
                    alternate_text="Click me".to_string()
                    base=ComponentProps {
                        style: Some("width: 160px; height: 100px; cursor: pointer; border-radius: 8px; object-fit: cover; border: 2px solid var(--rz-primary);".to_string()),
                        ..Default::default()
                    }
                    on_click=Some(std::sync::Arc::new(|_ev| {
                        Box::pin(async move {
                            web_sys::window().unwrap().alert_with_message("RadzenImage clicked!").ok();
                        })
                    }))
                />
            </div>

            <h2 style="margin-top: 2rem;">"Image — Inside RadzenCard"</h2>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenCard variant=Variant::Outlined>
                    <div style="padding: 0.75rem; display: flex; flex-direction: column; gap: 0.5rem; width: 180px;">
                        <RadzenImage
                            path=Some("https://picsum.photos/seed/card1/180/120".to_string())
                            alternate_text="Product photo".to_string()
                            base=ComponentProps {
                                style: Some("width: 100%; height: 120px; object-fit: cover; border-radius: 4px;".to_string()),
                                ..Default::default()
                            }
                        />
                        <RadzenText text_style=TextStyle::H6 text=Some("Product Name".to_string()) />
                        <RadzenText text_style=TextStyle::Body2 text=Some("$29.99".to_string()) />
                    </div>
                </RadzenCard>
            </div>

            // ── Labels ────────────────────────────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Labels — Basic"</h2>
            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                <RadzenLabel text=Some("Email address".to_string()) component=Some("email_input".to_string()) />
                <RadzenLabel text=Some("Password".to_string()) component=Some("password_input".to_string()) />
            </div>

            <h2 style="margin-top: 2rem;">"Labels — Rich Content"</h2>
            <RadzenLabel component=Some("required_field".to_string())>
                "Name "
                <span style="color: var(--rz-danger);">"*"</span>
            </RadzenLabel>

            <h2 style="margin-top: 2rem;">"Labels — No Association"</h2>
            <RadzenLabel text=Some("Standalone label (no for attribute)".to_string()) />

            <h2 style="margin-top: 2rem;">"Labels — Visibility"</h2>
            <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                <RadzenLabel text=Some("Visible label".to_string()) />
                <RadzenLabel
                    text=Some("Hidden (not rendered)".to_string())
                    base=ComponentProps { visible: Some(false), ..Default::default() }
                />
                <RadzenLabel text=Some("Also visible".to_string()) />
            </div>

            // ── Alert examples ────────────────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Alert — Styles"</h2>
            <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                <RadzenAlert alert_style=AlertStyle::Info    title=Some("Information".to_string()) text=Some("This is an informational message.".to_string()) />
                <RadzenAlert alert_style=AlertStyle::Success variant=Variant::Flat shade=Shade::Lighter title=Some("Done!".to_string()) text=Some("Your changes have been saved.".to_string()) on_close=Some(std::sync::Arc::new(|| log::info!("alert closed"))) />
                <RadzenAlert show_icon=false allow_close=false alert_style=AlertStyle::Warning size=AlertSize::ExtraSmall shade=Shade::Lighter variant=Variant::Flat>
                    <span>"Custom rich content inside the alert."</span>
                </RadzenAlert>
                <RadzenAlert alert_style=AlertStyle::Danger variant=Variant::Outlined icon=Some("bug_report".to_string()) title=Some("Error".to_string()) text=Some("Something went wrong. Please try again.".to_string()) />
            </div>
        </ErrorBoundary>
    }
}
