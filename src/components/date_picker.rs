//! RadzenDatePicker component — mirrors C# Radzen.Blazor.RadzenDatePicker<TValue>.
//!
//! # CSS class (mirrors Blazor exactly)
//! Root `<div>`: `rz-datepicker [rz-datepicker-inline] [rz-state-disabled] [caller-class]`
//!
//! Input: `rz-inputtext [rz-input-trigger] [rz-readonly]`
//!   - `rz-input-trigger` when `!ShowButton` (input itself opens popup)
//!   - `rz-readonly` when `ReadOnly`
//!
//! Trigger button:
//!   `rz-datepicker-trigger [rz-datepicker-field-button] rz-button rz-button-icon-only [rz-state-disabled] {ButtonClass}`
//!   - `rz-datepicker-field-button` only when `ShowInput` is true (mirrors Blazor exactly)
//!
//! Trigger icon span: `notranslate rzi rzi-calendar`
//!   - SCSS: `.rzi-calendar:before { content: 'calendar_today'; }` — Material Symbols glyph
//!
//! Popup container: `rz-datepicker-popup-container` (popup) | `rz-datepicker-inline-container` (inline)
//!   - Positioned absolutely via `position: absolute; z-index: var(--rz-popup-z-index)`
//!   - The Blazor <Popup> component handles this; we replicate with inline style
//!
//! Calendar: `rz-calendar`
//! Calendar header: `rz-calendar-header`
//! Prev button: `rz-button rz-button-md rz-variant-text rz-button-icon-only rz-secondary rz-shade-default rz-calendar-prev`
//! Next button: `rz-button rz-button-md rz-variant-text rz-button-icon-only rz-secondary rz-shade-default rz-calendar-next`
//! Prev icon: `notranslate rzi rz-calendar-prev-icon` (SCSS pseudo-element for chevron-left)
//! Next icon: `notranslate rzi rz-calendar-next-icon` (SCSS pseudo-element for chevron-right)
//! Title: `rz-calendar-title`
//! Table wrapper: `rz-calendar-view-container` (tabindex for keyboard nav)
//! Table: `rz-calendar-view rz-calendar-month-view`
//! Other-month td: `rz-datepicker-other-month`
//! Day span: `rz-state-default [rz-state-active] [rz-datepicker-today] [rz-state-disabled]`
//!
//! Blazor `GetComponentCssClass()`:
//! ```csharp
//! GetClassList("rz-datepicker")
//!     .Add("rz-datepicker-inline", Inline)
//!     .ToString()
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

fn last_day_of_month(year: i32, month: u32) -> NaiveDate {
    let (ny, nm) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    NaiveDate::from_ymd_opt(ny, nm, 1).expect("valid date") - Duration::days(1)
}

fn month_name(month: u32) -> &'static str {
    ["January","February","March","April","May","June",
     "July","August","September","October","November","December"]
        [(month - 1) as usize]
}

fn format_date(date: NaiveDate) -> String {
    format!("{}-{:02}-{:02}", date.year(), date.month(), date.day())
}

