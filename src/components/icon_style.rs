/// Visual style variant for Material Symbols icons.
///
/// Controls the rendering style of the icon glyph. Corresponds to the
/// `font-variation-settings` axis of the Material Symbols variable font.
///
/// Maps to CSS classes: `rzi-outlined`, `rzi-filled`, `rzi-rounded`, `rzi-sharp`.
/// When `None` (default), no style class is added and the font renders in its
/// default Outlined style — matching Blazor's `IconStyle? IconStyle` nullable param.
///
/// Mirrors `Radzen.IconStyle` in `Radzen.Blazor/Common.cs`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconStyle {
    /// Default — outlined strokes, no fill. No class added when this is the default.
    Outlined,

    /// Solid filled shapes.
    Filled,

    /// Rounded corners and strokes.
    Rounded,

    /// Sharp corners and geometric precision.
    Sharp,
}

impl IconStyle {
    /// Returns the lowercase token appended to `rzi-` to form the CSS class.
    ///
    /// Mirrors Blazor's `IconStyle.Value.ToString().ToLowerInvariant()`.
    pub fn as_str(&self) -> &'static str {
        match self {
            IconStyle::Outlined => "outlined",
            IconStyle::Filled => "filled",
            IconStyle::Rounded => "rounded",
            IconStyle::Sharp => "sharp",
        }
    }
}
