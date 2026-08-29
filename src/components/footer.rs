use dioxus::prelude::*;

use crate::config::FooterConfig;

#[component]
pub fn Footer(config: &'static FooterConfig, text: String) -> Element {
    rsx! {
        footer { class: "site-footer",
            div { class: "footer-container",
                p { "data-owner": config.owner.clone(), {text} }
            }
        }
    }
}