/// Build a flat 42-cell grid (6 rows × 7 cols, Sun–Sat) for the given month.
/// Cells before/after the current month are filled from adjacent months.
/// Mirrors Blazor's `StartDate` + `dayNumber` loop over 42 iterations.
fn build_calendar_weeks(year: i32, month: u32) -> Vec<Vec<NaiveDate>> {
    let first = NaiveDate::from_ymd_opt(year, month, 1).expect("valid date");
    let start_offset = {
        use chrono::Weekday;
        match first.weekday() {
            Weekday::Sun => 0i64,
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
        }
    };
    let grid_start = first - Duration::days(start_offset);
    (0..6)
        .map(|row| (0..7).map(|col| grid_start + Duration::days(row * 7 + col)).collect())
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// RadzenDatePicker
// ─────────────────────────────────────────────────────────────────────────────

/// RadzenDatePicker component.
///
/// A calendar-based date picker with optional text input and inline mode.
/// The value is `Option<NaiveDate>` — `None` represents an empty/cleared date.
#[component]
pub fn RadzenDatePicker(
    /// Base component properties (id, style, visible, attrs, locale, mouse events).
    #[prop(default = Default::default())]
    base: ComponentProps,

    /// Current date value. `None` = empty.
    #[prop(optional)]
    value: Option<RwSignal<Option<NaiveDate>>>,

    /// Placeholder text for the text input.
    #[prop(default = None, into)]
    placeholder: Option<String>,

    /// Whether to show the text input field. Default: `true`. Mirrors Blazor `ShowInput`.
    #[prop(default = true)]
    show_input: bool,

    /// Whether to show the calendar trigger button. Default: `true`. Mirrors Blazor `ShowButton`.
    #[prop(default = true)]
    show_button: bool,

    /// Whether the calendar is always visible (inline). Default: `false`.
    #[prop(default = false)]
    inline: bool,

    /// Whether the component is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Whether the component is read-only.
    #[prop(default = false)]
    read_only: bool,

    /// `name` attribute for the input.
    #[prop(default = None, into)]
    name: Option<String>,

    /// Tab order. Forced to `-1` when disabled.
    #[prop(default = 0)]
    tab_index: i32,

    /// Minimum selectable date (inclusive).
    #[prop(default = None)]
    min: Option<NaiveDate>,

    /// Maximum selectable date (inclusive).
    #[prop(default = None)]
    max: Option<NaiveDate>,

    /// Whether clicking a selected day again clears it. Default: `true`.
    #[prop(default = true)]
    allow_clear: bool,

    /// Called when the selected date changes.
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(Option<NaiveDate>) + Send + Sync>>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    let value_signal = value.unwrap_or_else(|| RwSignal::new(None));
    let effective_tab = if disabled { -1 } else { tab_index };

    // ── View state ────────────────────────────────────────────────────────────
    let today = chrono::Local::now().naive_local().date();
    let initial = value_signal.get_untracked().unwrap_or(today);
    let view_year  = RwSignal::new(initial.year());
    let view_month = RwSignal::new(initial.month());

    // ── Open state ────────────────────────────────────────────────────────────
    // Always open when inline; otherwise toggled by the button.
    let open = RwSignal::new(inline);

    // ── Root CSS ──────────────────────────────────────────────────────────────
    let caller_class = base.attrs.as_ref()
        .and_then(|a| a.get("class")).cloned().unwrap_or_default();
    let root_class = ClassList::create("rz-datepicker")
        .add("rz-datepicker-inline", inline)
        .add_disabled(disabled)
        .add_caller_class(if caller_class.is_empty() { None } else { Some(caller_class.as_str()) })
        .finish();

    // ── Input CSS ─────────────────────────────────────────────────────────────
    // Mirrors Blazor: `rz-inputtext @InputClass @(ReadOnly ? "rz-readonly" : "") @(!ShowButton ? "rz-input-trigger" : "")`
    let input_class = format!(
        "rz-inputtext{}{}",
        if !show_button { " rz-input-trigger" } else { "" },
        if read_only   { " rz-readonly"       } else { "" },
    );

    // ── Trigger button CSS ────────────────────────────────────────────────────
    // Mirrors Blazor exactly:
    // `rz-datepicker-trigger{(ShowInput ? " rz-datepicker-field-button" : "")} rz-button rz-button-icon-only{(Disabled ? " rz-state-disabled" : "")} {ButtonClass}`
    // NOTE: `rz-datepicker-field-button` only when ShowInput=true, NOT `ShowInput || !ShowButton`.
    let button_class = format!(
        "rz-datepicker-trigger{} rz-button rz-button-icon-only{}",
        if show_input { " rz-datepicker-field-button" } else { "" },
        if disabled   { " rz-state-disabled"          } else { "" },
    );

    let style    = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();

    // ── Commit ────────────────────────────────────────────────────────────────
    let on_change_cb = on_change.clone();
    let commit = Arc::new(move |new_date: Option<NaiveDate>| {
        value_signal.set(new_date);
        if let Some(ref cb) = on_change_cb { cb(new_date); }
        if !inline { open.set(false); }
    });

    // ── Toggle ────────────────────────────────────────────────────────────────
    // Mirrors Blazor `OnToggle` — called on `@onmousedown` of the trigger button.
    let toggle_open = move |_ev: web_sys::MouseEvent| {
        if disabled || read_only || inline { return; }
        let is_open = open.get_untracked();
        if !is_open {
            // Sync view to selected value (or today) when opening.
            let d = value_signal.get_untracked().unwrap_or(today);
            view_year.set(d.year());
            view_month.set(d.month());
        }
        open.set(!is_open);
    };

    // Close on blur — short delay so day clicks register first.
    let on_blur = move |_ev: web_sys::FocusEvent| {
        if !inline {
            gloo_timers::callback::Timeout::new(150, move || { open.set(false); }).forget();
        }
    };

    // ── Month navigation ──────────────────────────────────────────────────────
    let prev_month = move |_: web_sys::MouseEvent| {
        let (y, m) = (view_year.get_untracked(), view_month.get_untracked());
        if m == 1 { view_year.set(y - 1); view_month.set(12); }
        else       { view_month.set(m - 1); }
    };
    let next_month = move |_: web_sys::MouseEvent| {
        let (y, m) = (view_year.get_untracked(), view_month.get_untracked());
        if m == 12 { view_year.set(y + 1); view_month.set(1); }
        else        { view_month.set(m + 1); }
    };

    // Base events.
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb   = handle.on_context_menu.clone();

    // Abbreviated day names, Sun–Sat (Blazor default locale).
    let day_names = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

    view! {
        <div
            id=handle_id
            class=root_class
            style=style
            tabindex=effective_tab.to_string()
            on:blur=on_blur
            on:mouseenter=move |ev| enter_cb(ev)
            on:mouseleave=move |ev| leave_cb(ev)
            on:contextmenu=move |ev| ctx_cb(ev)
        >
            // ── Input + trigger button ────────────────────────────────────────
            // Mirrors Blazor: @if (!Inline) { … input … button … }
            // When inline=true NEITHER the input NOR the button are rendered —
            // the calendar is the entire component (rz-datepicker-inline-container).
            {(!inline).then(|| view! {
                // Input — mirrors: @if (ShowInput || !ShowButton) { <input … /> }
                // When ShowButton=false the input has `rz-input-trigger` class and
                // its `onmousedown` opens the popup (Blazor wires this via JS interop).
                {(show_input || !show_button).then(|| view! {
                    <input
                        type="text"
                        name=name.clone()
                        id=name.clone()
                        class=input_class
                        autocomplete="off"
                        placeholder=placeholder.unwrap_or_default()
                        disabled=disabled
                        readonly=true
                        tabindex=if disabled { "-1".to_string() } else { effective_tab.to_string() }
                        prop:value=move || value_signal.get().map(format_date).unwrap_or_default()
                        on:mousedown=if !show_button { Some(toggle_open) } else { None }
                    />
                })}

                // Trigger button — mirrors: @if (ShowButton) { <button @onmousedown=@OnToggle …> }
                // Positioned absolutely over the input's right edge by SCSS.
                // Uses `onmousedown` (fires before `blur`) to open/close the popup.
                // Icon span class: `notranslate rzi rzi-calendar`
                //   SCSS: `.rzi-calendar:before { content: 'calendar_today'; }` — Material Symbols.
                {show_button.then(|| view! {
                    <button
                        type="button"
                        class=button_class
                        tabindex="-1"
                        disabled=disabled
                        aria-haspopup="dialog"
                        aria-expanded=move || if open.get() { "true" } else { "false" }
                        on:mousedown=toggle_open
                    >
                        <span class="notranslate rzi rzi-calendar"></span>
                        <span class="rz-button-text"></span>
                    </button>
                })}
            })}

            // ── Popup / inline calendar ───────────────────────────────────────
            // Mirrors Blazor's <Popup> component.
            // - `rz-datepicker-popup-container` in popup mode (position:absolute overlay).
            // - `rz-datepicker-inline-container` in inline mode.
            // - z-index ensures the popup renders above other content.
            // - `onmousedown:prevent_default` prevents the blur event from firing
            //   before a day click can register (same as Blazor's mousedown handling).
            {move || {
                if !open.get() {
                    return None::<AnyView>.into_any();
                }

                let year     = view_year.get();
                let month    = view_month.get();
                let selected = value_signal.get();
                let weeks    = build_calendar_weeks(year, month);
                let commit_c = commit.clone();

                let container_class = if inline {
                    "rz-datepicker-inline-container"
                } else {
                    "rz-datepicker-popup-container"
                };

                // The Blazor <Popup> component renders with position:absolute and a
                // high z-index. We replicate that here since we don't have the JS popup.
                let popup_style = if inline {
                    String::new()
                } else {
                    "position:absolute;z-index:var(--rz-popup-z-index,1000);left:0;top:100%;min-width:100%".to_string()
                };

                // ── Header day-name cells ─────────────────────────────────────
                let header_cells: Vec<AnyView> = day_names.iter().map(|&n| view! {
                    <th scope="col"><span>{n}</span></th>
                }.into_any()).collect();

                // ── Calendar rows (always 6 — mirrors Blazor's `for i in 0..6`) ──
                let rows: Vec<AnyView> = weeks.into_iter().map(|week| {
                    let commit_row = commit_c.clone();
                    let cells: Vec<AnyView> = week.into_iter().map(|day| {
                        let commit_cell  = commit_row.clone();
                        let is_cur_month = day.year() == year && day.month() == month;
                        let is_selected  = selected.map_or(false, |s| s == day);
                        let is_today     = day == today;
                        let is_disabled  = disabled
                            || min.map_or(false, |mn| day < mn)
                            || max.map_or(false, |mx| day > mx);

                        // <td> — `rz-datepicker-other-month` for cells outside current month.
                        // Mirrors Blazor's GetDayCssClass on <td>.
                        let td_class = if !is_cur_month {
                            "rz-datepicker-other-month".to_string()
                        } else {
                            String::new()
                        };

                        // <span> — state classes. Mirrors Blazor's GetDayCssClass(date, dateArgs, false).
                        let span_class = ClassList::create("rz-state-default")
                            .add("rz-state-active",    is_selected)
                            .add("rz-datepicker-today", is_today && is_cur_month)
                            .add_disabled(is_disabled || !is_cur_month)
                            .finish();

                        let day_num = day.day().to_string();

                        view! {
                            <td
                                class=td_class
                                on:click=move |_| {
                                    if is_disabled || read_only || !is_cur_month { return; }
                                    let v = if allow_clear && is_selected { None } else { Some(day) };
                                    commit_cell(v);
                                }
                            >
                                <span class=span_class>{day_num}</span>
                            </td>
                        }.into_any()
                    }).collect();

                    view! { <tr>{cells}</tr> }.into_any()
                }).collect();

                view! {
                    <div
                        class=container_class
                        style=popup_style
                        on:mousedown=|ev: web_sys::MouseEvent| ev.prevent_default()
                    >
                        <div class="rz-calendar">

                            // ── Calendar header ───────────────────────────────
                            // Mirrors Blazor:
                            //   <div class="rz-calendar-header">
                            //     <button class="… rz-calendar-prev">
                            //       <span class="notranslate rzi rz-calendar-prev-icon"></span>
                            //     </button>
                            //     <div class="rz-calendar-title"> … </div>
                            //     <button class="… rz-calendar-next">
                            //       <span class="notranslate rzi rz-calendar-next-icon"></span>
                            //     </button>
                            //   </div>
                            // Prev/next icon classes have SCSS pseudo-elements for the chevron glyphs.
                            <div class="rz-calendar-header">
                                <button
                                    type="button"
                                    tabindex="-1"
                                    class="rz-button rz-button-md rz-variant-text rz-button-icon-only rz-secondary rz-shade-default rz-calendar-prev"
                                    disabled=disabled
                                    on:click=prev_month
                                >
                                    <span class="notranslate rzi rz-calendar-prev-icon"></span>
                                </button>

                                // Title — Blazor uses two RadzenDropDown components for month/year.
                                // We render plain text spans for simplicity (no dropdown needed for
                                // basic date-only selection).
                                <div class="rz-calendar-title">
                                    <span class="rz-calendar-month">{month_name(month)}</span>
                                    " "
                                    <span class="rz-calendar-year">{year.to_string()}</span>
                                </div>

                                <button
                                    type="button"
                                    tabindex="-1"
                                    class="rz-button rz-button-md rz-variant-text rz-button-icon-only rz-secondary rz-shade-default rz-calendar-next"
                                    disabled=disabled
                                    on:click=next_month
                                >
                                    <span class="notranslate rzi rz-calendar-next-icon"></span>
                                </button>
                            </div>

                            // ── Calendar grid ─────────────────────────────────
                            // Mirrors Blazor: <div class="rz-calendar-view-container" tabindex="…">
                            //   <table class="rz-calendar-view rz-calendar-month-view" …>
                            <div class="rz-calendar-view-container" tabindex=effective_tab.to_string()>
                                <table class="rz-calendar-view rz-calendar-month-view">
                                    <thead>
                                        <tr>{header_cells}</tr>
                                    </thead>
                                    <tbody>{rows}</tbody>
                                </table>
                            </div>

                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }.into_any()
}