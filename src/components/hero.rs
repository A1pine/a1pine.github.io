use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdBuilding, LdGithub, LdMail, LdMapPin, LdTwitter},
};

#[cfg(target_arch = "wasm32")]
use crate::components::model::is_working_hour;
use crate::components::model::{image_srcset, unsplash_with_width};
use crate::config::{HeroConfig, HeroSocialLink};

#[component]
pub fn Hero(config: &'static HeroConfig) -> Element {
    #[allow(unused_mut)]
    let mut is_available = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        let utc_hour = js_sys::Date::new_0().get_utc_hours() as i32;
        is_available.set(is_working_hour(
            utc_hour,
            config.timezone_offset_hours,
            config.working_hours_start,
            config.working_hours_end,
        ));
    });

    let status_title = if is_available() {
        config.available_title.clone()
    } else {
        config.offline_title.clone()
    };
    let image_source = unsplash_with_width(&config.image_url, 576);
    let image_sources = image_srcset(&config.image_url, &[320, 576, 800]);

    rsx! {
        section { id: config.anchor.clone(), class: "hero-section",
            div { class: "page-container hero-container",
                div { class: "hero-layout",
                    div { class: "profile-column entrance-scale",
                        div { class: "portrait-wrap",
                            div { class: "portrait-glow", aria_hidden: "true" }
                            img {
                                class: "portrait",
                                src: image_source,
                                srcset: image_sources,
                                sizes: "(min-width: 1024px) 288px, 256px",
                                alt: config.image_alt.clone(),
                                width: "576",
                                height: "576",
                                loading: "eager",
                                fetchpriority: "high",
                                decoding: "async",
                            }
                            span {
                                class: if is_available() { "status-dot available" } else { "status-dot offline" },
                                title: status_title,
                            }
                        }

                        div { class: "glass-card contact-card",
                            div { class: "contact-line",
                                Icon { icon: LdMapPin, width: 18, height: 18, class: "contact-icon" }
                                span { {config.location.clone()} }
                            }
                            div { class: "contact-line",
                                Icon { icon: LdBuilding, width: 18, height: 18, class: "contact-icon" }
                                span { {config.company.clone()} }
                            }
                        }
                    }

                    div { id: config.about_anchor.clone(), class: "hero-content entrance-up",
                        h1 { class: "hero-name",
                            {config.first_name.clone()}
                            " "
                            span { class: "gradient-name", {config.last_name.clone()} }
                        }
                        p { class: "hero-role", {config.role.clone()} }

                        div { class: "glass-card bio-card",
                            p { {config.bio.clone()} }
                        }

                        div { class: "interests-block",
                            h2 { class: "eyebrow", {config.interests_title.clone()} }
                            div { class: "interest-list",
                                for interest in &config.interests {
                                    span { class: "interest-chip", {interest.name.clone()} }
                                }
                            }
                        }

                        div { class: "social-list",
                            for social in &config.social_links {
                                SocialLink { link: social.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SocialLink(link: HeroSocialLink) -> Element {
    let label = format!("{} link: {}", link.platform, link.username);
    let platform = link.platform.clone();
    rsx! {
        a {
            class: "glass-card social-link",
            href: link.url.clone(),
            aria_label: label,
            SocialIcon { name: link.icon.clone() }
            span { {platform} }
        }
    }
}

#[component]
fn SocialIcon(name: String) -> Element {
    match name.as_str() {
        "twitter" => rsx! { Icon { icon: LdTwitter, width: 16, height: 16 } },
        "github" => rsx! { Icon { icon: LdGithub, width: 16, height: 16 } },
        _ => rsx! { Icon { icon: LdMail, width: 16, height: 16 } },
    }
}
