/// Alert size enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertSize {
    ExtraSmall,
    Small,
    Medium,
    Large,
}

impl AlertSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertSize::ExtraSmall => "xs",
            AlertSize::Small => "sm",
            AlertSize::Medium => "md",
            AlertSize::Large => "lg",
        }
    }
}
