//! RadzenPager component — mirrors C# Radzen.Blazor.RadzenPager.
//!
//! # CSS class (mirrors `GetComponentCssClass`)
//! `rz-pager rz-unselectable-text rz-helper-clearfix [rz-density-compact] [caller-class]`
//!
//! # Visibility rule (mirrors `GetVisible`)
//! Hidden when `Count <= PageSize && !AlwaysVisible`.
//!
//! # Page calculation (mirrors `CalculatePager`)
//! Centers the current page among `PageNumbersCount` visible page buttons,
//! clamped at both ends.
//!
//! # Keyboard navigation (mirrors `OnKeyDown`)
//! ArrowLeft / ArrowRight move focus between buttons.
//! Space / Enter activate the focused button.
//!
//! # Page-size selector
//! When `page_size_options` is non-empty, a `<select>` is rendered after the
//! navigation buttons — mirrors Blazor's page-size `<select>` inside the pager.
//!
//! # Summary
//! When `show_paging_summary=true`, "Page X of Y (Z items)" text is rendered
//! before the navigation buttons — mirrors Blazor's `rz-pager-summary` span.
//! A custom `paging_summary_template` overrides the format string.
//!
//! # AllowReload
//! When `allow_reload=true`, a reload button (`rz-pager-reload`) is shown after
//! the last-page button — mirrors Blazor's `PageReload` / `AllowReload`.

use crate::components::{
    Density, HorizontalAlign,
    base_component::{ComponentProps, use_radzen_base},
};
use leptos::prelude::*;
use std::sync::Arc;

// ─────────────────────────────────────────────────────────────────────────────
// PagerEventArgs  (mirrors Radzen.PagerEventArgs)
// ─────────────────────────────────────────────────────────────────────────────

/// Event args emitted by [`RadzenPager`] on every page change.
///
/// Mirrors `Radzen.PagerEventArgs`:
/// - `skip`  → number of items to skip (0-based offset)
/// - `top`   → page size (number of items to take)
/// - `page_index` → 0-based page index
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
///
/// Mirrors `Radzen.Blazor.PagingInformation`:
/// - `current_page` → 1-based page number for display
/// - `total_pages`  → total number of pages
/// - `total_count`  → total number of items
#[derive(Clone, Debug)]
pub struct PagingInformation {
    pub current_page: usize,
    pub total_pages: usize,
    pub total_count: usize,
}

// ─────────────────────────────────────────────────────────────────────────────
// Page calculation helper  (mirrors CalculatePager)
// ─────────────────────────────────────────────────────────────────────────────

/// Returns `(start_page, end_page, number_of_pages, number_of_page_links)`.
/// All values are **0-based** page indices.
///
/// Mirrors Blazor's `CalculatePager()`:
/// - `skip`             → current skip offset
/// - `page_size`        → items per page
/// - `count`            → total items
/// - `page_numbers_count` → max visible page buttons
fn calculate_pager(
    skip: usize,
    page_size: usize,
    count: usize,
    page_numbers_count: usize,
) -> (usize, usize, usize, usize) {
    if page_size == 0 || count == 0 {
        return (0, 0, 0, 0);
    }
    let number_of_pages = count.div_ceil(page_size);
    let current_page = skip / page_size;

    let visible = page_numbers_count.min(number_of_pages);
    let number_of_page_links = visible;

    // Centre the window on current_page (saturating arithmetic → usize safe).
    let half = visible / 2;
    let mut start_page = current_page.saturating_sub(half);
    let mut end_page = start_page + visible;

    // Clamp at the right boundary.
    if end_page > number_of_pages {
        end_page = number_of_pages;
        start_page = end_page.saturating_sub(visible);
    }

    (start_page, end_page, number_of_pages, number_of_page_links)
}

// ─────────────────────────────────────────────────────────────────────────────
// RadzenPager
// ─────────────────────────────────────────────────────────────────────────────

