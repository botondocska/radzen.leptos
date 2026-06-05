/// Where the pager bar is rendered relative to the data grid / data list.
///
/// Mirrors `Radzen.PagerPosition` in `Radzen.Blazor/Common.cs`.
///
/// Used by [`RadzenDataGrid`] and [`RadzenDataList`] via their
/// `PagerPosition` prop.  [`RadzenPager`] itself is position-agnostic —
/// it just renders wherever it is placed.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum PagerPosition {
    /// Render the pager below the data. Default.
    #[default]
    Bottom,

    /// Render the pager above the data.
    Top,

    /// Render the pager both above and below the data.
    TopAndBottom,
}
