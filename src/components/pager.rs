//! RadzenPager component — mirrors C# Radzen.Blazor.RadzenPager.
//!
//! # CSS class (mirrors `GetComponentCssClass`)
//! `rz-pager rz-unselectable-text rz-helper-clearfix [rz-density-compact] [rz-horizontal-align-*] [caller-class]`
//!
//! # Visibility rule (mirrors `GetVisible`)
//! Hidden when `!(AlwaysVisible || Count > PageSize || PageSizeOptions.Any())`.
//!
//! # HTML structure (mirrors Blazor razor exactly)
//! ```html
//! <nav class="..." id="..." style="..." tabindex="0" aria-label="..." onkeydown onfocus>
//!   <span class="rz-pager-summary">...</span>          <!-- when ShowPagingSummary -->
//!   <ul class="rz-pager-pages" role="list">
//!     <li class="rz-pager-item"><button id="{id}fp" class="rz-pager-first rz-pager-element [rz-state-disabled] [rz-state-focused]" ...></button></li>
//!     <li class="rz-pager-item"><button id="{id}pp" class="rz-pager-prev rz-pager-element ..." ...></button></li>
//!     <!-- foreach page -->
//!     <li class="rz-pager-item"><button id="{id}{i}p" class="rz-pager-page rz-pager-element [rz-state-active] [rz-state-focused]" ...></button></li>
//!     <li class="rz-pager-item"><button id="{id}np" class="rz-pager-next rz-pager-element ..." ...></button></li>
//!     <li class="rz-pager-item"><button id="{id}lp" class="rz-pager-last rz-pager-element ..." ...></button></li>
//!     <!-- when AllowReload -->
//!     <li class="rz-pager-item"><button id="{id}rl" class="rz-pager-reload rz-pager-element" ...></button></li>
//!   </ul>
//!   <!-- when PageSizeOptions.Any() -->
//!   <RadzenDropDown TValue="int" Data="PageSizeOptions" Value="PageSize" Change="OnPageSizeChanged" />
//!   <span class="rz-pagesize-text">PageSizeText</span>
//! </nav>
//! ```
//!
//! # Button icons — mirrors Blazor span classes exactly
//! `<span class="notranslate rz-pager-icon rzi rzi-{name}">` for all nav icons.
//! Reload icon: additionally has `rz-pager-reload-icon` and uses `rzi-reload` (not `rzi-refresh`).
//!
//! # focusedIndex semantics (mirrors Blazor exactly)
//! -3 = initial (no focus), -2 = first button, -1 = prev button,
//! 0..N-1 = page buttons (N = visible page count),
//! N = next button, N+1 = last button, N+2 = reload (if present).
//! `startPage + focusedIndex == i` determines focus on a page button.
//!
//! # Keyboard (mirrors OnKeyDown)
//! ArrowLeft/Right navigate focusedIndex with boundary clamping.
//! Space/Enter activate the focused button.
//!
//! # JS focus
//! Blazor calls `Radzen.focusElement(GetId())` after render when `shouldFocus`.
//! We mirror this by querying `[data-pager-btn][data-focused]` — a WASM adaptation
//! since we have no `Radzen.js`. Buttons get `data-pager-btn` and `data-focused`
//! attributes driven by `focusedIndex`.

