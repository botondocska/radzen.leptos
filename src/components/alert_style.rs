/// Alert style enumeration - semantic color styles for alerts
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertStyle {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
    Base,
}

impl AlertStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertStyle::Primary => "primary",
            AlertStyle::Secondary => "secondary",
            AlertStyle::Success => "success",
            AlertStyle::Danger => "danger",
            AlertStyle::Warning => "warning",
            AlertStyle::Info => "info",
            AlertStyle::Light => "light",
            AlertStyle::Dark => "dark",
            AlertStyle::Base => "base",
        }
    }
}
