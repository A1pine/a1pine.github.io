use dioxus::prelude::*;

use crate::components::model::{image_srcset, normalize_counts, unsplash_with_width};
use crate::components::reveal::use_reveal_observer;
use crate::config::PublicationsConfig;

#[component]
pub fn Research(config: &'static PublicationsConfig) -> Element {
    use_reveal_observer("#publications [data-reveal]");
    let counts = config
        .stats
        .iter()
        .map(|stat| stat.count)
        .collect::<Vec<_>>();
    let scales = normalize_counts(&counts);

    rsx! {
        section { id: config.anchor.clone(), class: "research-section content-section",
            div { class: "research-container",
                header { class: "research-header",
                    h2 { class: "reveal reveal-up", "data-reveal": "", {config.heading.clone()} }
                    p { class: "reveal reveal-up", "data-reveal": "", style: "--reveal-delay: 100ms", {config.subtitle.clone()} }
                }

                div { class: "research-stats",
                    div { class: "stat-summaries",
                        SummaryCard {
                            label: config.total_label.clone(),
                            value: config.total_value.clone(),
                            delay_ms: 0,
                        }
                        SummaryCard {
                            label: config.citations_label.clone(),
                            value: config.citations_value.clone(),
                            delay_ms: 150,
                        }
                    }

                    div { class: "glass-card output-card reveal reveal-card", "data-reveal": "",
                        div { class: "output-card-header",
                            h3 {
                                span { class: "stat-dot", aria_hidden: "true" }
                                {config.output_heading.clone()}
                            }
                            span { class: "output-range", {config.output_range.clone()} }
                        }
                        div { class: "bar-chart",
                            for (index, stat) in config.stats.iter().enumerate() {
                                div { class: "bar-column",
                                    div { class: "bar-tooltip",
                                        "{stat.count} "
                                        {config.papers_label.clone()}
                                        span { aria_hidden: "true" }
                                    }
                                    div { class: "bar-track",
                                        div {
                                            class: "bar-fill",
                                            style: format!(
                                                "--bar-scale: {}; --bar-delay: {}ms",
                                                scales[index], index * 100
                                            ),
                                        }
                                    }
                                    span { class: "bar-year", "{stat.year}" }
                                }
                            }
                        }
                    }
                }

                div { class: "publication-list",
                    for (index, publication) in config.items.iter().enumerate() {
                        article {
                            class: "glass-card publication-card reveal reveal-card",
                            "data-reveal": "",
                            style: format!("--reveal-delay: {}ms", index * 250),
                            div { class: "publication-image",
                                img {
                                    src: unsplash_with_width(&publication.image_url, 576),
                                    srcset: image_srcset(&publication.image_url, &[320, 576, 800]),
                                    sizes: "(min-width: 768px) 288px, calc(100vw - 80px)",
                                    alt: publication.image_alt.clone(),
                                    width: "576",
                                    height: "384",
                                    loading: "lazy",
                                    decoding: "async",
                                }
                            }
                            div { class: "publication-body",
                                p { class: "publication-venue", "{publication.venue} • {publication.year}" }
                                h3 { {publication.title.clone()} }
                                p { class: "publication-authors", {publication.authors.join(", ")} }
                                p { class: "publication-description", {publication.description.clone()} }
                                div { class: "publication-tags",
                                    for tag in &publication.tags { span { "#{tag}" } }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SummaryCard(label: String, value: String, delay_ms: usize) -> Element {
    rsx! {
        div {
            class: "glass-card summary-card reveal reveal-card",
            "data-reveal": "",
            style: format!("--reveal-delay: {delay_ms}ms"),
            div { class: "summary-label",
                span { class: "stat-dot", aria_hidden: "true" }
                span { {label} }
            }
            strong { {value} }
        }
    }
}
