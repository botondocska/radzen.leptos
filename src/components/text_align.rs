/// Horizontal text alignment for [`RadzenText`].
///
/// Mirrors `Radzen.TextAlign` as used in `RadzenText.cs` `BuildRenderTree`.
///
/// Variant order matches the C# switch statement in `BuildRenderTree` exactly:
/// Center, End, Justify, Start, Left, Right, JustifyAll.
///
/// `Left` is the default. Blazor's class-building is:
///   `.Add(alignClassName, TextAlign != TextAlign.Left)`
/// meaning `Left` resolves to `"rz-text-align-left"` internally but is never
/// appended to the class string.  `css_class()` returns `None` for `Left`
/// to reproduce this conditional — all other variants return `Some`.
#[derive(Clone, PartialEq, Default, Debug)]
pub enum TextAlign {
    /// Center-aligned text → `rz-text-align-center`.
    Center,
    /// Logical inline-end alignment → `rz-text-align-end`.
    End,
    /// Justify text → `rz-text-align-justify`.
    Justify,
    /// Logical inline-start alignment → `rz-text-align-start`.
    Start,
    /// Left-aligned text. Default — no alignment class is emitted.
    /// Blazor: `.Add("rz-text-align-left", TextAlign != TextAlign.Left)` →
    /// the string exists but the condition is false, so it is never added.
    #[default]
    Left,
    /// Right-aligned text → `rz-text-align-right`.
    Right,
    /// Justify all lines including the last → `rz-text-align-justify-all`.
    JustifyAll,
}

impl TextAlign {
    /// Returns the alignment CSS class for this value, or `None` for `Left`.
    ///
    /// `None` mirrors Blazor's `.Add(alignClassName, TextAlign != TextAlign.Left)`:
    /// the class is only appended when the condition is true.
    pub fn css_class(&self) -> Option<&'static str> {
        match self {
            TextAlign::Center     => Some("rz-text-align-center"),
            TextAlign::End        => Some("rz-text-align-end"),
            TextAlign::Justify    => Some("rz-text-align-justify"),
            TextAlign::Start      => Some("rz-text-align-start"),
            TextAlign::Left       => None,
            TextAlign::Right      => Some("rz-text-align-right"),
            TextAlign::JustifyAll => Some("rz-text-align-justify-all"),
        }
    }
}