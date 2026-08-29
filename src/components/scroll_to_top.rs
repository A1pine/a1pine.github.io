use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdArrowUp};

use crate::components::model::should_show_scroll_to_top;
use crate::config::ScrollToTopConfig;

#[component]
pub fn ScrollToTop(config: &'static ScrollToTopConfig, scroll_y: Signal<f64>) -> Element {
    let visible = should_show_scroll_to_top(scroll_y(), config.threshold_px);

    rsx! {
        button {
            class: if visible { "glass-card scroll-to-top is-visible" } else { "glass-card scroll-to-top" },
            r#type: "button",
            title: config.title.clone(),
            aria_label: config.title.clone(),
            aria_hidden: (!visible).to_string(),
            tabindex: if visible { "0" } else { "-1" },
            onclick: move |_| scroll_window_to_top(config.behavior),
            Icon { icon: LdArrowUp, width: 20, height: 20 }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn scroll_window_to_top(behavior: crate::config::ScrollBehavior) {
    let options = web_sys::ScrollToOptions::new();
    options.set_top(0.0);
    options.set_behavior(match behavior {
        crate::config::ScrollBehavior::Auto => web_sys::ScrollBehavior::Auto,
        crate::config::ScrollBehavior::Smooth => web_sys::ScrollBehavior::Smooth,
    });
    if let Some(window) = web_sys::window() {
        window.scroll_to_with_scroll_to_options(&options);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn scroll_window_to_top(_behavior: crate::config::ScrollBehavior) {}
