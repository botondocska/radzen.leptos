/// Specifies the size of a [`RadzenAlert`].
///
/// Controls the padding, font size, and icon size within the alert.
/// Maps to the `rz-alert-{xs|sm|md|lg}` CSS class.
///
/// Mirrors `Radzen.AlertSize` in `Radzen.Blazor/AlertSize.cs`.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AlertSize {
    /// The smallest alert size.
    ExtraSmall,

    /// Smaller than the default.
    Small,

    /// The default size.
    #[default]
    Medium,

    /// Larger than the default.
    Large,
}

impl AlertSize {
    /// Returns the CSS suffix used in `rz-alert-{suffix}`.
    ///
    /// Mirrors `GetAlertSize()` in `RadzenAlert.razor.cs`:
    /// Medium → `"md"`, Large → `"lg"`, Small → `"sm"`, ExtraSmall → `"xs"`.
    pub fn css_suffix(&self) -> &'static str {
        match self {
            AlertSize::Medium => "md",
            AlertSize::Large => "lg",
            AlertSize::Small => "sm",
            AlertSize::ExtraSmall => "xs",
        }
    }
}
