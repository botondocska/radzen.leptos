//! RadzenDatePicker component — mirrors C# Radzen.Blazor.RadzenDatePicker<TValue>.
//!
//! # CSS class order (mirrors Blazor exactly)
//! Root `<div>`: `rz-datepicker [rz-datepicker-inline] [rz-state-disabled] [caller-class]`
//! Trigger button: `rz-datepicker-trigger rz-button rz-button-icon-only [rz-state-disabled]`
//! Panel: `rz-datepicker-panel rz-shadow-1`
//! Calendar view: `rz-calendar-view`
//! Day cells: `rz-state-default [rz-state-active] [rz-calendar-other-month] [rz-state-disabled]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! GetClassList("rz-datepicker")
//!     .Add("rz-datepicker-inline", Inline)
//!     .ToString()
//! ```
//!
//! # Simplified scope
//! The full Blazor DatePicker supports Date, DateTime, Time, DateTimeOffset, and
//! range/multiple-date modes. For our grid use-case (date column filtering) we focus on:
//! - Date-only selection (year/month/day navigation)
//! - Optional text input (`show_input=true`)
//! - Optional inline mode (no trigger button — panel always visible)
//! - `Option<chrono::NaiveDate>` as the value type (nullable date)
//!
//! # HTML structure
//! ```html
//! <div class="rz-datepicker …" id="…" style="…">
//!   <!-- Text input (when show_input=true) -->
//!   <input class="rz-inputtext" type="text" value="…" readonly? … />
//!   <!-- Trigger button (when not inline) -->
//!   <button class="rz-datepicker-trigger rz-button rz-button-icon-only …" tabindex="-1">
//!     <span class="notranslate rzi rzi-calendar"></span>
//!   </button>
//!   <!-- Panel (shown when open, or always when inline) -->
//!   <div class="rz-datepicker-panel rz-shadow-1">
//!     <div class="rz-datepicker-calendar">
//!       <!-- Header row: prev / month-year / next -->
//!       <div class="rz-datepicker-header">
//!         <button class="rz-datepicker-prev …"><span class="rzi rzi-chevron-left"></span></button>
//!         <span class="rz-datepicker-title">…Month Year…</span>
//!         <button class="rz-datepicker-next …"><span class="rzi rzi-chevron-right"></span></button>
//!       </div>
//!       <!-- Calendar table -->
//!       <table class="rz-calendar-view">
//!         <thead> … day-name headers … </thead>
//!         <tbody> … day cells … </tbody>
//!       </table>
//!     </div>
//!   </div>
//! </div>
//! ```
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted when invisible.

use crate::components::{
    ClassList,
    base_component::{ComponentProps, use_radzen_base},
};
use chrono::{Datelike, Duration, NaiveDate};
use leptos::prelude::*;
use std::sync::Arc;

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Returns the first day of the given month.
fn first_day_of_month(year: i32, month: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, 1).expect("valid date")
}

/// Returns the last day of the given month.
fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    // Move to next month's day 1, then subtract 1 day.
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .expect("valid date")
        - Duration::days(1)
}

/// Format a date as `"Month YYYY"` e.g. `"January 2025"`.
fn format_month_year(year: i32, month: u32) -> String {
    let month_name = [
        "January", "February", "March", "April", "May", "June", "July", "August", "September",
        "October", "November", "December",
    ][(month - 1) as usize];
    format!("{} {}", month_name, year)
}

/// Format a `NaiveDate` as `"YYYY-MM-DD"`.
fn format_date(date: NaiveDate) -> String {
    format!("{}-{:02}-{:02}", date.year(), date.month(), date.day())
}

// ─────────────────────────────────────────────────────────────────────────────
// RadzenDatePicker
// ─────────────────────────────────────────────────────────────────────────────

