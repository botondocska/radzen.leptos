use leptos::prelude::*;

use crate::components::{ButtonStyle, ButtonSize, Variant, Shade, RadzenButton, RadzenBadge, BadgeStyle, RadzenCard};

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
                    {/* Primary button with text */}
                    <RadzenButton
                        text="Primary Button".to_string()
                        button_style=ButtonStyle::Primary
                        size=ButtonSize::ExtraSmall
                        variant=Variant::Filled
                        shade=Shade::Dark
                    />

                    {/* Success button */}
                    <RadzenButton
                        text="Save".to_string()
                        button_style=ButtonStyle::Success
                        size=ButtonSize::Small
                        variant=Variant::Flat
                        shade=Shade::Light
                    />

                    {/* Danger button with outlined variant */}
                    <RadzenButton
                        text="Delete".to_string()
                        button_style=ButtonStyle::Danger
                        size=ButtonSize::Medium
                        variant=Variant::Outlined
                        shade=Shade::Darker
                    />

                    {/* Secondary flat button */}
                    <RadzenButton
                        text="Cancel".to_string()
                        button_style=ButtonStyle::Secondary
                        size=ButtonSize::Large
                        variant=Variant::Text
                        shade=Shade::Lighter
                    />

                <h2 style="margin-top: 2rem;">"Badges"</h2>
                <div style="display: flex; gap: 1rem; flex-wrap: wrap;">
                    <RadzenBadge
                        text="Primary".to_string()
                        badge_style=BadgeStyle::Primary
                        variant=Variant::Filled
                    />
                    <RadzenBadge
                        text="Success".to_string()
                        badge_style=BadgeStyle::Success
                        variant=Variant::Filled
                    />
                    <RadzenBadge
                        text="Danger".to_string()
                        badge_style=BadgeStyle::Danger
                        variant=Variant::Text
                    />
                    <RadzenBadge
                        text="Warning".to_string()
                        badge_style=BadgeStyle::Warning
                        variant=Variant::Outlined
                    />
                    <RadzenBadge
                        text="Pill Badge".to_string()
                        badge_style=BadgeStyle::Info
                        variant=Variant::Flat
                    />
                </div>

                <h2 style="margin-top: 2rem;">"Cards"</h2>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; margin-bottom: 2rem;">
                    <RadzenCard
                        variant=Variant::Filled

                    >
                        <div style="padding: 1rem;">
                            <h3>"Card Title"</h3>
                            <p>"This is a filled card with sample content."</p>
                        </div>
                    </RadzenCard>
                    <RadzenCard
                        variant=Variant::Outlined
                    >
                        <div style="padding: 1rem;">
                            <h3>"Outlined Card"</h3>
                            <p>"This is an outlined card variant."</p>
                        </div>
                    </RadzenCard>
                    <RadzenCard
                        variant=Variant::Flat
                    >
                        <div style="padding: 1rem;">
                            <h3>"Flat Card"</h3>
                            <p>"This is a flat card variant with minimal styling."</p>
                        </div>
                    </RadzenCard>
                </div>
            </div>
        </ErrorBoundary>
    }
}