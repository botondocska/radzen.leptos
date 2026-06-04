/// Browser autocomplete behaviour for text input components.
///
/// Maps to the HTML `autocomplete` attribute value sent to the browser.
/// Mirrors `Radzen.AutoCompleteType` in `Radzen.Blazor/AutoCompleteType.cs`.
///
/// The default is [`AutoCompleteType::On`] — the browser may autofill based
/// on user history.  Use [`AutoCompleteType::Off`] to suppress autofill
/// entirely, or a semantic token (e.g. `Email`, `Username`) to give the
/// browser a hint about the expected value type.
///
/// Reference: <https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/autocomplete>
#[derive(Debug, Clone, PartialEq, Default)]
pub enum AutoCompleteType {
    /// Browser chooses what to suggest. Default.
    #[default]
    On,
    /// Autocomplete is disabled.
    Off,
    /// A full name.
    Name,
    /// A given (first) name.
    GivenName,
    /// A family (last) name.
    FamilyName,
    /// An email address.
    Email,
    /// A username or account name.
    Username,
    /// A new password (prompts password manager to offer to save).
    NewPassword,
    /// The current password for the account.
    CurrentPassword,
    /// A one-time code (OTP).
    OneTimeCode,
    /// A job title.
    OrganizationTitle,
    /// A company or organisation name.
    Organization,
    /// A street address (full, single field).
    StreetAddress,
    /// Address line 1.
    AddressLine1,
    /// Address line 2.
    AddressLine2,
    /// Address line 3.
    AddressLine3,
    /// A city or town.
    AddressLevel2,
    /// A state, province, or region.
    AddressLevel1,
    /// A postal / ZIP code.
    PostalCode,
    /// A country code (ISO 3166-1 alpha-2).
    CountryName,
    /// A telephone number.
    Tel,
    /// A URL.
    Url,
}

impl AutoCompleteType {
    /// Returns the HTML `autocomplete` attribute string for this variant.
    ///
    /// Mirrors `AutoCompleteType.GetAutoCompleteValue()` in C#.
    pub fn as_str(&self) -> &'static str {
        match self {
            AutoCompleteType::On => "on",
            AutoCompleteType::Off => "off",
            AutoCompleteType::Name => "name",
            AutoCompleteType::GivenName => "given-name",
            AutoCompleteType::FamilyName => "family-name",
            AutoCompleteType::Email => "email",
            AutoCompleteType::Username => "username",
            AutoCompleteType::NewPassword => "new-password",
            AutoCompleteType::CurrentPassword => "current-password",
            AutoCompleteType::OneTimeCode => "one-time-code",
            AutoCompleteType::OrganizationTitle => "organization-title",
            AutoCompleteType::Organization => "organization",
            AutoCompleteType::StreetAddress => "street-address",
            AutoCompleteType::AddressLine1 => "address-line1",
            AutoCompleteType::AddressLine2 => "address-line2",
            AutoCompleteType::AddressLine3 => "address-line3",
            AutoCompleteType::AddressLevel2 => "address-level2",
            AutoCompleteType::AddressLevel1 => "address-level1",
            AutoCompleteType::PostalCode => "postal-code",
            AutoCompleteType::CountryName => "country-name",
            AutoCompleteType::Tel => "tel",
            AutoCompleteType::Url => "url",
        }
    }
}
