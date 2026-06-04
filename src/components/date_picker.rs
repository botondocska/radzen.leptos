//! RadzenDatePicker component — mirrors C# Radzen.Blazor.RadzenDatePicker<TValue>.
//!
//! # Value type
//! In Blazor, TValue is generic (DateTime, DateTime?, DateOnly, DateOnly?, etc.).
//! In Rust we use `Option<NaiveDate>` for date-only and `Option<NaiveDateTime>`
//! when `show_time = true`. The public `value` prop is `RwSignal<Option<NaiveDate>>`;
//! time components are stored in separate pending signals and combined internally.
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
use chrono::{Datelike, Duration, NaiveDate, Timelike};
use leptos::prelude::*;
use std::sync::Arc;

// ─────────────────────────────────────────────────────────────────────────────
// DateRenderEventArgs — mirrors Blazor DateRenderEventArgs
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
/// Supported tokens: `yyyy`, `yy`, `MMMM`, `MMM`, `MM`, `M`, `dd`, `d`.
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

/// Try to parse a date string. Tries several common formats.
fn parse_date_input(s: &str) -> Option<NaiveDate> {
    let s = s.trim();
    for fmt in &["%Y-%m-%d", "%m/%d/%Y", "%d/%m/%Y", "%d.%m.%Y", "%Y.%m.%d",
                 "%-m/%-d/%Y", "%m/%d/%Y", "%Y/%m/%d"] {
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

/// ISO 8601 week number.
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
/// A calendar-based date picker. Set `show_time=true` to also display a time
/// picker below the calendar. The public value type is `Option<NaiveDate>`;
/// when time is needed the caller can use `on_change` with the date and read
/// the hour/minute from separate signals.
#[component]
pub fn RadzenDatePicker(
    // ── Base ──────────────────────────────────────────────────────────────────
    #[prop(default = Default::default())]
    base: ComponentProps,

    // ── Value ─────────────────────────────────────────────────────────────────
    /// Currently selected date. `None` = empty / unselected.
    #[prop(optional)]
    value: Option<RwSignal<Option<NaiveDate>>>,

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
    #[prop(default = true)]
    show_days: bool,

    /// Show ISO week number column. Default: `false`.
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

    /// Year range string like `"1900:2100"`.
    #[prop(default = "1900:2100".to_string(), into)]
    year_range: String,

    // ── Clear ─────────────────────────────────────────────────────────────────
    /// Show the × clear button when a value is set. Default: `true`.
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
    /// Which month to show when the picker opens.
    #[prop(default = None)]
    initial_view_date: Option<NaiveDate>,

    // ── Per-day customisation ─────────────────────────────────────────────────
    /// Called for every calendar cell. Mutate the args to disable a date or add CSS.
    #[prop(default = None)]
    date_render: Option<Arc<dyn Fn(NaiveDate) -> DateRenderEventArgs + Send + Sync>>,

    // ── Footer ────────────────────────────────────────────────────────────────
    /// Custom content rendered below the calendar grid inside the popup.
    #[prop(optional)]
    footer_template: Option<ChildrenFn>,

    // ── HTML ──────────────────────────────────────────────────────────────────
    /// `name` attribute for the input.
    #[prop(default = None, into)]
    name: Option<String>,

    /// Tab order. Forced to `-1` when disabled.
    #[prop(default = 0)]
    tab_index: i32,

    /// Extra HTML attributes spread onto the `<input>` element.
    /// Mirrors Blazor's `InputAttributes` parameter.
    #[prop(default = None)]
    input_attributes: Option<std::collections::HashMap<String, String>>,

    /// Extra CSS class(es) appended to the trigger calendar button.
    /// Mirrors Blazor's `ButtonClass` parameter.
    #[prop(default = String::new(), into)]
    button_class: String,

    // ── Callbacks ─────────────────────────────────────────────────────────────
    /// Called when the selected date changes.
    #[prop(default = None)]
    on_change: Option<Arc<dyn Fn(Option<NaiveDate>) + Send + Sync>>,
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
        .or_else(|| value_signal.get_untracked())
        .unwrap_or(today);
    let view_year  = RwSignal::new(init_view.year());
    let view_month = RwSignal::new(init_view.month());

    // ── Pending time state (hour/minute/second spinners) ──────────────────────
    let pending_hour   = RwSignal::new(0u32);
    let pending_minute = RwSignal::new(0u32);
    let pending_second = RwSignal::new(0u32);

    // ── Keyboard-focused day inside the calendar grid ─────────────────────────
    // None = no keyboard focus yet. Arrow keys move this; Enter/Space selects it.
    // Mirrors Blazor's OnCalendarKeyPress / shouldFocusDay mechanism.
    let focused_day: RwSignal<Option<NaiveDate>> = RwSignal::new(None);

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
    let trigger_button_class = {
        let base = format!(
            "rz-datepicker-trigger{} rz-button rz-button-icon-only{}",
            if show_input { " rz-datepicker-field-button" } else { "" },
            if disabled   { " rz-state-disabled"          } else { "" },
        );
        if button_class.is_empty() { base } else { format!("{} {}", base, button_class) }
    };

    let style     = base.style.clone().unwrap_or_default();
    let handle_id = handle.id.clone();

    // ── Formatted display value ───────────────────────────────────────────────
    let date_format_sv = StoredValue::new(date_format.clone());

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
            Some(d) => format_date_str(d, &date_format_sv.get_value()),
        }
    };

    // ── on_change stored so FnMut closures can clone it ───────────────────────
    // StoredValue + Arc avoids the FnOnce problem: every closure clones the Arc.
    let on_change_sv: StoredValue<Option<Arc<dyn Fn(Option<NaiveDate>) + Send + Sync>>> =
        StoredValue::new(on_change);

    // ── Commit a new date value ───────────────────────────────────────────────
    let commit = Arc::new(move |new_date: Option<NaiveDate>| {
        value_signal.set(new_date);
        if let Some(cb) = on_change_sv.get_value() {
            cb(new_date);
        }
        if !inline { open.set(false); }
    });

    // ── Toggle open ───────────────────────────────────────────────────────────
    let toggle_open = move |_ev: web_sys::MouseEvent| {
        if disabled || read_only || inline { return; }
        let is_open = open.get_untracked();
        if !is_open {
            let d = value_signal.get_untracked().unwrap_or(today);
            view_year.set(d.year());
            view_month.set(d.month());
        }
        open.set(!is_open);
    };

    let on_blur = move |_ev: web_sys::FocusEvent| {
        if !inline {
            gloo_timers::callback::Timeout::new(150, move || { open.set(false); }).forget();
        }
    };

    // ── Month navigation ──────────────────────────────────────────────────────
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
            commit_parse(Some(date));
            view_year.set(date.year());
            view_month.set(date.month());
        }
    });

    // ── Clear ─────────────────────────────────────────────────────────────────
    let commit_clear = commit.clone();
    let on_clear = move |ev: web_sys::MouseEvent| {
        ev.stop_propagation();
        commit_clear(None);
        multi_signal.set(Vec::new());
    };

    // commit_ok is stored so popup_child (FnMut) can clone it on each invocation.
    let commit_ok_sv = StoredValue::new(commit.clone());

    // ── Base events ───────────────────────────────────────────────────────────
    let enter_cb = handle.on_mouse_enter.clone();
    let leave_cb = handle.on_mouse_leave.clone();
    let ctx_cb   = handle.on_context_menu.clone();

    let day_names = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

    // ── Year / month lists ────────────────────────────────────────────────────
    let years: Vec<i32> = (year_from..=year_to).collect();
    let months_list: Vec<(u32, &'static str)> = (1u32..=12).map(|m| (m, month_abbr(m))).collect();

    // ── has_value helper ──────────────────────────────────────────────────────
    let has_value = move || -> bool {
        if multiple { !multi_signal.get().is_empty() }
        else { value_signal.get().is_some() }
    };

    // ── Input section (static — not reactive) ────────────────────────────────
    let input_section: AnyView = if inline {
        ().into_any()
    } else {
        let input_class_c       = input_class.clone();
        let trigger_class_c     = trigger_button_class.clone();
        let name_c              = name.clone();
        let tab_str             = if disabled { "-1".to_string() } else { effective_tab.to_string() };
        let placeholder_c       = placeholder.clone().unwrap_or_default();
        let parse_input         = parse_and_commit.clone();
        let parse_imm           = parse_and_commit.clone();
        let toggle_for_inp      = toggle_open;

        // InputAttributes spread — applied imperatively via NodeRef after mount.
        let extra_input_attrs: Vec<(String, String)> = input_attributes
            .as_ref()
            .map(|a| a.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default();
        let input_node_ref = NodeRef::<leptos::html::Input>::new();
        if !extra_input_attrs.is_empty() {
            let attrs_clone = extra_input_attrs.clone();
            Effect::new(move |_| {
                if let Some(el) = input_node_ref.get() {
                    use web_sys::wasm_bindgen::JsCast;
                    if let Some(el) = el.dyn_ref::<web_sys::HtmlElement>() {
                        for (k, v) in &attrs_clone {
                            el.set_attribute(k, v).ok();
                        }
                    }
                }
            });
        }

        let input_el: AnyView = if show_input || !show_button {
            leptos::html::input()
                .node_ref(input_node_ref)
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

        let button_el: AnyView = if show_button {
            leptos::html::button()
                .attr("type", "button")
                .attr("class", trigger_class_c)
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

        let clear_el: AnyView = if allow_clear && (show_input || !show_button) {
            leptos::html::button()
                .attr("type", "button")
                .attr("class", "notranslate rz-dropdown-clear-icon rzi rzi-times")
                .attr("aria-label", "Clear")
                .on(leptos::ev::click, on_clear)
                .attr("style", move || if has_value() { String::new() } else { "display:none".to_string() })
                .into_any()
        } else {
            ().into_any()
        };

        vec![input_el, button_el, clear_el]
            .into_iter()
            .collect_view()
            .into_any()
    };

    // ── Popup / inline calendar — FnMut closure ───────────────────────────────
    // All captures are either Copy (RwSignal, bool, i32, u32) or StoredValue / Arc.
    let commit_popup        = commit.clone();
    let date_render_sv      = StoredValue::new(date_render);
    let footer_sv           = StoredValue::new(footer_template);
    let hour_format_sv      = StoredValue::new(hour_format.clone());
    let cal_week_title_sv   = StoredValue::new(calendar_week_title.clone());

    // on_change for the multiple-mode day click (stored separately).
    let on_change_multi_sv: StoredValue<Option<Arc<dyn Fn(Option<NaiveDate>) + Send + Sync>>> =
        StoredValue::new(on_change_sv.get_value());

    let popup_child = move || -> AnyView {
        if !open.get() {
            return ().into_any();
        }

        let year   = view_year.get();
        let month  = view_month.get();
        let weeks  = build_calendar_weeks(year, month);
        let commit_c = commit_popup.clone();

        let container_class = if inline { "rz-datepicker-inline-container" }
                              else      { "rz-datepicker-popup-container" };
        let popup_style = if inline { String::new() }
            else { "position:absolute;z-index:var(--rz-popup-z-index,1000);left:0;top:100%;min-width:100%".to_string() };

        // is_selected helper.
        let is_selected = move |date: NaiveDate| -> bool {
            if multiple {
                multi_signal.get().iter().any(|d| *d == date)
            } else {
                value_signal.get().map_or(false, |sel| sel == date)
            }
        };

        // Header day-name cells.
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

        // Month / year dropdowns.
        let month_opts: Vec<AnyView> = months_list.iter().map(|(v, label)| {
            let v = *v;
            leptos::html::option()
                .attr("value", v.to_string())
                .attr("selected", v == month)
                .child(*label)
                .into_any()
        }).collect();
        let year_opts: Vec<AnyView> = years.iter().map(|&y| {
            leptos::html::option()
                .attr("value", y.to_string())
                .attr("selected", y == year)
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

        // Calendar rows.
        let rows: Vec<AnyView> = weeks.into_iter().map(|week| {
            let commit_row = commit_c.clone();
            let mut cells: Vec<AnyView> = Vec::new();

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
                let commit_cell = commit_row.clone();
                let is_cur_month = day.year() == year && day.month() == month;

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
                let is_today  = day == today;
                let sel       = is_selected(day);

                let td_class = if !is_cur_month {
                    match &args.attributes {
                        Some(extra) => format!("rz-datepicker-other-month {}", extra),
                        None => "rz-datepicker-other-month".to_string(),
                    }
                } else {
                    args.attributes.clone().unwrap_or_default()
                };

                let span_class = ClassList::create("rz-state-default")
                    .add("rz-state-active",    sel)
                    .add("rz-datepicker-today", is_today && is_cur_month)
                    .add_disabled(is_day_disabled || !is_cur_month)
                    .finish();

                let day_num = day.day().to_string();
                let tab = if is_day_disabled || !is_cur_month { "-1" } else { "0" };

                // on_change_multi_sv cloned for this cell's closure.
                let on_change_cell = on_change_multi_sv.get_value();

                // commit_cell is Arc — clone so both click and keydown can own a copy.
                let commit_cell_click = commit_cell.clone();
                let commit_cell_key   = commit_cell;

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
                                if let Some(ref cb) = on_change_cell {
                                    let first = multi_signal.get().first().copied();
                                    cb(first);
                                }
                            } else {
                                let new_date = if allow_clear && sel { None } else { Some(day) };
                                commit_cell_click(new_date);
                            }
                        })
                        .on(leptos::ev::keydown, move |ev: web_sys::KeyboardEvent| {
                            match ev.key().as_str() {
                                "Enter" | " " => {
                                    ev.prevent_default();
                                    if !is_day_disabled && is_cur_month && !multiple {
                                        let new_date = if allow_clear && sel { None } else { Some(day) };
                                        commit_cell_key(new_date);
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

        // Footer.
        let footer_child: Option<AnyView> = footer_sv.get_value().map(|f| {
            leptos::html::div()
                .attr("class", "rz-datepicker-footer")
                .child(f())
                .into_any()
        });

        // ── Time picker ────────────────────────────────────────────────────────
        let time_picker_child: AnyView = if show_time || time_only {
            let hf   = hour_format_sv.get_value();
            let is12 = hf == "12";

            let display_hour = move || -> u32 {
                let h = pending_hour.get();
                if is12 { if h == 0 { 12 } else if h > 12 { h - 12 } else { h } }
                else { h }
            };

            // Hour spinner.
            let hour_max = if is12 { 12u32 } else { 23 };
            let hour_min = if is12 { 1u32 }  else { 0 };
            let pad_h    = pad_hours;

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
                            let next_display = if display >= hour_max { hour_min } else { display + hours_step };
                            let next_raw = if is12 {
                                if next_display == 12 { if h < 12 { 0 } else { 12 } }
                                else if h < 12 { next_display } else { next_display + 12 }
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
                            let next_display = if display <= hour_min { hour_max } else { display - hours_step };
                            let next_raw = if is12 {
                                if next_display == 12 { if h < 12 { 0 } else { 12 } }
                                else if h < 12 { next_display } else { next_display + 12 }
                            } else { next_display };
                            pending_hour.set(next_raw % 24);
                        })
                        .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-down"))
                );

            // Minute spinner.
            let pad_m = pad_minutes;
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
                            pending_minute.set((m + minutes_step) % 60);
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
                            pending_minute.set(if m < minutes_step { 60 - minutes_step } else { m - minutes_step });
                        })
                        .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-down"))
                );

            // Second spinner.
            let pad_s = pad_seconds;
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
                                pending_second.set((s + seconds_step) % 60);
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
                                pending_second.set(if s < seconds_step { 60 - seconds_step } else { s - seconds_step });
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
                            .on(leptos::ev::click, move |_: web_sys::MouseEvent| {
                                if disabled { return; }
                                let h = pending_hour.get_untracked();
                                pending_hour.set((h + 12) % 24);
                            })
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
                            .on(leptos::ev::click, move |_: web_sys::MouseEvent| {
                                if disabled { return; }
                                let h = pending_hour.get_untracked();
                                pending_hour.set((h + 12) % 24);
                            })
                            .child(leptos::html::span().attr("class", "notranslate rzi rzi-chevron-down"))
                    )
                    .into_any()
            } else {
                ().into_any()
            };

            // OK button — commit_ok_sv cloned each invocation so popup_child stays FnMut.
            let ok_el: AnyView = if show_time_ok_button {
                let commit_ok = commit_ok_sv.get_value();
                leptos::html::button()
                    .attr("type", "button")
                    .attr("class", "rz-button rz-button-md rz-secondary")
                    .attr("tabindex", "0")
                    .on(leptos::ev::click, move |_: web_sys::MouseEvent| {
                        let current_date = value_signal.get_untracked().unwrap_or(today);
                        commit_ok(Some(current_date));
                    })
                    .child(leptos::html::span().attr("class", "rz-button-text").child("Ok"))
                    .into_any()
            } else {
                ().into_any()
            };

            let sep = || leptos::html::div()
                .attr("class", "rz-separator")
                .child(leptos::html::span().child(":"))
                .into_any();

            let mut time_children: Vec<AnyView> = Vec::new();
            if show_hour    { time_children.push(hour_el.into_any()); }
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
        let calendar_section: AnyView = if !time_only {
            leptos::html::div()
                .attr("class", "rz-calendar")
                .child(
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