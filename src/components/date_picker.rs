//! RadzenDatePicker component — mirrors C# Radzen.Blazor.RadzenDatePicker<TValue>.
//!
//! # CSS class (mirrors Blazor exactly)
//! Root `<div>`: `rz-datepicker [rz-datepicker-inline] [rz-state-disabled] [caller-class]`
//! Input: `rz-inputtext [rz-input-trigger] [rz-readonly]`
//! Trigger button: `rz-datepicker-trigger [rz-datepicker-field-button] rz-button rz-button-icon-only [rz-state-disabled]`
//! Calendar: `rz-calendar`
//! Calendar header: `rz-calendar-header`
//! Prev/Next buttons: `rz-button rz-button-md rz-variant-text rz-button-icon-only rz-secondary rz-shade-default rz-calendar-prev|next`
//! Table wrapper: `rz-calendar-view-container`
//! Table: `rz-calendar-view rz-calendar-month-view`
//! Other-month td: `rz-datepicker-other-month`
//! Day span: `rz-state-default [rz-state-active] [rz-datepicker-today] [rz-state-disabled]`
//! Time section: `rz-timepicker`
//! Hour/minute/second: `rz-hour-picker`, `rz-minute-picker`, `rz-second-picker`
//! AM/PM: `rz-ampm-picker`
//! Footer: `rz-datepicker-footer`
//! Week number cell: `rz-calendar-other-month rz-calendar-week-number`
//! Clear button: `notranslate rz-dropdown-clear-icon rzi rzi-times`
//!
//! # Visibility
//! Mirrors `@if (Visible)` — element fully omitted when invisible.

use crate::components::{
    ClassList,
    base_component::{ComponentProps, use_radzen_base},
};
use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use leptos::prelude::*;
use std::sync::Arc;

// ─────────────────────────────────────────────────────────────────────────────
// DateRender event args — mirrors Blazor DateRenderEventArgs
// ─────────────────────────────────────────────────────────────────────────────

/// Event args passed to the `date_render` callback for each calendar cell.
/// Mirrors Blazor's `DateRenderEventArgs`.
#[derive(Clone, Debug)]
pub struct DateRenderEventArgs {
    /// The date this cell represents.
    pub date: NaiveDate,
    /// Set to `true` to disable this date (prevents selection).
    pub disabled: bool,
    /// Extra CSS class to append to this cell's `<td>`.
    pub attributes: Option<String>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────────

fn month_name_full(month: u32) -> &'static str {
    ["January","February","March","April","May","June",
     "July","August","September","October","November","December"]
        [(month - 1) as usize]
}

fn month_abbr(month: u32) -> &'static str {
    ["Jan","Feb","Mar","Apr","May","Jun",
     "Jul","Aug","Sep","Oct","Nov","Dec"]
        [(month - 1) as usize]
}

/// Format a NaiveDate according to a simple format string.
/// Supported tokens: `yyyy`, `yy`, `MM`, `M`, `dd`, `d`.
/// Falls back to ISO (`YYYY-MM-DD`) for unknown tokens.
fn format_date_str(date: NaiveDate, fmt: &str) -> String {
    let y = date.year();
    let m = date.month();
    let d = date.day();
    fmt.replace("yyyy", &format!("{:04}", y))
       .replace("yy",   &format!("{:02}", y % 100))
       .replace("MMMM", month_name_full(m))
       .replace("MMM",  month_abbr(m))
       .replace("MM",   &format!("{:02}", m))
       .replace("M",    &format!("{}", m))
       .replace("dd",   &format!("{:02}", d))
       .replace("d",    &format!("{}", d))
}

fn format_datetime_str(dt: NaiveDateTime, date_fmt: &str, show_time: bool, hour_fmt: &str, show_seconds: bool) -> String {
    let date_part = format_date_str(dt.date(), date_fmt);
    if !show_time {
        return date_part;
    }
    let h = dt.hour();
    let m = dt.minute();
    let s = dt.second();
    let time_part = if hour_fmt == "12" {
        let (ampm, h12) = if h < 12 { ("AM", if h == 0 { 12 } else { h }) } else { ("PM", if h == 12 { 12 } else { h - 12 }) };
        if show_seconds { format!("{:02}:{:02}:{:02} {}", h12, m, s, ampm) }
        else { format!("{:02}:{:02} {}", h12, m, ampm) }
    } else {
        if show_seconds { format!("{:02}:{:02}:{:02}", h, m, s) }
        else { format!("{:02}:{:02}", h, m) }
    };
    format!("{} {}", date_part, time_part)
}

/// Parse a typed date string. Tries common formats.
fn parse_date_input(s: &str) -> Option<NaiveDate> {
    let s = s.trim();
    // Try a handful of common formats
    for fmt in &["%Y-%m-%d", "%m/%d/%Y", "%d/%m/%Y", "%d.%m.%Y", "%Y.%m.%d"] {
        if let Ok(d) = NaiveDate::parse_from_str(s, fmt) {
            return Some(d);
        }
    }
    None
}

/// Build a flat 42-cell grid (6 rows × 7 cols, Sun–Sat) for the given month.
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
    (0..6).map(|row| (0..7).map(|col| grid_start + Duration::days(row * 7 + col)).collect()).collect()
}

