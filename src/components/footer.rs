use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdHeart};

use crate::config::{FooterConfig, FooterTechnologyConfig, FooterTechnologyIcon};
use crate::localization::{translate, translate_template, use_locale};

const RUST_LOGO: Asset = asset!("/assets/brands/rust-logo.svg");
const VUE_LOGO: Asset = asset!("/assets/brands/vue-logo.svg");
const DIOXUS_LOGO: Asset = asset!("/assets/brands/dioxus-logo.png");

#[component]
pub fn Footer(config: &'static FooterConfig, text: String, year: i32) -> Element {
    let locale = use_locale();
    let owner = translate(locale, "footer.owner", &config.owner);
    let powered_by = translate(locale, "footer.powered_by", &config.powered_by);
    let hosted_by = translate(locale, "footer.hosted_by", &config.hosted_by);
    let localized_text = translate_template(
        locale,
        "footer.template",
        &text,
        &[
            ("year", &year.to_string()),
            ("owner", &owner),
            ("powered_by", &powered_by),
            ("hosted_by", &hosted_by),
        ],
    );
    rsx! {
        footer { class: "site-footer",
            div { class: "footer-container",
                p {
                    aria_label: localized_text.clone(),
                    "data-footer-text": localized_text,
                    "data-owner": owner.clone(),
                    span {
                        {translate_template(
                            locale,
                            "footer.copyright_prefix",
                            &format!("© Copyright {year} {}. Powered by ", config.owner),
                            &[("year", &year.to_string()), ("owner", &owner)],
                        )}
                    }
                    if config.technologies.is_empty() {
                        span { {powered_by} }
                    } else {
                        for (index, technology) in config.technologies.iter().enumerate() {
                            FooterTechnology { technology: technology.clone() }
                            if index + 1 < config.technologies.len() {
                                span { aria_hidden: "true",
                                    if index + 2 == config.technologies.len() {
                                        {translate(locale, "footer.and_separator", " and ")}
                                    } else {
                                        {translate(locale, "footer.separator", ", ")}
                                    }
                                }
                            }
                        }
                    }
                    span {
                        {translate(locale, "footer.powered_suffix", "")}
                    }
                    span { class: "footer-hosting-block",
                        span { {translate(locale, "footer.hosted_prefix", ". Hosted with ")} }
                        span { class: "footer-hosted",
                            span { class: "footer-heart", aria_hidden: "true",
                                Icon { icon: LdHeart, width: 16, height: 16 }
                            }
                            span { {hosted_by} }
                        }
                        span { {translate(locale, "footer.hosted_suffix", ".")} }
                    }
                }
            }
        }
    }
}

#[component]
fn FooterTechnology(technology: FooterTechnologyConfig) -> Element {
    let (source, class) = match technology.icon {
        FooterTechnologyIcon::Rust => (RUST_LOGO, "footer-brand-icon rust"),
        FooterTechnologyIcon::Vue => (VUE_LOGO, "footer-brand-icon vue"),
        FooterTechnologyIcon::Dioxus => (DIOXUS_LOGO, "footer-brand-icon dioxus"),
    };

    rsx! {
        span { class: "footer-technology",
            img {
                class,
                src: source,
                alt: "",
                aria_hidden: "true",
                width: "18",
                height: "18",
            }
            span { {technology.name} }
        }
    }
}
