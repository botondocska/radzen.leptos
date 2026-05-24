/// Stack direction — vertical or horizontal.
///
/// Mirrors `Radzen.Orientation` in `Radzen.Blazor/Common.cs`.
///
/// Controls the `flex-direction` CSS property on [`RadzenStack`]:
/// `Vertical` → `column` (default), `Horizontal` → `row`.
#[derive(Clone, PartialEq, Default, Debug)]
pub enum Orientation {
    /// Children are stacked top-to-bottom (`flex-direction: column`). Default.
    #[default]
    Vertical,

    /// Children are arranged left-to-right (`flex-direction: row`).
    Horizontal,
}