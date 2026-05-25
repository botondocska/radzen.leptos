//! Fluent CSS class builder — mirrors C# Radzen.Blazor.Rendering.ClassList.
//!
//! # Key API difference from C#
//!
//! Blazor: `ClassList.Create("rz-button").AddButtonSize(Size)...`
//! Rust:   `ClassList::create("rz-button").add_button_size(size)...`
//!
//! `create(root)` is the canonical entry point — it takes the component root
//! class (e.g. `"rz-button"`, `"rz-badge"`) as its first argument, exactly
//! mirroring C#'s `ClassList.Create(className)`.
//!
//! `new()` is kept for cases where no root class is needed (e.g. building a
//! partial class string that will be appended elsewhere).
//!
//! # Caller `attrs["class"]` ordering
//!
//! Blazor's `GetCssClass()` in `RadzenComponent` always appends the caller's
//! `class` attribute **last**, after all component-generated classes:
//!   `{component_classes} {caller_class}`
//!
//! Use `add_caller_class(Option<&str>)` as the **final** call in the chain
//! to reproduce this ordering exactly.

use crate::components::{
    BadgeStyle, ButtonSize, ButtonStyle, IconStyle, Shade, TextAlign, TextStyle, Variant,
};
use std::collections::HashMap;

/// Fluent builder for CSS class strings.
#[derive(Default)]
pub struct ClassList {
    classes: Vec<String>,
}

