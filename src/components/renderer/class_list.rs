//! Fluent CSS class builder — mirrors C# Radzen.Blazor.Rendering.ClassList.

use crate::components::{BadgeStyle, ButtonSize, ButtonStyle, IconStyle, Shade, TextAlign, TextStyle, Variant};
use std::collections::HashMap;

/// Fluent builder for CSS class strings.
#[derive(Default)]
pub struct ClassList {
    classes: Vec<String>,
}

impl ClassList {
    pub fn new() -> Self {
        Self { classes: Vec::new() }
    }

    /// Add a class conditionally.
    pub fn add<S: Into<String>>(mut self, class: S, condition: bool) -> Self {
        if condition {
            let s = class.into();
            if !s.trim().is_empty() {
                self.classes.push(s);
            }
        }
        self
    }

    /// Add a class unconditionally.
    pub fn add_class<S: Into<String>>(self, class: S) -> Self {
        self.add(class, true)
    }

    /// Add a class if the option is `Some` and non-empty.
    pub fn add_option(self, class: Option<&str>) -> Self {
        if let Some(c) = class { self.add_class(c) } else { self }
    }

    /// Merge the `"class"` key from an attrs map.
    pub fn add_from_attrs(self, attrs: &HashMap<String, String>) -> Self {
        if let Some(class) = attrs.get("class") {
            self.add_class(class.clone())
        } else {
            self
        }
    }

    /// Add `rz-state-disabled` conditionally.
    pub fn add_disabled(self, condition: bool) -> Self {
        self.add("rz-state-disabled", condition)
    }

    /// Add button size class — mirrors Blazor `ClassList.AddButtonSize`.
    pub fn add_button_size(self, size: ButtonSize) -> Self {
        let class = match size {
            ButtonSize::ExtraSmall => "rz-button-xs",
            ButtonSize::Small      => "rz-button-sm",
            ButtonSize::Medium     => "rz-button-md",
            ButtonSize::Large      => "rz-button-lg",
        };
        self.add_class(class)
    }

    /// Add variant class — mirrors Blazor `ClassList.AddVariant`.
    pub fn add_variant(self, variant: Variant) -> Self {
        let class = match variant {
            Variant::Filled   => "rz-variant-filled",
            Variant::Flat     => "rz-variant-flat",
            Variant::Outlined => "rz-variant-outlined",
            Variant::Text     => "rz-variant-text",
        };
        self.add_class(class)
    }

    /// Add button style (color) class.
    pub fn add_button_style(self, style: ButtonStyle) -> Self {
        let class = match style {
            ButtonStyle::Primary   => "rz-primary",
            ButtonStyle::Secondary => "rz-secondary",
            ButtonStyle::Light     => "rz-light",
            ButtonStyle::Base      => "rz-base",
            ButtonStyle::Dark      => "rz-dark",
            ButtonStyle::Success   => "rz-success",
            ButtonStyle::Warning   => "rz-warning",
            ButtonStyle::Danger    => "rz-danger",
            ButtonStyle::Info      => "rz-info",
        };
        self.add_class(class)
    }

    /// Add shade class — mirrors Blazor `ClassList.AddShade`.
    pub fn add_shade(self, shade: Shade) -> Self {
        let class = match shade {
            Shade::Lighter => "rz-shade-lighter",
            Shade::Light   => "rz-shade-light",
            Shade::Default => "rz-shade-default",
            Shade::Dark    => "rz-shade-dark",
            Shade::Darker  => "rz-shade-darker",
        };
        self.add_class(class)
    }

    /// Add badge style class.
    pub fn add_badge_style(self, style: BadgeStyle) -> Self {
        let class = match style {
            BadgeStyle::Primary   => "rz-badge-primary",
            BadgeStyle::Secondary => "rz-badge-secondary",
            BadgeStyle::Light     => "rz-badge-light",
            BadgeStyle::Base      => "rz-badge-base",
            BadgeStyle::Dark      => "rz-badge-dark",
            BadgeStyle::Success   => "rz-badge-success",
            BadgeStyle::Warning   => "rz-badge-warning",
            BadgeStyle::Danger    => "rz-badge-danger",
            BadgeStyle::Info      => "rz-badge-info",
        };
        self.add_class(class)
    }

