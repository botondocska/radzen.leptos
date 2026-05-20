/// Badge style enumeration - semantic color styles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BadgeStyle {
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

impl BadgeStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            BadgeStyle::Primary => "primary",
            BadgeStyle::Secondary => "secondary",
            BadgeStyle::Success => "success",
            BadgeStyle::Danger => "danger",
            BadgeStyle::Warning => "warning",
            BadgeStyle::Info => "info",
            BadgeStyle::Light => "light",
            BadgeStyle::Dark => "dark",
            BadgeStyle::Base => "base",
        }
    }
}