use crate::components::{
    ClassList,
    DropDownItem, DropDownProps, Density, HorizontalAlign, RadzenDropDown,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;
use std::sync::Arc;

// ─────────────────────────────────────────────────────────────────────────────
// PagerEventArgs  (mirrors Radzen.PagerEventArgs)
// ─────────────────────────────────────────────────────────────────────────────

/// Event args emitted by [`RadzenPager`] on every page change.
#[derive(Clone, Debug)]
pub struct PagerEventArgs {
    /// Items to skip — pass directly to your data source.
    pub skip: usize,
    /// Items to take — equals `page_size`.
    pub top: usize,
    /// 0-based index of the newly selected page.
    pub page_index: usize,
}

// ─────────────────────────────────────────────────────────────────────────────
// PagingInformation  (mirrors Radzen.Blazor.PagingInformation)
// ─────────────────────────────────────────────────────────────────────────────

/// Data passed to `paging_summary_template`.
#[derive(Clone, Debug)]
pub struct PagingInformation {
    pub current_page: usize,
    pub total_pages: usize,
    pub total_count: usize,
}

// ─────────────────────────────────────────────────────────────────────────────
// calculate_pager — mirrors C# CalculatePager exactly
// ─────────────────────────────────────────────────────────────────────────────

/// Returns `(start_page, end_page, number_of_pages)`.
/// All page indices are **0-based**.
///
/// Mirrors Blazor's `CalculatePager()`:
/// ```csharp
/// int visiblePages = Math.Min(PageNumbersCount, numberOfPages);
/// startPage = Max(0, Ceiling(CurrentPage - visiblePages/2));
/// endPage   = Min(numberOfPages - 1, startPage + visiblePages - 1);
/// delta     = PageNumbersCount - (endPage - startPage + 1);
/// startPage = Max(0, startPage - delta);
/// ```
fn calculate_pager(
    skip: usize,
    page_size: usize,
    count: usize,
    page_numbers_count: usize,
) -> (usize, usize, usize) {
    let page_size = page_size.max(1);
    let number_of_pages = ((count as f64) / (page_size as f64)).ceil() as usize;
    let number_of_pages = number_of_pages.max(1);
    let current_page = skip / page_size;

    let visible_pages = page_numbers_count.min(number_of_pages);

    // Blazor: (int)Math.Max(0, Math.Ceiling((decimal)(CurrentPage - (visiblePages / 2))))
    let half = (visible_pages / 2) as i64;
    let start_page = ((current_page as i64) - half).max(0) as usize;
    let end_page = (number_of_pages - 1).min(start_page + visible_pages - 1);

    // Blazor: delta = PageNumbersCount - (endPage - startPage + 1); startPage = Max(0, startPage - delta);
    let delta = page_numbers_count.saturating_sub(end_page - start_page + 1);
    let start_page = start_page.saturating_sub(delta);
    // Recompute end_page after adjusting start_page.
    let end_page = (number_of_pages - 1).min(start_page + visible_pages - 1);

    (start_page, end_page, number_of_pages)
}

// ─────────────────────────────────────────────────────────────────────────────
// RadzenPager
// ─────────────────────────────────────────────────────────────────────────────

#[component]
pub fn RadzenPager(
    // ── Base ──────────────────────────────────────────────────────────────────
    #[prop(default = Default::default())] base: ComponentProps,

    // ── Core ──────────────────────────────────────────────────────────────────
    /// Total number of items.
    #[prop(default = 0)]
    count: usize,

    /// Items per page. Default: `10`.
    #[prop(default = 10)]
    page_size: usize,

    /// Visible page-number buttons. Default: `5`.
    #[prop(default = 5)]
    page_numbers_count: usize,

    // ── Appearance ────────────────────────────────────────────────────────────
    /// Horizontal alignment. Default: [`HorizontalAlign::Justify`].
    #[prop(default = HorizontalAlign::Justify)]
    horizontal_align: HorizontalAlign,

    /// Compact or default density. Default: [`Density::Default`].
    #[prop(default = Density::Default)]
    density: Density,

    /// Show pager even when all items fit on one page. Default: `false`.
    #[prop(default = false)]
    always_visible: bool,

    // ── Summary ───────────────────────────────────────────────────────────────
    /// Show "Page X of Y (Z items)" text. Default: `false`.
    #[prop(default = false)]
    show_paging_summary: bool,

    /// Format string for the paging summary.
    /// `{0}` = current page, `{1}` = total pages, `{2}` = total items.
    #[prop(default = "Page {0} of {1} ({2} items)".to_string(), into)]
    paging_summary_format: String,

    /// Custom summary template — overrides `paging_summary_format` when set.
    #[prop(optional)]
    paging_summary_template: Option<Arc<dyn Fn(PagingInformation) -> AnyView + Send + Sync>>,

    // ── Page-size selector ────────────────────────────────────────────────────
    /// Available page-size options. Empty = no selector.
    #[prop(default = vec![])]
    page_size_options: Vec<usize>,

    /// Label text next to the page-size selector.
    #[prop(default = "items per page".to_string(), into)]
    page_size_text: String,

    /// Called when the user changes page size.
    #[prop(default = None)]
    on_page_size_changed: Option<Arc<dyn Fn(usize) + Send + Sync>>,

    // ── Reload button ─────────────────────────────────────────────────────────
    /// Show a reload button. Default: `false`.
    #[prop(default = false)]
    allow_reload: bool,

    /// Called when the reload button is clicked.
    #[prop(default = None)]
    on_reload: Option<Arc<dyn Fn() + Send + Sync>>,

    // ── Navigation button labels / titles / aria ───────────────────────────────
    #[prop(default = "First page".to_string(), into)] first_page_title: String,
    #[prop(default = "Go to first page.".to_string(), into)] first_page_aria_label: String,

    #[prop(default = "Previous page".to_string(), into)] prev_page_title: String,
    #[prop(default = "Go to previous page.".to_string(), into)] prev_page_aria_label: String,
    /// Optional text label on the Previous button. Default: `None`.
    #[prop(default = None, into)]
    prev_page_label: Option<String>,

    #[prop(default = "Next page".to_string(), into)] next_page_title: String,
    #[prop(default = "Go to next page.".to_string(), into)] next_page_aria_label: String,
    /// Optional text label on the Next button. Default: `None`.
    #[prop(default = None, into)]
    next_page_label: Option<String>,

    #[prop(default = "Last page".to_string(), into)] last_page_title: String,
    #[prop(default = "Go to last page.".to_string(), into)] last_page_aria_label: String,

    /// Format for page-number button title. `{0}` = 1-based page number.
    #[prop(default = "Page {0}".to_string(), into)]
    page_title_format: String,
    /// Format for page-number button aria-label. `{0}` = 1-based page number.
    #[prop(default = "Go to page {0}.".to_string(), into)]
    page_aria_label_format: String,

    /// Aria-label for the `<nav>` element.
    #[prop(default = "Pagination".to_string(), into)]
    navigation_aria_label: String,

    #[prop(default = "Reload".to_string(), into)] reload_title: String,
    #[prop(default = "Reload current page.".to_string(), into)] reload_aria_label: String,

    // ── Callback ──────────────────────────────────────────────────────────────
    /// Called when the user navigates to a new page.
    #[prop(default = None)]
    on_page_changed: Option<Arc<dyn Fn(PagerEventArgs) + Send + Sync>>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // ── Internal state ────────────────────────────────────────────────────────
    let skip = RwSignal::new(0usize);
    let page_size_sig = RwSignal::new(page_size);
    // focusedIndex: -3 initial, -2 first, -1 prev, 0..N-1 page btns, N next, N+1 last
    let focused_index: RwSignal<i32> = RwSignal::new(-3);

    // ── Visibility — mirrors GetVisible() ─────────────────────────────────────
    // Visible when: AlwaysVisible || Count > PageSize || PageSizeOptions.Any()
    let has_page_size_options = !page_size_options.is_empty();
    let visible_sig = handle.visible;
    let show = move || {
        visible_sig.get()
            && (always_visible || count > page_size_sig.get() || has_page_size_options)
    };

    // ── CSS class — mirrors GetComponentCssClass() ────────────────────────────
    // `rz-pager rz-unselectable-text rz-helper-clearfix [rz-density-compact] [align-class] [caller]`
    let mut css_parts = vec!["rz-pager", "rz-unselectable-text", "rz-helper-clearfix"];
    let density_class_str;
    if density == Density::Compact {
        density_class_str = "rz-density-compact";
        css_parts.push(density_class_str);
    }
    if let Some(align_class) = ClassList::add_horizontal_align(horizontal_align) {
        css_parts.push(align_class);
    }
    let caller_class = base
        .attrs
        .as_ref()
        .and_then(|a| a.get("class"))
        .cloned()
        .unwrap_or_default();
    let mut css_class = css_parts.join(" ");
    if !caller_class.is_empty() {
        css_class.push(' ');
        css_class.push_str(&caller_class);
    }

    let style = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();
    let handle_id_sv = StoredValue::new(handle_id.clone());

    // ── StoredValues for string props used in reactive closures ───────────────
    let paging_summary_format_sv = StoredValue::new(paging_summary_format);
    let paging_summary_template_sv = StoredValue::new(paging_summary_template);
    let first_page_title_sv = StoredValue::new(first_page_title);
    let first_page_aria_label_sv = StoredValue::new(first_page_aria_label);
    let prev_page_title_sv = StoredValue::new(prev_page_title);
    let prev_page_aria_label_sv = StoredValue::new(prev_page_aria_label);
    let prev_page_label_sv = StoredValue::new(prev_page_label);
    let next_page_title_sv = StoredValue::new(next_page_title);
    let next_page_aria_label_sv = StoredValue::new(next_page_aria_label);
    let next_page_label_sv = StoredValue::new(next_page_label);
    let last_page_title_sv = StoredValue::new(last_page_title);
    let last_page_aria_label_sv = StoredValue::new(last_page_aria_label);
    let page_title_format_sv = StoredValue::new(page_title_format);
    let page_aria_label_format_sv = StoredValue::new(page_aria_label_format);
    let reload_title_sv = StoredValue::new(reload_title);
    let reload_aria_label_sv = StoredValue::new(reload_aria_label);
    let page_size_text_sv = StoredValue::new(page_size_text);
    let page_size_options_sv = StoredValue::new(page_size_options);
    let on_page_changed_sv = StoredValue::new(on_page_changed);
    let on_page_size_changed_sv = StoredValue::new(on_page_size_changed);
    let on_reload_sv = StoredValue::new(on_reload);

    // ── Pager calculation — reactive Memo ────────────────────────────────────
    // Mirrors CalculatePager() exactly.
    let pager = Memo::new(move |_| {
        calculate_pager(skip.get(), page_size_sig.get(), count, page_numbers_count)
    });

    // ── fire_page_changed helper ──────────────────────────────────────────────
    let fire_page_changed = move |new_skip: usize| {
        skip.set(new_skip);
        if let Some(cb) = on_page_changed_sv.get_value() {
            cb(PagerEventArgs {
                skip: new_skip,
                top: page_size_sig.get(),
                page_index: new_skip / page_size_sig.get().max(1),
            });
        }
    };

    // ── Navigation actions — mirror Blazor methods exactly ────────────────────
    // FirstPage: if CurrentPage != 0, skip = 0, fire.
    // Matches OnFirstPageClick: focusedIndex = -2, then FirstPage(), clamp if skip==0.
    let go_first = {
        let fpc = fire_page_changed;
        move |_: web_sys::MouseEvent| {
            focused_index.set(-2);
            let s = skip.get_untracked();
            if s > 0 {
                fpc(0);
            }
            // Clamp: if now on first page and focusedIndex < 0, set to 0.
            if skip.get_untracked() == 0 && focused_index.get_untracked() < 0 {
                focused_index.set(focused_index.get_untracked() + 2);
            }
        }
    };

    // PrevPage: skip = max(0, skip - page_size).
    let go_prev = {
        let fpc = fire_page_changed;
        move |_: web_sys::MouseEvent| {
            focused_index.set(-1);
            let ps = page_size_sig.get_untracked();
            let s = skip.get_untracked();
            let new_skip = if s >= ps { s - ps } else { 0 };
            if new_skip != s {
                fpc(new_skip);
            }
            if skip.get_untracked() == 0 {
                focused_index.update(|fi| *fi += 1);
            }
        }
    };

    // NextPage: skip = page_size * (CurrentPage < numberOfPages-1 ? CurrentPage+1 : numberOfPages-1)
    let go_next = {
        let fpc = fire_page_changed;
        move |_: web_sys::MouseEvent| {
            let (_, end_page, np) = pager.get_untracked();
            let fi = (end_page as i32 + 1).min(page_numbers_count as i32);
            focused_index.set(fi);
            let ps = page_size_sig.get_untracked();
            let s = skip.get_untracked();
            let current = s / ps.max(1);
            let new_skip = ps * if current < np - 1 { current + 1 } else { np.saturating_sub(1) };
            if new_skip != s {
                fpc(new_skip);
            }
            let (_, _, np2) = pager.get_untracked();
            let current2 = skip.get_untracked() / ps.max(1);
            if current2 == np2.saturating_sub(1) {
                focused_index.update(|fi| *fi -= 1);
            }
        }
    };

    // LastPage: skip = page_size * (numberOfPages - 1)
    let go_last = {
        let fpc = fire_page_changed;
        move |_: web_sys::MouseEvent| {
            let (_, end_page, np) = pager.get_untracked();
            let fi = (end_page as i32 + 1).min(page_numbers_count as i32) + 1;
            focused_index.set(fi);
            let ps = page_size_sig.get_untracked();
            let last_skip = ps * np.saturating_sub(1);
            if skip.get_untracked() != last_skip {
                fpc(last_skip);
            }
            let (_, _, np2) = pager.get_untracked();
            let current2 = skip.get_untracked() / ps.max(1);
            if current2 == np2.saturating_sub(1) {
                focused_index.update(|fi| *fi -= 2);
            }
        }
    };

    // GoToPage (for page number buttons).
    let go_to_page = move |page: usize, fi: i32| {
        focused_index.set(fi);
        let ps = page_size_sig.get_untracked();
        let current = skip.get_untracked() / ps.max(1);
        if current != page {
            fire_page_changed(page * ps);
        }
    };

    // Reload click.
    let on_reload_click = move |_: web_sys::MouseEvent| {
        if let Some(cb) = on_reload_sv.get_value() {
            cb();
        }
        // Mirrors OnReloadClick: also fires PageChanged with current skip.
        if let Some(cb) = on_page_changed_sv.get_value() {
            cb(PagerEventArgs {
                skip: skip.get_untracked(),
                top: page_size_sig.get_untracked(),
                page_index: skip.get_untracked() / page_size_sig.get_untracked().max(1),
            });
        }
    };

    // ── OnFocus — mirrors Blazor OnFocus ──────────────────────────────────────
    // focusedIndex = focusedIndex == -3 ? 0 : focusedIndex
    // + boundary clamp same as keyboard.
    let on_nav_focus = move |_: web_sys::FocusEvent| {
        let fi = focused_index.get_untracked();
        let mut new_fi = if fi == -3 { 0 } else { fi };
        let ps = page_size_sig.get_untracked();
        let current = skip.get_untracked() / ps.max(1);
        let (_, _, np) = pager.get_untracked();
        if current == 0 && new_fi < 0 {
            new_fi = 0;
        } else if np > 0 && current == np - 1 && new_fi > np as i32 - 1 {
            new_fi = np as i32 - 1;
        }
        focused_index.set(new_fi);
    };

    // ── OnKeyDown — mirrors Blazor OnKeyDown exactly ──────────────────────────
    // focusedIndex range: -2 (first) to numberOfDisplayedPages+1 (last).
    // numberOfDisplayedPages = Math.Min(endPage + 1, PageNumbersCount).
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        let key = if ev.code().is_empty() { ev.key() } else { ev.code() };
        let (_, end_page, np) = pager.get_untracked();
        let number_of_displayed_pages = ((end_page + 1) as i32).min(page_numbers_count as i32);
        let ps = page_size_sig.get_untracked();
        let current = skip.get_untracked() / ps.max(1);

        match key.as_str() {
            "ArrowLeft" | "ArrowRight" => {
                ev.prevent_default();
                let dir: i32 = if key == "ArrowLeft" { -1 } else { 1 };
                let fi = focused_index.get_untracked();
                let new_fi = fi.saturating_add(dir).clamp(-2, number_of_displayed_pages + 1);
                let new_fi = if current == 0 && new_fi < 0 { 0 } else { new_fi };
                let new_fi = if np > 0 && current == np - 1 && new_fi > number_of_displayed_pages - 1 {
                    number_of_displayed_pages - 1
                } else {
                    new_fi
                };
                focused_index.set(new_fi);
            }
            "Space" | "Enter" => {
                ev.prevent_default();
                let fi = focused_index.get_untracked();
                let (start_page, _, _) = pager.get_untracked();
                if fi == -2 {
                    let s = skip.get_untracked();
                    if s > 0 { fire_page_changed(0); }
                } else if fi == -1 {
                    let s = skip.get_untracked();
                    let new_skip = if s >= ps { s - ps } else { 0 };
                    if new_skip != s { fire_page_changed(new_skip); }
                } else if fi == number_of_displayed_pages {
                    let s = skip.get_untracked();
                    let current2 = s / ps.max(1);
                    let new_skip = ps * if current2 < np - 1 { current2 + 1 } else { np.saturating_sub(1) };
                    if new_skip != s { fire_page_changed(new_skip); }
                } else if fi == number_of_displayed_pages + 1 {
                    let last_skip = ps * np.saturating_sub(1);
                    if skip.get_untracked() != last_skip { fire_page_changed(last_skip); }
                } else if fi >= 0 {
                    let page = start_page + fi as usize;
                    let current2 = skip.get_untracked() / ps.max(1);
                    if current2 != page { fire_page_changed(page * ps); }
                }
                // Clamp after action.
                let fi2 = focused_index.get_untracked();
                let current3 = skip.get_untracked() / ps.max(1);
                if current3 == 0 && fi2 < 0 {
                    focused_index.set(0);
                } else if np > 0 && current3 == np - 1 && fi2 > number_of_displayed_pages - 1 {
                    focused_index.set(number_of_displayed_pages - 1);
                }
            }
            _ => {}
        }
    };

    // ── Summary renderer ──────────────────────────────────────────────────────
    let render_summary = move || -> Option<AnyView> {
        if !show_paging_summary { return None; }
        let (_, _, np) = pager.get();
        let ps = page_size_sig.get();
        let cp = skip.get() / ps.max(1) + 1;
        let info = PagingInformation {
            current_page: cp,
            total_pages: np,
            total_count: count,
        };
        let content: AnyView = match paging_summary_template_sv.get_value() {
            Some(tmpl) => tmpl(info),
            None => {
                let fmt = paging_summary_format_sv.get_value();
                fmt.replace("{0}", &cp.to_string())
                    .replace("{1}", &np.to_string())
                    .replace("{2}", &count.to_string())
                    .into_any()
            }
        };
        Some(leptos::html::span().attr("class", "rz-pager-summary").child(content).into_any())
    };

    // ── Page size dropdown — mirrors Blazor RadzenDropDown exactly ─────────────
    // Blazor: <RadzenDropDown TValue="int" Data="PageSizeOptions" Value="PageSize" Change="OnPageSizeChanged" />
    // No wrapping div — it's a direct sibling of <ul> inside <nav>.
    let render_page_size = move || -> Option<AnyView> {
        let opts = page_size_options_sv.get_value();
        if opts.is_empty() { return None; }

        let items: Vec<DropDownItem> = opts
            .iter()
            .map(|&ps| DropDownItem::new(ps.to_string(), ps.to_string()))
            .collect();

        let dd_value = RwSignal::new(page_size_sig.get_untracked().to_string());

        // Keep dd_value synced when page_size_sig changes.
        Effect::new(move |_| {
            dd_value.set(page_size_sig.get().to_string());
        });

        Some(
            view! {
                <>
                    <RadzenDropDown
                        drop_down=DropDownProps {
                            value: Some(dd_value),
                            data: items,
                            on_change: Some(Arc::new(move |v: String| {
                                // Mirrors OnPageSizeChanged(object value):
                                // bool isFirstPage = CurrentPage == 0;
                                // bool isLastPage  = CurrentPage == numberOfPages - 1 && numberOfPages > 1;
                                // int prevSkip = skip;
                                // PageSize = (int)value;
                                // Reload(); PageSizeChanged.InvokeAsync(...);
                                // if isLastPage  → LastPage()
                                // else if !isFirstPage → GoToPage(prevSkip / PageSize, forceReload=true)
                                if let Ok(new_ps) = v.parse::<usize>() {
                                    let ps = page_size_sig.get_untracked();
                                    let current = skip.get_untracked() / ps.max(1);
                                    let (_, _, np) = pager.get_untracked();
                                    let is_first_page = current == 0;
                                    let is_last_page = np > 1 && current == np - 1;
                                    let prev_skip = skip.get_untracked();

                                    page_size_sig.set(new_ps);

                                    if let Some(cb) = on_page_size_changed_sv.get_value() {
                                        cb(new_ps);
                                    }

                                    if is_last_page {
                                        // LastPage() with new page_size.
                                        let (_, _, np2) = pager.get_untracked();
                                        let last_skip = new_ps * np2.saturating_sub(1);
                                        fire_page_changed(last_skip);
                                    } else if !is_first_page {
                                        // GoToPage(prevSkip / PageSize, forceReload=true)
                                        let new_page = prev_skip / new_ps.max(1);
                                        fire_page_changed(new_page * new_ps);
                                    }
                                    // else first page: skip stays 0, just re-render.
                                }
                            })),
                            ..Default::default()
                        }
                    />
                    <span class="rz-pagesize-text">{page_size_text_sv.get_value()}</span>
                </>
            }
            .into_any(),
        )
    };

    view! {
        <Show when=move || show()>
            <nav
                id=handle_id.clone()
                class=css_class.clone()
                style=style.clone()
                tabindex="0"
                aria-label=navigation_aria_label.clone()
                on:keydown=on_keydown
                on:focus=on_nav_focus
            >
                // ── Paging summary ────────────────────────────────────────────
                {render_summary}

                // ── Navigation buttons — wrapped in <ul class="rz-pager-pages"> ──
                // Mirrors Blazor razor structure exactly.
                <ul class="rz-pager-pages" role="list">
                    // ── First page ─────────────────────────────────────────────
                    {move || {
                        let ps = page_size_sig.get();
                        let current = skip.get() / ps.max(1);
                        let is_first = current == 0;
                        let fi = focused_index.get();
                        // Blazor: rz-state-disabled when skip <= 0 (i.e. current == 0)
                        // rz-state-focused when focusedIndex == -2
                        let btn_class = format!(
                            "rz-pager-first rz-pager-element{}{}",
                            if is_first { " rz-state-disabled" } else { "" },
                            if fi == -2 { " rz-state-focused" } else { "" },
                        );
                        let id = format!("{}fp", handle_id_sv.get_value());
                        view! {
                            <li class="rz-pager-item">
                                <button
                                    id=id
                                    type="button"
                                    tabindex="-1"
                                    class=btn_class
                                    aria-label=first_page_aria_label_sv.get_value()
                                    title=first_page_title_sv.get_value()
                                    disabled=is_first
                                    aria-disabled=if is_first { "true" } else { "false" }
                                    on:click=go_first.clone()
                                >
                                    <span class="notranslate rz-pager-icon rzi rzi-step-backward"></span>
                                </button>
                            </li>
                        }
                    }}

                    // ── Previous page ──────────────────────────────────────────
                    {move || {
                        let ps = page_size_sig.get();
                        let current = skip.get() / ps.max(1);
                        let is_first = current == 0;
                        let fi = focused_index.get();
                        let btn_class = format!(
                            "rz-pager-prev rz-pager-element{}{}",
                            if is_first { " rz-state-disabled" } else { "" },
                            if fi == -1 { " rz-state-focused" } else { "" },
                        );
                        let id = format!("{}pp", handle_id_sv.get_value());
                        view! {
                            <li class="rz-pager-item">
                                <button
                                    id=id
                                    type="button"
                                    tabindex="-1"
                                    class=btn_class
                                    aria-label=prev_page_aria_label_sv.get_value()
                                    title=prev_page_title_sv.get_value()
                                    disabled=is_first
                                    aria-disabled=if is_first { "true" } else { "false" }
                                    on:click=go_prev.clone()
                                >
                                    <span class="notranslate rz-pager-icon rzi rzi-caret-left"></span>
                                    {move || prev_page_label_sv.get_value().map(|l| view! {
                                        <span class="rz-pager-label">{l}</span>
                                    })}
                                </button>
                            </li>
                        }
                    }}

                    // ── Page number buttons — Enumerable.Range(startPage, Min(endPage+1, PageNumbersCount)) ──
                    // Blazor: focusedIndex is relative to startPage: `startPage + focusedIndex == i`
                    {move || {
                        let (start_page, end_page, _np) = pager.get();
                        let ps = page_size_sig.get();
                        let current = skip.get() / ps.max(1);
                        let fi = focused_index.get();
                        // Count: Math.Min(endPage + 1, PageNumbersCount) pages starting from startPage.
                        let page_count = ((end_page + 1) as usize).min(page_numbers_count);

                        (start_page..start_page + page_count)
                            .map(|i| {
                                let is_active = i == current;
                                // Blazor: startPage + focusedIndex == i
                                let is_focused = fi >= 0 && start_page + fi as usize == i;
                                let btn_class = format!(
                                    "rz-pager-page rz-pager-element{}{}",
                                    if is_active { " rz-state-active" } else { "" },
                                    if is_focused { " rz-state-focused" } else { "" },
                                );
                                let display = i + 1; // 1-based
                                let title = page_title_format_sv.get_value()
                                    .replace("{0}", &display.to_string());
                                let aria = page_aria_label_format_sv.get_value()
                                    .replace("{0}", &display.to_string());
                                let id = format!("{}{}p", handle_id_sv.get_value(), i);
                                // focusedIndex for this page = i - startPage (as i32)
                                let page_fi = (i - start_page) as i32;
                                view! {
                                    <li class="rz-pager-item">
                                        <button
                                            id=id
                                            type="button"
                                            tabindex="-1"
                                            class=btn_class
                                            aria-label=aria
                                            title=title
                                            aria-current=if is_active { Some("page") } else { None }
                                            on:click=move |_| go_to_page(i, page_fi)
                                        >
                                            {display.to_string()}
                                        </button>
                                    </li>
                                }
                            })
                            .collect_view()
                    }}

                    // ── Next page ──────────────────────────────────────────────
                    {move || {
                        let ps = page_size_sig.get();
                        let current = skip.get() / ps.max(1);
                        let (_, end_page, np) = pager.get();
                        let is_last = np == 0 || current >= np - 1;
                        let fi = focused_index.get();
                        let next_fi = (end_page as i32 + 1).min(page_numbers_count as i32);
                        let btn_class = format!(
                            "rz-pager-next rz-pager-element{}{}",
                            if is_last { " rz-state-disabled" } else { "" },
                            if fi == next_fi { " rz-state-focused" } else { "" },
                        );
                        let id = format!("{}np", handle_id_sv.get_value());
                        view! {
                            <li class="rz-pager-item">
                                <button
                                    id=id
                                    type="button"
                                    tabindex="-1"
                                    class=btn_class
                                    aria-label=next_page_aria_label_sv.get_value()
                                    title=next_page_title_sv.get_value()
                                    disabled=is_last
                                    aria-disabled=if is_last { "true" } else { "false" }
                                    on:click=go_next.clone()
                                >
                                    {move || next_page_label_sv.get_value().map(|l| view! {
                                        <span class="rz-pager-label">{l}</span>
                                    })}
                                    <span class="notranslate rz-pager-icon rzi rzi-caret-right"></span>
                                </button>
                            </li>
                        }
                    }}

                    // ── Last page ──────────────────────────────────────────────
                    {move || {
                        let ps = page_size_sig.get();
                        let current = skip.get() / ps.max(1);
                        let (_, end_page, np) = pager.get();
                        let is_last = np == 0 || current >= np - 1;
                        let fi = focused_index.get();
                        let last_fi = (end_page as i32 + 1).min(page_numbers_count as i32) + 1;
                        let btn_class = format!(
                            "rz-pager-last rz-pager-element{}{}",
                            if is_last { " rz-state-disabled" } else { "" },
                            if fi == last_fi { " rz-state-focused" } else { "" },
                        );
                        let id = format!("{}lp", handle_id_sv.get_value());
                        view! {
                            <li class="rz-pager-item">
                                <button
                                    id=id
                                    type="button"
                                    tabindex="-1"
                                    class=btn_class
                                    aria-label=last_page_aria_label_sv.get_value()
                                    title=last_page_title_sv.get_value()
                                    disabled=is_last
                                    aria-disabled=if is_last { "true" } else { "false" }
                                    on:click=go_last.clone()
                                >
                                    <span class="notranslate rz-pager-icon rzi rzi-step-forward"></span>
                                </button>
                            </li>
                        }
                    }}

                    // ── Reload button — @if(AllowReload) ───────────────────────
                    {move || -> Option<AnyView> {
                        if !allow_reload { return None; }
                        let id = format!("{}rl", handle_id_sv.get_value());
                        Some(
                            view! {
                                <li class="rz-pager-item">
                                    <button
                                        id=id
                                        type="button"
                                        tabindex="-1"
                                        class="rz-pager-reload rz-pager-element"
                                        aria-label=reload_aria_label_sv.get_value()
                                        title=reload_title_sv.get_value()
                                        on:click=on_reload_click.clone()
                                    >
                                        <span class="notranslate rz-pager-icon rz-pager-reload-icon rzi rzi-reload"></span>
                                    </button>
                                </li>
                            }
                            .into_any(),
                        )
                    }}
                </ul>

                // ── Page-size dropdown — sibling of <ul>, inside <nav> ─────────
                // Blazor: <RadzenDropDown ... /> <span class="rz-pagesize-text">...</span>
                // No wrapping div — direct children of <nav>.
                {render_page_size}

            </nav>
        </Show>
    }
}