    /// Add icon style class — emits `rzi-{style}`.
    pub fn add_icon_style(self, style: IconStyle) -> Self {
        let class = format!("rzi-{}", style.as_str());
        self.add_class(class)
    }

    /// Add text style CSS class (`rz-text-*`).
    ///
    /// Mirrors Blazor's `className` switch in `RadzenText.BuildRenderTree`.
    pub fn add_text_style(self, style: TextStyle) -> Self {
        self.add_class(style.css_class())
    }

    /// Add text alignment CSS class.
    ///
    /// `TextAlign::Left` emits **no** class — mirrors Blazor's conditional:
    /// `.Add(alignClassName, TextAlign != TextAlign.Left)`.
    pub fn add_text_align(self, align: TextAlign) -> Self {
        match align.css_class() {
            Some(class) => self.add_class(class),
            None        => self,
        }
    }

    /// Build the final space-joined class string.
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
        let css = ClassList::new().add_class("btn").add_class("btn-primary").finish();
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
        let css = ClassList::new().add_button_size(ButtonSize::ExtraSmall).finish();
        assert_eq!(css, "rz-button-xs");
    }

    #[test]
    fn test_add_button_style() {
        let css = ClassList::new().add_button_style(ButtonStyle::Danger).finish();
        assert_eq!(css, "rz-danger");
    }

    #[test]
    fn test_add_shade() {
        let css = ClassList::new().add_shade(Shade::Dark).finish();
        assert_eq!(css, "rz-shade-dark");
    }

    /// Canonical Blazor class order for RadzenButton:
    ///   rz-button → size → variant → style → disabled → shade
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
        let css = ClassList::new()
            .add_class("btn")
            .add_button_size(ButtonSize::Medium)
            .add_button_style(ButtonStyle::Primary)
            .add_variant(Variant::Filled)
            .add_disabled(false)
            .add_shade(Shade::Default)
            .finish();
        assert_eq!(css, "btn rz-button-md rz-primary rz-variant-filled rz-shade-default");
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
        let css = ClassList::new().add_class("btn").add_from_attrs(&attrs).finish();
        assert_eq!(css, "btn custom-class");
    }

    // ── RadzenText class-building tests ───────────────────────────────────────

    /// TextAlign::Left → no align class emitted.
    /// Mirrors: `.Add(alignClassName, TextAlign != TextAlign.Left)` where
    /// condition is false.
    #[test]
    fn text_left_align_emits_no_class() {
        let css = ClassList::new()
            .add_text_style(TextStyle::H3)
            .add_text_align(TextAlign::Left)
            .finish();
        assert_eq!(css, "rz-text-h3");
    }

    /// TextAlign::Center → `rz-text-align-center` appended.
    #[test]
    fn text_center_align_emits_class() {
        let css = ClassList::new()
            .add_text_style(TextStyle::Body1)
            .add_text_align(TextAlign::Center)
            .finish();
        assert_eq!(css, "rz-text-body1 rz-text-align-center");
    }

    #[test]
    fn text_style_subtitle1_class() {
        assert_eq!(
            ClassList::new().add_text_style(TextStyle::Subtitle1).finish(),
            "rz-text-subtitle1"
        );
    }

    #[test]
    fn text_style_display_h1_class() {
        assert_eq!(
            ClassList::new().add_text_style(TextStyle::DisplayH1).finish(),
            "rz-text-display-h1"
        );
    }

    #[test]
    fn text_style_overline_class() {
        assert_eq!(
            ClassList::new().add_text_style(TextStyle::Overline).finish(),
            "rz-text-overline"
        );
    }

    #[test]
    fn text_align_justify_all_class() {
        let css = ClassList::new()
            .add_text_style(TextStyle::Caption)
            .add_text_align(TextAlign::JustifyAll)
            .finish();
        assert_eq!(css, "rz-text-caption rz-text-align-justify-all");
    }

    /// All seven TextAlign values — Left produces no class, others produce one.
    #[test]
    fn text_align_all_variants() {
        assert_eq!(TextAlign::Center.css_class(),     Some("rz-text-align-center"));
        assert_eq!(TextAlign::End.css_class(),        Some("rz-text-align-end"));
        assert_eq!(TextAlign::Justify.css_class(),    Some("rz-text-align-justify"));
        assert_eq!(TextAlign::Start.css_class(),      Some("rz-text-align-start"));
        assert_eq!(TextAlign::Left.css_class(),       None);
        assert_eq!(TextAlign::Right.css_class(),      Some("rz-text-align-right"));
        assert_eq!(TextAlign::JustifyAll.css_class(), Some("rz-text-align-justify-all"));
    }

    /// All 19 TextStyle CSS classes match the C# className switch.
    #[test]
    fn text_style_all_classes() {
        let cases = [
            (TextStyle::DisplayH1, "rz-text-display-h1"),
            (TextStyle::DisplayH2, "rz-text-display-h2"),
            (TextStyle::DisplayH3, "rz-text-display-h3"),
            (TextStyle::DisplayH4, "rz-text-display-h4"),
            (TextStyle::DisplayH5, "rz-text-display-h5"),
            (TextStyle::DisplayH6, "rz-text-display-h6"),
            (TextStyle::H1,        "rz-text-h1"),
            (TextStyle::H2,        "rz-text-h2"),
            (TextStyle::H3,        "rz-text-h3"),
            (TextStyle::H4,        "rz-text-h4"),
            (TextStyle::H5,        "rz-text-h5"),
            (TextStyle::H6,        "rz-text-h6"),
            (TextStyle::Subtitle1, "rz-text-subtitle1"),
            (TextStyle::Subtitle2, "rz-text-subtitle2"),
            (TextStyle::Body1,     "rz-text-body1"),
            (TextStyle::Body2,     "rz-text-body2"),
            (TextStyle::Button,    "rz-text-button"),
            (TextStyle::Caption,   "rz-text-caption"),
            (TextStyle::Overline,  "rz-text-overline"),
        ];
        for (style, expected) in cases {
            assert_eq!(style.css_class(), expected, "failed for {:?}", style);
        }
    }

    /// TagName::Auto resolves to None; all explicit variants resolve to Some.
    #[test]
    fn tag_name_auto_is_none() {
        use crate::components::TagName;
        assert_eq!(TagName::Auto.as_str(), None);
        assert_eq!(TagName::Div.as_str(),    Some("div"));
        assert_eq!(TagName::Span.as_str(),   Some("span"));
        assert_eq!(TagName::P.as_str(),      Some("p"));
        assert_eq!(TagName::H1.as_str(),     Some("h1"));
        assert_eq!(TagName::H6.as_str(),     Some("h6"));
        assert_eq!(TagName::A.as_str(),      Some("a"));
        assert_eq!(TagName::Button.as_str(), Some("button"));
        assert_eq!(TagName::Pre.as_str(),    Some("pre"));
    }

    /// TextStyle auto_tag matches the C# tagName switch.
    #[test]
    fn text_style_auto_tags() {
        assert_eq!(TextStyle::DisplayH1.auto_tag(), "h1");
        assert_eq!(TextStyle::DisplayH2.auto_tag(), "h2");
        assert_eq!(TextStyle::DisplayH3.auto_tag(), "h3");
        assert_eq!(TextStyle::DisplayH4.auto_tag(), "h4");
        assert_eq!(TextStyle::DisplayH5.auto_tag(), "h5");
        assert_eq!(TextStyle::DisplayH6.auto_tag(), "h6");
        assert_eq!(TextStyle::H1.auto_tag(),        "h1");
        assert_eq!(TextStyle::H2.auto_tag(),        "h2");
        assert_eq!(TextStyle::H3.auto_tag(),        "h3");
        assert_eq!(TextStyle::H4.auto_tag(),        "h4");
        assert_eq!(TextStyle::H5.auto_tag(),        "h5");
        assert_eq!(TextStyle::H6.auto_tag(),        "h6");
        assert_eq!(TextStyle::Subtitle1.auto_tag(), "h6");
        assert_eq!(TextStyle::Subtitle2.auto_tag(), "h6");
        assert_eq!(TextStyle::Body1.auto_tag(),     "p");
        assert_eq!(TextStyle::Body2.auto_tag(),     "p");
        assert_eq!(TextStyle::Button.auto_tag(),    "span");
        assert_eq!(TextStyle::Caption.auto_tag(),   "span");
        assert_eq!(TextStyle::Overline.auto_tag(),  "span");
    }
}