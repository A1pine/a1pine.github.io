use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdArrowUpRight};

use crate::components::model::{
    PublicationActionKind, image_srcset, normalize_counts, publication_action_kind,
    unsplash_with_width,
};
use crate::components::reveal::use_reveal_observer;
use crate::config::{AnimationConfig, PublicationType, PublicationsConfig};
use crate::localization::{translate, translate_template, use_locale};

#[component]
pub fn Research(
    config: &'static PublicationsConfig,
    animation: &'static AnimationConfig,
) -> Element {
    use_reveal_observer("#publications [data-reveal]");
    let counts = config
        .stats
        .iter()
        .map(|stat| stat.count)
        .collect::<Vec<_>>();
    let scales = normalize_counts(&counts);
    let locale = use_locale();
    let empty_message = translate(locale, "publications.empty", &config.empty_message);
    let last_updated = (!config.last_updated.is_empty()).then(|| {
        translate_template(
            locale,
            "publications.last_updated_template",
            &config.last_updated_template,
            &[("date", config.last_updated.as_str())],
        )
    });

    rsx! {
        section { id: config.anchor.clone(), class: "research-section content-section",
            div { class: "research-container",
                header { class: "research-header",
                    h2 { class: "reveal reveal-up", "data-reveal": "", style: format!("--reveal-duration: {}ms", animation.section_duration_ms), {config.heading.clone()} }
                    p { class: "reveal reveal-up", "data-reveal": "", style: format!("--reveal-delay: 100ms; --reveal-duration: {}ms", animation.section_duration_ms), {config.subtitle.clone()} }
                }

                div { class: "research-stats",
                    div { class: "stat-summaries",
                        SummaryCard {
                            label: config.total_label.clone(),
                            value: config.total_value.clone(),
                            delay_ms: 0,
                            duration_ms: animation.section_duration_ms,
                        }
                        SummaryCard {
                            label: config.citations_label.clone(),
                            value: config.citations_value.clone(),
                            delay_ms: animation.stats_stagger_ms as usize,
                            duration_ms: animation.section_duration_ms,
                        }
                    }

                    div { class: "glass-card output-card reveal reveal-card", "data-reveal": "", style: format!("--reveal-duration: {}ms", animation.section_duration_ms),
                        div { class: "output-card-header",
                            h3 {
                                span { class: "stat-dot", aria_hidden: "true" }
                                {config.output_heading.clone()}
                            }
                            span { class: "output-range", {config.output_range.clone()} }
                        }
                        if !config.stats.is_empty() {
                            div {
                                class: "bar-chart",
                                tabindex: "0",
                                aria_label: config.output_heading.clone(),
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
                                                    scales[index], index * animation.bar_stagger_ms as usize
                                                ),
                                            }
                                        }
                                        span { class: "bar-year", "{stat.year}" }
                                    }
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
                            "data-source-id": publication.source_id.clone(),
                            "data-publication-type": publication.publication_type.as_str(),
                            "data-code-available": publication.code_available.to_string(),
                            style: format!(
                                "--reveal-delay: {}ms; --reveal-duration: {}ms",
                                index * animation.item_stagger_ms as usize,
                                animation.news_item_duration_ms,
                            ),
                            div { class: "publication-image",
                                if publication.image_url.is_empty() {
                                    div { class: "publication-cover",
                                        span { class: "publication-cover-mark", aria_hidden: "true" }
                                        span { class: "publication-cover-text",
                                            {publication.venue.clone()}
                                        }
                                    }
                                } else {
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
                            }
                            div { class: "publication-body",
                                div { class: "publication-topline",
                                    p { class: "publication-venue", "{publication.venue} • {publication.year}" }
                                    span { class: "publication-arrow", aria_hidden: "true",
                                        Icon { icon: LdArrowUpRight, width: 18, height: 18 }
                                    }
                                }
                                h3 { {publication.title.clone()} }
                                p { class: "publication-authors",
                                    for (author_index, author) in publication.authors.iter().enumerate() {
                                        span {
                                            class: if author == &config.highlight_author { "highlight-author" } else { "" },
                                            {author.clone()}
                                            if author_index + 1 < publication.authors.len() { ", " }
                                        }
                                    }
                                }
                                if !publication.description.is_empty() {
                                    p { class: "publication-description", {publication.description.clone()} }
                                }
                                div { class: "publication-footer",
                                    div { class: "publication-tags",
                                        PublicationTypeTag { publication_type: publication.publication_type }
                                        if !publication.tags.is_empty() {
                                            for tag in &publication.tags { span { "#{tag}" } }
                                        }
                                    }
                                    div { class: "publication-actions",
                                        PublicationAction {
                                            label: config.pdf_label.clone(),
                                            url: publication.pdf_url.clone(),
                                            primary: false,
                                        }
                                        if publication.code_available {
                                            PublicationAction {
                                                label: config.code_label.clone(),
                                                url: publication.code_url.clone(),
                                                primary: true,
                                            }
                                        }
                                        span {
                                            class: "publication-citations",
                                            {translate_template(
                                                locale,
                                                "publications.citation_template",
                                                &config.citation_template,
                                                &[("count", &publication.citations.to_string())],
                                            )}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if config.items.is_empty() {
                    div { class: "glass-card publication-empty reveal reveal-card", "data-reveal": "",
                        {empty_message.clone()}
                    }
                }
                if let Some(updated) = last_updated.clone() {
                    p { class: "publications-updated", {updated} }
                }
            }
        }
    }
}

#[component]
fn PublicationTypeTag(publication_type: PublicationType) -> Element {
    let locale = use_locale();
    let kind = publication_type.as_str();
    let label = translate(
        locale,
        &format!("publications.type.{kind}"),
        publication_type.label(),
    );
    rsx! {
        span {
            class: "publication-type publication-type-{kind}",
            {label}
        }
    }
}

#[component]
fn PublicationAction(label: String, url: String, primary: bool) -> Element {
    let class = if primary {
        "publication-action primary"
    } else {
        "publication-action secondary"
    };
    match publication_action_kind(&url) {
        PublicationActionKind::Disabled => rsx! {
            button { class, r#type: "button", disabled: true, {label} }
        },
        PublicationActionKind::Internal => rsx! {
            a { class, href: url, {label} }
        },
        PublicationActionKind::External => rsx! {
            a {
                class,
                href: url,
                target: "_blank",
                rel: "noopener noreferrer",
                {label}
            }
        },
    }
}

#[component]
fn SummaryCard(label: String, value: String, delay_ms: usize, duration_ms: u32) -> Element {
    rsx! {
        div {
            class: "glass-card summary-card reveal reveal-card",
            "data-reveal": "",
            style: format!("--reveal-delay: {delay_ms}ms; --reveal-duration: {duration_ms}ms"),
            div { class: "summary-label",
                span { class: "stat-dot", aria_hidden: "true" }
                span { {label} }
            }
            strong { {value} }
        }
    }
}
