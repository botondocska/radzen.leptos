/// HTML `type` attribute of a button element.
#[derive(Clone, PartialEq, Default, Debug)]
pub enum ButtonType {
    /// Generic button — does not submit its parent form. Default.
    #[default]
    Button,

    /// Submits the parent form when clicked.
    Submit,

    /// Resets all inputs in the parent form when clicked.
    Reset,
}

impl ButtonType {
    /// Returns the lowercase string used as the HTML `type` attribute value.
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset  => "reset",
        }
    }
}