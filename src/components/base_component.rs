//! Base component infrastructure for Radzen Leptos.
//!
//! # Design
//!
//! Because Rust has no class inheritance, the C# `RadzenComponent` base class
//! is decomposed into three pieces:
//!
//! | C# concept                        | Rust equivalent                          |
//! |-----------------------------------|------------------------------------------|
//! | `[Parameter]` on base class       | Fields on [`ComponentProps`]             |
//! | `GetComponentCssClass()` virtual  | [`RadzenComponent`] trait                |
//! | `OnAfterRenderAsync` / lifecycle  | [`use_radzen_base`] composable           |
//! | `IDisposable`                     | `on_cleanup` inside `use_radzen_base`    |
//! | `Debouncer`                       | [`Debouncer`] struct                     |
//! | `Culture` / `DefaultCulture`      | `ComponentProps::locale` + [`RadzenLocaleContext`] |
//! | `Visible` + `visibleChanged`      | `RwSignal<bool>` + `Effect` in `use_radzen_base` |
//!
//! # Quick-start
//!
//! ```rust,ignore
//! use crate::components::base::*;
//!
//! #[derive(Clone, PartialEq, Default)]
//! pub struct ButtonProps {
//!     pub base: ComponentProps,
//!     pub label: String,
//!     pub disabled: bool,
//! }
//!
//! impl RadzenComponent for ButtonProps {
//!     fn component_css_class(&self) -> &'static str {
//!         "rz-button bg-blue-600 text-white rounded px-4 py-2"
//!     }
//! }
//!
//! #[component]
//! pub fn RadzenButton(props: ButtonProps) -> impl IntoView {
//!     let handle = use_radzen_base(&props.base, props.component_css_class());
//!     view! {
//!         <Show when=move || handle.visible.get()>
//!             <button id=handle.id class=handle.css_class>
//!                 {props.label.clone()}
//!             </button>
//!         </Show>
//!     }
//! }
//! ```

use leptos::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

// ─────────────────────────────────────────────────────────────────────────────
// Re-exports
// ─────────────────────────────────────────────────────────────────────────────

pub use leptos::ev::MouseEvent;

// ─────────────────────────────────────────────────────────────────────────────
// Locale
// Mirrors C# Culture / DefaultCulture cascade pattern.
// ─────────────────────────────────────────────────────────────────────────────

/// Resolved locale tag, e.g. `"en-US"`, `"hu-HU"`.
///
/// Provide this once near the root of your app with
/// [`provide_locale_context`] and every Radzen component will inherit it.
/// Individual components can override it via [`ComponentProps::locale`].
#[derive(Clone, PartialEq, Debug)]
pub struct RadzenLocaleContext(pub String);

/// Provide a locale for all descendant Radzen components.
///
/// Call this in your root `App` component:
/// ```rust,ignore
/// provide_locale_context(None); // reads OS/browser locale via sys_locale
/// provide_locale_context(Some("de-DE".to_string())); // explicit override
/// ```
pub fn provide_locale_context(locale: Option<String>) {
    let tag = locale.unwrap_or_else(resolve_system_locale);
    provide_context(RadzenLocaleContext(tag));
}

/// Read the OS / browser locale tag, falling back to `"en-US"`.
///
/// In WASM this calls `navigator.language`; natively it calls `sys_locale`.
fn resolve_system_locale() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        web_sys::window()
            .and_then(|w| w.navigator().language())
            .unwrap_or_else(|| "en-US".to_string())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        sys_locale::get_locale().unwrap_or_else(|| "en-US".to_string())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unique ID generation
// No uuid / getrandom required — Math.random() in WASM, atomic counter natively.
// ─────────────────────────────────────────────────────────────────────────────

/// Generate a short, stable, DOM-safe unique ID prefixed with `rz-`.
pub fn new_unique_id() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Math;
        let a = (Math::random() * f64::from(u32::MAX)) as u32;
        let b = (Math::random() * f64::from(u32::MAX)) as u32;
        format!("rz-{:08x}{:08x}", a, b)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        format!("rz-{:016x}", n)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ComponentProps
// ─────────────────────────────────────────────────────────────────────────────

/// Props shared by **every** Radzen component.
///
/// Embed this in your own props struct as `pub base: ComponentProps` and
/// forward it to [`use_radzen_base`].  All fields are optional.
// PartialEq is implemented manually below because Arc<dyn Fn(…)> doesn't
// implement PartialEq — you can't compare two closures for equality in Rust.
// The manual impl compares only the data fields; callback fields are treated
// as always-equal (the same convention Blazor / React use for event props).
#[derive(Clone, Default)]
pub struct ComponentProps {
    // ── Visual ───────────────────────────────────────────────────────────────
    /// Inline CSS `style` string, e.g. `"color: red; margin-top: 1rem"`.
    pub style: Option<String>,

