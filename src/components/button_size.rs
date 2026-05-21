/// Specifies the size of a [`RadzenButton`].
///
/// Controls the padding, font size, and overall dimensions of a button.
/// Maps to the `rz-button-{xs|sm|md|lg}` CSS class applied by
/// [`ClassList::add_button_size`].
///
/// Mirrors `Radzen.ButtonSize` in `Radzen.Blazor/Common.cs`.
#[derive(Clone, PartialEq, Default, Debug)]
pub enum ButtonSize {
    /// The smallest button size.
    ExtraSmall,

    /// Smaller than the default.
    Small,

    /// The default size.
    #[default]
    Medium,

    /// Larger than the default.
    Large,
}
