/// Spacing density of a component — controls padding and font-size compactness.
///
/// Mirrors `Radzen.Density` in `Radzen.Blazor/Common.cs`.
///
/// Used by [`RadzenPager`] and [`RadzenDataGrid`] to switch between normal
/// and compact spacing modes.
///
/// Maps to the `rz-density-compact` CSS class when `Compact`.
/// No class is added for `Default`.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum Density {
    /// Normal spacing. Default. No CSS class added.
    #[default]
    Default,

    /// Reduced padding / font size — adds `rz-density-compact` CSS class.
    Compact,
}

impl Density {
    /// Returns the extra CSS class to append, or `None` for `Default`.
    pub fn css_class(&self) -> Option<&'static str> {
        match self {
            Density::Default => None,
            Density::Compact => Some("rz-density-compact"),
        }
    }
}
