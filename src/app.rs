use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::components::initialize_theme;
use crate::components::{Hero, InteractiveBackground, Navbar};
use crate::config::{SiteConfig, site_config};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
}

#[component]
pub fn App() -> Element {
    let config = site_config();

    rsx! {
        document::Title { {config.site.title.clone()} }
        document::Meta {
            name: "description",
            content: config.site.description.clone(),
        }
        document::Meta {
            property: "og:title",
            content: config.site.title.clone(),
        }
        document::Meta {
            property: "og:description",
            content: config.site.description.clone(),
        }
        document::Meta {
            property: "og:image",
            content: config.site.social_image_url.clone(),
        }
        document::Link { rel: "canonical", href: config.site.canonical_url.clone() }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! { SiteShell { config: site_config() } }
}

#[component]
fn SiteShell(config: &'static SiteConfig) -> Element {
    let build_year = env!("ARCADEMIC_BUILD_YEAR").parse::<i32>().unwrap_or(2026);
    let footer_text = config.footer_text(build_year);
    #[allow(unused_mut)]
    let mut theme = use_signal(|| config.site.default_theme);

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        let Some(window) = web_sys::window() else {
            return;
        };
        let stored = window
            .local_storage()
            .ok()
            .flatten()
            .and_then(|storage| storage.get_item(&config.site.theme_storage_key).ok())
            .flatten();
        let prefers_dark = window
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
            .is_some_and(|query| query.matches());
        theme.set(initialize_theme(
            config.site.default_theme,
            stored.as_deref(),
            prefers_dark,
        ));
    });

    let theme_name = theme().as_str();
    let design_style = format!(
        "--primary: {}; --primary-dark: {}; --secondary: {}; --light-bg: {}; \
         --dark-bg: {}; --light-text: {}; --dark-text: {}; --light-card: {}; \
         --dark-card: {}; --theme-duration: {}ms; --image-duration: {}ms; \
         --hover-lift: {}px;",
        config.design.primary,
        config.design.primary_dark,
        config.design.secondary,
        config.design.light_background,
        config.design.dark_background,
        config.design.light_text,
        config.design.dark_text,
        config.design.light_card,
        config.design.dark_card,
        config.animation.theme_transition_ms,
        config.animation.image_transition_ms,
        config.animation.hover_lift_px,
    );

    rsx! {
        div { class: "app-root", "data-theme": theme_name, style: design_style,
            InteractiveBackground { config: &config.background }
            Navbar { config: &config.navbar, theme }

            main { class: "site-main",
                Hero { config: &config.hero }

                section { id: config.news.anchor.clone(),
                    h2 { {config.news.heading.clone()} }
                    ol {
                        for item in &config.news.items {
                            li {
                                time { {item.date.clone()} }
                                span { {item.tag.clone()} }
                                h3 { {item.title.clone()} }
                                p { {item.description.clone()} }
                            }
                        }
                    }
                }

                section { id: config.publications.anchor.clone(),
                    h2 { {config.publications.heading.clone()} }
                    p { {config.publications.subtitle.clone()} }
                    dl {
                        dt { {config.publications.total_label.clone()} }
                        dd { {config.publications.total_value.clone()} }
                        dt { {config.publications.citations_label.clone()} }
                        dd { {config.publications.citations_value.clone()} }
                    }
                    h3 { {config.publications.output_heading.clone()} }
                    p { {config.publications.output_range.clone()} }
                    ul {
                        for stat in &config.publications.stats {
                            li {
                                "{stat.year}: {stat.count} "
                                {config.publications.papers_label.clone()}
                            }
                        }
                    }
                    div {
                        for publication in &config.publications.items {
                            article {
                                img {
                                    src: publication.image_url.clone(),
                                    alt: publication.image_alt.clone(),
                                    width: "576",
                                    height: "384",
                                    loading: "lazy",
                                }
                                p { "{publication.venue} • {publication.year}" }
                                h3 { {publication.title.clone()} }
                                p {
                                    for (index, author) in publication.authors.iter().enumerate() {
                                        span {
                                            {author.clone()}
                                            if index + 1 < publication.authors.len() { ", " }
                                        }
                                    }
                                }
                                p { {publication.description.clone()} }
                                ul {
                                    for tag in &publication.tags {
                                        li { "#{tag}" }
                                    }
                                }
                                if publication.pdf_url.is_empty() {
                                    button { r#type: "button", disabled: true,
                                        {config.publications.pdf_label.clone()}
                                    }
                                } else {
                                    a { href: publication.pdf_url.clone(),
                                        {config.publications.pdf_label.clone()}
                                    }
                                }
                                if publication.code_url.is_empty() {
                                    button { r#type: "button", disabled: true,
                                        {config.publications.code_label.clone()}
                                    }
                                } else {
                                    a { href: publication.code_url.clone(),
                                        {config.publications.code_label.clone()}
                                    }
                                }
                            }
                        }
                    }
                }

                section { id: config.teaching.anchor.clone(),
                    h2 { {config.teaching.heading.clone()} }
                    div {
                        for course in &config.teaching.items {
                            article {
                                p { {course.code.clone()} }
                                h3 { {course.title.clone()} }
                                p {
                                    {config.teaching.semester_prefix.clone()}
                                    ": "
                                    {course.semester.clone()}
                                }
                                p { {course.description.clone()} }
                            }
                        }
                    }
                }

                section { id: config.activity.anchor.clone(),
                    h2 { {config.activity.heading.clone()} }
                    div { role: "img",
                        aria_label: config.activity.heading.clone(),
                        p { {config.activity.months.join(" ")} }
                        p { {config.activity.day_labels.join(" ")} }
                        p {
                            {config.activity.less_label.clone()}
                            " – "
                            {config.activity.more_label.clone()}
                        }
                    }
                }
            }

            footer { class: "site-footer", p { {footer_text} } }
            button {
                class: "scroll-top-placeholder",
                r#type: "button",
                title: config.scroll_to_top.title.clone(),
                aria_hidden: "true",
                tabindex: "-1",
                {config.scroll_to_top.title.clone()}
            }
        }
    }
}

#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
#[allow(clippy::unused_async)]
pub async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}
