/// Color intensity shade applied on top of a [`ButtonStyle`].
///
/// For `Variant::Filled` this shifts the background color darker or lighter.
/// For `Variant::Outlined`, `Flat`, and `Text` it shifts the text/border
/// color tone instead (background is transparent or near-transparent).
#[derive(Clone, PartialEq, Default, Debug)]
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
    /// Returns the Tailwind color token suffix for this shade.
    ///
    /// Combine with a [`ButtonStyle::token`] to build a full Tailwind class,
    /// e.g. `bg-primary-dark`, `text-danger-lighter`.
    pub fn suffix(&self) -> &'static str {
        match self {
            Shade::Lighter => "lighter",
            Shade::Light   => "light",
            Shade::Default => "DEFAULT",
            Shade::Dark    => "dark",
            Shade::Darker  => "darker",
        }
    }
}