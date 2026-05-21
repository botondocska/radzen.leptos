//! Fluent CSS class builder — mirrors C# Radzen.Blazor.Rendering.ClassList.
//!
//! # Example
//! ```rust,ignore
//! use radzen_leptos::components::renderer::ClassList;
//! use radzen_leptos::components::{ButtonSize, ButtonStyle, Variant};
//!
//! let css = ClassList::new()
//!     .add("custom-class")
//!     .add_button_size(ButtonSize::Large)
//!     .add_button_style(ButtonStyle::Danger)
//!     .add_variant(Variant::Outlined)
//!     .finish();
//! ```

use crate::components::{BadgeStyle, ButtonSize, ButtonStyle, Shade, Variant, IconStyle};
use std::collections::HashMap;

/// Fluent builder for CSS class strings.
///
/// Accumulates CSS classes and provides convenience methods to add classes
/// based on enum values. The final class string is built via `.finish()`.
#[derive(Default)]
pub struct ClassList {
    classes: Vec<String>,
}

impl ClassList {
    /// Create a new empty ClassList builder.
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }

    /// Add a single class, optionally conditional.
    ///
    /// # Arguments
    /// * `class` — CSS class name (may contain spaces for multiple classes)
    /// * `condition` — if `false`, the class is not added
    ///
    /// If `class` is empty or only whitespace when `condition` is true, it is ignored.
    pub fn add<S: Into<String>>(mut self, class: S, condition: bool) -> Self {
        if condition {
            let s = class.into();
            if !s.trim().is_empty() {
                self.classes.push(s);
            }
        }
        self
    }

    /// Add a single class unconditionally.
    pub fn add_class<S: Into<String>>(self, class: S) -> Self {
        self.add(class, true)
    }

    /// Add a class if a string reference is present and non-empty.
    pub fn add_option(self, class: Option<&str>) -> Self {
        if let Some(c) = class {
            self.add_class(c)
        } else {
            self
        }
    }

    /// Extract and add the "class" key from a HashMap of attributes.
    ///
    /// Used to merge user-provided classes from element attributes.
    pub fn add_from_attrs(self, attrs: &HashMap<String, String>) -> Self {
        if let Some(class) = attrs.get("class") {
            self.add_class(class.clone())
        } else {
            self
        }
    }

    /// Add the "rz-state-disabled" class if the condition is true.
    pub fn add_disabled(self, condition: bool) -> Self {
        self.add("rz-state-disabled", condition)
    }

    /// Add button size class based on [`ButtonSize`].
    pub fn add_button_size(self, size: ButtonSize) -> Self {
        let class = match size {
            ButtonSize::ExtraSmall => "rz-button-xs",
            ButtonSize::Small => "rz-button-sm",
            ButtonSize::Medium => "rz-button-md",
            ButtonSize::Large => "rz-button-lg",
        };
        self.add_class(class)
    }

    /// Add variant class based on [`Variant`].
    pub fn add_variant(self, variant: Variant) -> Self {
        let class = match variant {
            Variant::Filled => "rz-variant-filled",
            Variant::Flat => "rz-variant-flat",
            Variant::Outlined => "rz-variant-outlined",
            Variant::Text => "rz-variant-text",
        };
        self.add_class(class)
    }

    /// Add button style (color) class based on [`ButtonStyle`].
    pub fn add_button_style(self, style: ButtonStyle) -> Self {
        let class = match style {
            ButtonStyle::Primary => "rz-primary",
            ButtonStyle::Secondary => "rz-secondary",
            ButtonStyle::Light => "rz-light",
            ButtonStyle::Base => "rz-base",
            ButtonStyle::Dark => "rz-dark",
            ButtonStyle::Success => "rz-success",
            ButtonStyle::Warning => "rz-warning",
            ButtonStyle::Danger => "rz-danger",
            ButtonStyle::Info => "rz-info",
        };
        self.add_class(class)
    }

    /// Add shade (color intensity) class based on [`Shade`].
    pub fn add_shade(self, shade: Shade) -> Self {
        let class = match shade {
            Shade::Lighter => "rz-shade-lighter",
            Shade::Light => "rz-shade-light",
            Shade::Default => "rz-shade-default",
            Shade::Dark => "rz-shade-dark",
            Shade::Darker => "rz-shade-darker",
        };
        self.add_class(class)
    }

    /// Add badge style (color) class based on [`BadgeStyle`].
    pub fn add_badge_style(self, style: BadgeStyle) -> Self {
        let class = match style {
            BadgeStyle::Primary => "rz-badge-primary",
            BadgeStyle::Secondary => "rz-badge-secondary",
            BadgeStyle::Light => "rz-badge-light",
            BadgeStyle::Base => "rz-badge-base",
            BadgeStyle::Dark => "rz-badge-dark",
            BadgeStyle::Success => "rz-badge-success",
            BadgeStyle::Warning => "rz-badge-warning",
            BadgeStyle::Danger => "rz-badge-danger",
            BadgeStyle::Info => "rz-badge-info",
        };
        self.add_class(class)
    }

    /// Add icon style class based on [`IconStyle`].
    ///
    /// Emits `rzi-{style}` e.g. `rzi-filled`, `rzi-rounded`.
    /// Pass the unwrapped value; call only when `Option<IconStyle>` is `Some`.
    pub fn add_icon_style(self, style: IconStyle) -> Self {
        let class = format!("rzi-{}", style.as_str());
        self.add_class(class)
    }
 
    /// Finish building and return the class string.
    ///
    /// Classes are joined with spaces. Empty strings are filtered out.
    pub fn finish(self) -> String {
        self.classes
            .into_iter()
            .filter(|c| !c.trim().is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl ToString for ClassList {
    fn to_string(&self) -> String {
        self.classes
            .iter()
            .filter(|c| !c.trim().is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_single_class() {
        let css = ClassList::new().add_class("btn").finish();
        assert_eq!(css, "btn");
    }

    #[test]
    fn test_add_multiple_classes() {
        let css = ClassList::new()
            .add_class("btn")
            .add_class("btn-primary")
            .finish();
        assert_eq!(css, "btn btn-primary");
    }

    #[test]
    fn test_conditional_add() {
        let css = ClassList::new()
            .add_class("btn")
            .add("disabled", true)
            .add("hidden", false)
            .finish();
        assert_eq!(css, "btn disabled");
    }

    #[test]
    fn test_add_variant() {
        let css = ClassList::new().add_variant(Variant::Outlined).finish();
        assert_eq!(css, "rz-variant-outlined");
    }

    #[test]
    fn test_add_button_size() {
        let css = ClassList::new().add_button_size(ButtonSize::Large).finish();
        assert_eq!(css, "rz-button-lg");
    }

    #[test]
    fn test_add_button_size_extra_small() {
        let css = ClassList::new()
            .add_button_size(ButtonSize::ExtraSmall)
            .finish();
        assert_eq!(css, "rz-button-xs");
    }

    #[test]
    fn test_add_button_style() {
        let css = ClassList::new()
            .add_button_style(ButtonStyle::Danger)
            .finish();
        assert_eq!(css, "rz-danger");
    }

    #[test]
    fn test_add_shade() {
        let css = ClassList::new().add_shade(Shade::Dark).finish();
        assert_eq!(css, "rz-shade-dark");
    }

    /// Verifies the canonical Blazor class order for RadzenButton:
    ///   rz-button → size → variant → style → disabled → shade
    /// Source: RadzenButton.razor.cs GetComponentCssClass()
    #[test]
    fn test_button_class_order_matches_blazor() {
        let css = ClassList::new()
            .add_class("rz-button")
            .add_button_size(ButtonSize::Medium)
            .add_variant(Variant::Filled)
            .add_button_style(ButtonStyle::Primary)
            .add_disabled(true)
            .add_shade(Shade::Default)
            .finish();
        assert_eq!(
            css,
            "rz-button rz-button-md rz-variant-filled rz-primary rz-state-disabled rz-shade-default"
        );
    }

    #[test]
    fn test_combined_build() {
        // Non-disabled case — disabled class must be absent, shade must follow style
        let css = ClassList::new()
            .add_class("btn")
            .add_button_size(ButtonSize::Medium)
            .add_button_style(ButtonStyle::Primary)
            .add_variant(Variant::Filled)
            .add_disabled(false)
            .add_shade(Shade::Default)
            .finish();
        assert_eq!(
            css,
            "btn rz-button-md rz-primary rz-variant-filled rz-shade-default"
        );
    }

    #[test]
    fn test_empty_class() {
        let css = ClassList::new().add("", true).add_class("btn").finish();
        assert_eq!(css, "btn");
    }

    #[test]
    fn test_add_from_attrs() {
        let mut attrs = HashMap::new();
        attrs.insert("class".to_string(), "custom-class".to_string());
        let css = ClassList::new()
            .add_class("btn")
            .add_from_attrs(&attrs)
            .finish();
        assert_eq!(css, "btn custom-class");
    }
}
