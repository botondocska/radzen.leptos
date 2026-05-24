/// Main-axis distribution of children inside a [`RadzenStack`].
///
/// Mirrors `Radzen.JustifyContent` in `Radzen.Blazor/Common.cs`.
///
/// Maps to the CSS `justify-content` property.
///
/// | Variant      | CSS value          |
/// |--------------|--------------------|
/// | Normal       | `normal`           |
/// | Center       | `center`           |
/// | Start        | `flex-start`       |
/// | End          | `flex-end`         |
/// | Left         | `left`             |
/// | Right        | `right`            |
/// | SpaceBetween | `space-between`    |
/// | SpaceAround  | `space-around`     |
/// | SpaceEvenly  | `space-evenly`     |
/// | Stretch      | `stretch`          |
#[derive(Clone, PartialEq, Default, Debug)]
pub enum JustifyContent {
    /// Browser default — not emitted. Default.
    #[default]
    Normal,

    /// Center children on the main axis.
    Center,

    /// Align children to the start of the main axis.
    Start,

    /// Align children to the end of the main axis.
    End,

    /// Align children to the left.
    Left,

    /// Align children to the right.
    Right,

    /// Distribute children with equal space between them.
    SpaceBetween,

    /// Distribute children with equal space around them.
    SpaceAround,

    /// Distribute children with equal space between and around them.
    SpaceEvenly,

    /// Stretch children to fill the main axis.
    Stretch,
}

impl JustifyContent {
    /// Returns the CSS `justify-content` value for this variant,
    /// or `None` for [`JustifyContent::Normal`] (browser default — not emitted).
    pub fn css_value(&self) -> Option<&'static str> {
        match self {
            JustifyContent::Normal       => None,
            JustifyContent::Center       => Some("center"),
            JustifyContent::Start        => Some("flex-start"),
            JustifyContent::End          => Some("flex-end"),
            JustifyContent::Left         => Some("left"),
            JustifyContent::Right        => Some("right"),
            JustifyContent::SpaceBetween => Some("space-between"),
            JustifyContent::SpaceAround  => Some("space-around"),
            JustifyContent::SpaceEvenly  => Some("space-evenly"),
            JustifyContent::Stretch      => Some("stretch"),
        }
    }
}