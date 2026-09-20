use std::io;
use std::path::Path;
use std::{cell::RefCell, rc::Rc};

use lol_html::html_content::ContentType;
use lol_html::{RewriteStrSettings, element, rewrite_str, text};

use crate::config::SiteConfig;

/// Finalizes and verifies a generated Pages index.
///
/// # Errors
///
/// Returns an error when the file cannot be read/written, HTML rewriting
/// fails, a configured value is absent, or asset paths omit the base path.
pub fn finalize_pages(
    index_path: &Path,
    base_path: &str,
    config: &SiteConfig,
    build_year: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(index_path)?;
    let language = config.site.language.clone();
    let body_theme_variables = format!(
        "--light-bg: {}; --dark-bg: {}; --light-text: {}; --dark-text: {};",
        config.design.light_background,
        config.design.dark_background,
        config.design.light_text,
        config.design.dark_text,
    );
    let wasm_preload = wasm_preload_markup(index_path, base_path)?;
    let rewritten = rewrite_str(
        &source,
        RewriteStrSettings {
            element_content_handlers: vec![
                element!("html", move |element| {
                    element.set_attribute("lang", &language)?;
                    Ok(())
                }),
                element!("body", move |element| {
                    element.set_attribute("style", &body_theme_variables)?;
                    Ok(())
                }),
                element!("head", move |element| {
                    element.append(&wasm_preload, ContentType::Html);
                    Ok(())
                }),
                element!("[href]", |element| {
                    normalize_root_reference(element, "href")?;
                    Ok(())
                }),
                element!("[src]", |element| {
                    normalize_root_reference(element, "src")?;
                    Ok(())
                }),
            ],
            ..RewriteStrSettings::default()
        },
    )
    .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;

    verify_configured_content(&rewritten, base_path, config, build_year)?;
    std::fs::write(index_path, rewritten)?;
    Ok(())
}

fn wasm_preload_markup(index_path: &Path, base_path: &str) -> io::Result<String> {
    let assets = index_path
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "index has no parent"))?
        .join("assets");
    let wasm = std::fs::read_dir(assets)?
        .filter_map(Result::ok)
        .map(|entry| entry.file_name())
        .find(|name| {
            Path::new(name)
                .extension()
                .is_some_and(|extension| extension == "wasm")
        })
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "generated WASM asset is missing")
        })?;
    let wasm = wasm.to_string_lossy();
    let base = base_path.trim_matches('/');
    let href = if base.is_empty() {
        format!("/assets/{wasm}")
    } else {
        format!("/{base}/assets/{wasm}")
    };
    Ok(format!(
        r#"<link rel="preload" href="{href}" as="fetch" type="application/wasm" crossorigin>"#
    ))
}

fn normalize_root_reference(
    element: &mut lol_html::html_content::Element<'_, '_>,
    attribute: &str,
) -> Result<(), lol_html::errors::AttributeNameError> {
    if let Some(value) = element.get_attribute(attribute)
        && let Some(normalized) = value.strip_prefix("/./")
    {
        element.set_attribute(attribute, &format!("/{normalized}"))?;
    }
    Ok(())
}

