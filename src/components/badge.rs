use crate::components::{
    ClassList,
    badge_style::BadgeStyle,
    base_component::{ComponentProps, use_radzen_base},
    shade::Shade,
    variant::Variant,
};
use leptos::prelude::*;

/// RadzenBadge component
/// A small label component for displaying counts, statuses, or short text labels
#[component]
pub fn RadzenBadge(
    /// Base component properties (id, class, style, etc.)
    #[prop(default = Default::default())]
    base: ComponentProps,
    /// Text to display in the badge
    #[prop(default = String::new(), into = true)]
    text: String,
    /// Semantic color style of the badge
    #[prop(default = BadgeStyle::Primary)]
    badge_style: BadgeStyle,
    /// Visual variant (Filled, Flat, Outlined, Text)
    #[prop(default = Variant::Filled)]
    variant: Variant,
    /// Color shade intensity
    #[prop(default = Shade::Default)]
    shade: Shade,
    /// Render as pill-shaped instead of rectangular
    #[prop(default = false)]
    is_pill: bool,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "rz-badge");

    let classes = ClassList::default()
        .add("rz-badge", true)
        .add_badge_style(badge_style)
        .add_variant(variant)
        .add_shade(shade)
        .add("rz-badge-pill", is_pill);

    let combined_class = format!("{} {}", handle.css_class, classes.finish());

    view! {
        <span
            id=base.id.clone()
            class=combined_class
            style=base.style.clone().unwrap_or_default()
        >
            {text}
        </span>
    }
}