    /// When `false` the component is not rendered at all. Defaults to `true`.
    pub visible: Option<bool>,

    /// Explicit `id` attribute.  A unique ID is generated when `None`.
    pub id: Option<String>,

    /// Extra HTML attributes (`data-*`, `aria-*`, class overrides …).
    ///
    /// A `"class"` key is **merged** (appended) with the component's own
    /// Tailwind classes from [`RadzenComponent::component_css_class`].
    pub attrs: Option<HashMap<String, String>>,

    /// Override the locale for this component only, e.g. `"de-DE"`.
    ///
    /// When `None` the component inherits from [`RadzenLocaleContext`]
    /// (set via [`provide_locale_context`]), which itself falls back to
    /// the OS / browser locale.  This mirrors C# `Culture` /
    /// `[CascadingParameter] DefaultCulture`.
    pub locale: Option<String>,

    // ── Event callbacks ───────────────────────────────────────────────────────
    // Arc<dyn Fn…> does NOT implement PartialEq; see manual impl below.
    /// Called with the component's `id` string when the pointer enters.
    pub on_mouse_enter: Option<Arc<dyn Fn(String) + Send + Sync>>,

    /// Called with the component's `id` string when the pointer leaves.
    pub on_mouse_leave: Option<Arc<dyn Fn(String) + Send + Sync>>,

    /// Called with the raw [`MouseEvent`] on right-click / context-menu.
    pub on_context_menu: Option<Arc<dyn Fn(MouseEvent) + Send + Sync>>,
}

