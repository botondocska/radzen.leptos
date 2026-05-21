use leptos::prelude::*;
use std::collections::HashMap;

use crate::components::{
    BadgeStyle, ButtonSize, ButtonStyle, ComponentProps, IconStyle, RadzenBadge, RadzenButton,
    RadzenCard, RadzenIcon, Shade, Variant,
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
                <h1>"Welcome to Leptos Radzen Demo"</h1>

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

                // ── Badges — Invisible ────────────────────────────────────────
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

            </div>
        </ErrorBoundary>
    }
}