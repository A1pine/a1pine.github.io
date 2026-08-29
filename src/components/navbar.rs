use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdMenu, LdMoon, LdSun, LdX},
};

use crate::components::persist_theme;
use crate::config::{NavbarConfig, ThemeChoice};

#[component]
pub fn Navbar(
    config: &'static NavbarConfig,
    mut theme: Signal<ThemeChoice>,
    scroll_y: Signal<f64>,
) -> Element {
    let mut mobile_open = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    let _scroll_tracker =
        use_hook(move || std::rc::Rc::new(web_trackers::ScrollTracker::install(scroll_y)));

    let is_dark = theme() == ThemeChoice::Dark;
    let toggle_theme = move |_| {
        let next = theme().toggled();
        theme.set(next);
        persist_theme(&crate::config::site_config().site.theme_storage_key, next);
    };

    rsx! {
        nav { class: "navbar", aria_label: "Primary navigation",
            div { class: "nav-container",
                div { class: "nav-row",
                    a { class: "brand", href: config.brand_url.clone(), {config.brand.clone()} }

                    div { class: "desktop-nav",
                        for link in &config.links {
                            a { class: "nav-link", href: link.href.clone(),
                                {link.name.clone()}
                                span { class: "nav-link-underline", aria_hidden: "true" }
                            }
                        }
                        button {
                            class: "icon-button theme-toggle",
                            r#type: "button",
                            aria_label: config.theme_toggle_label.clone(),
                            onclick: toggle_theme,
                            if is_dark {
                                Icon { icon: LdSun, width: 20, height: 20, class: "nav-icon" }
                            } else {
                                Icon { icon: LdMoon, width: 20, height: 20, class: "nav-icon" }
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
                                span { class: "cv-label", {config.cv_label.clone()} }
                            }
                        }
                    }

                    div { class: "mobile-controls",
                        button {
                            class: "icon-button theme-toggle",
                            r#type: "button",
                            aria_label: config.theme_toggle_label.clone(),
                            onclick: toggle_theme,
                            if is_dark {
                                Icon { icon: LdSun, width: 20, height: 20, class: "nav-icon" }
                            } else {
                                Icon { icon: LdMoon, width: 20, height: 20, class: "nav-icon" }
                            }
                        }
                        button {
                            class: "menu-button",
                            r#type: "button",
                            aria_label: if mobile_open() {
                                config.mobile_menu_close_label.clone()
                            } else {
                                config.mobile_menu_open_label.clone()
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

            div { class: "reading-progress", aria_hidden: "true" }

            if mobile_open() {
                div { class: "mobile-menu",
                    div { class: "mobile-menu-links",
                        for link in &config.links {
                            a {
                                class: "mobile-nav-link",
                                href: link.href.clone(),
                                onclick: move |_| mobile_open.set(false),
                                {link.name.clone()}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod web_trackers {
    use std::cell::Cell;
    use std::rc::Rc;

    use dioxus::prelude::WritableExt;
    use wasm_bindgen::{JsCast, closure::Closure};
    use web_sys::{AddEventListenerOptions, Event, HtmlElement};

    use crate::components::model::scroll_progress;

    pub struct ScrollTracker {
        window: web_sys::Window,
        listener: Closure<dyn FnMut(Event)>,
        animation_callback: Closure<dyn FnMut(f64)>,
        frame: Rc<Cell<i32>>,
    }

    impl ScrollTracker {
        pub fn install(mut scroll_y: dioxus::prelude::Signal<f64>) -> Option<Self> {
            let window = web_sys::window()?;
            let root = window
                .document()?
                .document_element()?
                .dyn_into::<HtmlElement>()
                .ok()?;
            let frame = Rc::new(Cell::new(0));

            scroll_y.set(apply_scroll_progress(&window, &root));

            let callback_window = window.clone();
            let callback_root = root;
            let callback_frame = Rc::clone(&frame);
            let animation_callback = Closure::wrap(Box::new(move |_timestamp: f64| {
                callback_frame.set(0);
                scroll_y.set(apply_scroll_progress(&callback_window, &callback_root));
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
