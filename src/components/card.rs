use leptos::prelude::*;
use crate::components::{
    base_component::{ComponentProps, use_radzen_base},
    variant::Variant,
    ClassList,
};

/// RadzenCard component
/// A versatile container for grouping related content with consistent styling
#[component]
pub fn RadzenCard(
    /// Base component properties (id, class, style, etc.)
    #[prop(default = Default::default())]
    base: ComponentProps,
    /// Visual variant (Filled, Flat, Outlined, Text)
    #[prop(default = Variant::Filled)]
    variant: Variant,
    /// Child content to display inside the card
    children: Children,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "rz-card");

    let classes = ClassList::default()
        .add("rz-card", true)
        .add_variant(variant);

    let combined_class = format!("{} {}", handle.css_class, classes.finish());

    view! {
        <div
            id=base.id.clone()
            class=combined_class
            style=base.style.clone().unwrap_or_default()
        >
            {children()}
        </div>
    }
}
