use dioxus::prelude::*;

use crate::components::reveal::use_reveal_observer;
use crate::config::{AnimationConfig, NewsConfig, TagStyle};

#[component]
pub fn News(config: &'static NewsConfig, animation: &'static AnimationConfig) -> Element {
    use_reveal_observer("#news [data-reveal]");

    rsx! {
        section { id: config.anchor.clone(), class: "news-section content-section",
            div { class: "news-container",
                h2 { class: "section-heading news-heading reveal reveal-up", "data-reveal": "",
                    style: format!("--reveal-duration: {}ms", animation.section_duration_ms),
                    span { class: "section-accent", aria_hidden: "true" }
                    {config.heading.clone()}
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
                                    time { {item.date.clone()} }
                                    span {
                                        class: "news-tag",
                                        style: tag_style(config, &item.tag),
                                        {item.tag.clone()}
                                    }
                                }
                                h3 { {item.title.clone()} }
                                p { {item.description.clone()} }
                            }
                        }
                    }
                }
            }
        }
    }
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
        let style = tag_style(config, "Research");
        assert!(style.contains("#f3e8ff"));
        assert!(style.contains("#581c8766"));
    }

    #[test]
    fn unknown_tag_falls_back_to_the_first_palette() {
        let config = &crate::config::site_config().news;
        assert_eq!(tag_style(config, "Unknown"), tag_style(config, "Product"));
    }
}
