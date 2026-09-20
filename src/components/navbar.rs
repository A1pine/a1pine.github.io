use dioxus::prelude::WritableExt as _;
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdMenu, LdX},
};

use crate::config::NavbarConfig;
use crate::localization::{Locale, select_locale, translate, use_locale};

#[component]
pub fn Navbar(
    config: &'static NavbarConfig,
    scroll_top_visible: Signal<bool>,
    scroll_top_threshold: u32,
) -> Element {
    let mut mobile_open = use_signal(|| false);
    let locale = use_locale();
    let locale_signal = try_consume_context::<Signal<Locale>>();
    let language_toggle_label = translate(locale, "navbar.language_toggle", "Switch language");
    let chinese_label = translate(locale, "navbar.language_chinese", "中文");
    let english_label = translate(locale, "navbar.language_english", "English");
    let primary_navigation_label = translate(
        locale,
        "navbar.primary_navigation",
        &config.primary_navigation_label,
    );
    let brand = translate(locale, "navbar.brand", &config.brand);
    let cv_label = translate(locale, "navbar.cv", &config.cv_label);
    let mobile_open_label = translate(locale, "navbar.mobile_open", &config.mobile_menu_open_label);
    let mobile_close_label = translate(
        locale,
        "navbar.mobile_close",
        &config.mobile_menu_close_label,
    );

    #[cfg(target_arch = "wasm32")]
    let _scroll_tracker = use_hook(move || {
        std::rc::Rc::new(web_trackers::ScrollTracker::install(
            scroll_top_visible,
            scroll_top_threshold,
        ))
    });

    rsx! {
        nav {
            class: "navbar",
            aria_label: primary_navigation_label,
            onkeydown: move |event| {
                if event.key() == Key::Escape && mobile_open() {
                    mobile_open.set(false);
                    focus_mobile_menu_button();
                }
            },
            div { class: "nav-container",
                div { class: "nav-row",
                    a { class: "brand", href: config.brand_url.clone(), {brand.clone()} }

                    div { class: "desktop-nav",
                        for link in &config.links {
                            a { class: "nav-link", href: link.href.clone(),
                                {translate(locale, navigation_key(&link.href), &link.name)}
                                span { class: "nav-link-underline", aria_hidden: "true" }
                            }
                        }
                        if config.show_cv {
                            a { class: "cv-button", href: config.cv_url.clone(),
                                span { class: "cv-glow", aria_hidden: "true" }
                                span { class: "cv-particles", aria_hidden: "true",
                                    for index in 0..5 {
                                        span { class: "cv-particle", "data-particle": "{index}" }
                                    }
                                }
                                span { class: "cv-label", {cv_label.clone()} }
                            }
                        }
                    }

                    div { class: "nav-actions",
                        div { class: "language-toggle", role: "group", aria_label: language_toggle_label,
                            button {
                                class: if locale == Locale::ChineseSimplified {
                                    "language-option is-active"
                                } else {
                                    "language-option"
                                },
                                r#type: "button",
                                aria_label: chinese_label,
                                "aria-pressed": (locale == Locale::ChineseSimplified).to_string(),
                                onclick: move |_| {
                                    if let Some(signal) = locale_signal {
                                        select_locale(signal, Locale::ChineseSimplified);
                                    }
                                },
                                "中"
                            }
                            button {
                                class: if locale == Locale::English {
                                    "language-option is-active"
                                } else {
                                    "language-option"
                                },
                                r#type: "button",
                                aria_label: english_label,
                                "aria-pressed": (locale == Locale::English).to_string(),
                                onclick: move |_| {
                                    if let Some(signal) = locale_signal {
                                        select_locale(signal, Locale::English);
                                    }
                                },
                                "EN"
                            }
                        }
                        div { class: "mobile-controls",
                            button {
                                id: "mobile-menu-button",
                                class: "menu-button",
                                r#type: "button",
                                aria_label: if mobile_open() {
                                    mobile_close_label.clone()
                                } else {
                                    mobile_open_label.clone()
                                },
                                "aria-expanded": mobile_open().to_string(),
                                onclick: move |_| mobile_open.toggle(),
                                if mobile_open() {
                                    Icon { icon: LdX, width: 24, height: 24, class: "nav-icon" }
                                } else {
                                    Icon { icon: LdMenu, width: 24, height: 24, class: "nav-icon" }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "reading-progress", aria_hidden: "true" }

            if mobile_open() {
                div { class: "mobile-menu",
                    div { class: "mobile-menu-links",
                        for link in &config.links {
                            a {
                                class: "mobile-nav-link",
                                href: link.href.clone(),
                                onclick: move |_| mobile_open.set(false),
                                {translate(locale, navigation_key(&link.href), &link.name)}
                            }
                        }
                    }
                }
            }
        }
    }
}

fn navigation_key(href: &str) -> &'static str {
    match href {
        "#home" => "navbar.home",
        "#about" => "navbar.about",
        "#experience" => "navbar.experience",
        "#publications" => "navbar.publications",
        "#activity" => "navbar.activity",
        _ => "navbar.unknown",
    }
}

#[cfg(target_arch = "wasm32")]
fn focus_mobile_menu_button() {
    use wasm_bindgen::JsCast as _;

    if let Some(button) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("mobile-menu-button"))
        .and_then(|element| element.dyn_into::<web_sys::HtmlElement>().ok())
    {
        let _ = button.focus();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn focus_mobile_menu_button() {}

#[cfg(target_arch = "wasm32")]
mod web_trackers {
    use std::cell::Cell;
    use std::rc::Rc;

    use dioxus::prelude::WritableExt;
    use wasm_bindgen::{JsCast, closure::Closure};
    use web_sys::{AddEventListenerOptions, Event, HtmlElement};

    use crate::components::model::{scroll_progress, should_show_scroll_to_top};

    pub struct ScrollTracker {
        window: web_sys::Window,
        listener: Closure<dyn FnMut(Event)>,
        animation_callback: Closure<dyn FnMut(f64)>,
        frame: Rc<Cell<i32>>,
    }

    impl ScrollTracker {
        pub fn install(
            mut scroll_top_visible: dioxus::prelude::Signal<bool>,
            threshold: u32,
        ) -> Option<Self> {
            let window = web_sys::window()?;
            let root = window
                .document()?
                .document_element()?
                .dyn_into::<HtmlElement>()
                .ok()?;
            let frame = Rc::new(Cell::new(0));
            let initial_scroll = apply_scroll_progress(&window, &root);
            let initial_visible = should_show_scroll_to_top(initial_scroll, threshold);
            let last_visible = Rc::new(Cell::new(initial_visible));

            scroll_top_visible.set(initial_visible);

            let callback_window = window.clone();
            let callback_root = root;
            let callback_frame = Rc::clone(&frame);
            let callback_visible = Rc::clone(&last_visible);
            let animation_callback = Closure::wrap(Box::new(move |_timestamp: f64| {
                callback_frame.set(0);
                let scroll_y = apply_scroll_progress(&callback_window, &callback_root);
                let visible = should_show_scroll_to_top(scroll_y, threshold);
                if visible != callback_visible.get() {
                    callback_visible.set(visible);
                    scroll_top_visible.set(visible);
                }
            }) as Box<dyn FnMut(f64)>);
            let animation_function = animation_callback
                .as_ref()
                .unchecked_ref::<js_sys::Function>()
                .clone();

            let listener_window = window.clone();
            let listener_frame = Rc::clone(&frame);
            let listener = Closure::wrap(Box::new(move |_event: Event| {
                if listener_frame.get() == 0
                    && let Ok(id) = listener_window.request_animation_frame(&animation_function)
                {
                    listener_frame.set(id);
                }
            }) as Box<dyn FnMut(Event)>);

            let options = AddEventListenerOptions::new();
            options.set_passive(true);
            window
                .add_event_listener_with_callback_and_add_event_listener_options(
                    "scroll",
                    listener.as_ref().unchecked_ref(),
                    &options,
                )
                .ok()?;
            window
                .add_event_listener_with_callback("resize", listener.as_ref().unchecked_ref())
                .ok()?;

            Some(Self {
                window,
                listener,
                animation_callback,
                frame,
            })
        }
    }

    impl Drop for ScrollTracker {
        fn drop(&mut self) {
            let callback = self.listener.as_ref().unchecked_ref();
            let _ = self
                .window
                .remove_event_listener_with_callback("scroll", callback);
            let _ = self
                .window
                .remove_event_listener_with_callback("resize", callback);
            if self.frame.get() != 0 {
                let _ = self.window.cancel_animation_frame(self.frame.get());
            }
            let _ = &self.animation_callback;
        }
    }

    fn apply_scroll_progress(window: &web_sys::Window, root: &HtmlElement) -> f64 {
        let viewport = window
            .inner_height()
            .ok()
            .and_then(|height| height.as_f64())
            .unwrap_or(0.0);
        let max_scroll = (f64::from(root.scroll_height()) - viewport).max(0.0);
        let scroll_y = window.scroll_y().unwrap_or(0.0);
        let progress = scroll_progress(scroll_y, max_scroll);
        let _ = root
            .style()
            .set_property("--scroll-progress", &progress.to_string());
        scroll_y
    }
}
