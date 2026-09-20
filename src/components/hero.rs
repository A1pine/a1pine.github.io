use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::ld_icons::{LdBuilding, LdGithub, LdMail, LdMapPin, LdTwitter},
};

#[cfg(target_arch = "wasm32")]
use crate::components::model::is_working_hour;
use crate::components::model::{image_srcset, unsplash_with_width};
use crate::config::{HeroBioLink, HeroConfig, HeroSocialLink};
use crate::localization::{Locale, translate, translate_template, use_locale};

const PROFILE_PHOTO: Asset = asset!("/assets/profile-photo.jpg", AssetOptions::builder());

#[derive(Debug, Clone, PartialEq, Eq)]
struct BioSegment {
    text: String,
    italic: bool,
    link: Option<HeroBioLink>,
}

fn bio_segments(
    bio: &str,
    italic_phrases: &[String],
    links: &[(String, &HeroBioLink)],
) -> Vec<BioSegment> {
    let phrases = italic_phrases
        .iter()
        .map(String::as_str)
        .filter(|phrase| !phrase.is_empty())
        .collect::<Vec<_>>();
    let candidates = links
        .iter()
        .filter(|(text, _)| !text.is_empty())
        .map(|(text, link)| (text.as_str(), Some(*link)))
        .chain(phrases.into_iter().map(|phrase| (phrase, None)))
        .collect::<Vec<_>>();
    let mut segments = Vec::new();
    let mut remaining = bio;

    while !remaining.is_empty() {
        let next = candidates
            .iter()
            .filter_map(|(text, link)| remaining.find(*text).map(|start| (start, *text, *link)))
            .min_by(|(left_start, left_text, _), (right_start, right_text, _)| {
                left_start
                    .cmp(right_start)
                    .then_with(|| right_text.len().cmp(&left_text.len()))
            });

        let Some((start, text, link)) = next else {
            segments.push(BioSegment {
                text: remaining.to_owned(),
                italic: false,
                link: None,
            });
            break;
        };

        if start > 0 {
            segments.push(BioSegment {
                text: remaining[..start].to_owned(),
                italic: false,
                link: None,
            });
        }
        segments.push(BioSegment {
            text: (*text).to_owned(),
            italic: link.is_none(),
            link: link.cloned(),
        });
        remaining = &remaining[start + text.len()..];
    }

    segments
}

