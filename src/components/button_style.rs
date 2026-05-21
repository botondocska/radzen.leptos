/// Semantic color style of a button.
///
/// Determines the button's color scheme based on its purpose.
/// Maps to a set of Tailwind color tokens defined in `tailwind.config.js`
/// under the `colors` key (e.g. `primary`, `secondary`, `danger`, …).
#[derive(Clone, PartialEq, Default, Debug)]
pub enum ButtonStyle {
    /// Main action in a form or dialog (e.g. "Save"). Default.
    #[default]
    Primary,

    /// Secondary action (e.g. "Cancel", "Close").
    Secondary,

    /// Lighter, less prominent styling.
    Light,

    /// Base UI neutral styling.
    Base,

    /// Dark styling.
    Dark,

    /// Positive / confirmation action (e.g. "Confirm", "Apply").
    Success,

    /// Destructive action (e.g. "Delete", "Remove").
    Danger,

    /// Cautionary action or state.
    Warning,

    /// Informational action or state.
    Info,
}

impl ButtonStyle {
    pub fn token(&self) -> &'static str {
        match self {
            ButtonStyle::Primary => "primary",
            ButtonStyle::Secondary => "secondary",
            ButtonStyle::Light => "light",
            ButtonStyle::Base => "base",
            ButtonStyle::Dark => "dark",
            ButtonStyle::Success => "success",
            ButtonStyle::Danger => "danger",
            ButtonStyle::Warning => "warning",
            ButtonStyle::Info => "info",
        }
    }
}
