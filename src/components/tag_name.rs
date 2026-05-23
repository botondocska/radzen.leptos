/// HTML element tag rendered by [`RadzenText`].
///
/// Mirrors `Radzen.TagName` in `Radzen.Blazor/RadzenText.cs`.
///
/// Variant order matches the C# enum exactly.
/// When set to [`TagName::Auto`] (the default) the tag is derived from
/// [`TextStyle::auto_tag`] — e.g. `TextStyle::H3` renders `<h3>`.
/// Any explicit variant overrides that automatic choice regardless of style,
/// which is useful for semantic correctness (e.g. applying `H5` styling to
/// an `<h2>` element).
#[derive(Clone, PartialEq, Default, Debug)]
pub enum TagName {
    /// Render as `<div>`.
    Div,
    /// Render as `<span>`.
    Span,
    /// Render as `<p>`.
    P,
    /// Render as `<h1>`.
    H1,
    /// Render as `<h2>`.
    H2,
    /// Render as `<h3>`.
    H3,
    /// Render as `<h4>`.
    H4,
    /// Render as `<h5>`.
    H5,
    /// Render as `<h6>`.
    H6,
    /// Render as `<a>`.
    A,
    /// Render as `<button>`.
    Button,
    /// Render as `<pre>`.
    Pre,
    /// Tag is chosen automatically from [`TextStyle`]. Default.
    #[default]
    Auto,
}

impl TagName {
    /// Returns the literal HTML tag string when an explicit tag is set,
    /// or `None` for [`TagName::Auto`] (caller must fall back to
    /// [`TextStyle::auto_tag`]).
    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            TagName::Div    => Some("div"),
            TagName::Span   => Some("span"),
            TagName::P      => Some("p"),
            TagName::H1     => Some("h1"),
            TagName::H2     => Some("h2"),
            TagName::H3     => Some("h3"),
            TagName::H4     => Some("h4"),
            TagName::H5     => Some("h5"),
            TagName::H6     => Some("h6"),
            TagName::A      => Some("a"),
            TagName::Button => Some("button"),
            TagName::Pre    => Some("pre"),
            TagName::Auto   => None,
        }
    }
}