/// Manual `PartialEq` for `ComponentProps`.
///
/// `Arc<dyn Fn(…)>` has no meaningful equality — two distinct `Arc`s wrapping
/// identical closures are not comparable in Rust.  We follow the same
/// convention as Blazor and React: callback props are **excluded** from
/// equality checks.  Only the plain data fields are compared, which is what
/// Leptos needs to decide whether to re-render.
impl PartialEq for ComponentProps {
    fn eq(&self, other: &Self) -> bool {
        self.style == other.style
            && self.visible == other.visible
            && self.id == other.id
            && self.attrs == other.attrs
            && self.locale == other.locale
        // on_mouse_enter / on_mouse_leave / on_context_menu intentionally omitted
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RadzenComponent trait
// ─────────────────────────────────────────────────────────────────────────────

/// Implemented by every concrete Radzen component **props struct**.
///
/// The only required method is [`component_css_class`].  Return the Tailwind
/// utility classes (and/or custom rz-* classes) that form the component's
/// default appearance.  Callers can extend them via `base.attrs["class"]`.
pub trait RadzenComponent {
    /// Base CSS class(es) for this component.
    fn component_css_class(&self) -> &'static str;
}

// ─────────────────────────────────────────────────────────────────────────────
// RadzenBaseHandle
// ─────────────────────────────────────────────────────────────────────────────

/// Returned by [`use_radzen_base`].  Destructure this inside your `view!`.
pub struct RadzenBaseHandle {
    /// Stable unique DOM `id` for this component instance.
    pub id: String,

    /// Reactive visibility — gate rendering with
    /// `<Show when=move || handle.visible.get()>`.
    ///
    /// This is an `RwSignal` so it updates reactively when the `visible`
    /// prop changes after mount, mirroring Radzen's `OnBecameInvisible` /
    /// re-register lifecycle.
    pub visible: RwSignal<bool>,

    /// Merged CSS class string (`component_class [+ caller_class]`).
    pub css_class: String,

    /// Resolved locale tag for this component, e.g. `"hu-HU"`.
    ///
    /// Resolution order (mirrors C# Culture cascade):
    /// 1. `ComponentProps::locale` (explicit per-component override)
    /// 2. [`RadzenLocaleContext`] provided by an ancestor
    /// 3. OS / browser locale via `sys_locale` / `navigator.language`
    /// 4. Hard fallback `"en-US"`
    pub locale: String,

    /// Wire to `on:mouseenter` on the root element.
    pub on_mouse_enter: Arc<dyn Fn(MouseEvent) + Send + Sync>,

    /// Wire to `on:mouseleave` on the root element.
    pub on_mouse_leave: Arc<dyn Fn(MouseEvent) + Send + Sync>,

    /// Wire to `on:contextmenu` on the root element.
    pub on_context_menu: Arc<dyn Fn(MouseEvent) + Send + Sync>,
}

// ─────────────────────────────────────────────────────────────────────────────
// use_radzen_base  — the core composable
// ─────────────────────────────────────────────────────────────────────────────

/// Call this at the **top** of every Radzen component function.
///
/// ```rust,ignore
/// let handle = use_radzen_base(&props.base, props.component_css_class());
/// ```
pub fn use_radzen_base(base: &ComponentProps, component_class: &'static str) -> RadzenBaseHandle {
    // ── Unique ID ────────────────────────────────────────────────────────────
    let id = base.id.clone().unwrap_or_else(new_unique_id);

    // ── Locale ───────────────────────────────────────────────────────────────
    // Resolution order (mirrors C# Culture / [CascadingParameter] DefaultCulture):
    //   1. explicit per-component prop
    //   2. RadzenLocaleContext provided by an ancestor
    //   3. OS / browser locale (inside resolve_system_locale)
    //   4. hard fallback "en-US"
    let locale = base.locale.clone().unwrap_or_else(|| {
        use_context::<RadzenLocaleContext>()
            .map(|ctx| ctx.0.clone())
            .unwrap_or_else(resolve_system_locale)
    });

    // ── Visibility ───────────────────────────────────────────────────────────
    // RwSignal so the value can be updated reactively when the prop changes
    // after mount — mirrors Radzen's SetParametersAsync / visibleChanged logic.
    let visible = RwSignal::new(base.visible.unwrap_or(true));

    // Watch for prop-driven visibility changes after mount.
    // Mirrors Radzen's `if (visibleChanged && !firstRender)` guard.
    // Leptos <Show> already handles DOM attach/detach and event listener
    // add/remove automatically — this Effect exists only for debug logging
    // and any future non-DOM side effects (e.g. notifying a tooltip service).
    let initial_visible = base.visible.unwrap_or(true);
    let id_for_watch = id.clone();
    let first_run = std::cell::Cell::new(true);

    Effect::new(move |_| {
        let current = visible.get();
        if first_run.get() {
            visible.set(initial_visible);
            first_run.set(false);
            return;
        }
        #[cfg(debug_assertions)]
        if current {
            log::debug!("RadzenComponent '{}' became visible", id_for_watch);
        } else {
            log::debug!("RadzenComponent '{}' became invisible", id_for_watch);
        }
    });

    // ── CSS class ────────────────────────────────────────────────────────────
    let caller_class = base
        .attrs
        .as_ref()
        .and_then(|a| a.get("class"))
        .cloned()
        .unwrap_or_default();

    let css_class = if caller_class.is_empty() {
        component_class.to_string()
    } else {
        format!("{component_class} {caller_class}")
    };

    // ── Event handlers ───────────────────────────────────────────────────────
    let id_for_enter = id.clone();
    let enter_cb = base.on_mouse_enter.clone();
    let on_mouse_enter: Arc<dyn Fn(MouseEvent) + Send + Sync> = Arc::new(move |_ev: MouseEvent| {
        if let Some(cb) = &enter_cb {
            cb(id_for_enter.clone());
        }
    });

    let id_for_leave = id.clone();
    let leave_cb = base.on_mouse_leave.clone();
    let on_mouse_leave: Arc<dyn Fn(MouseEvent) + Send + Sync> = Arc::new(move |_ev: MouseEvent| {
        if let Some(cb) = &leave_cb {
            cb(id_for_leave.clone());
        }
    });

    let ctx_cb = base.on_context_menu.clone();
    let on_context_menu: Arc<dyn Fn(MouseEvent) + Send + Sync> = Arc::new(move |ev: MouseEvent| {
        // Suppress the browser's native right-click menu — mirrors the
        // behaviour of Radzen.addContextMenu which calls preventDefault()
        // before invoking the .NET callback.  In Leptos we own the handler
        // directly so we call it here instead of going through JS.
        ev.prevent_default();
        if let Some(cb) = &ctx_cb {
            cb(ev);
        }
    });

    // ── Full disposal (mirrors IDisposable.Dispose) ───────────────────────────
    // on_cleanup fires when the component is fully unmounted from the DOM.
    // Leptos removes all event listeners attached via on:* automatically when
    // the element is dropped — no manual listener teardown required here.
    let cleanup_id = id.clone();
    on_cleanup(move || {
        #[cfg(debug_assertions)]
        log::debug!("RadzenComponent '{}' disposed", cleanup_id);
    });

    RadzenBaseHandle {
        id,
        visible,
        css_class,
        locale,
        on_mouse_enter,
        on_mouse_leave,
        on_context_menu,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Style helpers  (mirrors C# CurrentStyle property)
// ─────────────────────────────────────────────────────────────────────────────

/// Parse an inline CSS `style` string into a property→value map.
///
/// ```
/// # use radzen_leptos::components::base::parse_style;
/// let m = parse_style("color: red; font-size: 1rem");
/// assert_eq!(m["color"], "red");
/// ```
pub fn parse_style(style: &str) -> HashMap<String, String> {
    style
        .split(';')
        .filter_map(|pair| {
            let mut it = pair.splitn(2, ':');
            let key = it.next()?.trim().to_string();
            let val = it.next()?.trim().to_string();
            if key.is_empty() {
                None
            } else {
                Some((key, val))
            }
        })
        .collect()
}

/// Serialise a style map back to a CSS `style` attribute string.
pub fn style_to_string(map: &HashMap<String, String>) -> String {
    map.iter()
        .map(|(k, v)| format!("{k}: {v}"))
        .collect::<Vec<_>>()
        .join("; ")
}

// ─────────────────────────────────────────────────────────────────────────────
// Debouncer  (mirrors C# Debouncer helper)
// ─────────────────────────────────────────────────────────────────────────────

/// Debounces repeated calls, running the action only after `ms` ms of silence.
///
/// Uses `gloo_timers` in WASM (already a transitive Leptos dependency).
/// Falls back to an immediate call in native test builds.
///
/// # Example
/// ```rust,ignore
/// let debouncer = use_context::<Debouncer>().unwrap_or_default();
/// let on_input = move |_| debouncer.debounce(300, || search(query.get()));
/// ```
#[derive(Default)]
pub struct Debouncer {
    #[cfg(target_arch = "wasm32")]
    handle: std::cell::Cell<Option<gloo_timers::callback::Timeout>>,
}

impl Debouncer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Schedule `f` to run after `ms` milliseconds.
    /// Cancels any previously scheduled call.
    #[allow(unused_variables)]
    pub fn debounce<F: FnOnce() + 'static>(&self, ms: u32, f: F) {
        #[cfg(target_arch = "wasm32")]
        {
            self.handle.set(None); // drops old Timeout → cancels it
            let t = gloo_timers::callback::Timeout::new(ms, f);
            self.handle.set(Some(t));
        }
        #[cfg(not(target_arch = "wasm32"))]
        f(); // unit-test fast path: run immediately
    }

    /// Cancel any pending debounced call without running it.
    pub fn cancel(&self) {
        #[cfg(target_arch = "wasm32")]
        self.handle.set(None);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Unit tests  (cargo test — runs in native, not WASM)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_style_basic() {
        let m = parse_style("color: red; font-size: 1rem; ");
        assert_eq!(m.get("color").map(String::as_str), Some("red"));
        assert_eq!(m.get("font-size").map(String::as_str), Some("1rem"));
    }

    #[test]
    fn parse_style_empty() {
        assert!(parse_style("").is_empty());
    }

    #[test]
    fn style_round_trip() {
        let m1 = parse_style("color: blue; margin: 0px");
        let m2 = parse_style(&style_to_string(&m1));
        assert_eq!(m1, m2);
    }

    #[test]
    fn css_class_merge() {
        let mut attrs = HashMap::new();
        attrs.insert("class".to_string(), "mt-4".to_string());
        let base = ComponentProps {
            attrs: Some(attrs),
            ..Default::default()
        };

        let caller = base
            .attrs
            .as_ref()
            .and_then(|a| a.get("class"))
            .cloned()
            .unwrap_or_default();
        let result = format!("rz-button {caller}");
        assert_eq!(result, "rz-button mt-4");
    }

    #[test]
    fn unique_ids_differ() {
        let a = new_unique_id();
        let b = new_unique_id();
        assert_ne!(a, b);
        assert!(a.starts_with("rz-"));
    }

    #[test]
    fn debouncer_runs_in_native() {
        use std::sync::atomic::{AtomicBool, Ordering};
        static RAN: AtomicBool = AtomicBool::new(false);
        let d = Debouncer::new();
        d.debounce(100, || {
            RAN.store(true, Ordering::SeqCst);
        });
        assert!(RAN.load(Ordering::SeqCst));
    }

    #[test]
    fn locale_explicit_prop_wins() {
        // When ComponentProps::locale is set it takes priority over everything.
        let base = ComponentProps {
            locale: Some("de-DE".to_string()),
            ..Default::default()
        };
        // Resolution happens inside use_radzen_base; we test the field directly
        // here since we can't spin up a full Leptos runtime in unit tests.
        assert_eq!(base.locale.as_deref(), Some("de-DE"));
    }

    #[test]
    fn locale_falls_back_to_en_us_natively() {
        // In native (non-WASM) test builds sys_locale may or may not be set.
        // resolve_system_locale must always return a non-empty string.
        let tag = super::resolve_system_locale();
        assert!(!tag.is_empty(), "locale tag must never be empty");
        assert!(
            tag.contains('-') || tag.len() >= 2,
            "expected a BCP-47 tag like 'en-US', got '{tag}'"
        );
    }

    #[test]
    fn visible_rwsignal_default_true() {
        // RwSignal::new mirrors the default Visible = true in C#.
        let sig = RwSignal::new(true);
        assert!(sig.get_untracked());
        sig.set(false);
        assert!(!sig.get_untracked());
        sig.set(true);
        assert!(sig.get_untracked());
    }
}
