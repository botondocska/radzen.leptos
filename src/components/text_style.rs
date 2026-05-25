/// Typography style of a [`RadzenText`] component.
///
/// Controls both the default HTML tag rendered and the CSS class applied.
/// Mirrors `Radzen.TextStyle` in `Radzen.Blazor/RadzenText.cs`.
///
/// Tag name auto-resolution (used when [`TagName::Auto`] is set, which is
/// the default):
///
/// | Variant      | Auto tag | CSS class              |
/// |--------------|----------|------------------------|
/// | DisplayH1    | h1       | rz-text-display-h1     |
/// | DisplayH2    | h2       | rz-text-display-h2     |
/// | DisplayH3    | h3       | rz-text-display-h3     |
/// | DisplayH4    | h4       | rz-text-display-h4     |
/// | DisplayH5    | h5       | rz-text-display-h5     |
/// | DisplayH6    | h6       | rz-text-display-h6     |
/// | H1           | h1       | rz-text-h1             |
/// | H2           | h2       | rz-text-h2             |
/// | H3           | h3       | rz-text-h3             |
/// | H4           | h4       | rz-text-h4             |
/// | H5           | h5       | rz-text-h5             |
/// | H6           | h6       | rz-text-h6             |
/// | Subtitle1    | h6       | rz-text-subtitle1      |
/// | Subtitle2    | h6       | rz-text-subtitle2      |
/// | Body1        | p        | rz-text-body1          |
/// | Body2        | p        | rz-text-body2          |
/// | Button       | span     | rz-text-button         |
/// | Caption      | span     | rz-text-caption        |
/// | Overline     | span     | rz-text-overline       |
#[derive(Clone, PartialEq, Default, Debug)]
pub enum TextStyle {
    /// Display as largest display header.
    DisplayH1,
    /// Display as second-largest display header.
    DisplayH2,
    /// Display as third display header.
    DisplayH3,
    /// Display as fourth display header.
    DisplayH4,
    /// Display as fifth display header.
    DisplayH5,
    /// Display as sixth display header.
    DisplayH6,

    /// Standard H1 heading.
    H1,
    /// Standard H2 heading.
    H2,
    /// Standard H3 heading.
    H3,
    /// Standard H4 heading.
    H4,
    /// Standard H5 heading.
    H5,
    /// Standard H6 heading.
    H6,

    /// Subtitle — large.
    Subtitle1,
    /// Subtitle — small.
    Subtitle2,

    /// Body paragraph — default.
    #[default]
    Body1,
    /// Body paragraph — small.
    Body2,

    /// Button-style inline text.
    Button,
    /// Caption text.
    Caption,
    /// Overline label text.
    Overline,
}

impl TextStyle {
    /// CSS class emitted for this style.
    ///
    /// Mirrors the `className` switch in `BuildRenderTree`.
    pub fn css_class(&self) -> &'static str {
        match self {
            TextStyle::DisplayH1 => "rz-text-display-h1",
            TextStyle::DisplayH2 => "rz-text-display-h2",
            TextStyle::DisplayH3 => "rz-text-display-h3",
            TextStyle::DisplayH4 => "rz-text-display-h4",
            TextStyle::DisplayH5 => "rz-text-display-h5",
            TextStyle::DisplayH6 => "rz-text-display-h6",
            TextStyle::H1 => "rz-text-h1",
            TextStyle::H2 => "rz-text-h2",
            TextStyle::H3 => "rz-text-h3",
            TextStyle::H4 => "rz-text-h4",
            TextStyle::H5 => "rz-text-h5",
            TextStyle::H6 => "rz-text-h6",
            TextStyle::Subtitle1 => "rz-text-subtitle1",
            TextStyle::Subtitle2 => "rz-text-subtitle2",
            TextStyle::Body1 => "rz-text-body1",
            TextStyle::Body2 => "rz-text-body2",
            TextStyle::Button => "rz-text-button",
            TextStyle::Caption => "rz-text-caption",
            TextStyle::Overline => "rz-text-overline",
        }
    }

    /// Default HTML tag for this style when [`TagName::Auto`] is active.
    ///
    /// Mirrors the `tagName` switch in `BuildRenderTree`.
    pub fn auto_tag(&self) -> &'static str {
        match self {
            TextStyle::DisplayH1 | TextStyle::H1 => "h1",
            TextStyle::DisplayH2 | TextStyle::H2 => "h2",
            TextStyle::DisplayH3 | TextStyle::H3 => "h3",
            TextStyle::DisplayH4 | TextStyle::H4 => "h4",
            TextStyle::DisplayH5 | TextStyle::H5 => "h5",
            TextStyle::DisplayH6 | TextStyle::H6 | TextStyle::Subtitle1 | TextStyle::Subtitle2 => {
                "h6"
            }
            TextStyle::Body1 | TextStyle::Body2 => "p",
            TextStyle::Button | TextStyle::Caption | TextStyle::Overline => "span",
        }
    }
}