/// RadzenDatePicker component.
///
/// A calendar-based date picker with optional text input and inline mode.
/// The value is `Option<NaiveDate>` — `None` represents an empty/cleared date.
///
/// # Popup mode (default)
/// ```rust,ignore
/// let date = RwSignal::new(None::<NaiveDate>);
/// <RadzenDatePicker value=date placeholder=Some("Pick a date") />
/// ```
///
/// # Inline mode (calendar always visible, no trigger button)
/// ```rust,ignore
/// <RadzenDatePicker value=date inline=true />
/// ```
#[component]
pub fn RadzenDatePicker(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Current date value. `None` = empty.
    #[prop(optional)]
    value: Option<RwSignal<Option<NaiveDate>>>,

    /// Placeholder text for the text input (when `show_input=true`).
    #[prop(default = None, into)]
    placeholder: Option<String>,

    /// Whether to show a text input next to the trigger button. Default: `true`.
    #[prop(default = true)]
    show_input: bool,

    /// Whether the calendar is always visible (no trigger button). Default: `false`.
    #[prop(default = false)]
    inline: bool,

    /// Whether the component is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Whether the component is read-only.
    #[prop(default = false)]
    read_only: bool,

    /// `name` attribute for the hidden input.
    #[prop(default = None, into)]
    name: Option<String>,

    /// Tab order. Forced to `-1` when disabled.
    #[prop(default = 0)]
    tab_index: i32,

    /// Minimum selectable date (inclusive). `None` = no minimum.
    #[prop(default = None)]
    min: Option<NaiveDate>,

    /// Maximum selectable date (inclusive). `None` = no maximum.
    #[prop(default = None)]
    max: Option<NaiveDate>,

    /// Whether to allow clearing the date by clicking the selected day again. Default: `true`.
    #[prop(default = true)]
    allow_clear: bool,

    /// Called when the selected date changes.
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(Option<NaiveDate>) + Send + Sync>>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    // Visibility.
    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    let value_signal = value.unwrap_or_else(|| RwSignal::new(None));
    let effective_tab = if disabled { -1 } else { tab_index };

    // ── View state: which year/month the calendar is showing ──────────────────
    let today = chrono::Local::now().naive_local().date();
    let initial = value_signal.get_untracked().unwrap_or(today);
    let view_year = RwSignal::new(initial.year());
    let view_month = RwSignal::new(initial.month());

    // ── Open state (ignored when inline=true) ─────────────────────────────────
    let open = RwSignal::new(inline);

    // ── CSS ───────────────────────────────────────────────────────────────────
    let caller_class = base
        .attrs
        .as_ref()
        .and_then(|a| a.get("class"))
        .cloned()
        .unwrap_or_default();

    let root_class = ClassList::create("rz-datepicker")
        .add("rz-datepicker-inline", inline)
        .add_disabled(disabled)
        .add_caller_class(if caller_class.is_empty() {
            None
        } else {
            Some(caller_class.as_str())
        })
        .finish();

    let style = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();

    // ── Commit a date selection ───────────────────────────────────────────────
    let on_change_cb = on_change.clone();
    let commit = Arc::new(move |new_date: Option<NaiveDate>| {
        value_signal.set(new_date);
        if let Some(ref cb) = on_change_cb {
            cb(new_date);
        }
        if !inline {
            open.set(false);
        }
    });

    // ── Toggle open ───────────────────────────────────────────────────────────
    let commit_close = commit.clone();
    let toggle_open = move |_ev: web_sys::MouseEvent| {
        if disabled || read_only {
            return;
        }
        if !inline {
            let is_open = open.get_untracked();
            if !is_open {
                // Sync view to selected or today.
                let d = value_signal.get_untracked().unwrap_or(today);
                view_year.set(d.year());
                view_month.set(d.month());
            }
            open.set(!is_open);
        }
        let _ = commit_close.clone(); // keep borrow
    };

    // Close on blur.
    let on_blur = move |_ev: web_sys::FocusEvent| {
        if !inline {
            gloo_timers::callback::Timeout::new(150, move || {
                open.set(false);
            })
            .forget();
        }
    };

    // ── Navigation ────────────────────────────────────────────────────────────
    let prev_month = move |_ev: web_sys::MouseEvent| {
        let (y, m) = (view_year.get_untracked(), view_month.get_untracked());
        if m == 1 {
            view_year.set(y - 1);
            view_month.set(12);
        } else {
            view_month.set(m - 1);
        }
    };

    let next_month = move |_ev: web_sys::MouseEvent| {
        let (y, m) = (view_year.get_untracked(), view_month.get_untracked());
        if m == 12 {
            view_year.set(y + 1);
            view_month.set(1);
        } else {
            view_month.set(m + 1);
        }
    };

    // Base events.
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb = handle.on_context_menu.clone();

    // Day names (Sun–Sat to match Blazor default locale).
    let day_names = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

    Some(
        leptos::html::div()
            .attr("id", handle_id)
            .attr("class", root_class)
            .attr("style", style)
            .on(leptos::ev::blur, on_blur)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            // ── Text input ────────────────────────────────────────────────────
            .child(show_input.then(|| {
                leptos::html::input()
                    .attr("type", "text")
                    .attr("name", name.clone())
                    .attr("class", "rz-inputtext rz-datepicker-input")
                    .attr("placeholder", placeholder.clone().unwrap_or_default())
                    .attr("readonly", true) // value controlled by calendar only
                    .attr("disabled", disabled)
                    .attr("tabindex", effective_tab.to_string())
                    .prop("value", move || {
                        value_signal
                            .get()
                            .map(format_date)
                            .unwrap_or_default()
                    })
            }))
            // ── Trigger button (non-inline) ───────────────────────────────────
            .child((!inline).then(|| {
                leptos::html::button()
                    .attr(
                        "class",
                        format!(
                            "rz-datepicker-trigger rz-button rz-button-icon-only{}",
                            if disabled { " rz-state-disabled" } else { "" }
                        ),
                    )
                    .attr("type", "button")
                    .attr("tabindex", "-1")
                    .attr("disabled", disabled)
                    .on(leptos::ev::click, toggle_open)
                    .child(
                        leptos::html::span()
                            .attr("class", "notranslate rzi rzi-calendar"),
                    )
            }))
            // ── Calendar panel ────────────────────────────────────────────────
            .child(move || {
                if !open.get() {
                    return None::<AnyView>.into_any();
                }

                let year = view_year.get();
                let month = view_month.get();
                let first = first_day_of_month(year, month);
                let last = last_day_of_month(year, month);
                let selected = value_signal.get();

                // Number of leading blank cells (0=Sunday index of first day).
                // chrono: Monday=0..Sunday=6. We want Sunday=0.
                let start_weekday = {
                    use chrono::Weekday;
                    match first.weekday() {
                        Weekday::Sun => 0,
                        Weekday::Mon => 1,
                        Weekday::Tue => 2,
                        Weekday::Wed => 3,
                        Weekday::Thu => 4,
                        Weekday::Fri => 5,
                        Weekday::Sat => 6,
                    }
                };

                // Build weeks as Vec<Vec<Option<NaiveDate>>>.
                let total_days = last.day() as usize;
                let total_cells = start_weekday + total_days;
                let num_rows = (total_cells + 6) / 7;
                let mut weeks: Vec<Vec<Option<NaiveDate>>> =
                    vec![vec![None; 7]; num_rows];
                for day in 1..=total_days {
                    let cell_index = start_weekday + day - 1;
                    let row = cell_index / 7;
                    let col = cell_index % 7;
                    weeks[row][col] = NaiveDate::from_ymd_opt(year, month, day as u32);
                }

                let commit_day = commit.clone();

                Some(
                    leptos::html::div()
                        .attr("class", "rz-datepicker-panel rz-shadow-1")
                        .on(leptos::ev::mousedown, |ev: web_sys::MouseEvent| {
                            ev.prevent_default(); // prevent blur
                        })
                        .child(
                            leptos::html::div()
                                .attr("class", "rz-datepicker-calendar")
                                // ── Header ────────────────────────────────────
                                .child(
                                    leptos::html::div()
                                        .attr("class", "rz-datepicker-header")
                                        .child(
                                            leptos::html::button()
                                                .attr("type", "button")
                                                .attr("class", "rz-datepicker-prev rz-button rz-button-icon-only")
                                                .on(leptos::ev::click, prev_month)
                                                .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-left")),
                                        )
                                        .child(
                                            leptos::html::span()
                                                .attr("class", "rz-datepicker-title")
                                                .child(format_month_year(year, month)),
                                        )
                                        .child(
                                            leptos::html::button()
                                                .attr("type", "button")
                                                .attr("class", "rz-datepicker-next rz-button rz-button-icon-only")
                                                .on(leptos::ev::click, next_month)
                                                .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-right")),
                                        ),
                                )
                                // ── Calendar table ─────────────────────────────
                                .child(
                                    leptos::html::table()
                                        .attr("class", "rz-calendar-view")
                                        // Day-name header.
                                        .child(
                                            leptos::html::thead().child(
                                                leptos::html::tr().child(
                                                    day_names
                                                        .iter()
                                                        .map(|n| {
                                                            leptos::html::th()
                                                                .attr("scope", "col")
                                                                .child(*n)
                                                                .into_any()
                                                        })
                                                        .collect_view(),
                                                ),
                                            ),
                                        )
                                        // Day rows.
                                        .child(
                                            leptos::html::tbody().child(
                                                weeks
                                                    .into_iter()
                                                    .map(|week| {
                                                        let commit_row = commit_day.clone();
                                                        leptos::html::tr().child(
                                                            week.into_iter()
                                                                .map(move |day_opt| {
                                                                    let commit_cell = commit_row.clone();
                                                                    match day_opt {
                                                                        None => leptos::html::td()
                                                                            .attr("class", "rz-calendar-other-month")
                                                                            .into_any(),
                                                                        Some(day) => {
                                                                            let is_selected = selected.map_or(false, |s| s == day);
                                                                            let is_today = day == today;
                                                                            let below_min = min.map_or(false, |mn| day < mn);
                                                                            let above_max = max.map_or(false, |mx| day > mx);
                                                                            let is_disabled = disabled || below_min || above_max;

                                                                            let cell_class = ClassList::create("rz-state-default")
                                                                                .add("rz-state-active", is_selected)
                                                                                .add("rz-datepicker-today", is_today)
                                                                                .add_disabled(is_disabled)
                                                                                .finish();

                                                                            let on_day_click = move |_ev: web_sys::MouseEvent| {
                                                                                if is_disabled || read_only {
                                                                                    return;
                                                                                }
                                                                                let new_val = if allow_clear && is_selected {
                                                                                    None
                                                                                } else {
                                                                                    Some(day)
                                                                                };
                                                                                commit_cell(new_val);
                                                                            };

                                                                            leptos::html::td()
                                                                                .on(leptos::ev::click, on_day_click)
                                                                                .child(
                                                                                    leptos::html::span()
                                                                                        .attr("class", cell_class)
                                                                                        .child(day.day().to_string()),
                                                                                )
                                                                                .into_any()
                                                                        }
                                                                    }
                                                                })
                                                                .collect_view(),
                                                        ).into_any()
                                                    })
                                                    .collect_view(),
                                            ),
                                        ),
                                ),
                        ),
                )
                .into_any()
            }),
    )
    .into_any()
}