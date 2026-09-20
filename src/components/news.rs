use dioxus::prelude::*;

use crate::components::reveal::use_reveal_observer;
use crate::config::{AnimationConfig, NewsConfig, NewsItemLink, TagStyle};
use crate::localization::{translate, use_locale};

#[component]
pub fn News(config: &'static NewsConfig, animation: &'static AnimationConfig) -> Element {
    use_reveal_observer("#experience [data-reveal]");
    let locale = use_locale();
    let descriptions = config
        .items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let description = translate(locale, &news_key(index, "description"), &item.description);
            let links = item
                .links
                .iter()
                .enumerate()
                .map(|(link_index, link)| {
                    (
                        translate(locale, &news_link_key(index, link_index), &link.text),
                        link,
                    )
                })
                .collect::<Vec<_>>();
            description_segments(&description, &links)
        })
        .collect::<Vec<_>>();

    rsx! {
        section { id: config.anchor.clone(), class: "news-section content-section",
            div { class: "news-container",
                h2 { class: "section-heading news-heading reveal reveal-up", "data-reveal": "",
                    style: format!("--reveal-duration: {}ms", animation.section_duration_ms),
                    span { class: "section-accent", aria_hidden: "true" }
                    {translate(locale, "news.heading", &config.heading)}
                }
                div { class: "news-timeline",
                    for (index, item) in config.items.iter().enumerate() {
                        article {
                            class: "news-item reveal reveal-card",
                            "data-reveal": "",
                            style: format!(
                                "--reveal-delay: {}ms; --reveal-duration: {}ms",
                                index * animation.item_stagger_ms as usize,
                                animation.news_item_duration_ms,
                            ),
                            div { class: "timeline-dot", aria_hidden: "true",
                                span { class: "timeline-dot-ring" }
                            }
                            div { class: "glass-card news-card",
                                div { class: "news-meta",
                                    time { {translate(locale, &news_key(index, "date"), &item.date)} }
                                    span {
                                        class: "news-tag",
                                        style: tag_style(config, &item.tag),
                                        {translate(locale, &news_key(index, "tag"), &item.tag)}
                                    }
                                }
                                h3 { {translate(locale, &news_key(index, "title"), &item.title)} }
                                p {
                                    for segment in descriptions[index].clone() {
                                        if let Some(link) = &segment.link {
                                            a {
                                                class: "news-description-link",
                                                href: link.url.clone(),
                                                target: "_blank",
                                                rel: "noopener noreferrer",
                                                {segment.text}
                                            }
                                        } else {
                                            {segment.text}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn news_key(index: usize, field: &str) -> String {
    format!("news.item.{}.{}", index + 1, field)
}

fn news_link_key(index: usize, link_index: usize) -> String {
    format!("news.item.{}.link.{}.text", index + 1, link_index + 1)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DescriptionSegment {
    text: String,
    link: Option<NewsItemLink>,
}

fn description_segments(
    description: &str,
    links: &[(String, &NewsItemLink)],
) -> Vec<DescriptionSegment> {
    let links = links
        .iter()
        .filter(|(text, _)| !text.is_empty())
        .collect::<Vec<_>>();
    let mut segments = Vec::new();
    let mut remaining = description;

    while !remaining.is_empty() {
        let next = links
            .iter()
            .filter_map(|(text, link)| {
                remaining
                    .find(text.as_str())
                    .map(|start| (start, text, link))
            })
            .min_by(|(left_start, left_text, _), (right_start, right_text, _)| {
                left_start
                    .cmp(right_start)
                    .then_with(|| right_text.len().cmp(&left_text.len()))
            });

        let Some((start, text, link)) = next else {
            segments.push(DescriptionSegment {
                text: remaining.to_owned(),
                link: None,
            });
            break;
        };

        if start > 0 {
            segments.push(DescriptionSegment {
                text: remaining[..start].to_owned(),
                link: None,
            });
        }
        segments.push(DescriptionSegment {
            text: (*text).clone(),
            link: Some((*link).clone()),
        });
        remaining = &remaining[start + text.len()..];
    }

    segments
}

fn tag_style(config: &NewsConfig, tag: &str) -> String {
    let style = config
        .tag_styles
        .iter()
        .find(|style| style.name == tag)
        .or_else(|| config.tag_styles.first());
    style.map_or_else(String::new, tag_css_variables)
}

fn tag_css_variables(style: &TagStyle) -> String {
    format!(
        "--tag-bg: {}; --tag-text: {}; --tag-dark-bg: {}; --tag-dark-text: {};",
        style.light_background, style.light_text, style.dark_background, style.dark_text
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_tag_uses_its_own_palette() {
        let config = &crate::config::site_config().news;
        let style = tag_style(config, "Industry");
        assert!(style.contains("#f3e8ff"));
        assert!(style.contains("#581c8766"));
    }

    #[test]
    fn unknown_tag_falls_back_to_the_first_palette() {
        let config = &crate::config::site_config().news;
        assert_eq!(tag_style(config, "Unknown"), tag_style(config, "Education"));
    }

    #[test]
    fn description_segments_link_configured_names_in_order() {
        let first = NewsItemLink {
            text: "Prof. Junhua Zhao".to_owned(),
            url: "https://www.zhaojunhua.org/".to_owned(),
        };
        let second = NewsItemLink {
            text: "Prof. Jianwei Huang".to_owned(),
            url: "https://jianwei.cuhk.edu.cn/".to_owned(),
        };
        let description = "Advised by Prof. Junhua Zhao and Prof. Jianwei Huang.";
        let links = vec![(first.text.clone(), &first), (second.text.clone(), &second)];

        let segments = description_segments(description, &links);

        assert_eq!(
            segments,
            vec![
                DescriptionSegment {
                    text: "Advised by ".to_owned(),
                    link: None,
                },
                DescriptionSegment {
                    text: first.text.clone(),
                    link: Some(first),
                },
                DescriptionSegment {
                    text: " and ".to_owned(),
                    link: None,
                },
                DescriptionSegment {
                    text: second.text.clone(),
                    link: Some(second),
                },
                DescriptionSegment {
                    text: ".".to_owned(),
                    link: None,
                },
            ]
        );
    }
}
