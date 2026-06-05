use leptos::prelude::*;
use chrono::NaiveDate;
use std::collections::HashMap;

use crate::components::{
    AlertStyle, BadgeStyle, ButtonSize, ButtonStyle, ComponentProps, Orientation,
    RadzenAlert, RadzenBadge, RadzenButton, RadzenCard, RadzenCheckBox,
     RadzenIcon, RadzenLabel,RadzenLink, RadzenNumeric, RadzenPassword,
    RadzenSelectBar, RadzenSelectBarItem, RadzenStack, RadzenText, RadzenTextArea, RadzenTextBox,
    Shade, TextAlign, TextStyle, Variant,
};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    // ── Timed-alert demo state ────────────────────────────────────────────────
    let show_success = RwSignal::new(false);
    let show_danger  = RwSignal::new(false);
    let show_info    = RwSignal::new(false);

    let description = RwSignal::new(String::new());
    let live_search = RwSignal::new(String::new());
    let username    = RwSignal::new(String::new());
    let notes       = RwSignal::new(String::new());
    let password    = RwSignal::new(String::new());

    fn trigger(signal: RwSignal<bool>, ms: u32) {
        signal.set(true);
        gloo_timers::callback::Timeout::new(ms, move || signal.set(false)).forget();
    }

    // ── CheckBox signals ──────────────────────────────────────────────────────
    let chk_single   = RwSignal::new(Some(false));
    let chk_tri      = RwSignal::new(None::<bool>);
    let chk_disabled = RwSignal::new(Some(true));
    let chk_readonly = RwSignal::new(Some(true));

    // ── DatePicker signals ────────────────────────────────────────────────────
    let date_basic       = RwSignal::new(None::<NaiveDate>);
    let date_bound       = RwSignal::new(Some(NaiveDate::from_ymd_opt(2025, 6, 15).unwrap()));
    let date_inline      = RwSignal::new(None::<NaiveDate>);
    let date_range_start = RwSignal::new(None::<NaiveDate>);
    let date_range_end   = RwSignal::new(None::<NaiveDate>);

    // ── Numeric signals ───────────────────────────────────────────────────────
    let num_basic    = RwSignal::new(None::<f64>);
    let num_bound    = RwSignal::new(Some(42.0_f64));
    let num_clamped  = RwSignal::new(Some(5.0_f64));
    let num_decimal  = RwSignal::new(Some(3.14_f64));
    let num_currency = RwSignal::new(Some(1999.99_f64));

    // ── SelectBar signals ─────────────────────────────────────────────────────
    let sb_view     = RwSignal::new("list".to_string());
    let sb_multi    = RwSignal::new(vec!["bold".to_string()]);
    let sb_operator = RwSignal::new("And".to_string());

    // ── DropDown signals ──────────────────────────────────────────────────────
    let dd_single   = RwSignal::new(String::new());
    let dd_prefilled = RwSignal::new("2".to_string());
    let dd_multi    = RwSignal::new(Vec::<String>::new());
    let dd_filtered = RwSignal::new(String::new());

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>
                <ul>
                    {move || errors.get().into_iter()
                        .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                        .collect_view()}
                </ul>
            }
        }>
        <div class="container">

        // ── Timed Alert Demo ──────────────────────────────────────────────────
        <h2>"Timed Alert Demo"</h2>
        <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
            "Click a button — the matching alert appears for 3 seconds then dismisses itself."
        </p>
        <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center; margin-bottom: 1rem;">
            <RadzenButton text="Show Success".to_string() button_style=ButtonStyle::Success icon=Some("check_circle".to_string())
                on_click=Some(std::sync::Arc::new(move |_ev| Box::pin(async move { trigger(show_success, 3000); }))) />
            <RadzenButton text="Show Error".to_string() button_style=ButtonStyle::Danger icon=Some("error".to_string())
                on_click=Some(std::sync::Arc::new(move |_ev| Box::pin(async move { trigger(show_danger, 3000); }))) />
            <RadzenButton text="Show Info".to_string() button_style=ButtonStyle::Info icon=Some("info".to_string())
                on_click=Some(std::sync::Arc::new(move |_ev| Box::pin(async move { trigger(show_info, 3000); }))) />
        </div>
        <div style="display: flex; flex-direction: column; gap: 0.5rem; min-height: 3rem;">
            <Show when=move || show_success.get()>
                <RadzenAlert alert_style=AlertStyle::Success title=Some("Success!".to_string())
                    text=Some("The operation completed successfully.".to_string())
                    on_close=Some(std::sync::Arc::new(move || show_success.set(false))) />
            </Show>
            <Show when=move || show_danger.get()>
                <RadzenAlert alert_style=AlertStyle::Danger title=Some("Error!".to_string())
                    text=Some("Something went wrong. Please try again.".to_string())
                    on_close=Some(std::sync::Arc::new(move || show_danger.set(false))) />
            </Show>
            <Show when=move || show_info.get()>
                <RadzenAlert alert_style=AlertStyle::Info variant=Variant::Flat shade=Shade::Lighter
                    title=Some("Did you know?".to_string())
                    text=Some("This alert will dismiss itself after 3 seconds.".to_string())
                    on_close=Some(std::sync::Arc::new(move || show_info.set(false))) />
            </Show>
        </div>

        // ── Buttons ───────────────────────────────────────────────────────────
        <h2 style="margin-top: 2rem;">"Buttons"</h2>
        <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
            <RadzenButton text="Primary".to_string()  button_style=ButtonStyle::Primary  size=ButtonSize::ExtraSmall variant=Variant::Filled  shade=Shade::Dark />
            <RadzenButton text="Save".to_string()     button_style=ButtonStyle::Success  size=ButtonSize::Small     variant=Variant::Flat     shade=Shade::Light />
            <RadzenButton text="Delete".to_string()   button_style=ButtonStyle::Danger   size=ButtonSize::Medium    variant=Variant::Outlined shade=Shade::Darker
                base=ComponentProps { attrs: Some(HashMap::from([("class".to_string(), "rz-border-radius-10".to_string())])), ..Default::default() } />
            <RadzenButton text="Cancel".to_string()   button_style=ButtonStyle::Secondary size=ButtonSize::Large    variant=Variant::Text     shade=Shade::Lighter />
        </div>

        // ── Badges ────────────────────────────────────────────────────────────
        <h2 style="margin-top: 2rem;">"Badges — Styles"</h2>
        <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center;">
            <RadzenBadge text=Some("Primary".to_string())   badge_style=BadgeStyle::Primary />
            <RadzenBadge text=Some("Secondary".to_string()) badge_style=BadgeStyle::Secondary />
            <RadzenBadge text=Some("Success".to_string())   badge_style=BadgeStyle::Success />
            <RadzenBadge text=Some("Danger".to_string())    badge_style=BadgeStyle::Danger />
            <RadzenBadge text=Some("Warning".to_string())   badge_style=BadgeStyle::Warning />
            <RadzenBadge text=Some("Info".to_string())      badge_style=BadgeStyle::Info />
            <RadzenBadge text=Some("Light".to_string())     badge_style=BadgeStyle::Light />
            <RadzenBadge text=Some("Dark".to_string())      badge_style=BadgeStyle::Dark />
            <RadzenBadge text=Some("Base".to_string())      badge_style=BadgeStyle::Base />
        </div>

        <h2 style="margin-top: 2rem;">"Badges — Variants"</h2>
        <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center;">
            <RadzenBadge text=Some("Filled".to_string())   badge_style=BadgeStyle::Danger variant=Variant::Filled />
            <RadzenBadge text=Some("Flat".to_string())     badge_style=BadgeStyle::Danger variant=Variant::Flat />
            <RadzenBadge text=Some("Outlined".to_string()) badge_style=BadgeStyle::Danger variant=Variant::Outlined />
            <RadzenBadge text=Some("Text".to_string())     badge_style=BadgeStyle::Danger variant=Variant::Text />
        </div>

        <h2 style="margin-top: 2rem;">"Badges — Shades"</h2>
        <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center;">
            <RadzenBadge text=Some("Lighter".to_string()) badge_style=BadgeStyle::Info shade=Shade::Lighter />
            <RadzenBadge text=Some("Light".to_string())   badge_style=BadgeStyle::Info shade=Shade::Light />
            <RadzenBadge text=Some("Default".to_string()) badge_style=BadgeStyle::Info shade=Shade::Default />
            <RadzenBadge text=Some("Dark".to_string())    badge_style=BadgeStyle::Info shade=Shade::Dark />
            <RadzenBadge text=Some("Darker".to_string())  badge_style=BadgeStyle::Info shade=Shade::Darker />
        </div>

        <h2 style="margin-top: 2rem;">"Badges — Pill Shape"</h2>
        <div style="display: flex; gap: 0.75rem; flex-wrap: wrap; align-items: center;">
            <RadzenBadge text=Some("Rectangular".to_string()) badge_style=BadgeStyle::Primary />
            <RadzenBadge text=Some("Pill".to_string())        badge_style=BadgeStyle::Primary is_pill=true />
            <RadzenBadge text=Some("3".to_string())           badge_style=BadgeStyle::Danger  is_pill=true />
            <RadzenBadge text=Some("12".to_string())          badge_style=BadgeStyle::Warning is_pill=true />
            <RadzenBadge text=Some("99+".to_string())         badge_style=BadgeStyle::Success is_pill=true />
        </div>

        // ── Cards ─────────────────────────────────────────────────────────────
        <h2 style="margin-top: 2rem;">"Cards"</h2>
        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; margin-bottom: 2rem;">
            <RadzenCard variant=Variant::Filled>
                <div style="padding: 1rem;"><h3>"Card Title"</h3><p>"Filled card."</p></div>
            </RadzenCard>
            <RadzenCard variant=Variant::Outlined>
                <div style="padding: 1rem;"><h3>"Outlined Card"</h3><p>"Outlined variant."</p></div>
            </RadzenCard>
            <RadzenCard variant=Variant::Flat>
                <div style="padding: 1rem;"><h3>"Flat Card"</h3><p>"Flat variant."</p></div>
            </RadzenCard>
        </div>

        // ── Icons ─────────────────────────────────────────────────────────────
        <h2 style="margin-top: 2rem;">"Icons — Basic"</h2>
        <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
            <RadzenIcon icon=Some("home".to_string()) />
            <RadzenIcon icon=Some("settings".to_string()) />
            <RadzenIcon icon=Some("account_circle".to_string()) />
            <RadzenIcon icon=Some("check_circle".to_string()) />
            <RadzenIcon icon=Some("notifications".to_string()) />
            <RadzenIcon icon=Some("favorite".to_string()) />
        </div>

        <h2 style="margin-top: 2rem;">"Icons — Colors"</h2>
        <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
            <RadzenIcon icon=Some("home".to_string())         icon_color=Some("var(--rz-primary)".to_string()) />
            <RadzenIcon icon=Some("check_circle".to_string()) icon_color=Some("var(--rz-success)".to_string()) />
            <RadzenIcon icon=Some("warning".to_string())      icon_color=Some("var(--rz-warning)".to_string()) />
            <RadzenIcon icon=Some("error".to_string())        icon_color=Some("var(--rz-danger)".to_string()) />
        </div>

        // ── Text ──────────────────────────────────────────────────────────────
        <h2 style="margin-top: 2rem;">"Text — Styles"</h2>
        <div style="display: flex; flex-direction: column; gap: 0.5rem;">
            <RadzenText text_style=TextStyle::H1 text=Some("H1 – Heading 1".to_string()) />
            <RadzenText text_style=TextStyle::H2 text=Some("H2 – Heading 2".to_string()) />
            <RadzenText text_style=TextStyle::H3 text=Some("H3 – Heading 3".to_string()) />
            <RadzenText text_style=TextStyle::Body1 text=Some("Body1 — default paragraph.".to_string()) />
            <RadzenText text_style=TextStyle::Body2 text=Some("Body2 — smaller paragraph.".to_string()) />
            <RadzenText text_style=TextStyle::Caption text=Some("Caption — descriptive text.".to_string()) />
        </div>

        // ── Stack ─────────────────────────────────────────────────────────────
        <h2 style="margin-top: 2rem;">"Stack — Vertical"</h2>
        <RadzenStack gap=Some("0.5rem".to_string())>
            <RadzenBadge text=Some("First".to_string())  badge_style=BadgeStyle::Primary />
            <RadzenBadge text=Some("Second".to_string()) badge_style=BadgeStyle::Secondary />
            <RadzenBadge text=Some("Third".to_string())  badge_style=BadgeStyle::Info />
        </RadzenStack>

        <h2 style="margin-top: 2rem;">"Stack — Horizontal"</h2>
        <RadzenStack orientation=Orientation::Horizontal gap=Some("1rem".to_string())>
            <RadzenBadge text=Some("First".to_string())  badge_style=BadgeStyle::Primary />
            <RadzenBadge text=Some("Second".to_string()) badge_style=BadgeStyle::Secondary />
            <RadzenBadge text=Some("Third".to_string())  badge_style=BadgeStyle::Info />
        </RadzenStack>

        // ── Links ─────────────────────────────────────────────────────────────
        <h2 style="margin-top: 2rem;">"Links"</h2>
        <div style="display: flex; gap: 1rem; flex-wrap: wrap; align-items: center;">
            <RadzenLink path="/" text="Home" />
            <RadzenLink path="/" text="Disabled" disabled=true />
            <RadzenLink path="/" text="With Icon" icon=Some("home".to_string()) />
        </div>

        // ── Label ─────────────────────────────────────────────────────────────
        <h2 style="margin-top: 2rem;">"Labels"</h2>
        <div style="display: flex; flex-direction: column; gap: 0.5rem;">
            <RadzenLabel text=Some("Email address".to_string()) component=Some("email_input".to_string()) />
            <RadzenLabel>
                "Required field "
                <span style="color: var(--rz-danger);">"*"</span>
            </RadzenLabel>
        </div>

        // ── Alert ─────────────────────────────────────────────────────────────
        <h2 style="margin-top: 2rem;">"Alerts"</h2>
        <div style="display: flex; flex-direction: column; gap: 0.5rem;">
            <RadzenAlert alert_style=AlertStyle::Info    title=Some("Info".to_string())    text=Some("An informational message.".to_string()) />
            <RadzenAlert alert_style=AlertStyle::Success title=Some("Success".to_string()) text=Some("Changes saved.".to_string()) variant=Variant::Flat />
            <RadzenAlert alert_style=AlertStyle::Warning title=Some("Warning".to_string()) text=Some("Proceed with caution.".to_string()) allow_close=false />
            <RadzenAlert alert_style=AlertStyle::Danger  title=Some("Error".to_string())   text=Some("Something went wrong.".to_string()) variant=Variant::Outlined />
        </div>

        // ── TextBox ───────────────────────────────────────────────────────────
        <h2 style="margin-top: 2rem;">"TextBox"</h2>
        <div style="display: flex; flex-direction: column; gap: 0.75rem; max-width: 320px;">
            <RadzenTextBox placeholder=Some("Basic textbox".to_string()) />
            <RadzenTextBox value=username placeholder=Some("Bound value".to_string()) />
            <RadzenTextBox value=live_search immediate=true placeholder=Some("Live (immediate)".to_string()) />
            <RadzenTextBox value=RwSignal::new("Disabled".to_string()) disabled=true />
            <RadzenTextBox value=RwSignal::new("ReadOnly".to_string()) read_only=true />
        </div>

        // ── TextArea ──────────────────────────────────────────────────────────
        <h2 style="margin-top: 2rem;">"TextArea"</h2>
        <div style="display: flex; flex-direction: column; gap: 0.75rem; max-width: 500px;">
            <RadzenTextArea placeholder=Some("Basic textarea".to_string()) />
            <RadzenTextArea value=description rows=4 placeholder=Some("Bound value".to_string()) />
            <RadzenTextArea value=notes rows=5 immediate=true placeholder=Some("Live (immediate)".to_string()) />
        </div>

        // ── Password ──────────────────────────────────────────────────────────
        <h2 style="margin-top: 2rem;">"Password"</h2>
        <div style="display: flex; flex-direction: column; gap: 0.75rem; max-width: 320px;">
            <RadzenPassword placeholder=Some("Enter password".to_string()) />
            <RadzenPassword immediate=true value=password placeholder=Some("Live bound".to_string()) />
            <RadzenPassword value=RwSignal::new("disabled_value".to_string()) disabled=true />
        </div>

        // ════════════════════════════════════════════════════════════════════════
        // CheckBox
        // ════════════════════════════════════════════════════════════════════════

        <h2 style="margin-top: 2rem;">"CheckBox — Basic"</h2>
        <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
            "Click the box to toggle between checked and unchecked."
        </p>
        <div style="display: flex; gap: 1.5rem; flex-wrap: wrap; align-items: center;">
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <RadzenCheckBox value=chk_single />
                <RadzenText text_style=TextStyle::Body2>
                    {move || match chk_single.get() {
                        Some(true)  => "Checked",
                        Some(false) => "Unchecked",
                        None        => "—",
                    }}
                </RadzenText>
            </div>
        </div>

        <h2 style="margin-top: 2rem;">"CheckBox — Tri-state"</h2>
        <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
            "Cycles false → indeterminate → true → false."
        </p>
        <div style="display: flex; gap: 1.5rem; flex-wrap: wrap; align-items: center;">
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <RadzenCheckBox value=chk_tri tri_state=true />
                <RadzenText text_style=TextStyle::Body2>
                    {move || match chk_tri.get() {
                        Some(true)  => "true",
                        Some(false) => "false",
                        None        => "indeterminate",
                    }}
                </RadzenText>
            </div>
        </div>

        <h2 style="margin-top: 2rem;">"CheckBox — Disabled and ReadOnly"</h2>
        <div style="display: flex; gap: 2rem; flex-wrap: wrap; align-items: center;">
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <RadzenCheckBox value=chk_disabled disabled=true />
                <RadzenLabel text=Some("Disabled (checked)".to_string()) />
            </div>
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <RadzenCheckBox value=chk_readonly read_only=true />
                <RadzenLabel text=Some("ReadOnly (checked)".to_string()) />
            </div>
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <RadzenCheckBox value=RwSignal::new(Some(false)) disabled=true />
                <RadzenLabel text=Some("Disabled (unchecked)".to_string()) />
            </div>
        </div>

        <h2 style="margin-top: 2rem;">"CheckBox — onChange Callback"</h2>
        <div style="display: flex; align-items: center; gap: 0.5rem;">
            <RadzenCheckBox
                on_change=Some(std::sync::Arc::new(|v| {
                    log::info!("CheckBox changed: {:?}", v);
                }))
            />
            <RadzenLabel text=Some("Check me (watch the browser console)".to_string()) />
        </div>

        <h2 style="margin-top: 2rem;">"CheckBox — Agreement Form"</h2>
        <RadzenCard variant=Variant::Outlined>
            <div style="padding: 1rem; display: flex; flex-direction: column; gap: 0.75rem; max-width: 360px;">
                <div style="display: flex; align-items: center; gap: 0.75rem;">
                    <RadzenCheckBox value=RwSignal::new(Some(true)) />
                    <RadzenLabel text=Some("I agree to the terms and conditions".to_string()) />
                </div>
                <div style="display: flex; align-items: center; gap: 0.75rem;">
                    <RadzenCheckBox />
                    <RadzenLabel text=Some("Subscribe to newsletter".to_string()) />
                </div>
                <div style="display: flex; align-items: center; gap: 0.75rem;">
                    <RadzenCheckBox />
                    <RadzenLabel text=Some("Remember me on this device".to_string()) />
                </div>
                <RadzenButton text="Submit".to_string() button_style=ButtonStyle::Primary />
            </div>
        </RadzenCard>

        <h2 style="margin-top: 2rem;">"CheckBox — Visibility"</h2>
        <div style="display: flex; gap: 1.5rem; flex-wrap: wrap; align-items: center;">
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <RadzenCheckBox />
                <RadzenLabel text=Some("Visible".to_string()) />
            </div>
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <RadzenCheckBox base=ComponentProps { visible: Some(false), ..Default::default() } />
                <RadzenLabel text=Some("(hidden checkbox here)".to_string()) />
            </div>
            <div style="display: flex; align-items: center; gap: 0.5rem;">
                <RadzenCheckBox />
                <RadzenLabel text=Some("Also visible".to_string()) />
            </div>
        </div>

        // ════════════════════════════════════════════════════════════════════════
        // DatePicker
        // ════════════════════════════════════════════════════════════════════════
        /*
        <h2 style="margin-top: 2rem;">"DatePicker — Basic"</h2>
        <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
        "Click the calendar icon to open the popup."
        </p>
        <div style="max-width: 280px;">
        <RadzenDatePicker value=date_basic placeholder=Some("Pick a date".to_string()) />
        </div>
        <RadzenText text_style=TextStyle::Body2>
        {move || date_basic.get()
        .map(|d| format!("Selected: {}", d))
        .unwrap_or_else(|| "No date selected".to_string())}
        </RadzenText>
        
        <h2 style="margin-top: 2rem;">"DatePicker — Pre-filled Value"</h2>
        <div style="max-width: 280px;">
        <RadzenDatePicker value=date_bound />
        </div>
        <RadzenText text_style=TextStyle::Body2>
        {move || date_bound.get().map(|d| format!("Value: {}", d)).unwrap_or_default()}
        </RadzenText>
        
        <h2 style="margin-top: 2rem;">"DatePicker — Min / Max Clamping"</h2>
        <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
        "Only dates in 2025 are selectable."
        </p>
        <div style="max-width: 280px;">
        <RadzenDatePicker
        value=RwSignal::new(None::<NaiveDate>)
        min=Some(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        max=Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap())
        placeholder=Some("2025 only".to_string())
        />
        </div>
        
        <h2 style="margin-top: 2rem;">"DatePicker — Inline (always open)"</h2>
        <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
        "No trigger button — calendar is always visible."
        </p>
        <div style="display: inline-block;">
        <RadzenDatePicker value=date_inline inline=true />
        </div>
        <RadzenText text_style=TextStyle::Body2>
        {move || date_inline.get()
        .map(|d| format!("Selected: {}", d))
        .unwrap_or_else(|| "None".to_string())}
        </RadzenText>
        
        <h2 style="margin-top: 2rem;">"DatePicker — Icon Only (no text input)"</h2>
        <div style="max-width: 60px;">
        <RadzenDatePicker
        value=RwSignal::new(None::<NaiveDate>)
        show_input=false
        />
        </div>
        
        <h2 style="margin-top: 2rem;">"DatePicker — Disabled"</h2>
        <div style="max-width: 280px;">
        <RadzenDatePicker
        value=RwSignal::new(Some(NaiveDate::from_ymd_opt(2025, 3, 14).unwrap()))
                disabled=true
            />
        </div>

        <h2 style="margin-top: 2rem;">"DatePicker — onChange Callback"</h2>
        <div style="max-width: 280px;">
            <RadzenDatePicker
                value=RwSignal::new(None::<NaiveDate>)
                on_change=Some(std::sync::Arc::new(|d| {
                    log::info!("DatePicker changed: {:?}", d);
                }))
                placeholder=Some("Select date".to_string())
            />
        </div>

        <h2 style="margin-top: 2rem;">"DatePicker — Date Range Form"</h2>
        <RadzenCard variant=Variant::Outlined>
            <div style="padding: 1rem; display: flex; flex-direction: column; gap: 0.75rem; max-width: 360px;">
                <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                    <RadzenLabel text=Some("Start date".to_string()) component=Some("range_start".to_string()) />
                    <RadzenDatePicker value=date_range_start name=Some("range_start".to_string()) placeholder=Some("From".to_string()) />
                </div>
                <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                    <RadzenLabel text=Some("End date".to_string()) component=Some("range_end".to_string()) />
                    <RadzenDatePicker value=date_range_end name=Some("range_end".to_string()) placeholder=Some("To".to_string()) />
                </div>
                <RadzenText text_style=TextStyle::Body2>
                    {move || {
                        let s = date_range_start.get().map(|d| d.to_string()).unwrap_or_else(|| "—".to_string());
                        let e = date_range_end.get().map(|d| d.to_string()).unwrap_or_else(|| "—".to_string());
                        format!("Range: {} → {}", s, e)
                    }}
                </RadzenText>
            </div>
        </RadzenCard>
        */
        
        // ════════════════════════════════════════════════════════════════════════
        // Numeric
        // ════════════════════════════════════════════════════════════════════════

        <h2 style="margin-top: 2rem;">"Numeric — Basic"</h2>
        <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
            "Use the up/down buttons or arrow keys."
        </p>
        <div style="max-width: 200px;">
            <RadzenNumeric value=num_basic placeholder=Some("Enter number".to_string()) />
        </div>
        <RadzenText text_style=TextStyle::Body2>
            {move || num_basic.get().map(|v| format!("Value: {}", v)).unwrap_or_else(|| "Empty".to_string())}
        </RadzenText>

        <h2 style="margin-top: 2rem;">"Numeric — Pre-filled, Step 5"</h2>
        <div style="max-width: 200px;">
            <RadzenNumeric value=num_bound step=5.0 />
        </div>
        <RadzenText text_style=TextStyle::Body2>
            {move || format!("Value: {}", num_bound.get().unwrap_or(0.0))}
        </RadzenText>

        <h2 style="margin-top: 2rem;">"Numeric — Min / Max Clamping (1–10)"</h2>
        <div style="max-width: 200px;">
            <RadzenNumeric value=num_clamped min=Some(1.0) max=Some(10.0) step=1.0 />
        </div>
        <RadzenText text_style=TextStyle::Body2>
            {move || format!("Value: {}", num_clamped.get().unwrap_or(0.0))}
        </RadzenText>

        <h2 style="margin-top: 2rem;">"Numeric — Decimal Places"</h2>
        <div style="display: flex; flex-direction: column; gap: 0.75rem; max-width: 200px;">
            <div>
                <RadzenLabel text=Some("0 decimals".to_string()) />
                <RadzenNumeric value=RwSignal::new(Some(42.0_f64)) decimals=Some(0) step=1.0 />
            </div>
            <div>
                <RadzenLabel text=Some("2 decimals".to_string()) />
                <RadzenNumeric value=num_decimal decimals=Some(2) step=0.01 />
            </div>
            <div>
                <RadzenLabel text=Some("4 decimals".to_string()) />
                <RadzenNumeric value=RwSignal::new(Some(1.0_f64)) decimals=Some(4) step=0.0001 />
            </div>
        </div>

        <h2 style="margin-top: 2rem;">"Numeric — Text Alignment"</h2>
        <div style="display: flex; flex-direction: column; gap: 0.5rem; max-width: 220px;">
            <RadzenNumeric value=RwSignal::new(Some(100.0_f64)) text_align=TextAlign::Left   decimals=Some(0) />
            <RadzenNumeric value=RwSignal::new(Some(200.0_f64)) text_align=TextAlign::Center decimals=Some(0) />
            <RadzenNumeric value=RwSignal::new(Some(300.0_f64)) text_align=TextAlign::Right  decimals=Some(0) />
        </div>

        <h2 style="margin-top: 2rem;">"Numeric — No Up/Down Buttons"</h2>
        <div style="max-width: 200px;">
            <RadzenNumeric value=num_currency show_updown=false decimals=Some(2) placeholder=Some("Amount".to_string()) />
        </div>

        <h2 style="margin-top: 2rem;">"Numeric — Disabled and ReadOnly"</h2>
        <div style="display: flex; flex-direction: column; gap: 0.75rem; max-width: 200px;">
            <RadzenNumeric value=RwSignal::new(Some(99.0_f64)) disabled=true />
            <RadzenNumeric value=RwSignal::new(Some(42.0_f64)) read_only=true />
        </div>

        <h2 style="margin-top: 2rem;">"Numeric — onChange Callback"</h2>
        <div style="max-width: 200px;">
            <RadzenNumeric
                value=RwSignal::new(Some(0.0_f64))
                step=1.0
                on_change=Some(std::sync::Arc::new(|v| {
                    log::info!("Numeric changed: {:?}", v);
                }))
            />
        </div>

        <h2 style="margin-top: 2rem;">"Numeric — Quantity / Price Form"</h2>
        <RadzenCard variant=Variant::Outlined>
            <div style="padding: 1rem; display: flex; flex-direction: column; gap: 0.75rem; max-width: 360px;">
                <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                    <RadzenLabel text=Some("Quantity".to_string()) component=Some("qty".to_string()) />
                    <RadzenNumeric name=Some("qty".to_string()) value=RwSignal::new(Some(1.0_f64))
                        min=Some(1.0) max=Some(999.0) step=1.0 decimals=Some(0) />
                </div>
                <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                    <RadzenLabel text=Some("Unit Price (€)".to_string()) component=Some("price".to_string()) />
                    <RadzenNumeric name=Some("price".to_string()) value=RwSignal::new(Some(9.99_f64))
                        min=Some(0.0) step=0.01 decimals=Some(2) />
                </div>
                <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                    <RadzenLabel text=Some("Discount (%)".to_string()) component=Some("discount".to_string()) />
                    <RadzenNumeric name=Some("discount".to_string()) value=RwSignal::new(Some(0.0_f64))
                        min=Some(0.0) max=Some(100.0) step=5.0 decimals=Some(0) />
                </div>
                <div style="display: flex; justify-content: flex-end;">
                    <RadzenButton text="Add to cart".to_string() button_style=ButtonStyle::Primary icon=Some("shopping_cart".to_string()) />
                </div>
            </div>
        </RadzenCard>

        // ════════════════════════════════════════════════════════════════════════
        // SelectBar
        // ════════════════════════════════════════════════════════════════════════

        <h2 style="margin-top: 2rem;">"SelectBar — Basic (single selection)"</h2>
        <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
            "One button active at a time. Keyboard: arrows to navigate, Space/Enter to select."
        </p>
        <RadzenSelectBar value=sb_view>
            <RadzenSelectBarItem value="list"  text="List"  icon=Some("list".to_string()) />
            <RadzenSelectBarItem value="grid"  text="Grid"  icon=Some("grid_view".to_string()) />
            <RadzenSelectBarItem value="table" text="Table" icon=Some("table_rows".to_string()) />
        </RadzenSelectBar>
        <RadzenText text_style=TextStyle::Body2>
            {move || format!("View: {}", sb_view.get())}
        </RadzenText>

        <h2 style="margin-top: 2rem;">"SelectBar — Multiple Selection"</h2>
        <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
            "Toggle multiple buttons independently."
        </p>
        <RadzenSelectBar value_multiple=sb_multi multiple=true>
            <RadzenSelectBarItem value="bold"      text="B" />
            <RadzenSelectBarItem value="italic"    text="I" />
            <RadzenSelectBarItem value="underline" text="U" />
            <RadzenSelectBarItem value="strike"    text="S" />
        </RadzenSelectBar>
        <RadzenText text_style=TextStyle::Body2>
            {move || format!("Active: [{}]", sb_multi.get().join(", "))}
        </RadzenText>

        <h2 style="margin-top: 2rem;">"SelectBar — Filter Operator (And / Or)"</h2>
        <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
            "Typical DataGrid filter operator toggle."
        </p>
        <RadzenSelectBar value=sb_operator size=ButtonSize::Small>
            <RadzenSelectBarItem value="And" text="And" />
            <RadzenSelectBarItem value="Or"  text="Or" />
        </RadzenSelectBar>
        <RadzenText text_style=TextStyle::Body2>
            {move || format!("Operator: {}", sb_operator.get())}
        </RadzenText>

        <h2 style="margin-top: 2rem;">"SelectBar — Icon Only"</h2>
        <RadzenSelectBar value=RwSignal::new("center".to_string())>
            <RadzenSelectBarItem value="left"    icon=Some("format_align_left".to_string()) />
            <RadzenSelectBarItem value="center"  icon=Some("format_align_center".to_string()) />
            <RadzenSelectBarItem value="right"   icon=Some("format_align_right".to_string()) />
            <RadzenSelectBarItem value="justify" icon=Some("format_align_justify".to_string()) />
        </RadzenSelectBar>

        <h2 style="margin-top: 2rem;">"SelectBar — Sizes"</h2>
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <div>
                <RadzenLabel text=Some("ExtraSmall".to_string()) />
                <RadzenSelectBar value=RwSignal::new("a".to_string()) size=ButtonSize::ExtraSmall>
                    <RadzenSelectBarItem value="a" text="Alpha" />
                    <RadzenSelectBarItem value="b" text="Beta" />
                    <RadzenSelectBarItem value="c" text="Gamma" />
                </RadzenSelectBar>
            </div>
            <div>
                <RadzenLabel text=Some("Small".to_string()) />
                <RadzenSelectBar value=RwSignal::new("a".to_string()) size=ButtonSize::Small>
                    <RadzenSelectBarItem value="a" text="Alpha" />
                    <RadzenSelectBarItem value="b" text="Beta" />
                    <RadzenSelectBarItem value="c" text="Gamma" />
                </RadzenSelectBar>
            </div>
            <div>
                <RadzenLabel text=Some("Medium (default)".to_string()) />
                <RadzenSelectBar value=RwSignal::new("a".to_string()) size=ButtonSize::Medium>
                    <RadzenSelectBarItem value="a" text="Alpha" />
                    <RadzenSelectBarItem value="b" text="Beta" />
                    <RadzenSelectBarItem value="c" text="Gamma" />
                </RadzenSelectBar>
            </div>
            <div>
                <RadzenLabel text=Some("Large".to_string()) />
                <RadzenSelectBar value=RwSignal::new("a".to_string()) size=ButtonSize::Large>
                    <RadzenSelectBarItem value="a" text="Alpha" />
                    <RadzenSelectBarItem value="b" text="Beta" />
                    <RadzenSelectBarItem value="c" text="Gamma" />
                </RadzenSelectBar>
            </div>
        </div>

        <h2 style="margin-top: 2rem;">"SelectBar — Vertical Orientation"</h2>
        <RadzenSelectBar value=RwSignal::new("top".to_string()) orientation=Orientation::Vertical>
            <RadzenSelectBarItem value="top"    text="Top"    icon=Some("vertical_align_top".to_string()) />
            <RadzenSelectBarItem value="middle" text="Middle" icon=Some("vertical_align_center".to_string()) />
            <RadzenSelectBarItem value="bottom" text="Bottom" icon=Some("vertical_align_bottom".to_string()) />
        </RadzenSelectBar>

        <h2 style="margin-top: 2rem;">"SelectBar — With Disabled Item"</h2>
        <RadzenSelectBar value=RwSignal::new("free".to_string())>
            <RadzenSelectBarItem value="free"       text="Free" />
            <RadzenSelectBarItem value="pro"        text="Pro" />
            <RadzenSelectBarItem value="enterprise" text="Enterprise" disabled=true />
        </RadzenSelectBar>

        <h2 style="margin-top: 2rem;">"SelectBar — Fully Disabled"</h2>
        <RadzenSelectBar value=RwSignal::new("b".to_string()) disabled=true>
            <RadzenSelectBarItem value="a" text="Option A" />
            <RadzenSelectBarItem value="b" text="Option B" />
            <RadzenSelectBarItem value="c" text="Option C" />
        </RadzenSelectBar>

        <h2 style="margin-top: 2rem;">"SelectBar — onChange Callback"</h2>
        <RadzenSelectBar
            value=RwSignal::new("one".to_string())
            on_change=Some(std::sync::Arc::new(|v| {
                log::info!("SelectBar changed: {}", v);
            }))
        >
            <RadzenSelectBarItem value="one"   text="One" />
            <RadzenSelectBarItem value="two"   text="Two" />
            <RadzenSelectBarItem value="three" text="Three" />
        </RadzenSelectBar>

        // ════════════════════════════════════════════════════════════════════════
        // DropDown
        // ════════════════════════════════════════════════════════════════════════
        /*
        <h2 style="margin-top: 2rem;">"DropDown — Basic"</h2>
        <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
        "Click to open; click an item to select and close."
        </p>
        <div style="max-width: 280px;">
        <RadzenDropDown
        value=dd_single
        data=vec![
            DropDownItem::new("1", "Orders"),
            DropDownItem::new("2", "Employees"),
            DropDownItem::new("3", "Products"),
            DropDownItem::new("4", "Customers"),
            ]
            placeholder=Some("Select category…".to_string())
            />
            </div>
            <RadzenText text_style=TextStyle::Body2>
            {move || {
                let v = dd_single.get();
                if v.is_empty() { "Nothing selected".to_string() }
                else { format!("Selected id: {}", v) }
            }}
            </RadzenText>
            
        <h2 style="margin-top: 2rem;">"DropDown — Pre-selected Value"</h2>
        <div style="max-width: 280px;">
            <RadzenDropDown
            value=dd_prefilled
            data=vec![
                DropDownItem::new("1", "January"),
                DropDownItem::new("2", "February"),
                DropDownItem::new("3", "March"),
                DropDownItem::new("4", "April"),
                ]
                />
                </div>
                <RadzenText text_style=TextStyle::Body2>
            {move || format!("Month id: {}", dd_prefilled.get())}
            </RadzenText>
            
            <h2 style="margin-top: 2rem;">"DropDown — With Search Filtering"</h2>
            <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
            "Type in the search box inside the panel to filter the list."
            </p>
            <div style="max-width: 280px;">
            <RadzenDropDown
            value=dd_filtered
            allow_filtering=true
            filter_placeholder="Search country…".to_string()
            placeholder=Some("Country…".to_string())
            data=vec![
                DropDownItem::new("at", "Austria"),
                DropDownItem::new("be", "Belgium"),
                DropDownItem::new("hr", "Croatia"),
                DropDownItem::new("cz", "Czech Republic"),
                DropDownItem::new("dk", "Denmark"),
                DropDownItem::new("fi", "Finland"),
                DropDownItem::new("fr", "France"),
                DropDownItem::new("de", "Germany"),
                DropDownItem::new("gr", "Greece"),
                DropDownItem::new("hu", "Hungary"),
                DropDownItem::new("ie", "Ireland"),
                DropDownItem::new("it", "Italy"),
                DropDownItem::new("nl", "Netherlands"),
                DropDownItem::new("pl", "Poland"),
                DropDownItem::new("pt", "Portugal"),
                DropDownItem::new("ro", "Romania"),
                DropDownItem::new("es", "Spain"),
                DropDownItem::new("se", "Sweden"),
                ]
                />
                </div>
                <RadzenText text_style=TextStyle::Body2>
                {move || {
                    let v = dd_filtered.get();
                    if v.is_empty() { "None".to_string() } else { format!("Code: {}", v) }
                }}
                </RadzenText>
                
                <h2 style="margin-top: 2rem;">"DropDown — Multiple Selection"</h2>
                <p style="color: var(--rz-base-700); margin-bottom: 0.75rem; font-size: 0.875rem;">
                "Each item toggles independently; a checkbox indicates selection state."
                </p>
        <div style="max-width: 280px;">
        <RadzenDropDown
        value_multiple=dd_multi
        multiple=true
        allow_select_all=true
        data=vec![
                    DropDownItem::new("design",  "Design"),
                    DropDownItem::new("dev",     "Development"),
                    DropDownItem::new("qa",      "Quality Assurance"),
                    DropDownItem::new("devops",  "DevOps"),
                    DropDownItem::new("pm",      "Project Management"),
                ]
                placeholder=Some("Choose teams…".to_string())
                />
                </div>
        <RadzenText text_style=TextStyle::Body2>
            {move || {
                let sel = dd_multi.get();
                if sel.is_empty() { "None selected".to_string() }
                else { format!("Selected: {}", sel.join(", ")) }
            }}
        </RadzenText>

        <h2 style="margin-top: 2rem;">"DropDown — Disabled Item"</h2>
        <div style="max-width: 280px;">
            <RadzenDropDown
                value=RwSignal::new(String::new())
                data=vec![
                    DropDownItem::new("free",       "Free"),
                    DropDownItem::new("starter",    "Starter"),
                    DropDownItem::new("pro",        "Pro"),
                    DropDownItem::new("enterprise", "Enterprise").disabled(),
                ]
                placeholder=Some("Choose plan…".to_string())
            />
        </div>

        <h2 style="margin-top: 2rem;">"DropDown — Fully Disabled"</h2>
        <div style="max-width: 280px;">
            <RadzenDropDown
                value=RwSignal::new("2".to_string())
                disabled=true
                data=vec![
                    DropDownItem::new("1", "Option A"),
                    DropDownItem::new("2", "Option B"),
                    DropDownItem::new("3", "Option C"),
                ]
            />
            </div>
            
            <h2 style="margin-top: 2rem;">"DropDown — onChange Callback"</h2>
            <div style="max-width: 280px;">
            <RadzenDropDown
                value=RwSignal::new(String::new())
                data=vec![
                    DropDownItem::new("xs", "Extra Small"),
                    DropDownItem::new("sm", "Small"),
                    DropDownItem::new("md", "Medium"),
                    DropDownItem::new("lg", "Large"),
                    DropDownItem::new("xl", "Extra Large"),
                ]
                placeholder=Some("Select size…".to_string())
                on_change=Some(std::sync::Arc::new(|v| {
                    log::info!("DropDown changed: {}", v);
                }))
            />
        </div>

        <h2 style="margin-top: 2rem;">"DropDown — Product Filter Form"</h2>
        <RadzenCard variant=Variant::Outlined>
            <div style="padding: 1rem; display: flex; flex-direction: column; gap: 0.75rem; max-width: 400px;">
                <RadzenText text_style=TextStyle::H6 text=Some("Filter Products".to_string()) />
                <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                    <RadzenLabel text=Some("Category".to_string()) component=Some("cat_filter".to_string()) />
                    <RadzenDropDown
                        value=RwSignal::new(String::new())
                        name=Some("cat_filter".to_string())
                        data=vec![
                            DropDownItem::new("electronics", "Electronics"),
                            DropDownItem::new("clothing",    "Clothing"),
                            DropDownItem::new("food",        "Food & Beverage"),
                            DropDownItem::new("books",       "Books"),
                            ]
                            placeholder=Some("All categories".to_string())
                            />
                            </div>
                            <div style="display: flex; flex-direction: column; gap: 0.25rem;">
                            <RadzenLabel text=Some("Sort by".to_string()) component=Some("sort_filter".to_string()) />
                    <RadzenDropDown
                    value=RwSignal::new("relevance".to_string())
                    name=Some("sort_filter".to_string())
                    data=vec![
                        DropDownItem::new("relevance",   "Relevance"),
                        DropDownItem::new("price_asc",   "Price: Low to High"),
                        DropDownItem::new("price_desc",  "Price: High to Low"),
                        DropDownItem::new("newest",      "Newest First"),
                        ]
                        />
                        </div>
                        <div style="display: flex; gap: 0.5rem; justify-content: flex-end;">
                        <RadzenButton text="Reset".to_string()  button_style=ButtonStyle::Secondary variant=Variant::Flat />
                        <RadzenButton text="Apply".to_string()  button_style=ButtonStyle::Primary />
                        </div>
            </div>
        </RadzenCard>
        */
        </div> // .container
        </ErrorBoundary>
    }
}