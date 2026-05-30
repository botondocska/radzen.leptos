/// Semantic style/severity of a [`RadzenAlert`].
///
/// Determines the alert's color scheme and default icon.
/// Maps to the `rz-{style}` CSS class (e.g. `rz-success`, `rz-danger`).
///
/// Mirrors `Radzen.AlertStyle` in `Radzen.Blazor/AlertStyle.cs`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AlertStyle {
    /// Primary styling — main action color.
    Primary,

    /// Secondary styling — supporting action color.
    Secondary,

    /// Success — positive outcome, green tones. Default icon: `check_circle`.
    Success,

    /// Danger — destructive or error state, red tones. Default icon: `error`.
    Danger,

    /// Warning — cautionary state, amber tones. Default icon: `warning_amber`.
    Warning,

    /// Info — informational state, blue tones. Default icon: `info`.
    Info,

    /// Light — subtle light styling.
    Light,

    /// Dark — dark styling.
    Dark,

    /// Base UI neutral styling. Default. Default icon: `lightbulb`.
    #[default]
    Base,
}

impl AlertStyle {
    /// Returns the lowercase token used to build the `rz-{token}` CSS class.
    ///
    /// Mirrors `Enum.GetName<AlertStyle>(AlertStyle).ToLowerInvariant()` in C#.
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertStyle::Primary => "primary",
            AlertStyle::Secondary => "secondary",
            AlertStyle::Success => "success",
            AlertStyle::Danger => "danger",
            AlertStyle::Warning => "warning",
            AlertStyle::Info => "info",
            AlertStyle::Light => "light",
            AlertStyle::Dark => "dark",
            AlertStyle::Base => "base",
        }
    }

    /// Returns the default Material icon name for this alert style.
    ///
    /// Mirrors `GetIcon()` in `RadzenAlert.razor.cs`:
    /// - Success → `"check_circle"`
    /// - Danger  → `"error"`
    /// - Warning → `"warning_amber"`
    /// - Info    → `"info"`
    /// - _       → `"lightbulb"`
    pub fn default_icon(&self) -> &'static str {
        match self {
            AlertStyle::Success => "check_circle",
            AlertStyle::Danger => "error",
            AlertStyle::Warning => "warning_amber",
            AlertStyle::Info => "info",
            _ => "lightbulb",
        }
    }
}
