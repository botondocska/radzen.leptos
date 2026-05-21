/// Visual appearance variant of a button (or badge).
///
/// Controls how the color style is expressed — as a solid fill,
/// a subtle fill, a border outline, or text only.
#[derive(Clone, PartialEq, Default, Debug)]
pub enum Variant {
    /// Solid background in the style color, white text, no border. Default.
    #[default]
    Filled,

    /// Muted background (`lighter` shade ~16-20% opacity), colored text, no border.
    Flat,

    /// Transparent background, colored border (1px solid), colored text.
    Outlined,

    /// No background, no border — colored text only. Minimal chrome.
    Text,
}
