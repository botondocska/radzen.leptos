/// Flex-wrap behaviour for children inside a [`RadzenStack`].
///
/// Mirrors `Radzen.FlexWrap` in `Radzen.Blazor/Common.cs`.
///
/// Maps to the CSS `flex-wrap` property.
///
/// | Variant     | CSS value      |
/// |-------------|----------------|
/// | NoWrap      | `nowrap`       |
/// | Wrap        | `wrap`         |
/// | WrapReverse | `wrap-reverse` |
#[derive(Clone, PartialEq, Default, Debug)]
pub enum FlexWrap {
    /// All children stay on a single line — may overflow. Default.
    #[default]
    NoWrap,

    /// Children wrap onto multiple lines.
    Wrap,

    /// Children wrap onto multiple lines in reverse order.
    WrapReverse,
}

impl FlexWrap {
    /// Returns the CSS `flex-wrap` value for this variant,
    /// or `None` for [`FlexWrap::NoWrap`] (browser default — not emitted).
    pub fn css_value(&self) -> Option<&'static str> {
        match self {
            FlexWrap::NoWrap => None,
            FlexWrap::Wrap => Some("wrap"),
            FlexWrap::WrapReverse => Some("wrap-reverse"),
        }
    }
}
