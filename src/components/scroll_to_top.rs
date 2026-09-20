use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdArrowUp};

use crate::config::ScrollToTopConfig;
use crate::localization::{translate, use_locale};

#[component]
pub fn ScrollToTop(config: &'static ScrollToTopConfig, visible: Signal<bool>) -> Element {
    let locale = use_locale();
    let is_visible = visible();
    let title = translate(locale, "scroll_to_top.title", &config.title);
    rsx! {
        button {
            class: if is_visible { "glass-card scroll-to-top is-visible" } else { "glass-card scroll-to-top" },
            r#type: "button",
            title: title.clone(),
            aria_label: title,
            aria_hidden: (!is_visible).to_string(),
            tabindex: if is_visible { "0" } else { "-1" },
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