/// ISO 8601 week number (mirrors Blazor's Calendar.GetWeekOfYear).
fn iso_week_number(date: NaiveDate) -> u32 {
    date.iso_week().week()
}

/// Parse YearRange string like "1900:2100" into (from, to).
fn parse_year_range(range: &str) -> (i32, i32) {
    let parts: Vec<&str> = range.split(':').collect();
    let from = parts.first().and_then(|s| s.parse().ok()).unwrap_or(1900);
    let to   = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(2100);
    (from, to)
}

// ─────────────────────────────────────────────────────────────────────────────
// RadzenDatePicker
// ─────────────────────────────────────────────────────────────────────────────

/// RadzenDatePicker component.
///
/// A calendar-based date/time picker with optional text input and inline mode.
/// The value is `Option<NaiveDateTime>` — `None` represents an empty/cleared field.
/// For date-only use cases, the time component is ignored when `show_time=false`.
#[component]
pub fn RadzenDatePicker(
    // ── Base ──────────────────────────────────────────────────────────────────
    #[prop(default = Default::default())]
    base: ComponentProps,

    // ── Value ─────────────────────────────────────────────────────────────────
    /// Current date/time value. `None` = empty.
    #[prop(optional)]
    value: Option<RwSignal<Option<NaiveDateTime>>>,

    // ── Display ───────────────────────────────────────────────────────────────
    /// Date format string. Supported tokens: `yyyy`, `yy`, `MMMM`, `MMM`, `MM`, `M`, `dd`, `d`.
    /// Default: `"M/d/yyyy"`. Mirrors Blazor `DateFormat`.
    #[prop(default = "M/d/yyyy".to_string(), into)]
    date_format: String,

    /// Placeholder text for the text input.
    #[prop(default = None, into)]
    placeholder: Option<String>,

    // ── Layout ────────────────────────────────────────────────────────────────
    /// Whether to show the text input field. Default: `true`.
    #[prop(default = true)]
    show_input: bool,

    /// Whether to show the calendar trigger button. Default: `true`.
    #[prop(default = true)]
    show_button: bool,

    /// Whether the calendar is always visible (inline). Default: `false`.
    #[prop(default = false)]
    inline: bool,

    /// Show the day grid. Default: `true`. Set `false` for month/year-only pickers.
    /// Mirrors Blazor `ShowDays`.
    #[prop(default = true)]
    show_days: bool,

    /// Show ISO week number column. Default: `false`. Mirrors Blazor `ShowCalendarWeek`.
    #[prop(default = false)]
    show_calendar_week: bool,

    /// Column header for week numbers. Default: `"#"`.
    #[prop(default = "#".to_string(), into)]
    calendar_week_title: String,

    // ── Time ──────────────────────────────────────────────────────────────────
    /// Show the time picker section below the calendar. Default: `false`.
    #[prop(default = false)]
    show_time: bool,

    /// Show only the time picker, hide the calendar entirely. Default: `false`.
    /// Mirrors Blazor `TimeOnly`.
    #[prop(default = false)]
    time_only: bool,

    /// Show the hour spinner. Default: `true`.
    #[prop(default = true)]
    show_hour: bool,

    /// Show the minutes spinner. Default: `true`.
    #[prop(default = true)]
    show_minutes: bool,

    /// Show the seconds spinner. Default: `false`.
    #[prop(default = false)]
    show_seconds: bool,

    /// Hour format: `"24"` or `"12"`. Default: `"24"`.
    #[prop(default = "24".to_string(), into)]
    hour_format: String,

    /// Step for the hour spinner. Default: `1`.
    #[prop(default = 1u32)]
    hours_step: u32,

    /// Step for the minutes spinner. Default: `1`.
    #[prop(default = 1u32)]
    minutes_step: u32,

    /// Step for the seconds spinner. Default: `1`.
    #[prop(default = 1u32)]
    seconds_step: u32,

    /// Pad hours with leading zero. Default: `false`.
    #[prop(default = false)]
    pad_hours: bool,

    /// Pad minutes with leading zero. Default: `false`.
    #[prop(default = false)]
    pad_minutes: bool,

    /// Pad seconds with leading zero. Default: `false`.
    #[prop(default = false)]
    pad_seconds: bool,

    /// Show OK button in the time picker. Default: `true`.
    #[prop(default = true)]
    show_time_ok_button: bool,

    // ── State ─────────────────────────────────────────────────────────────────
    /// Whether the component is disabled.
    #[prop(default = false)]
    disabled: bool,

    /// Whether the component is read-only.
    #[prop(default = false)]
    read_only: bool,

    /// Whether user can type a date into the text input. Default: `true`.
    #[prop(default = true)]
    allow_input: bool,

    /// Parse on every keystroke when `allow_input=true`. Default: `false`.
    #[prop(default = false)]
    immediate: bool,

    // ── Constraints ───────────────────────────────────────────────────────────
    /// Minimum selectable date (inclusive).
    #[prop(default = None)]
    min: Option<NaiveDate>,

    /// Maximum selectable date (inclusive).
    #[prop(default = None)]
    max: Option<NaiveDate>,

    /// Year range string like `"1900:2100"`. Bounds the year dropdown and navigation.
    #[prop(default = "1900:2100".to_string(), into)]
    year_range: String,

    // ── Clear ─────────────────────────────────────────────────────────────────
    /// Show the × clear button when a value is set. Default: `true`.
    /// Mirrors Blazor `AllowClear`.
    #[prop(default = true)]
    allow_clear: bool,

    // ── Multiple ──────────────────────────────────────────────────────────────
    /// Allow selecting multiple dates. Default: `false`.
    #[prop(default = false)]
    multiple: bool,

    /// Signal holding multiple selected dates (used when `multiple=true`).
    #[prop(optional)]
    value_multiple: Option<RwSignal<Vec<NaiveDate>>>,

    // ── Initial view ──────────────────────────────────────────────────────────
    /// Which month to show when the picker opens (not the selected date, just the view).
    /// Mirrors Blazor `InitialViewDate`.
    #[prop(default = None)]
    initial_view_date: Option<NaiveDate>,

    // ── Per-day customisation ─────────────────────────────────────────────────
    /// Called for every calendar cell. Mutate the args to disable a date or add CSS.
    /// Mirrors Blazor `DateRender` (`EventCallback<DateRenderEventArgs>`).
    #[prop(default = None)]
    date_render: Option<Arc<dyn Fn(NaiveDate) -> DateRenderEventArgs + Send + Sync>>,

    // ── Footer ────────────────────────────────────────────────────────────────
    /// Custom content rendered below the calendar grid inside the popup.
    /// Mirrors Blazor `FooterTemplate`.
    #[prop(optional)]
    footer_template: Option<ChildrenFn>,

    // ── HTML ──────────────────────────────────────────────────────────────────
    /// `name` attribute for the input.
    #[prop(default = None, into)]
    name: Option<String>,

    /// Tab order. Forced to `-1` when disabled.
    #[prop(default = 0)]
    tab_index: i32,

    // ── Callbacks ─────────────────────────────────────────────────────────────
    /// Called when the selected date/time changes.
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(Option<NaiveDateTime>) + Send + Sync>>,
) -> impl IntoView {
    let handle = use_radzen_base(&base, "");

    if !handle.visible.get_untracked() {
        return None::<AnyView>.into_any();
    }

    // ── Year range ────────────────────────────────────────────────────────────
    let (year_from, year_to) = parse_year_range(&year_range);

    // ── Value signal ──────────────────────────────────────────────────────────
    let value_signal = value.unwrap_or_else(|| RwSignal::new(None));
    let multi_signal = value_multiple.unwrap_or_else(|| RwSignal::new(Vec::new()));

    let effective_tab = if disabled { -1 } else { tab_index };
    let today = chrono::Local::now().naive_local().date();

    // ── View state (month/year shown in the calendar) ─────────────────────────
    let init_view = initial_view_date
        .or_else(|| value_signal.get_untracked().map(|dt| dt.date()))
        .unwrap_or(today);
    let view_year  = RwSignal::new(init_view.year());
    let view_month = RwSignal::new(init_view.month());

    // ── Time state (hour/minute/second spinners, decoupled from value) ─────────
    // Pending time is separate from committed value — only committed on OK / day click.
    let init_time = value_signal.get_untracked()
        .map(|dt| (dt.hour(), dt.minute(), dt.second()))
        .unwrap_or((0, 0, 0));
    let pending_hour   = RwSignal::new(init_time.0);
    let pending_minute = RwSignal::new(init_time.1);
    let pending_second = RwSignal::new(init_time.2);

    // ── Open state ────────────────────────────────────────────────────────────
    let open = RwSignal::new(inline);

    // ── CSS ───────────────────────────────────────────────────────────────────
    let caller_class = base.attrs.as_ref()
        .and_then(|a| a.get("class")).cloned().unwrap_or_default();
    let root_class = ClassList::create("rz-datepicker")
        .add("rz-datepicker-inline", inline)
        .add_disabled(disabled)
        .add_caller_class(if caller_class.is_empty() { None } else { Some(caller_class.as_str()) })
        .finish();

    let input_class = format!(
        "rz-inputtext{}{}",
        if !show_button { " rz-input-trigger" } else { "" },
        if read_only   { " rz-readonly"       } else { "" },
    );
    let button_class = format!(
        "rz-datepicker-trigger{} rz-button rz-button-icon-only{}",
        if show_input { " rz-datepicker-field-button" } else { "" },
        if disabled   { " rz-state-disabled"          } else { "" },
    );

    let style     = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();

    // ── Formatted display value ───────────────────────────────────────────────
    let date_format_sv  = StoredValue::new(date_format.clone());
    let hour_format_sv  = StoredValue::new(hour_format.clone());
    let formatted_value = move || -> String {
        if multiple {
            let dates = multi_signal.get();
            if dates.is_empty() { return String::new(); }
            return dates.iter()
                .map(|d| format_date_str(*d, &date_format_sv.get_value()))
                .collect::<Vec<_>>()
                .join(", ");
        }
        match value_signal.get() {
            None => String::new(),
            Some(dt) => format_datetime_str(
                dt,
                &date_format_sv.get_value(),
                show_time,
                &hour_format_sv.get_value(),
                show_seconds,
            ),
        }
    };

    // ── Commit a new value ────────────────────────────────────────────────────
    let on_change_cb = on_change.clone();
    let commit = Arc::new(move |new_dt: Option<NaiveDateTime>| {
        value_signal.set(new_dt);
        if let Some(ref cb) = on_change_cb { cb(new_dt); }
        if !inline { open.set(false); }
    });

    // ── Build NaiveDateTime from a selected date + current pending time ────────
    let make_datetime = move |date: NaiveDate| -> NaiveDateTime {
        let time = if show_time {
            NaiveTime::from_hms_opt(pending_hour.get_untracked(), pending_minute.get_untracked(), pending_second.get_untracked())
                .unwrap_or_default()
        } else {
            NaiveTime::from_hms_opt(0, 0, 0).unwrap()
        };
        NaiveDateTime::new(date, time)
    };

    // ── Toggle open ───────────────────────────────────────────────────────────
    let toggle_open = move |_ev: web_sys::MouseEvent| {
        if disabled || read_only || inline { return; }
        let is_open = open.get_untracked();
        if !is_open {
            // Sync view to selected date (or today) when opening.
            let d = value_signal.get_untracked().map(|dt| dt.date()).unwrap_or(today);
            view_year.set(d.year());
            view_month.set(d.month());
            // Sync pending time spinners.
            if let Some(dt) = value_signal.get_untracked() {
                pending_hour.set(dt.hour());
                pending_minute.set(dt.minute());
                pending_second.set(dt.second());
            }
        }
        open.set(!is_open);
    };

    let on_blur = move |_ev: web_sys::FocusEvent| {
        if !inline {
            gloo_timers::callback::Timeout::new(150, move || { open.set(false); }).forget();
        }
    };

    // ── Month navigation (clamped to year_range) ───────────────────────────────
    let prev_month = move |_: web_sys::MouseEvent| {
        if disabled { return; }
        let (y, m) = (view_year.get_untracked(), view_month.get_untracked());
        let (ny, nm) = if m == 1 { (y - 1, 12) } else { (y, m - 1) };
        if ny >= year_from { view_year.set(ny); view_month.set(nm); }
    };
    let next_month = move |_: web_sys::MouseEvent| {
        if disabled { return; }
        let (y, m) = (view_year.get_untracked(), view_month.get_untracked());
        let (ny, nm) = if m == 12 { (y + 1, 1) } else { (y, m + 1) };
        if ny <= year_to { view_year.set(ny); view_month.set(nm); }
    };

    // ── Parse typed input ─────────────────────────────────────────────────────
    let commit_parse = commit.clone();
    let parse_and_commit = Arc::new(move |text: String| {
        let trimmed = text.trim().to_string();
        if trimmed.is_empty() {
            commit_parse(None);
            return;
        }
        if let Some(date) = parse_date_input(&trimmed) {
            let dt = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
            commit_parse(Some(dt));
            // Sync view to newly typed date.
            view_year.set(date.year());
            view_month.set(date.month());
        }
        // Invalid → keep previous (mirrors Blazor: parse failure is a no-op).
    });

    // ── Clear ─────────────────────────────────────────────────────────────────
    let commit_clear = commit.clone();
    let on_clear = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        commit_clear(None);
        multi_signal.set(Vec::new());
    };

    // ── OK button (time picker) ────────────────────────────────────────────────
    let commit_ok = commit.clone();
    let on_ok_click = move |_: web_sys::MouseEvent| {
        // Commit current date with pending time.
        let dt = value_signal.get_untracked()
            .map(|existing| NaiveDateTime::new(
                existing.date(),
                NaiveTime::from_hms_opt(pending_hour.get_untracked(), pending_minute.get_untracked(), pending_second.get_untracked())
                    .unwrap_or_default(),
            ))
            .or_else(|| {
                // No date selected yet — use today + pending time.
                NaiveTime::from_hms_opt(pending_hour.get_untracked(), pending_minute.get_untracked(), pending_second.get_untracked())
                    .map(|t| NaiveDateTime::new(today, t))
            });
        commit_ok(dt);
    };

    // ── AM/PM toggle ──────────────────────────────────────────────────────────
    let toggle_ampm = move |_: web_sys::MouseEvent| {
        if disabled { return; }
        let h = pending_hour.get_untracked();
        pending_hour.set((h + 12) % 24);
    };

    // ── Base events ───────────────────────────────────────────────────────────
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb   = handle.on_context_menu.clone();

    let day_names = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

    // ── Year list for the year dropdown ───────────────────────────────────────
    let years: Vec<i32> = (year_from..=year_to).collect();
    let months_list: Vec<(u32, &'static str)> = (1u32..=12).map(|m| (m, month_abbr(m))).collect();

    // ── Build input section eagerly as AnyView (avoids tuple-arity issues) ────
    let has_value = move || -> bool {
        if multiple { !multi_signal.get().is_empty() }
        else { value_signal.get().is_some() }
    };

    let input_section: AnyView = if inline {
        ().into_any()
    } else {
        let input_class_c   = input_class.clone();
        let button_class_c  = button_class.clone();
        let name_c          = name.clone();
        let tab_str         = if disabled { "-1".to_string() } else { effective_tab.to_string() };
        let placeholder_c   = placeholder.clone().unwrap_or_default();
        let parse_input     = parse_and_commit.clone();
        let parse_imm       = parse_and_commit.clone();
        let toggle_for_inp  = toggle_open;

        // Input element.
        let input_el: AnyView = if show_input || !show_button {
            leptos::html::input()
                .attr("type", "text")
                .attr("name", name_c.clone())
                .attr("id", name_c.clone())
                .attr("class", input_class_c)
                .attr("autocomplete", "off")
                .attr("placeholder", placeholder_c)
                .attr("disabled", disabled)
                .attr("readonly", !allow_input || read_only)
                .attr("tabindex", tab_str)
                .prop("value", move || formatted_value())
                .on(leptos::ev::change, move |ev: web_sys::Event| {
                    use web_sys::wasm_bindgen::JsCast;
                    if let Some(inp) = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                        parse_input(inp.value());
                    }
                })
                .on(leptos::ev::input, move |ev: web_sys::Event| {
                    if !immediate { return; }
                    use web_sys::wasm_bindgen::JsCast;
                    if let Some(inp) = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                        parse_imm(inp.value());
                    }
                })
                .on(leptos::ev::mousedown, move |ev: web_sys::MouseEvent| {
                    if !show_button { toggle_for_inp(ev); }
                })
                .into_any()
        } else {
            ().into_any()
        };

        // Trigger button.
        let button_el: AnyView = if show_button {
            leptos::html::button()
                .attr("type", "button")
                .attr("class", button_class_c)
                .attr("tabindex", "-1")
                .attr("disabled", disabled)
                .attr("aria-haspopup", "dialog")
                .attr("aria-expanded", move || if open.get() { "true" } else { "false" })
                .on(leptos::ev::mousedown, toggle_open)
                .child(leptos::html::span().attr("class", "notranslate rzi rzi-calendar"))
                .child(leptos::html::span().attr("class", "rz-button-text"))
                .into_any()
        } else {
            ().into_any()
        };

        // Clear button — mirrors Blazor: `@if (AllowClear && HasValue && (ShowInput || !ShowButton))`
        let clear_el: AnyView = if allow_clear && (show_input || !show_button) {
            leptos::html::button()
                .attr("type", "button")
                .attr("class", "notranslate rz-dropdown-clear-icon rzi rzi-times")
                .attr("aria-label", "Clear")
                .on(leptos::ev::click, on_clear)
                // Only visible when there is a value.
                .attr("style", move || if has_value() { String::new() } else { "display:none".to_string() })
                .into_any()
        } else {
            ().into_any()
        };

        vec![input_el, button_el, clear_el].into_iter().collect_view().into_any()
    };

    // ── Popup / inline calendar — reactive closure ─────────────────────────────
    let commit_popup       = commit.clone();
    let date_render_sv     = StoredValue::new(date_render);
    let footer_sv          = StoredValue::new(footer_template);
    let hour_format_popup  = StoredValue::new(hour_format.clone());
    let cal_week_title_sv  = StoredValue::new(calendar_week_title.clone());

    let popup_child = move || -> AnyView {
        if !open.get() {
            return ().into_any();
        }

        let year     = view_year.get();
        let month    = view_month.get();
        let weeks    = build_calendar_weeks(year, month);
        let commit_c = commit_popup.clone();

        let container_class = if inline {
            "rz-datepicker-inline-container"
        } else {
            "rz-datepicker-popup-container"
        };
        let popup_style = if inline {
            String::new()
        } else {
            "position:absolute;z-index:var(--rz-popup-z-index,1000);left:0;top:100%;min-width:100%".to_string()
        };

        // ── is_selected helper ─────────────────────────────────────────────────
        let is_selected = move |date: NaiveDate| -> bool {
            if multiple {
                multi_signal.get().iter().any(|d| *d == date)
            } else {
                value_signal.get().map_or(false, |dt| dt.date() == date)
            }
        };

        // ── Header day-name cells ──────────────────────────────────────────────
        let mut header_cells: Vec<AnyView> = Vec::new();
        if show_calendar_week {
            header_cells.push(
                leptos::html::th()
                    .attr("scope", "col")
                    .attr("class", "rz-datepicker-week-number")
                    .child(leptos::html::span().child(cal_week_title_sv.get_value()))
                    .into_any()
            );
        }
        for &name in &day_names {
            header_cells.push(view! { <th scope="col"><span>{name}</span></th> }.into_any());
        }

        // ── Month dropdown ─────────────────────────────────────────────────────
        let month_opts: Vec<AnyView> = months_list.iter().map(|(v, label)| {
            let v = *v;
            let selected = v == month;
            leptos::html::option()
                .attr("value", v.to_string())
                .attr("selected", selected)
                .child(*label)
                .into_any()
        }).collect();

        let year_opts: Vec<AnyView> = years.iter().map(|&y| {
            let selected = y == year;
            leptos::html::option()
                .attr("value", y.to_string())
                .attr("selected", selected)
                .child(y.to_string())
                .into_any()
        }).collect();

        let month_select = leptos::html::select()
            .attr("class", "rz-calendar-month-dropdown rz-dropdown-label rz-inputtext")
            .attr("disabled", disabled)
            .on(leptos::ev::change, move |ev: web_sys::Event| {
                use web_sys::wasm_bindgen::JsCast;
                if let Some(sel) = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok()) {
                    if let Ok(v) = sel.value().parse::<u32>() { view_month.set(v); }
                }
            })
            .child(month_opts);

        let year_select = leptos::html::select()
            .attr("class", "rz-calendar-year-dropdown rz-dropdown-label rz-inputtext")
            .attr("disabled", disabled)
            .on(leptos::ev::change, move |ev: web_sys::Event| {
                use web_sys::wasm_bindgen::JsCast;
                if let Some(sel) = ev.target().and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok()) {
                    if let Ok(v) = sel.value().parse::<i32>() { view_year.set(v); }
                }
            })
            .child(year_opts);

        // ── Calendar rows ──────────────────────────────────────────────────────
        let rows: Vec<AnyView> = weeks.into_iter().map(|week| {
            let commit_row = commit_c.clone();
            let mut cells: Vec<AnyView> = Vec::new();

            // Week number column.
            if show_calendar_week {
                let wn = iso_week_number(week[0]);
                cells.push(
                    leptos::html::td()
                        .attr("class", "rz-calendar-other-month rz-calendar-week-number")
                        .child(wn.to_string())
                        .into_any()
                );
            }

            for day in week {
                let commit_cell  = commit_row.clone();
                let is_cur_month = day.year() == year && day.month() == month;

                // Apply date_render callback.
                let mut args = DateRenderEventArgs {
                    date: day,
                    disabled: false,
                    attributes: None,
                };
                if let Some(ref dr) = date_render_sv.get_value() {
                    args = dr(day);
                }

                let extra_min_max_disabled =
                    min.map_or(false, |mn| day < mn) || max.map_or(false, |mx| day > mx);
                let is_day_disabled = disabled || args.disabled || extra_min_max_disabled;

                let is_today     = day == today;
                let sel          = is_selected(day);

                let td_class = if !is_cur_month {
                    match &args.attributes {
                        Some(extra) => format!("rz-datepicker-other-month {}", extra),
                        None => "rz-datepicker-other-month".to_string(),
                    }
                } else {
                    args.attributes.clone().unwrap_or_default()
                };

                let span_class = ClassList::create("rz-state-default")
                    .add("rz-state-active",     sel)
                    .add("rz-datepicker-today",  is_today && is_cur_month)
                    .add_disabled(is_day_disabled || !is_cur_month)
                    .finish();

                let day_num = day.day().to_string();
                let tab = if is_day_disabled || !is_cur_month { "-1" } else { "0" };

                cells.push(
                    leptos::html::td()
                        .attr("class", td_class)
                        .attr("role", "button")
                        .attr("tabindex", tab)
                        .on(leptos::ev::click, move |_| {
                            if is_day_disabled || read_only || !is_cur_month { return; }
                            if multiple {
                                multi_signal.update(|v| {
                                    if let Some(pos) = v.iter().position(|d| *d == day) {
                                        v.remove(pos);
                                    } else {
                                        v.push(day);
                                    }
                                });
                                // For multiple mode, on_change fires separately below.
                                if let Some(ref cb) = on_change {
                                    // Fire with first selected or None.
                                    let first = multi_signal.get().first().map(|d| NaiveDateTime::new(*d, NaiveTime::from_hms_opt(0,0,0).unwrap()));
                                    cb(first);
                                }
                                if !inline { /* keep open for multi */ }
                            } else {
                                let new_dt = if allow_clear && sel {
                                    None
                                } else {
                                    Some(make_datetime(day))
                                };
                                commit_cell(new_dt);
                            }
                        })
                        .on(leptos::ev::keydown, move |ev: web_sys::KeyboardEvent| {
                            match ev.key().as_str() {
                                "Enter" | " " => {
                                    ev.prevent_default();
                                    if !is_day_disabled && is_cur_month {
                                        if !multiple {
                                            let new_dt = if allow_clear && sel { None } else { Some(make_datetime(day)) };
                                            commit_cell(new_dt);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        })
                        .child(leptos::html::span().attr("class", span_class).child(day_num))
                        .into_any()
                );
            }

            leptos::html::tr().child(cells).into_any()
        }).collect();

        // ── Footer ─────────────────────────────────────────────────────────────
        let footer_child: Option<AnyView> = footer_sv.get_value().map(|f| {
            leptos::html::div()
                .attr("class", "rz-datepicker-footer")
                .child(f())
                .into_any()
        });

        // ── Time picker ────────────────────────────────────────────────────────
        // Mirrors Blazor's rz-timepicker section.
        let time_picker_child: AnyView = if show_time || time_only {
            let hf = hour_format_popup.get_value();
            let is12 = hf == "12";

            // Displayed hour value (1-12 for 12h, 0-23 for 24h).
            let display_hour = move || -> u32 {
                let h = pending_hour.get();
                if is12 { if h == 0 { 12 } else if h > 12 { h - 12 } else { h } }
                else { h }
            };

            // Hour spinner.
            let hour_step = hours_step;
            let hour_max  = if is12 { 12u32 } else { 23 };
            let hour_min  = if is12 { 1u32 }  else { 0 };
            let pad_h     = pad_hours;

            let hour_el = leptos::html::div()
                .attr("class", "rz-hour-picker")
                .child(
                    leptos::html::button()
                        .attr("type", "button")
                        .attr("class", "rz-button rz-button-icon-only rz-variant-text rz-secondary")
                        .attr("tabindex", "-1")
                        .attr("disabled", disabled)
                        .on(leptos::ev::click, move |_| {
                            if disabled { return; }
                            let h = pending_hour.get_untracked();
                            let display = if is12 { if h == 0 { 12 } else if h > 12 { h - 12 } else { h } } else { h };
                            let next_display = if display >= hour_max { hour_min } else { display + hour_step };
                            let next_raw = if is12 {
                                if next_display == 12 { if h < 12 { 0 } else { 12 } } else if h < 12 { next_display } else { next_display + 12 }
                            } else { next_display };
                            pending_hour.set(next_raw % 24);
                        })
                        .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-up"))
                )
                .child(move || {
                    let v = display_hour();
                    let s = if pad_h { format!("{:02}", v) } else { format!("{}", v) };
                    leptos::html::span().attr("class", "rz-time-value").child(s)
                })
                .child(
                    leptos::html::button()
                        .attr("type", "button")
                        .attr("class", "rz-button rz-button-icon-only rz-variant-text rz-secondary")
                        .attr("tabindex", "-1")
                        .attr("disabled", disabled)
                        .on(leptos::ev::click, move |_| {
                            if disabled { return; }
                            let h = pending_hour.get_untracked();
                            let display = if is12 { if h == 0 { 12 } else if h > 12 { h - 12 } else { h } } else { h };
                            let next_display = if display <= hour_min { hour_max } else { display - hour_step };
                            let next_raw = if is12 {
                                if next_display == 12 { if h < 12 { 0 } else { 12 } } else if h < 12 { next_display } else { next_display + 12 }
                            } else { next_display };
                            pending_hour.set(next_raw % 24);
                        })
                        .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-down"))
                );

            // Minute spinner.
            let min_step = minutes_step;
            let pad_m    = pad_minutes;
            let minute_el = leptos::html::div()
                .attr("class", "rz-minute-picker")
                .child(
                    leptos::html::button()
                        .attr("type", "button")
                        .attr("class", "rz-button rz-button-icon-only rz-variant-text rz-secondary")
                        .attr("tabindex", "-1")
                        .attr("disabled", disabled)
                        .on(leptos::ev::click, move |_| {
                            if disabled { return; }
                            let m = pending_minute.get_untracked();
                            pending_minute.set((m + min_step) % 60);
                        })
                        .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-up"))
                )
                .child(move || {
                    let v = pending_minute.get();
                    let s = if pad_m { format!("{:02}", v) } else { format!("{}", v) };
                    leptos::html::span().attr("class", "rz-time-value").child(s)
                })
                .child(
                    leptos::html::button()
                        .attr("type", "button")
                        .attr("class", "rz-button rz-button-icon-only rz-variant-text rz-secondary")
                        .attr("tabindex", "-1")
                        .attr("disabled", disabled)
                        .on(leptos::ev::click, move |_| {
                            if disabled { return; }
                            let m = pending_minute.get_untracked();
                            pending_minute.set(if m < min_step { 60 - min_step } else { m - min_step });
                        })
                        .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-down"))
                );

            // Second spinner (optional).
            let sec_step = seconds_step;
            let pad_s    = pad_seconds;
            let second_el: AnyView = if show_seconds {
                leptos::html::div()
                    .attr("class", "rz-second-picker")
                    .child(
                        leptos::html::button()
                            .attr("type", "button")
                            .attr("class", "rz-button rz-button-icon-only rz-variant-text rz-secondary")
                            .attr("tabindex", "-1")
                            .attr("disabled", disabled)
                            .on(leptos::ev::click, move |_| {
                                if disabled { return; }
                                let s = pending_second.get_untracked();
                                pending_second.set((s + sec_step) % 60);
                            })
                            .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-up"))
                    )
                    .child(move || {
                        let v = pending_second.get();
                        let s = if pad_s { format!("{:02}", v) } else { format!("{}", v) };
                        leptos::html::span().attr("class", "rz-time-value").child(s)
                    })
                    .child(
                        leptos::html::button()
                            .attr("type", "button")
                            .attr("class", "rz-button rz-button-icon-only rz-variant-text rz-secondary")
                            .attr("tabindex", "-1")
                            .attr("disabled", disabled)
                            .on(leptos::ev::click, move |_| {
                                if disabled { return; }
                                let s = pending_second.get_untracked();
                                pending_second.set(if s < sec_step { 60 - sec_step } else { s - sec_step });
                            })
                            .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-down"))
                    )
                    .into_any()
            } else {
                ().into_any()
            };

            // AM/PM picker.
            let ampm_el: AnyView = if is12 {
                leptos::html::div()
                    .attr("class", "rz-ampm-picker")
                    .child(
                        leptos::html::button()
                            .attr("type", "button")
                            .attr("tabindex", if disabled { "-1" } else { "0" })
                            .attr("disabled", disabled)
                            .on(leptos::ev::click, toggle_ampm)
                            .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-up"))
                    )
                    .child(move || {
                        let h = pending_hour.get();
                        leptos::html::span().child(if h < 12 { "AM" } else { "PM" })
                    })
                    .child(
                        leptos::html::button()
                            .attr("type", "button")
                            .attr("tabindex", if disabled { "-1" } else { "0" })
                            .attr("disabled", disabled)
                            .on(leptos::ev::click, toggle_ampm)
                            .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-down"))
                    )
                    .into_any()
            } else {
                ().into_any()
            };

            // OK button.
            let ok_el: AnyView = if show_time_ok_button {
                leptos::html::button()
                    .attr("type", "button")
                    .attr("class", "rz-button rz-button-md rz-secondary")
                    .attr("tabindex", "0")
                    .on(leptos::ev::click, on_ok_click)
                    .child(leptos::html::span().attr("class", "rz-button-text").child("Ok"))
                    .into_any()
            } else {
                ().into_any()
            };

            // Separators between h:m and m:s.
            let sep = || leptos::html::div().attr("class", "rz-separator").child(leptos::html::span().child(":")).into_any();

            let mut time_children: Vec<AnyView> = Vec::new();
            if show_hour   { time_children.push(hour_el.into_any()); }
            if show_minutes {
                if show_hour { time_children.push(sep()); }
                time_children.push(minute_el.into_any());
            }
            if show_seconds {
                if show_minutes { time_children.push(sep()); }
                time_children.push(second_el);
            }
            if is12 { time_children.push(ampm_el); }
            if show_time_ok_button { time_children.push(ok_el); }

            leptos::html::div()
                .attr("class", "rz-timepicker")
                .child(time_children.into_iter().collect_view())
                .into_any()
        } else {
            ().into_any()
        };

        // ── Assemble calendar ──────────────────────────────────────────────────
        // Calendar section — only when !time_only.
        let calendar_section: AnyView = if !time_only {
            leptos::html::div()
                .attr("class", "rz-calendar")
                .child(
                    // Header.
                    leptos::html::div()
                        .attr("class", "rz-calendar-header")
                        .child(
                            leptos::html::button()
                                .attr("type", "button")
                                .attr("tabindex", "-1")
                                .attr("aria-label", "Previous month")
                                .attr("class", "rz-button rz-button-md rz-variant-text rz-button-icon-only rz-secondary rz-shade-default rz-calendar-prev")
                                .attr("disabled", disabled)
                                .on(leptos::ev::click, prev_month)
                                .child(leptos::html::span().attr("class", "notranslate rzi rz-calendar-prev-icon"))
                        )
                        .child(
                            leptos::html::button()
                                .attr("type", "button")
                                .attr("tabindex", "-1")
                                .attr("aria-label", "Next month")
                                .attr("class", "rz-button rz-button-md rz-variant-text rz-button-icon-only rz-secondary rz-shade-default rz-calendar-next")
                                .attr("disabled", disabled)
                                .on(leptos::ev::click, next_month)
                                .child(leptos::html::span().attr("class", "notranslate rzi rz-calendar-next-icon"))
                        )
                        .child(
                            leptos::html::div()
                                .attr("class", "rz-calendar-title")
                                .child(month_select)
                                .child(year_select)
                        )
                )
                // Day grid — only when show_days.
                .child(if show_days {
                    leptos::html::div()
                        .attr("class", "rz-calendar-view-container")
                        .attr("tabindex", effective_tab.to_string())
                        .child(
                            leptos::html::table()
                                .attr("class", "rz-calendar-view rz-calendar-month-view")
                                .child(leptos::html::thead().child(leptos::html::tr().child(header_cells)))
                                .child(leptos::html::tbody().child(rows))
                        )
                        .into_any()
                } else {
                    ().into_any()
                })
                // Footer.
                .child(footer_child)
                .into_any()
        } else {
            ().into_any()
        };

        leptos::html::div()
            .attr("class", container_class)
            .attr("style", popup_style)
            .on(leptos::ev::mousedown, |ev: web_sys::MouseEvent| ev.prevent_default())
            .child(calendar_section)
            .child(time_picker_child)
            .into_any()
    };

    // ── Final render ──────────────────────────────────────────────────────────
    Some(
        leptos::html::div()
            .attr("id", handle_id)
            .attr("class", root_class)
            .attr("style", style)
            .attr("tabindex", effective_tab.to_string())
            .on(leptos::ev::blur, on_blur)
            .on(leptos::ev::mouseenter, move |ev| enter_cb(ev))
            .on(leptos::ev::mouseleave, move |ev| leave_cb(ev))
            .on(leptos::ev::contextmenu, move |ev| ctx_cb(ev))
            .child(input_section)
            .child(popup_child),
    )
    .into_any()
}