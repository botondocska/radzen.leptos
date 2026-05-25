use leptos::prelude::*;
use std::collections::HashMap;

use crate::components::{
    BadgeStyle, ButtonSize, ButtonStyle, ComponentProps, IconStyle, RadzenBadge, RadzenButton,
    RadzenCard, RadzenIcon, RadzenText, Shade, TagName, TextAlign, TextStyle, Variant,
    AlignItems, FlexWrap, JustifyContent, Orientation, RadzenStack,
    NavLinkMatch, RadzenLink, RadzenImage, ImageClickHandler, ImageClickFuture,
};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
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

                // ── Buttons ───────────────────────────────────────────────────
                <h2>"Buttons"</h2>
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
                // Mirrors the sections from TextPage.razor + the TextStyles,
                // TextTagName, TextDisplayHeadings, TextAlignment sub-examples.
                // ══════════════════════════════════════════════════════════════

                // Page heading — mirrors TextPage.razor lines 3-8
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

                // ── Text — Text Style ─────────────────────────────────────────
                // Mirrors TextPage.razor lines 10-17 + TextStyles sub-example.
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

                // TextStyles sub-example: one row per style group
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

                // ── Text — Tag Name ───────────────────────────────────────────
                // Mirrors TextPage.razor lines 19-27 + TextTagName sub-example.
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

                // TextTagName sub-example — style and tag intentionally differ
                <div style="display: flex; flex-direction: column; gap: 0.5rem;">
                    // H5 visual style rendered as semantic <h2>
                    <RadzenText
                        text_style=TextStyle::H5
                        tag_name=TagName::H2
                        text=Some("H5 style on an <h2> element".to_string())
                    />
                    // Subtitle1 style rendered as <p>
                    <RadzenText
                        text_style=TextStyle::Subtitle1
                        tag_name=TagName::P
                        text=Some("Subtitle1 style on a <p> element".to_string())
                    />
                    // Body1 style rendered as <div>
                    <RadzenText
                        text_style=TextStyle::Body1
                        tag_name=TagName::Div
                        text=Some("Body1 style on a <div> element".to_string())
                    />
                    // Caption style rendered as <span>
                    <RadzenText
                        text_style=TextStyle::Caption
                        tag_name=TagName::Span
                        text=Some("Caption style on a <span> element".to_string())
                    />
                </div>

                // ── Text — Display Headings ───────────────────────────────────
                // Mirrors TextPage.razor lines 29-37 + TextDisplayHeadings sub-example.
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
                    "Use display headings to emphasise a text or page title. Display headings are usually larger than traditional headings."
                </RadzenText>

                // TextDisplayHeadings sub-example
                <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                    <RadzenText text_style=TextStyle::DisplayH1 text=Some("Display H1".to_string()) />
                    <RadzenText text_style=TextStyle::DisplayH2 text=Some("Display H2".to_string()) />
                    <RadzenText text_style=TextStyle::DisplayH3 text=Some("Display H3".to_string()) />
                    <RadzenText text_style=TextStyle::DisplayH4 text=Some("Display H4".to_string()) />
                    <RadzenText text_style=TextStyle::DisplayH5 text=Some("Display H5".to_string()) />
                    <RadzenText text_style=TextStyle::DisplayH6 text=Some("Display H6".to_string()) />
                </div>

                // ── Text — Text Align ─────────────────────────────────────────
                // Mirrors TextPage.razor lines 39-47 + TextAlignment sub-example.
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

                // TextAlignment sub-example — one block per alignment value
                <div style="display: flex; flex-direction: column; gap: 0.5rem; border: 1px solid var(--rz-base-300); border-radius: 4px; padding: 1rem;">
                    <RadzenText
                        text_style=TextStyle::Body1
                        text_align=TextAlign::Left
                        text=Some("TextAlign.Left — Radzen Leptos Components are open source and free for commercial use.".to_string())
                    />
                    <RadzenText
                        text_style=TextStyle::Body1
                        text_align=TextAlign::Center
                        text=Some("TextAlign.Center — Radzen Leptos Components are open source and free for commercial use.".to_string())
                    />
                    <RadzenText
                        text_style=TextStyle::Body1
                        text_align=TextAlign::Right
                        text=Some("TextAlign.Right — Radzen Leptos Components are open source and free for commercial use.".to_string())
                    />
                    <RadzenText
                        text_style=TextStyle::Body1
                        text_align=TextAlign::Justify
                        text=Some("TextAlign.Justify — Radzen Leptos Components are open source and free for commercial use. You can install them from crates.io or build your own copy from source.".to_string())
                    />
                    <RadzenText
                        text_style=TextStyle::Body1
                        text_align=TextAlign::Start
                        text=Some("TextAlign.Start — logical inline-start (same as Left in LTR documents).".to_string())
                    />
                    <RadzenText
                        text_style=TextStyle::Body1
                        text_align=TextAlign::End
                        text=Some("TextAlign.End — logical inline-end (same as Right in LTR documents).".to_string())
                    />
                </div>

                // ── Text — Children (rich content) ────────────────────────────
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
                    " is not set, child content is rendered instead. This allows mixing styled markup inside a typed text container."
                </RadzenText>

                // ── Text — Visibility ─────────────────────────────────────────
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
                <RadzenText
                    text_style=TextStyle::Body1
                    base=ComponentProps {
                        attrs: Some(HashMap::from([("class".to_string(), "rz-mb-4".to_string())])),
                        ..Default::default()
                    }
                >
                    "The second paragraph below has Visible=false — it renders nothing (no DOM node)."
                </RadzenText>
                <RadzenText
                    text_style=TextStyle::Body1
                    text=Some("I am visible.".to_string())
                />
                <RadzenText
                    text_style=TextStyle::Body1
                    text=Some("I am hidden — you should not see this.".to_string())
                    base=ComponentProps { visible: Some(false), ..Default::default() }
                />
                <RadzenText
                    text_style=TextStyle::Body1
                    text=Some("I am also visible.".to_string())
                />

                // ── Stack ─────────────────────────────────────────────────────────────
                <h2 style="margin-top: 2rem;">"Stack"</h2>

                // Vertical (default)
                <h3 style="margin-top: 1rem;">"Stack — Vertical (default)"</h3>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Three badges stacked top-to-bottom with 0.5rem gap. This is the default orientation — no extra props needed beyond gap."
                </p>
                <RadzenStack gap=Some("0.5rem".to_string())>
                    <RadzenBadge text=Some("First".to_string())  badge_style=BadgeStyle::Primary />
                    <RadzenBadge text=Some("Second".to_string()) badge_style=BadgeStyle::Secondary />
                    <RadzenBadge text=Some("Third".to_string())  badge_style=BadgeStyle::Info />
                </RadzenStack>

                // Horizontal
                <h3 style="margin-top: 1rem;">"Stack — Horizontal"</h3>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Three badges arranged left-to-right with 1rem gap between them."
                </p>
                <RadzenStack orientation=Orientation::Horizontal gap=Some("1rem".to_string())>
                    <RadzenBadge text=Some("First".to_string())  badge_style=BadgeStyle::Primary />
                    <RadzenBadge text=Some("Second".to_string()) badge_style=BadgeStyle::Secondary />
                    <RadzenBadge text=Some("Third".to_string())  badge_style=BadgeStyle::Info />
                </RadzenStack>

                // AlignItems
                <h3 style="margin-top: 1rem;">"Stack — AlignItems Center"</h3>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Three badges in a bordered row. The middle badge is made taller via style. All three should be vertically centered — their midpoints aligned on the same horizontal line."
                </p>
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

                // JustifyContent
                <h3 style="margin-top: 1rem;">"Stack — JustifyContent SpaceBetween"</h3>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Cancel button pushed to the far left, Save button pushed to the far right, with all remaining space between them. The container spans full width."
                </p>
                <RadzenStack
                    orientation=Orientation::Horizontal
                    justify_content=JustifyContent::SpaceBetween
                    base=ComponentProps {
                        style: Some("width: 100%;".to_string()),
                        ..Default::default()
                    }
                >
                    <RadzenButton text="Cancel".to_string() button_style=ButtonStyle::Secondary />
                    <RadzenButton text="Cancel".to_string() button_style=ButtonStyle::Secondary />
                    <RadzenButton text="Save".to_string()   button_style=ButtonStyle::Primary />
                </RadzenStack>

                // Wrap
                <h3 style="margin-top: 1rem;">"Stack — Wrap"</h3>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Five badges in a 300px-wide bordered container. Because they don't all fit on one line, they wrap: the first few appear on row 1, the rest continue on row 2."
                </p>
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

                // Reverse
                <h3 style="margin-top: 1rem;">"Stack — Reverse"</h3>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "Badges 1, 2, 3 are passed in that order in markup, but rendered right-to-left: 3 · 2 · 1."
                </p>
                <RadzenStack
                    orientation=Orientation::Horizontal
                    gap=Some("0.5rem".to_string())
                    reverse=true
                    base=ComponentProps {
                        style: Some("width: 100%;".to_string()),
                        ..Default::default()
                    }
                >       
                    <RadzenBadge text=Some("1".to_string()) badge_style=BadgeStyle::Primary />
                    <RadzenBadge text=Some("2".to_string()) badge_style=BadgeStyle::Secondary />
                    <RadzenBadge text=Some("3".to_string()) badge_style=BadgeStyle::Info />
                </RadzenStack>

                // Visibility
                <h3 style="margin-top: 1rem;">"Stack — Visibility"</h3>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                    "A visible green badge, then a RadzenStack with Visible=false (renders nothing — no gap, no space), then another visible blue badge. Only two badges should appear."
                </p>
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
            <h2 style="margin-top: 2rem;">"Links — Basic"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "Default link with text only."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                <RadzenLink path="/" text="Home" />
                <RadzenLink path="/about" text="About" />
                <RadzenLink
                    path="https://radzen.com"
                    text="Radzen"
                    target=Some("_blank".to_string())
                />
            </div>
            
            // ── Links — With Icon ─────────────────────────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Links — With Icon"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "Links with a Material icon before the label."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                <RadzenLink path="/" text="Home" icon=Some("home".to_string()) />
                <RadzenLink
                    path="https://radzen.com"
                    text="Visit Radzen"
                    icon=Some("open_in_new".to_string())
                    target=Some("_blank".to_string())
                />
                <RadzenLink
                    path="/settings"
                    text="Settings"
                    icon=Some("settings".to_string())
                    icon_color=Some("var(--rz-primary)".to_string())
                />
            </div>
            
            // ── Links — Disabled ──────────────────────────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Links — Disabled"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "Disabled links have no href, no target, and gain rz-link-disabled."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                <RadzenLink path="/active"   text="Active link"   />
                <RadzenLink path="/disabled" text="Disabled link" disabled=true />
                <RadzenLink
                    path="/disabled-icon"
                    text="Disabled with icon"
                    icon=Some("lock".to_string())
                    disabled=true
                />
            </div>
            
            // ── Links — Match mode ────────────────────────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Links — Match Mode"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "Prefix match (default) vs exact match for active-state detection."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                <RadzenLink path="/" text="Home (Prefix)" match_=NavLinkMatch::Prefix />
                <RadzenLink path="/" text="Home (Exact)"  match_=NavLinkMatch::All />
            </div>
            
            // ── Links — Children ──────────────────────────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Links — Children (Rich Content)"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "When children are provided, text is ignored."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                <RadzenLink path="/">
                    <strong>"🏠 Go Home"</strong>
                </RadzenLink>
            </div>
            
            // ── Links — Visibility ────────────────────────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Links — Visibility"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "The second link has Visible=false — it renders nothing."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
                <RadzenLink path="/" text="Visible" />
                <RadzenLink
                    path="/"
                    text="Hidden (not rendered)"
                    base=ComponentProps { visible: Some(false), ..Default::default() }
                />
                <RadzenLink path="/" text="Also visible" />
            </div>
            // ══════════════════════════════════════════════════════════════
            // RadzenImage examples
            // Tests every prop: path, alternate_text, attrs["alt"] merge,
            // style, on_click (role/tabindex), keyboard Enter/Space,
            // attrs["class"], visibility, base64 data URL, external URL.
            // ══════════════════════════════════════════════════════════════

            // ── Image — Basic ─────────────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Image — Basic"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "Minimal usage: path only. No click handler, no extra attrs. "
                "Alt defaults to \"image\"."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenImage
                    path=Some("https://picsum.photos/seed/basic/120/80".to_string())
                />
            </div>

            // ── Image — Alternate Text ────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Image — Alternate Text"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "Custom alternate_text prop. Inspect the DOM: alt should be "
                "\"A scenic mountain landscape\"."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenImage
                    path=Some("https://picsum.photos/seed/alttext/120/80".to_string())
                    alternate_text="A scenic mountain landscape".to_string()
                />
            </div>

            // ── Image — attrs["alt"] Merge (GetAlternateText) ─────────────
            <h2 style="margin-top: 2rem;">"Image — Alt Attribute Merge"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "When base.attrs contains \"alt\", Blazor's GetAlternateText() "
                "appends it: \"{alternate_text} {attrs[alt]}\". "
                "Inspect DOM: alt should be \"Photo (uploaded by user)\"."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenImage
                    path=Some("https://picsum.photos/seed/altmerge/120/80".to_string())
                    alternate_text="Photo".to_string()
                    base=ComponentProps {
                        attrs: Some(HashMap::from([
                            ("alt".to_string(), "(uploaded by user)".to_string()),
                        ])),
                        ..Default::default()
                    }
                />
            </div>

            // ── Image — Styling ───────────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Image — Styling"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "base.style controls inline CSS. Tests size, border, border-radius, "
                "and object-fit."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                // Custom size
                <div style="display: flex; flex-direction: column; gap: 0.25rem; align-items: center;">
                    <RadzenImage
                        path=Some("https://picsum.photos/seed/style1/200/120".to_string())
                        alternate_text="Sized image".to_string()
                        base=ComponentProps {
                            style: Some("width: 200px; height: 120px;".to_string()),
                            ..Default::default()
                        }
                    />
                    <span style="font-size: 0.75rem; color: var(--rz-base-600);">"200×120px"</span>
                </div>

                // Circular with border
                <div style="display: flex; flex-direction: column; gap: 0.25rem; align-items: center;">
                    <RadzenImage
                        path=Some("https://picsum.photos/seed/circle/100/100".to_string())
                        alternate_text="Circular image".to_string()
                        base=ComponentProps {
                            style: Some("width: 100px; height: 100px; border-radius: 50%; border: 3px solid var(--rz-primary); object-fit: cover;".to_string()),
                            ..Default::default()
                        }
                    />
                    <span style="font-size: 0.75rem; color: var(--rz-base-600);">"Circular + border"</span>
                </div>

                // Rounded corners + box shadow
                <div style="display: flex; flex-direction: column; gap: 0.25rem; align-items: center;">
                    <RadzenImage
                        path=Some("https://picsum.photos/seed/rounded/150/100".to_string())
                        alternate_text="Rounded image".to_string()
                        base=ComponentProps {
                            style: Some("width: 150px; height: 100px; border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.2); object-fit: cover;".to_string()),
                            ..Default::default()
                        }
                    />
                    <span style="font-size: 0.75rem; color: var(--rz-base-600);">"Rounded + shadow"</span>
                </div>

                // Thumbnail (small)
                <div style="display: flex; flex-direction: column; gap: 0.25rem; align-items: center;">
                    <RadzenImage
                        path=Some("https://picsum.photos/seed/thumb/48/48".to_string())
                        alternate_text="Thumbnail".to_string()
                        base=ComponentProps {
                            style: Some("width: 48px; height: 48px; object-fit: cover; border-radius: 4px;".to_string()),
                            ..Default::default()
                        }
                    />
                    <span style="font-size: 0.75rem; color: var(--rz-base-600);">"48px thumbnail"</span>
                </div>
            </div>

            // ── Image — Clickable (on_click) ──────────────────────────────
            <h2 style="margin-top: 2rem;">"Image — Clickable"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "When on_click is set, Blazor adds role=\"button\" and tabindex=\"0\". "
                "Click the image — a browser alert should appear. "
                "Inspect DOM to verify role and tabindex attributes."
            </p>
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
                            web_sys::window()
                                .unwrap()
                                .alert_with_message("RadzenImage clicked!")
                                .ok();
                        })
                    }))
                />
            </div>

            // ── Image — Keyboard Activation (Enter / Space) ───────────────
            <h2 style="margin-top: 2rem;">"Image — Keyboard Activation"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "Tab to this image and press Enter or Space — the same click "
                "handler fires. Mirrors Blazor's OnKeyDown logic (Code ?? Key "
                "checked for \"Enter\" or \"Space\")."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenImage
                    path=Some("https://picsum.photos/seed/keyboard/160/100".to_string())
                    alternate_text="Tab to me and press Enter or Space".to_string()
                    base=ComponentProps {
                        style: Some("width: 160px; height: 100px; cursor: pointer; border-radius: 8px; object-fit: cover; border: 2px dashed var(--rz-success);".to_string()),
                        ..Default::default()
                    }
                    on_click=Some(std::sync::Arc::new(|_ev| {
                        Box::pin(async move {
                            web_sys::window()
                                .unwrap()
                                .alert_with_message("Keyboard activation worked!")
                                .ok();
                        })
                    }))
                />
            </div>

            // ── Image — No Click (no role/tabindex) ───────────────────────
            <h2 style="margin-top: 2rem;">"Image — No Click Handler"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "Without on_click, no role or tabindex attributes are added to "
                "the DOM — mirrors Blazor's Click.HasDelegate = false branch. "
                "Inspect DOM to verify."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenImage
                    path=Some("https://picsum.photos/seed/noclick/160/100".to_string())
                    alternate_text="No click handler".to_string()
                    base=ComponentProps {
                        style: Some("width: 160px; height: 100px; border-radius: 8px; object-fit: cover;".to_string()),
                        ..Default::default()
                    }
                />
            </div>

            // ── Image — attrs["class"] passthrough ────────────────────────
            <h2 style="margin-top: 2rem;">"Image — Caller Class"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "base.attrs[\"class\"] is appended via GetCssClass(). "
                "Inspect DOM: class attribute should be \"rz-border-radius-10\"."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenImage
                    path=Some("https://picsum.photos/seed/callerclass/160/100".to_string())
                    alternate_text="Has caller class".to_string()
                    base=ComponentProps {
                        style: Some("width: 160px; height: 100px; object-fit: cover;".to_string()),
                        attrs: Some(HashMap::from([
                            ("class".to_string(), "rz-border-radius-10".to_string()),
                        ])),
                        ..Default::default()
                    }
                />
            </div>

            // ── Image — Base64 Data URL ───────────────────────────────────
            <h2 style="margin-top: 2rem;">"Image — Base64 Data URL"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "path accepts data URLs — mirrors the base64 example in the C# "
                "XML doc comments. A small inline SVG is used here."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenImage
                    // Inline SVG as data URL — no network request needed.
                    path=Some("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='120' height='80'%3E%3Crect width='120' height='80' fill='%234f46e5'/%3E%3Ctext x='50%25' y='50%25' dominant-baseline='middle' text-anchor='middle' fill='white' font-family='sans-serif' font-size='14'%3EBase64%3C/text%3E%3C/svg%3E".to_string())
                    alternate_text="Inline SVG via data URL".to_string()
                    base=ComponentProps {
                        style: Some("width: 120px; height: 80px; border-radius: 6px;".to_string()),
                        ..Default::default()
                    }
                />
            </div>

            // ── Image — Visibility ────────────────────────────────────────
            <h2 style="margin-top: 2rem;">"Image — Visibility"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "The middle image has Visible=false — it renders nothing "
                "(no DOM node, no blank space). Only the first and third "
                "images should appear."
            </p>
            <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: flex-start;">
                <RadzenImage
                    path=Some("https://picsum.photos/seed/vis1/120/80".to_string())
                    alternate_text="Visible image 1".to_string()
                    base=ComponentProps {
                        style: Some("width: 120px; height: 80px; border-radius: 6px; object-fit: cover;".to_string()),
                        ..Default::default()
                    }
                />
                <RadzenImage
                    path=Some("https://picsum.photos/seed/hidden/120/80".to_string())
                    alternate_text="Hidden — should not appear".to_string()
                    base=ComponentProps {
                        visible: Some(false),
                        style: Some("width: 120px; height: 80px;".to_string()),
                        ..Default::default()
                    }
                />
                <RadzenImage
                    path=Some("https://picsum.photos/seed/vis2/120/80".to_string())
                    alternate_text="Visible image 2".to_string()
                    base=ComponentProps {
                        style: Some("width: 120px; height: 80px; border-radius: 6px; object-fit: cover;".to_string()),
                        ..Default::default()
                    }
                />
            </div>

            // ── Image — Inside RadzenCard ─────────────────────────────────
            <h2 style="margin-top: 2rem;">"Image — Inside RadzenCard"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "Mirrors the common usage pattern from the C# XML doc comments: "
                "RadzenImage inside a RadzenCard with RadzenText labels."
            </p>
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
                <RadzenCard variant=Variant::Outlined>
                    <div style="padding: 0.75rem; display: flex; flex-direction: column; gap: 0.5rem; width: 180px;">
                        <RadzenImage
                            path=Some("https://picsum.photos/seed/card2/180/120".to_string())
                            alternate_text="Another product".to_string()
                            base=ComponentProps {
                                style: Some("width: 100%; height: 120px; object-fit: cover; border-radius: 4px;".to_string()),
                                ..Default::default()
                            }
                        />
                        <RadzenText text_style=TextStyle::H6 text=Some("Another Item".to_string()) />
                        <RadzenText text_style=TextStyle::Body2 text=Some("$49.99".to_string()) />
                    </div>
                </RadzenCard>
            </div>
        </ErrorBoundary>
    }
}