/// Writes a GitHub Pages 404 document that preserves SSR content and redirects
/// unknown paths to the configured site root.
///
/// # Errors
///
/// Returns an error when the finalized index cannot be read, rewritten, or
/// written to `not_found_path`.
pub fn write_pages_404(
    index_path: &Path,
    not_found_path: &Path,
    base_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(index_path)?;
    let redirect = pages_root(base_path);
    let refresh_source = format!("0; url={redirect}");
    let refresh = html_escape::encode_double_quoted_attribute(&refresh_source);
    let meta = format!(r#"<meta http-equiv="refresh" content="{refresh}">"#);
    let rewritten = rewrite_str(
        &source,
        RewriteStrSettings {
            element_content_handlers: vec![
                element!("head", move |element| {
                    element.append(&meta, ContentType::Html);
                    Ok(())
                }),
                element!("body", |element| {
                    element.set_attribute("data-pages-404", "true")?;
                    Ok(())
                }),
            ],
            ..RewriteStrSettings::default()
        },
    )
    .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
    std::fs::write(not_found_path, rewritten)?;
    Ok(())
}

fn pages_root(base_path: &str) -> String {
    let base_path = base_path.trim_matches('/');
    if base_path.is_empty() {
        "/".to_owned()
    } else {
        format!("/{base_path}/")
    }
}

pub fn configured_values(config: &SiteConfig, build_year: i32) -> Vec<String> {
    let mut values = base_configured_values(config, build_year);

    values.extend(
        config
            .navbar
            .links
            .iter()
            .flat_map(|link| [link.name.clone(), link.href.clone()]),
    );
    values.extend(config.hero.interests.iter().map(|item| item.name.clone()));
    values.extend(config.hero.bio_italic_phrases.clone());
    values.extend(
        config
            .hero
            .bio_links
            .iter()
            .flat_map(|link| [link.text.clone(), link.url.clone()]),
    );
    values.extend(config.hero.social_links.iter().flat_map(|link| {
        [
            link.platform.clone(),
            link.url.clone(),
            link.username.clone(),
        ]
    }));
    if config.news.enabled {
        values.extend(config.news.items.iter().flat_map(|item| {
            let mut item_values = [
                item.date.clone(),
                item.title.clone(),
                item.description.clone(),
                item.tag.clone(),
            ]
            .into_iter()
            .collect::<Vec<_>>();
            item_values.extend(
                item.links
                    .iter()
                    .flat_map(|link| [link.text.clone(), link.url.clone()]),
            );
            item_values
        }));
    }
    if config.publications.enabled {
        values.extend(
            config
                .publications
                .stats
                .iter()
                .flat_map(|stat| [stat.year.to_string(), stat.count.to_string()]),
        );
        values.extend(config.publications.items.iter().flat_map(|item| {
            let mut item_values = vec![
                item.title.clone(),
                item.venue.clone(),
                item.year.to_string(),
                item.description.clone(),
                item.image_alt.clone(),
            ];
            item_values.extend(item.authors.clone());
            item_values.extend(item.tags.clone());
            item_values.extend([item.pdf_url.clone(), item.code_url.clone()]);
            item_values
        }));
    }
    if config.teaching.enabled {
        values.extend(config.teaching.items.iter().flat_map(|item| {
            [
                item.code.clone(),
                item.title.clone(),
                item.semester.clone(),
                item.description.clone(),
                item.materials_url.clone(),
            ]
        }));
    }
    if config.activity.enabled {
        values.extend(config.activity.months.clone());
        values.extend(config.activity.day_labels.clone());
    }
    values.extend(
        config
            .footer
            .technologies
            .iter()
            .map(|technology| technology.name.clone()),
    );
    values
        .into_iter()
        .filter(|value| !value.is_empty())
        .collect()
}

fn base_configured_values(config: &SiteConfig, build_year: i32) -> Vec<String> {
    let mut values = vec![
        config.site.title.clone(),
        config.site.description.clone(),
        config.site.canonical_url.clone(),
        config.site.social_image_url.clone(),
        config.site.social_image_alt.clone(),
        config.site.robots.clone(),
        config.site.google_site_verification.clone(),
        config.site.skip_to_content_label.clone(),
        config.navbar.primary_navigation_label.clone(),
        config.navbar.brand.clone(),
        config.navbar.mobile_menu_open_label.clone(),
        config.hero.first_name.clone(),
        config.hero.last_name.clone(),
        config.hero.role.clone(),
        config.hero.location.clone(),
        config.hero.company.clone(),
        config.hero.image_alt.clone(),
        config.hero.interests_title.clone(),
        config.hero.offline_title.clone(),
        config.scroll_to_top.title.clone(),
        config.footer_text(build_year),
    ];
    if config.navbar.show_cv {
        values.push(config.navbar.cv_label.clone());
    }
    if config.news.enabled {
        values.push(config.news.heading.clone());
    }
    if config.publications.enabled {
        values.extend([
            config.publications.heading.clone(),
            config.publications.subtitle.clone(),
            config.publications.total_label.clone(),
            config.publications.total_value.clone(),
            config.publications.citations_label.clone(),
            config.publications.citations_value.clone(),
            config.publications.output_heading.clone(),
            config.publications.output_range.clone(),
            config.publications.papers_label.clone(),
            config.publications.pdf_label.clone(),
            config.publications.code_label.clone(),
        ]);
    }
    if config.teaching.enabled {
        values.extend([
            config.teaching.heading.clone(),
            config.teaching.semester_prefix.clone(),
        ]);
    }
    if config.activity.enabled {
        values.extend([
            config.activity.heading.clone(),
            config.activity.vibe_badge_url.clone(),
            config.activity.vibe_profile_url.clone(),
            config.activity.vibe_badge_alt.clone(),
            config.activity.github_username.clone(),
            config.activity.profile_url.clone(),
            config.activity.profile_label.clone(),
            config.activity.loading_label.clone(),
            config.activity.less_label.clone(),
            config.activity.more_label.clone(),
        ]);
    }
    values
}

fn verify_configured_content(
    html: &str,
    base_path: &str,
    config: &SiteConfig,
    build_year: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let decoded = html_escape::decode_html_entities(html);
    for expected in configured_values(config, build_year) {
        if !decoded.contains(&expected) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("generated index is missing configured value: {expected}"),
            )
            .into());
        }
    }
    verify_biography_markup(html, config)?;

    if !base_path.is_empty() {
        let prefix = format!("/{}/", base_path.trim_matches('/'));
        if !html.contains(&format!("href=\"{prefix}assets/"))
            || !html.contains(&format!("src=\"{prefix}"))
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("generated assets do not use base path {prefix}"),
            )
            .into());
        }
    }
    Ok(())
}

