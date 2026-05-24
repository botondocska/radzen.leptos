/// Cross-axis alignment of children inside a [`RadzenStack`].
///
/// Mirrors `Radzen.AlignItems` in `Radzen.Blazor/Common.cs`.
///
/// Maps to the CSS `align-items` property.
///
/// | Variant  | CSS value     |
/// |----------|---------------|
/// | Normal   | `normal`      |
/// | Center   | `center`      |
/// | Start    | `flex-start`  |
/// | End      | `flex-end`    |
/// | Stretch  | `stretch`     |
#[derive(Clone, PartialEq, Default, Debug)]
pub enum AlignItems {
    /// Browser default — no alignment class applied (`normal`). Default.
    #[default]
    Normal,

    /// Center children on the cross axis.
    Center,

    /// Align children to the start of the cross axis.
    Start,

    /// Align children to the end of the cross axis.
    End,

    /// Stretch children to fill the cross axis.
    Stretch,
}

impl AlignItems {
    /// Returns the CSS `align-items` value for this variant,
    /// or `None` for [`AlignItems::Normal`] (browser default — not emitted).
    pub fn css_value(&self) -> Option<&'static str> {
        match self {
            AlignItems::Normal  => None,
            AlignItems::Center  => Some("center"),
            AlignItems::Start   => Some("flex-start"),
            AlignItems::End     => Some("flex-end"),
            AlignItems::Stretch => Some("stretch"),
        }
    }
}