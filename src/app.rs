use dioxus::prelude::*;

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
    let mut theme = use_signal(|| config.site.default_theme);
    let theme_name = theme().as_str();

    rsx! {
        div { class: "phase-one-shell", "data-theme": theme_name,
            header {
                nav { aria_label: config.navbar.mobile_menu_open_label.clone(),
                    a { href: config.navbar.brand_url.clone(), {config.navbar.brand.clone()} }
                    for link in &config.navbar.links {
                        a { href: link.href.clone(), {link.name.clone()} }
                    }
                    button {
                        r#type: "button",
                        aria_label: config.navbar.theme_toggle_label.clone(),
                        onclick: move |_| theme.set(theme().toggled()),
                        {config.navbar.theme_toggle_label.clone()}
                    }
                    if config.navbar.show_cv {
                        a { href: config.navbar.cv_url.clone(), {config.navbar.cv_label.clone()} }
                    }
                }
            }

            main {
                section { id: config.hero.anchor.clone(),
                    img {
                        src: config.hero.image_url.clone(),
                        alt: config.hero.image_alt.clone(),
                        width: "576",
                        height: "576",
                    }
                    div { id: config.hero.about_anchor.clone(),
                        h1 {
                            {config.hero.first_name.clone()}
                            " "
                            {config.hero.last_name.clone()}
                        }
                        p { {config.hero.role.clone()} }
                        p { {config.hero.location.clone()} }
                        p { {config.hero.company.clone()} }
                        p { {config.hero.bio.clone()} }
                        h2 { {config.hero.interests_title.clone()} }
                        ul {
                            for interest in &config.hero.interests {
                                li { {interest.name.clone()} }
                            }
                        }
                        ul {
                            for social in &config.hero.social_links {
                                li {
                                    a {
                                        href: social.url.clone(),
                                        aria_label: format!(
                                            "{} link: {}",
                                            social.platform, social.username
                                        ),
                                        {social.platform.clone()}
                                    }
                                }
                            }
                        }
                    }
                }

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

            footer { p { {footer_text} } }
            button {
                r#type: "button",
                title: config.scroll_to_top.title.clone(),
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
