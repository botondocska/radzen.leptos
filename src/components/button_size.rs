/// Controls the padding, font size, and overall dimensions of a button.
#[derive(Clone, PartialEq, Default, Debug)]
pub enum ButtonSize {
    /// Default size — `px-4 py-2 text-sm`. Default.
    #[default]
    Medium,

    /// Larger than default — `px-6 py-3 text-base`.
    Large,

    /// Smaller than default — `px-3 py-1.5 text-sm`.
    Small,

    /// Smallest available — `px-2 py-1 text-xs`.
    ExtraSmall,
}

impl ButtonSize {
    /// Returns the Tailwind utility classes for padding and font size.
    pub fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Medium     => "px-4 py-2 text-sm",
            ButtonSize::Large      => "px-6 py-3 text-base",
            ButtonSize::Small      => "px-3 py-1.5 text-sm",
            ButtonSize::ExtraSmall => "px-2 py-1 text-xs",
        }
    }
}