/// Color intensity shade applied on top of a [`ButtonStyle`].
///
/// For `Variant::Filled` this shifts the background color darker or lighter.
/// For `Variant::Outlined`, `Flat`, and `Text` it shifts the text/border
/// color tone instead (background is transparent or near-transparent).
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum Shade {
    /// Most washed-out — uses the `lighter` color token.
    Lighter,

    /// Slightly lighter than default — uses the `light` color token.
    Light,

    /// Base color — no modification. Default.
    #[default]
    Default,

    /// Slightly darker than default — uses the `dark` color token.
    Dark,

    /// Most saturated / darkest — uses the `darker` color token.
    Darker,
}

impl Shade {
    /// Returns the full CSS class for this shade.
    ///
    /// This is the canonical mapping used by [`ClassList::add_shade`].
    /// Mirrors Blazor's `ClassList.AddShade` which emits `rz-shade-{value}`.
    pub fn css_class(&self) -> &'static str {
        match self {
            Shade::Lighter => "rz-shade-lighter",
            Shade::Light => "rz-shade-light",
            Shade::Default => "rz-shade-default",
            Shade::Dark => "rz-shade-dark",
            Shade::Darker => "rz-shade-darker",
        }
    }

    /// Returns the Tailwind color token suffix for this shade.
    ///
    /// Combine with a [`ButtonStyle::token`] to build a full Tailwind class,
    /// e.g. `bg-primary-dark`, `text-danger-lighter`.
    ///
    /// Prefer [`css_class`] when building Radzen CSS class strings.
    pub fn suffix(&self) -> &'static str {
        match self {
            Shade::Lighter => "lighter",
            Shade::Light => "light",
            Shade::Default => "DEFAULT",
            Shade::Dark => "dark",
            Shade::Darker => "darker",
        }
    }
}