impl ClassList {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// Start a new empty builder.
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }

    /// Start a builder with a root class — mirrors `ClassList.Create(root)`.
    ///
    /// This is the canonical entry point. The root class (e.g. `"rz-button"`,
    /// `"rz-badge"`) is added unconditionally as the first class.
    pub fn create(root: &str) -> Self {
        let mut list = Self::new();
        if !root.trim().is_empty() {
            list.classes.push(root.to_string());
        }
        list
    }

    // ── Core add methods ──────────────────────────────────────────────────────

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
        match class {
            Some(c) if !c.trim().is_empty() => self.add_class(c),
            _ => self,
        }
    }

    /// Merge the `"class"` key from an attrs map.
    pub fn add_from_attrs(self, attrs: &HashMap<String, String>) -> Self {
        if let Some(class) = attrs.get("class") {
            self.add_class(class.clone())
        } else {
            self
        }
    }

    /// Append the caller-supplied `attrs["class"]` as the **last** class.
    ///
    /// Mirrors `RadzenComponent.GetCssClass()` which always appends the caller
    /// `class` attribute after all component-generated classes.
    ///
    /// Call this as the final step in the builder chain, passing
    /// `base.attrs.as_ref().and_then(|a| a.get("class")).map(String::as_str)`.
    pub fn add_caller_class(self, class: Option<&str>) -> Self {
        self.add_option(class)
    }

    // ── Semantic helpers ──────────────────────────────────────────────────────

    /// Add `rz-state-disabled` conditionally.
    pub fn add_disabled(self, condition: bool) -> Self {
        self.add("rz-state-disabled", condition)
    }

    /// Add button size class — mirrors Blazor `ClassList.AddButtonSize`.
    pub fn add_button_size(self, size: ButtonSize) -> Self {
        let class = match size {
            ButtonSize::ExtraSmall => "rz-button-xs",
            ButtonSize::Small => "rz-button-sm",
            ButtonSize::Medium => "rz-button-md",
            ButtonSize::Large => "rz-button-lg",
        };
        self.add_class(class)
    }

    /// Add variant class — mirrors Blazor `ClassList.AddVariant`.
    pub fn add_variant(self, variant: Variant) -> Self {
        let class = match variant {
            Variant::Filled => "rz-variant-filled",
            Variant::Flat => "rz-variant-flat",
            Variant::Outlined => "rz-variant-outlined",
            Variant::Text => "rz-variant-text",
        };
        self.add_class(class)
    }

    /// Add button style (color) class.
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

    /// Add shade class — mirrors Blazor `ClassList.AddShade`.
    ///
    /// Uses `Shade::css_class()` so the mapping lives in one place.
    pub fn add_shade(self, shade: Shade) -> Self {
        self.add_class(shade.css_class())
    }

    /// Add badge style class.
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
            None => self,
        }
    }

    // ── Terminal ──────────────────────────────────────────────────────────────

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
    fn test_create_sets_root_class() {
        let css = ClassList::create("rz-button").finish();
        assert_eq!(css, "rz-button");
    }

    #[test]
    fn test_create_empty_root_ignored() {
        let css = ClassList::create("").add_class("btn").finish();
        assert_eq!(css, "btn");
    }

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
    fn test_add_shade_uses_css_class() {
        assert_eq!(
            ClassList::new().add_shade(Shade::Dark).finish(),
            "rz-shade-dark"
        );
        assert_eq!(
            ClassList::new().add_shade(Shade::Lighter).finish(),
            "rz-shade-lighter"
        );
        assert_eq!(
            ClassList::new().add_shade(Shade::Default).finish(),
            "rz-shade-default"
        );
    }

    /// Canonical Blazor class order for RadzenButton:
    ///   rz-button → size → variant → style → disabled → shade → [icon-only] → [caller]
    #[test]
    fn test_button_class_order_matches_blazor() {
        let css = ClassList::create("rz-button")
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

    /// Canonical Blazor class order for RadzenBadge:
    ///   rz-badge → rz-badge-{style} → variant → shade → [pill] → [caller]
    #[test]
    fn test_badge_class_order_matches_blazor() {
        let css = ClassList::create("rz-badge")
            .add_badge_style(BadgeStyle::Danger)
            .add_variant(Variant::Outlined)
            .add_shade(Shade::Light)
            .add("rz-badge-pill", true)
            .add_caller_class(Some("my-custom-class"))
            .finish();
        assert_eq!(
            css,
            "rz-badge rz-badge-danger rz-variant-outlined rz-shade-light rz-badge-pill my-custom-class"
        );
    }

    /// Caller class always comes last.
    #[test]
    fn test_caller_class_is_last() {
        let css = ClassList::create("rz-card")
            .add_variant(Variant::Filled)
            .add_caller_class(Some("caller-extra"))
            .finish();
        assert_eq!(css, "rz-card rz-variant-filled caller-extra");
    }

    /// `add_caller_class(None)` is a no-op.
    #[test]
    fn test_caller_class_none_is_noop() {
        let css = ClassList::create("rz-card")
            .add_variant(Variant::Filled)
            .add_caller_class(None)
            .finish();
        assert_eq!(css, "rz-card rz-variant-filled");
    }

    #[test]
    fn test_combined_build() {
        let css = ClassList::create("btn")
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
    fn test_empty_class_ignored() {
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

    // ── RadzenText class-building tests ───────────────────────────────────────

    /// TextAlign::Left → no align class emitted.
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
            ClassList::new()
                .add_text_style(TextStyle::Subtitle1)
                .finish(),
            "rz-text-subtitle1"
        );
    }

    #[test]
    fn text_style_display_h1_class() {
        assert_eq!(
            ClassList::new()
                .add_text_style(TextStyle::DisplayH1)
                .finish(),
            "rz-text-display-h1"
        );
    }

    #[test]
    fn text_style_overline_class() {
        assert_eq!(
            ClassList::new()
                .add_text_style(TextStyle::Overline)
                .finish(),
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

    /// All seven TextAlign values.
    #[test]
    fn text_align_all_variants() {
        assert_eq!(TextAlign::Center.css_class(), Some("rz-text-align-center"));
        assert_eq!(TextAlign::End.css_class(), Some("rz-text-align-end"));
        assert_eq!(
            TextAlign::Justify.css_class(),
            Some("rz-text-align-justify")
        );
        assert_eq!(TextAlign::Start.css_class(), Some("rz-text-align-start"));
        assert_eq!(TextAlign::Left.css_class(), None);
        assert_eq!(TextAlign::Right.css_class(), Some("rz-text-align-right"));
        assert_eq!(
            TextAlign::JustifyAll.css_class(),
            Some("rz-text-align-justify-all")
        );
    }

    /// All 19 TextStyle CSS classes.
    #[test]
    fn text_style_all_classes() {
        let cases = [
            (TextStyle::DisplayH1, "rz-text-display-h1"),
            (TextStyle::DisplayH2, "rz-text-display-h2"),
            (TextStyle::DisplayH3, "rz-text-display-h3"),
            (TextStyle::DisplayH4, "rz-text-display-h4"),
            (TextStyle::DisplayH5, "rz-text-display-h5"),
            (TextStyle::DisplayH6, "rz-text-display-h6"),
            (TextStyle::H1, "rz-text-h1"),
            (TextStyle::H2, "rz-text-h2"),
            (TextStyle::H3, "rz-text-h3"),
            (TextStyle::H4, "rz-text-h4"),
            (TextStyle::H5, "rz-text-h5"),
            (TextStyle::H6, "rz-text-h6"),
            (TextStyle::Subtitle1, "rz-text-subtitle1"),
            (TextStyle::Subtitle2, "rz-text-subtitle2"),
            (TextStyle::Body1, "rz-text-body1"),
            (TextStyle::Body2, "rz-text-body2"),
            (TextStyle::Button, "rz-text-button"),
            (TextStyle::Caption, "rz-text-caption"),
            (TextStyle::Overline, "rz-text-overline"),
        ];
        for (style, expected) in cases {
            assert_eq!(style.css_class(), expected, "failed for {:?}", style);
        }
    }

    #[test]
    fn tag_name_auto_is_none() {
        use crate::components::TagName;
        assert_eq!(TagName::Auto.as_str(), None);
        assert_eq!(TagName::Div.as_str(), Some("div"));
        assert_eq!(TagName::Span.as_str(), Some("span"));
        assert_eq!(TagName::P.as_str(), Some("p"));
        assert_eq!(TagName::H1.as_str(), Some("h1"));
        assert_eq!(TagName::H6.as_str(), Some("h6"));
        assert_eq!(TagName::A.as_str(), Some("a"));
        assert_eq!(TagName::Button.as_str(), Some("button"));
        assert_eq!(TagName::Pre.as_str(), Some("pre"));
    }

    #[test]
    fn text_style_auto_tags() {
        assert_eq!(TextStyle::DisplayH1.auto_tag(), "h1");
        assert_eq!(TextStyle::DisplayH6.auto_tag(), "h6");
        assert_eq!(TextStyle::H1.auto_tag(), "h1");
        assert_eq!(TextStyle::H6.auto_tag(), "h6");
        assert_eq!(TextStyle::Subtitle1.auto_tag(), "h6");
        assert_eq!(TextStyle::Subtitle2.auto_tag(), "h6");
        assert_eq!(TextStyle::Body1.auto_tag(), "p");
        assert_eq!(TextStyle::Body2.auto_tag(), "p");
        assert_eq!(TextStyle::Button.auto_tag(), "span");
        assert_eq!(TextStyle::Caption.auto_tag(), "span");
        assert_eq!(TextStyle::Overline.auto_tag(), "span");
    }

    // ── Shade::css_class canonical values ────────────────────────────────────

    #[test]
    fn shade_css_class_all_variants() {
        assert_eq!(Shade::Lighter.css_class(), "rz-shade-lighter");
        assert_eq!(Shade::Light.css_class(), "rz-shade-light");
        assert_eq!(Shade::Default.css_class(), "rz-shade-default");
        assert_eq!(Shade::Dark.css_class(), "rz-shade-dark");
        assert_eq!(Shade::Darker.css_class(), "rz-shade-darker");
    }
}
