/// Horizontal alignment of content inside a component.
///
/// Mirrors `Radzen.HorizontalAlign` in `Radzen.Blazor/Common.cs`.
///
/// Used by [`RadzenPager`] (`HorizontalAlign` prop) and [`RadzenDataGrid`]
/// (`PagerHorizontalAlign` prop) to control how pager buttons are laid out.
///
/// | Variant | CSS / behaviour                                    |
/// |---------|----------------------------------------------------|
/// | Left    | Align to the left                                  |
/// | Center  | Center horizontally                                |
/// | Right   | Align to the right                                 |
/// | Justify | Spread buttons across the full width (default)     |
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum HorizontalAlign {
    /// Spread content to fill the full width. Default.
    #[default]
    Justify,

    /// Align content to the left.
    Left,

    /// Center content horizontally.
    Center,

    /// Align content to the right.
    Right,
}

impl HorizontalAlign {
    /// CSS `justify-content` value for a flex row, or `None` for `Justify`
    /// (which uses `space-between` in the Radzen pager theme).
    pub fn css_justify(&self) -> &'static str {
        match self {
            HorizontalAlign::Justify => "space-between",
            HorizontalAlign::Left => "flex-start",
            HorizontalAlign::Center => "center",
            HorizontalAlign::Right => "flex-end",
        }
    }
}