fn verify_biography_markup(html: &str, config: &SiteConfig) -> io::Result<()> {
    let biography = Rc::new(RefCell::new(String::new()));
    let biography_output = Rc::clone(&biography);
    let italic_text = Rc::new(RefCell::new(String::new()));
    let italic_output = Rc::clone(&italic_text);
    let italic_count = Rc::new(RefCell::new(0_usize));
    let italic_count_output = Rc::clone(&italic_count);

    rewrite_str(
        html,
        RewriteStrSettings {
            element_content_handlers: vec![
                text!(".bio-card p", move |chunk| {
                    biography_output.borrow_mut().push_str(chunk.as_str());
                    Ok(())
                }),
                text!(".bio-card em", move |chunk| {
                    italic_output.borrow_mut().push_str(chunk.as_str());
                    Ok(())
                }),
                element!(".bio-card em", move |_| {
                    *italic_count_output.borrow_mut() += 1;
                    Ok(())
                }),
            ],
            ..RewriteStrSettings::default()
        },
    )
    .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;

    let biography = {
        let value = biography.borrow();
        html_escape::decode_html_entities(value.as_str()).into_owned()
    };
    if biography != config.hero.bio {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "generated biography text does not match hero.bio",
        ));
    }

    let italic_text = {
        let value = italic_text.borrow();
        html_escape::decode_html_entities(value.as_str()).into_owned()
    };
    if *italic_count.borrow() != config.hero.bio_italic_phrases.len()
        || config
            .hero
            .bio_italic_phrases
            .iter()
            .any(|phrase| !italic_text.contains(phrase))
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "generated biography does not contain the configured italic phrases",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_values_cover_every_repeated_collection() {
        let config = crate::config::site_config();
        let values = configured_values(config, config.footer.fixed_year);
        for item in &config.news.items {
            assert!(values.contains(&item.title));
        }
        for item in &config.publications.items {
            assert!(values.contains(&item.title));
        }
        for item in &config.teaching.items {
            assert!(values.contains(&item.title));
        }
    }

    #[test]
    fn content_verifier_reports_the_missing_value() {
        let config = crate::config::site_config();
        let error = verify_configured_content(
            "<html><body></body></html>",
            "",
            config,
            config.footer.fixed_year,
        )
        .expect_err("empty page must fail");
        assert!(error.to_string().contains(&config.site.title));
    }

    #[test]
    fn biography_verifier_requires_complete_text_and_emphasis() {
        let config = crate::config::site_config();
        let biography = &config.hero.bio;
        let marked_up = config
            .hero
            .bio_italic_phrases
            .iter()
            .fold(biography.clone(), |markup, phrase| {
                markup.replacen(phrase, &format!("<em>{phrase}</em>"), 1)
            });
        let html = format!(r#"<div class="bio-card"><p>{marked_up}</p></div>"#);

        verify_biography_markup(&html, config).expect("valid biography markup");

        let unformatted = format!(r#"<div class="bio-card"><p>{biography}</p></div>"#);
        let error = verify_biography_markup(&unformatted, config)
            .expect_err("unformatted biography must fail");
        assert!(error.to_string().contains("configured italic phrases"));
    }

    #[test]
    fn pages_404_preserves_content_and_targets_the_configured_root() {
        let directory =
            std::env::temp_dir().join(format!("arcademic-finalizer-{}", std::process::id()));
        std::fs::create_dir_all(&directory).expect("temporary directory");
        let index = directory.join("index.html");
        let not_found = directory.join("404.html");
        std::fs::write(
            &index,
            "<html><head><title>Profile</title></head><body>SSR body</body></html>",
        )
        .expect("temporary index");

        write_pages_404(&index, &not_found, "/arcademic-rust/").expect("404 document");
        let output = std::fs::read_to_string(&not_found).expect("generated 404");
        assert!(output.contains("SSR body"));
        assert!(output.contains("data-pages-404=\"true\""));
        assert!(output.contains("0; url=/arcademic-rust/"));

        std::fs::remove_dir_all(directory).expect("remove temporary directory");
    }
}