/// RadzenPager component.
///
/// Standalone pagination bar.  Used directly or embedded inside
/// [`RadzenDataGrid`] / [`RadzenDataList`].
///
/// # Minimal usage
/// ```rust,ignore
/// let skip = RwSignal::new(0usize);
/// <RadzenPager
///     count=250
///     page_size=20
///     on_page_changed=Some(Arc::new(move |args: PagerEventArgs| {
///         skip.set(args.skip);
///     }))
/// />
/// ```
#[component]
pub fn RadzenPager(
    // ── Base ──────────────────────────────────────────────────────────────────
    #[prop(default = Default::default())] base: ComponentProps,

    // ── Core ──────────────────────────────────────────────────────────────────
    /// Total number of items across all pages.
    #[prop(default = 0)]
    count: usize,

    /// Number of items per page. Default: `10`.
    #[prop(default = 10)]
    page_size: usize,

    /// Number of page-number buttons visible at once. Default: `5`.
    #[prop(default = 5)]
    page_numbers_count: usize,

    // ── Appearance ────────────────────────────────────────────────────────────
    /// Horizontal alignment of pager buttons. Default: [`HorizontalAlign::Justify`].
    #[prop(default = HorizontalAlign::Justify)]
    horizontal_align: HorizontalAlign,

    /// Compact or default density. Default: [`Density::Default`].
    #[prop(default = Density::Default)]
    density: Density,

    /// Show pager even when all items fit on one page. Default: `false`.
    #[prop(default = false)]
    always_visible: bool,

    // ── Summary ───────────────────────────────────────────────────────────────
    /// Show "Page X of Y (Z items)" summary text. Default: `false`.
    #[prop(default = false)]
    show_paging_summary: bool,

    /// Format string for the paging summary.
    /// Placeholders: `{0}` = current page, `{1}` = total pages, `{2}` = total items.
    /// Default: `"Page {0} of {1} ({2} items)"`.
    #[prop(default = "Page {0} of {1} ({2} items)".to_string(), into)]
    paging_summary_format: String,

    /// Custom summary template — overrides `paging_summary_format` when set.
    #[prop(optional)]
    paging_summary_template: Option<Arc<dyn Fn(PagingInformation) -> AnyView + Send + Sync>>,

    // ── Page-size selector ────────────────────────────────────────────────────
    /// Available page-size options shown in the selector dropdown.
    /// When empty, no selector is shown.
    #[prop(default = vec![])]
    page_size_options: Vec<usize>,

    /// Label text next to the page-size selector. Default: `"items per page"`.
    #[prop(default = "items per page".to_string(), into)]
    page_size_text: String,

    /// Called when the user changes the page size.
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
    /// Optional text label on the Previous button (e.g. "Prev"). Default: empty.
    #[prop(default = None, into)]
    prev_page_label: Option<String>,

    #[prop(default = "Next page".to_string(), into)] next_page_title: String,
    #[prop(default = "Go to next page.".to_string(), into)] next_page_aria_label: String,
    /// Optional text label on the Next button (e.g. "Next"). Default: empty.
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

    /// Aria-label for the `<nav>` element wrapping all buttons.
    #[prop(default = "Page navigation".to_string(), into)]
    navigation_aria_label: String,

    #[prop(default = "Reload".to_string(), into)] reload_title: String,
    #[prop(default = "Reload data.".to_string(), into)] reload_aria_label: String,

    // ── Callback ──────────────────────────────────────────────────────────────
    /// Called when the user navigates to a new page.
    /// Receives [`PagerEventArgs`] with `skip`, `top`, and `page_index`.
    #[prop(default = None)]
    on_page_changed: Option<Arc<dyn Fn(PagerEventArgs) + Send + Sync>>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // ── skip signal — internal navigation state ───────────────────────────────
    let skip = RwSignal::new(0usize);
    let page_size_sig = RwSignal::new(page_size);

    // ── Visibility — mirrors GetVisible() ────────────────────────────────────
    // Hidden when count <= page_size AND not always_visible.
    let visible_sig = handle.visible;
    let show = move || visible_sig.get() && (always_visible || count > page_size_sig.get());

    // ── Pager calculation — reactive Memo ────────────────────────────────────
    let pager = Memo::new(move |_| {
        calculate_pager(skip.get(), page_size_sig.get(), count, page_numbers_count)
    });

    // ── CSS class — mirrors GetComponentCssClass() ────────────────────────────
    // `rz-pager rz-unselectable-text rz-helper-clearfix [rz-density-compact] [caller]`
    let density_class = density.css_class().unwrap_or("").to_string();
    let caller_class = base
        .attrs
        .as_ref()
        .and_then(|a| a.get("class"))
        .cloned()
        .unwrap_or_default();
    let css_class = {
        let mut parts = vec!["rz-pager", "rz-unselectable-text", "rz-helper-clearfix"];
        if !density_class.is_empty() {
            parts.push(density_class.as_str());
        }
        let mut s = parts.join(" ");
        if !caller_class.is_empty() {
            s.push(' ');
            s.push_str(&caller_class);
        }
        s
    };

    let style = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();

    // ── Keyboard focus tracking ───────────────────────────────────────────────
    // Mirrors Blazor's `focusedIndex` field + `OnKeyDown`.
    let focused_index: RwSignal<i32> = RwSignal::new(-1);

    // ── Navigation helpers ────────────────────────────────────────────────────
    let on_page_changed_sv = StoredValue::new(on_page_changed);

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

    let go_first = {
        let fpc = fire_page_changed;
        move |_: web_sys::MouseEvent| {
            if skip.get_untracked() > 0 {
                fpc(0);
            }
        }
    };

    let go_prev = {
        let fpc = fire_page_changed;
        move |_: web_sys::MouseEvent| {
            let ps = page_size_sig.get_untracked();
            let s = skip.get_untracked();
            if s >= ps {
                fpc(s - ps);
            }
        }
    };

    let go_next = {
        let fpc = fire_page_changed;
        move |_: web_sys::MouseEvent| {
            let ps = page_size_sig.get_untracked();
            let s = skip.get_untracked();
            let (_, _, np, _) = pager.get_untracked();
            if s / ps.max(1) + 1 < np {
                fpc(s + ps);
            }
        }
    };

    let go_last = {
        let fpc = fire_page_changed;
        move |_: web_sys::MouseEvent| {
            let ps = page_size_sig.get_untracked();
            let (_, _, np, _) = pager.get_untracked();
            if np > 0 {
                let last_skip = (np - 1) * ps;
                if skip.get_untracked() != last_skip {
                    fpc(last_skip);
                }
            }
        }
    };

    // ── Page-size change ──────────────────────────────────────────────────────
    let on_page_size_changed_sv = StoredValue::new(on_page_size_changed);
    let on_ps_change = move |ev: web_sys::Event| {
        use web_sys::wasm_bindgen::JsCast;
        if let Some(sel) = ev
            .target()
            .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
        {
            if let Ok(new_ps) = sel.value().parse::<usize>() {
                // Maintain position: keep current item visible.
                let current_item = skip.get_untracked();
                let new_skip = (current_item / new_ps.max(1)) * new_ps;
                page_size_sig.set(new_ps);
                skip.set(new_skip);
                fire_page_changed(new_skip);
                if let Some(cb) = on_page_size_changed_sv.get_value() {
                    cb(new_ps);
                }
            }
        }
    };

    // ── Reload ────────────────────────────────────────────────────────────────
    let on_reload_sv = StoredValue::new(on_reload);
    let on_reload_click = move |_: web_sys::MouseEvent| {
        if let Some(cb) = on_reload_sv.get_value() {
            cb();
        }
    };

    // ── Keyboard handler — mirrors OnKeyDown ──────────────────────────────────
    // Collects all pager button elements by querying the nav's own children.
    // ArrowLeft → move focus left, ArrowRight → right, Space/Enter → click.
    let nav_ref = NodeRef::<leptos::html::Nav>::new();
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        let key = if ev.code().is_empty() {
            ev.key()
        } else {
            ev.code()
        };

        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::wasm_bindgen::JsCast;
            if let Some(nav) = nav_ref.get() {
                let btns = nav
                    .query_selector_all("button[data-pager-btn], a[data-pager-btn]")
                    .ok();
                let count_btns = btns.as_ref().map(|nl| nl.length() as i32).unwrap_or(0);

                match key.as_str() {
                    "ArrowLeft" => {
                        ev.prevent_default();
                        let next = (focused_index.get_untracked() - 1).max(0);
                        focused_index.set(next);
                        if let Some(ref nl) = btns {
                            if let Some(el) = nl.item(next as u32) {
                                if let Ok(btn) = el.dyn_into::<web_sys::HtmlElement>() {
                                    btn.focus().ok();
                                }
                            }
                        }
                    }
                    "ArrowRight" => {
                        ev.prevent_default();
                        let next = (focused_index.get_untracked() + 1).min(count_btns - 1);
                        focused_index.set(next);
                        if let Some(ref nl) = btns {
                            if let Some(el) = nl.item(next as u32) {
                                if let Ok(btn) = el.dyn_into::<web_sys::HtmlElement>() {
                                    btn.focus().ok();
                                }
                            }
                        }
                    }
                    "Space" | "Enter" => {
                        ev.prevent_default();
                        let idx = focused_index.get_untracked();
                        if idx >= 0 {
                            if let Some(ref nl) = btns {
                                if let Some(el) = nl.item(idx as u32) {
                                    if let Ok(btn) = el.dyn_into::<web_sys::HtmlElement>() {
                                        btn.click();
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        let _ = key; // silence unused in non-wasm
    };

    // ── Summary format helper ─────────────────────────────────────────────────
    let paging_summary_format_sv = StoredValue::new(paging_summary_format);
    let paging_summary_template_sv = StoredValue::new(paging_summary_template);

    let render_summary = move || -> Option<AnyView> {
        if !show_paging_summary {
            return None;
        }
        let (_, _, np, _) = pager.get();
        let cp = skip.get() / page_size_sig.get().max(1) + 1;
        let info = PagingInformation {
            current_page: cp,
            total_pages: np,
            total_count: count,
        };
        let content: AnyView = match paging_summary_template_sv.get_value() {
            Some(tmpl) => tmpl(info),
            None => {
                let fmt = paging_summary_format_sv.get_value();
                let text = fmt
                    .replace("{0}", &cp.to_string())
                    .replace("{1}", &np.to_string())
                    .replace("{2}", &count.to_string());
                text.into_any()
            }
        };
        Some(
            leptos::html::span()
                .attr("class", "rz-pager-summary")
                .child(content)
                .into_any(),
        )
    };

    // ── String helpers stored ─────────────────────────────────────────────────
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

    // ── justify-content inline style ──────────────────────────────────────────
    let justify = horizontal_align.css_justify();

    view! {
        <Show when=move || show()>
            <div
                id=handle_id.clone()
                class=css_class.clone()
                style=style.clone()
            >
                // ── Summary ───────────────────────────────────────────────────
                {render_summary}

                // ── Nav — all interactive buttons ─────────────────────────────
                <nav
                    node_ref=nav_ref
                    aria-label=navigation_aria_label.clone()
                    style=format!("display:flex;align-items:center;justify-content:{justify};gap:0.125rem;")
                    on:keydown=on_keydown
                >
                    // First page
                    {move || {
                        let (sp, _, np, _) = pager.get();
                        let is_first = sp == 0 || np == 0;
                        let disabled = is_first;
                        let btn_class = if disabled {
                            "rz-pager-element rz-pager-first rz-state-disabled"
                        } else {
                            "rz-pager-element rz-pager-first"
                        };
                        view! {
                            <button
                                type="button"
                                class=btn_class
                                title=first_page_title_sv.get_value()
                                aria-label=first_page_aria_label_sv.get_value()
                                disabled=disabled
                                data-pager-btn="true"
                                on:click=go_first.clone()
                                on:focus=move |_| focused_index.set(0)
                            >
                                <span class="notranslate rzi rzi-step-backward"></span>
                            </button>
                        }
                    }}

                    // Previous page
                    {move || {
                        let (_, _, np, _) = pager.get();
                        let current = skip.get() / page_size_sig.get().max(1);
                        let disabled = current == 0 || np == 0;
                        let btn_class = if disabled {
                            "rz-pager-element rz-pager-prev rz-state-disabled"
                        } else {
                            "rz-pager-element rz-pager-prev"
                        };
                        view! {
                            <button
                                type="button"
                                class=btn_class
                                title=prev_page_title_sv.get_value()
                                aria-label=prev_page_aria_label_sv.get_value()
                                disabled=disabled
                                data-pager-btn="true"
                                on:click=go_prev.clone()
                                on:focus=move |_| focused_index.set(1)
                            >
                                <span class="notranslate rzi rzi-caret-left"></span>
                                {move || prev_page_label_sv.get_value().map(|l| {
                                    view! { <span class="rz-button-text">{l}</span> }
                                })}
                            </button>
                        }
                    }}

                    // Page-number buttons — mirrors the @for loop in Blazor razor
                    {move || {
                        let (start, end, np, _) = pager.get();
                        let current = skip.get() / page_size_sig.get().max(1);
                        (start..end)
                            .map(|p| {
                                let is_active = p == current;
                                let btn_class = if is_active {
                                    "rz-pager-element rz-pager-page rz-state-active"
                                } else {
                                    "rz-pager-element rz-pager-page"
                                };
                                let display = p + 1; // 1-based for display
                                let title = page_title_format_sv
                                    .get_value()
                                    .replace("{0}", &display.to_string());
                                let aria = page_aria_label_format_sv
                                    .get_value()
                                    .replace("{0}", &display.to_string());
                                let fpc = fire_page_changed;
                                let ps = page_size_sig.get_untracked();
                                let btn_idx = (p - start + 2) as i32; // offset after first+prev
                                view! {
                                    <button
                                        type="button"
                                        class=btn_class
                                        title=title
                                        aria-label=aria
                                        aria-current=if is_active { Some("page") } else { None }
                                        data-pager-btn="true"
                                        on:click=move |_| { fpc(p * ps); }
                                        on:focus=move |_| focused_index.set(btn_idx)
                                    >
                                        {display.to_string()}
                                    </button>
                                }
                            })
                            .collect_view()
                    }}

                    // Next page
                    {move || {
                        let (_, _, np, _) = pager.get();
                        let current = skip.get() / page_size_sig.get().max(1);
                        let disabled = current + 1 >= np || np == 0;
                        let btn_class = if disabled {
                            "rz-pager-element rz-pager-next rz-state-disabled"
                        } else {
                            "rz-pager-element rz-pager-next"
                        };
                        let (_, _, _, nl) = pager.get();
                        let next_idx = (nl as i32) + 2; // after page buttons
                        view! {
                            <button
                                type="button"
                                class=btn_class
                                title=next_page_title_sv.get_value()
                                aria-label=next_page_aria_label_sv.get_value()
                                disabled=disabled
                                data-pager-btn="true"
                                on:click=go_next.clone()
                                on:focus=move |_| focused_index.set(next_idx)
                            >
                                {move || next_page_label_sv.get_value().map(|l| {
                                    view! { <span class="rz-button-text">{l}</span> }
                                })}
                                <span class="notranslate rzi rzi-caret-right"></span>
                            </button>
                        }
                    }}

                    // Last page
                    {move || {
                        let (_, _, np, _) = pager.get();
                        let current = skip.get() / page_size_sig.get().max(1);
                        let disabled = current + 1 >= np || np == 0;
                        let btn_class = if disabled {
                            "rz-pager-element rz-pager-last rz-state-disabled"
                        } else {
                            "rz-pager-element rz-pager-last"
                        };
                        let (_, _, _, nl) = pager.get();
                        let last_idx = (nl as i32) + 3;
                        view! {
                            <button
                                type="button"
                                class=btn_class
                                title=last_page_title_sv.get_value()
                                aria-label=last_page_aria_label_sv.get_value()
                                disabled=disabled
                                data-pager-btn="true"
                                on:click=go_last.clone()
                                on:focus=move |_| focused_index.set(last_idx)
                            >
                                <span class="notranslate rzi rzi-step-forward"></span>
                            </button>
                        }
                    }}

                    // Reload button — mirrors @if(AllowReload) { <button … /> }
                    {move || -> Option<AnyView> {
                        if !allow_reload {
                            return None;
                        }
                        Some(
                            leptos::html::button()
                                .attr("type", "button")
                                .attr("class", "rz-pager-element rz-pager-reload")
                                .attr("title", reload_title_sv.get_value())
                                .attr("aria-label", reload_aria_label_sv.get_value())
                                .attr("data-pager-btn", "true")
                                .on(leptos::ev::click, on_reload_click.clone())
                                .child(
                                    leptos::html::span()
                                        .attr("class", "notranslate rzi rzi-refresh"),
                                )
                                .into_any(),
                        )
                    }}
                </nav>

                // ── Page-size selector ────────────────────────────────────────
                // Mirrors Blazor: @if (PageSizeOptions != null && PageSizeOptions.Any())
                // { <select class="rz-paginator-rpp-options"> … </select> }
                {move || -> Option<AnyView> {
                    let opts = page_size_options_sv.get_value();
                    if opts.is_empty() {
                        return None;
                    }
                    let current_ps = page_size_sig.get();
                    let options: Vec<AnyView> = opts
                        .iter()
                        .map(|&ps| {
                            leptos::html::option()
                                .attr("value", ps.to_string())
                                .prop("selected", ps == current_ps)
                                .child(ps.to_string())
                                .into_any()
                        })
                        .collect();

                    Some(
                        leptos::html::div()
                            .attr("class", "rz-paginator-rpp-options")
                            .child(
                                leptos::html::select()
                                    .attr("aria-label", "Page size")
                                    .on(leptos::ev::change, on_ps_change.clone())
                                    .child(options),
                            )
                            .child(
                                leptos::html::span()
                                    .attr("class", "rz-pagesize-text")
                                    .child(page_size_text_sv.get_value()),
                            )
                            .into_any(),
                    )
                }}
            </div>
        </Show>
    }
}
