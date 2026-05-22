/// Horizontal text alignment for [`RadzenText`].
///
/// Mirrors `Radzen.TextAlign` as used in `RadzenText.cs` `BuildRenderTree`.
///
/// `Left` is the default and emits **no** alignment CSS class (same behaviour
/// as Blazor: `.Add(alignClassName, TextAlign != TextAlign.Left)`).
/// All other variants emit a `rz-text-align-*` class.
#[derive(Clone, PartialEq, Default, Debug)]
pub enum TextAlign {
    /// Left-aligned text. Default — no alignment class is emitted.
    #[default]
    Left,
    /// Right-aligned text → `rz-text-align-right`.
    Right,
    /// Center-aligned text → `rz-text-align-center`.
    Center,
    /// Justify text → `rz-text-align-justify`.
    Justify,
    /// Logical inline-start alignment → `rz-text-align-start`.
    Start,
    /// Logical inline-end alignment → `rz-text-align-end`.
    End,
    /// Justify all lines including the last → `rz-text-align-justify-all`.
    JustifyAll,
}

impl TextAlign {
    /// Returns the alignment CSS class for this value, or `None` when the
    /// value is `Left` (no class emitted — mirrors Blazor's conditional).
    pub fn css_class(&self) -> Option<&'static str> {
        match self {
            TextAlign::Left => None,
            TextAlign::Right => Some("rz-text-align-right"),
            TextAlign::Center => Some("rz-text-align-center"),
            TextAlign::Justify => Some("rz-text-align-justify"),
            TextAlign::Start => Some("rz-text-align-start"),
            TextAlign::End => Some("rz-text-align-end"),
            TextAlign::JustifyAll => Some("rz-text-align-justify-all"),
        }
    }
}