#[component]
pub fn Hero(config: &'static HeroConfig) -> Element {
    let locale = use_locale();
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
        translate(locale, "hero.available", &config.available_title)
    } else {
        translate(locale, "hero.offline", &config.offline_title)
    };
    let biography = translate(locale, "hero.bio", &config.bio);
    let italic_phrases = if locale == Locale::ChineseSimplified {
        vec![
            translate(locale, "hero.bio_italic.cuhk", ""),
            translate(locale, "hero.bio_italic.slai", ""),
            translate(locale, "hero.bio_italic.netease", ""),
        ]
    } else {
        config.bio_italic_phrases.clone()
    };
    let bio_links = config
        .bio_links
        .iter()
        .enumerate()
        .map(|(index, link)| (translate(locale, &bio_link_key(index), &link.text), link))
        .collect::<Vec<_>>();
    let local_profile = config.image_url == "/assets/profile-photo.jpg";
    let image_source = if local_profile {
        PROFILE_PHOTO.to_string()
    } else {
        unsplash_with_width(&config.image_url, 576)
    };
    let image_sources = (!local_profile).then(|| image_srcset(&config.image_url, &[320, 576, 800]));

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
                                alt: translate(locale, "hero.image_alt", &config.image_alt),
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
                                span { {translate(locale, "hero.location", &config.location)} }
                            }
                            div { class: "contact-line",
                                Icon { icon: LdBuilding, width: 18, height: 18, class: "contact-icon" }
                                span { {translate(locale, "hero.company", &config.company)} }
                            }
                        }
                    }

                    div { id: config.about_anchor.clone(), class: "hero-content entrance-up",
                        h1 { class: "hero-name",
                            if locale == Locale::ChineseSimplified {
                                span { class: "gradient-name", {translate(locale, "hero.name", "谈旭宁")} }
                            } else {
                                {config.first_name.clone()}
                                " "
                                span { class: "gradient-name", {config.last_name.clone()} }
                            }
                        }
                        p { class: "hero-role", {translate(locale, "hero.role", &config.role)} }

                        div { class: "glass-card bio-card",
                            p {
                                for segment in bio_segments(&biography, &italic_phrases, &bio_links) {
                                    if let Some(link) = &segment.link {
                                        a {
                                            class: "bio-link",
                                            href: link.url.clone(),
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            strong { {segment.text} }
                                        }
                                    } else if segment.italic {
                                        em { {segment.text} }
                                    } else {
                                        {segment.text}
                                    }
                                }
                            }
                        }

                        div { class: "interests-block",
                            h2 { class: "eyebrow",
                                {translate(locale, "hero.interests_title", &config.interests_title)}
                            }
                            div { class: "interest-list",
                                for (index, interest) in config.interests.iter().enumerate() {
                                    span { class: "interest-chip",
                                        {translate(locale, interest_key(index), &interest.name)}
                                    }
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
    let locale = use_locale();
    let fallback = format!("{} link: {}", link.platform, link.username);
    let label = translate_template(
        locale,
        "hero.social_link_template",
        &fallback,
        &[("platform", &link.platform), ("username", &link.username)],
    );
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

fn interest_key(index: usize) -> &'static str {
    match index {
        0 => "hero.interest.ai_science",
        1 => "hero.interest.energy",
        2 => "hero.interest.multi_agent",
        3 => "hero.interest.trustworthy",
        4 => "hero.interest.quantum",
        _ => "hero.interest.unknown",
    }
}

fn bio_link_key(index: usize) -> String {
    format!("hero.bio_link.{}.text", index + 1)
}

#[component]
fn SocialIcon(name: String) -> Element {
    match name.as_str() {
        "twitter" => rsx! { Icon { icon: LdTwitter, width: 16, height: 16 } },
        "github" => rsx! { Icon { icon: LdGithub, width: 16, height: 16 } },
        _ => rsx! { Icon { icon: LdMail, width: 16, height: 16 } },
    }
}

#[cfg(test)]
mod tests {
    use super::{BioSegment, bio_segments};
    use crate::config::HeroBioLink;

    #[test]
    fn biography_segments_preserve_text_and_emphasis_order() {
        let bio = "Study at Alpha Institute and Beta Lab.";
        let phrases = vec!["Beta Lab".to_owned(), "Alpha Institute".to_owned()];

        let segments = bio_segments(bio, &phrases, &[]);

        assert_eq!(
            segments,
            vec![
                BioSegment {
                    text: "Study at ".to_owned(),
                    italic: false,
                    link: None,
                },
                BioSegment {
                    text: "Alpha Institute".to_owned(),
                    italic: true,
                    link: None,
                },
                BioSegment {
                    text: " and ".to_owned(),
                    italic: false,
                    link: None,
                },
                BioSegment {
                    text: "Beta Lab".to_owned(),
                    italic: true,
                    link: None,
                },
                BioSegment {
                    text: ".".to_owned(),
                    italic: false,
                    link: None,
                },
            ]
        );
        assert_eq!(
            segments
                .iter()
                .map(|segment| segment.text.as_str())
                .collect::<String>(),
            bio
        );
    }

    #[test]
    fn biography_segments_link_advisor_names() {
        let bio = "Advised by Prof. Junhua Zhao and Prof. Jianwei Huang.";
        let first = HeroBioLink {
            text: "Prof. Junhua Zhao".to_owned(),
            url: "https://www.zhaojunhua.org/".to_owned(),
        };
        let second = HeroBioLink {
            text: "Prof. Jianwei Huang".to_owned(),
            url: "https://jianwei.cuhk.edu.cn/".to_owned(),
        };
        let links = vec![(first.text.clone(), &first), (second.text.clone(), &second)];

        let segments = bio_segments(bio, &[], &links);

        assert_eq!(
            segments
                .iter()
                .filter(|segment| segment.link.is_some())
                .map(|segment| segment.text.as_str())
                .collect::<Vec<_>>(),
            vec!["Prof. Junhua Zhao", "Prof. Jianwei Huang"]
        );
        assert_eq!(
            segments
                .iter()
                .map(|segment| segment.link.is_some())
                .collect::<Vec<_>>(),
            vec![false, true, false, true, false]
        );
    }
}
