use dioxus::prelude::*;
use serde::Deserialize;

use arcademic_rust::components::{Activity, Footer, Hero, News, Research, Teaching};
use arcademic_rust::config::{SiteConfig, site_config};

#[derive(Deserialize)]
struct AlternateFixture {
    site: AlternateSite,
    navbar: AlternateNavbar,
    hero: AlternateHero,
    news: AlternateHeading,
    publications: AlternateHeading,
    teaching: AlternateHeading,
    activity: AlternateHeading,
    footer: AlternateFooter,
}

#[derive(Deserialize)]
struct AlternateSite {
    title: String,
    description: String,
}

#[derive(Deserialize)]
struct AlternateNavbar {
    brand: String,
}

#[derive(Deserialize)]
struct AlternateHero {
    first_name: String,
    last_name: String,
    role: String,
    location: String,
    company: String,
    bio: String,
    bio_italic_phrases: Vec<String>,
    image_alt: String,
    interests: Vec<String>,
}

#[derive(Deserialize)]
struct AlternateHeading {
    heading: String,
}

#[derive(Deserialize)]
struct AlternateFooter {
    owner: String,
    powered_by: String,
    hosted_by: String,
}

#[component]
fn ConfiguredPage(config: &'static SiteConfig) -> Element {
    let footer_year = config.footer_year(config.footer.fixed_year);
    let footer_text = config.footer_text(config.footer.fixed_year);
    rsx! {
        p { {config.site.title.clone()} }
        p { {config.navbar.brand.clone()} }
        Hero { config: &config.hero }
        News { config: &config.news, animation: &config.animation }
        Research { config: &config.publications, animation: &config.animation }
        Teaching { config: &config.teaching, animation: &config.animation }
        Activity { config: &config.activity, animation: &config.animation }
        Footer { config: &config.footer, text: footer_text, year: footer_year }
    }
}

#[test]
fn alternate_fixture_replaces_the_production_identity_and_content() {
    let production = site_config();
    let forbidden = [
        production.navbar.brand.clone(),
        production.hero.company.clone(),
        production.news.items[0].title.clone(),
        production.activity.vibe_profile_url.clone(),
        production.footer.owner.clone(),
    ];
    let fixture: AlternateFixture =
        toml::from_str(include_str!("fixtures/alternate-identity.toml"))
            .expect("alternate fixture");
    let mut alternate = production.clone();
    apply_fixture(&mut alternate, fixture);
    alternate.validate().expect("alternate configuration");

    let alternate = Box::leak(Box::new(alternate));
    let mut dom =
        VirtualDom::new_with_props(ConfiguredPage, ConfiguredPageProps { config: alternate });
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);

    assert!(html.contains(&alternate.site.title));
    assert!(html.contains(&alternate.hero.first_name));
    assert!(html.contains(&alternate.news.heading));
    assert!(html.contains(&alternate.publications.heading));
    assert!(html.contains(&alternate.teaching.heading));
    assert!(html.contains(&alternate.activity.heading));
    for value in forbidden {
        assert!(!html.contains(&value), "production value leaked: {value}");
    }
}

fn apply_fixture(config: &mut SiteConfig, fixture: AlternateFixture) {
    config.site.title = fixture.site.title;
    config.site.description = fixture.site.description;
    config.navbar.brand = fixture.navbar.brand;
    config.hero.first_name = fixture.hero.first_name;
    config.hero.last_name = fixture.hero.last_name;
    config.hero.role = fixture.hero.role;
    config.hero.location = fixture.hero.location;
    config.hero.company = fixture.hero.company;
    config.hero.bio = fixture.hero.bio;
    config.hero.bio_italic_phrases = fixture.hero.bio_italic_phrases;
    config.hero.image_alt = fixture.hero.image_alt;
    for (interest, replacement) in config.hero.interests.iter_mut().zip(fixture.hero.interests) {
        interest.name = replacement;
    }
    for (index, social) in config.hero.social_links.iter_mut().enumerate() {
        social.platform = format!("Archive Contact {index}");
        social.username = format!("archive-{index}");
        social.url = format!("https://example.org/contact/{index}");
    }

    config.news.heading = fixture.news.heading;
    for (index, item) in config.news.items.iter_mut().enumerate() {
        item.date = format!("Note {index}");
        item.title = format!("Archive update {index}");
        item.description = format!("Alternate archive description {index}.");
        item.links.clear();
    }

    config.publications.heading = fixture.publications.heading;
    for (index, item) in config.publications.items.iter_mut().enumerate() {
        item.title = format!("Analytical note {index}");
        item.venue = format!("Archive Volume {index}");
        item.authors = vec![format!("A. Author {index}")];
        item.description = format!("Alternate publication description {index}.");
        item.tags = vec![format!("Archive{index}")];
        item.image_alt = format!("Archive illustration {index}");
    }

    config.teaching.heading = fixture.teaching.heading;
    for (index, item) in config.teaching.items.iter_mut().enumerate() {
        item.code = format!("ADA{index}00");
        item.title = format!("Analytical engine lecture {index}");
        item.description = format!("Alternate lecture description {index}.");
    }

    config.activity.heading = fixture.activity.heading;
    "https://example.org/archive/vibe-badge.svg".clone_into(&mut config.activity.vibe_badge_url);
    "https://example.org/archive/vibe".clone_into(&mut config.activity.vibe_profile_url);
    "Archive VibeUsage".clone_into(&mut config.activity.vibe_badge_alt);
    config.footer.owner = fixture.footer.owner;
    config.footer.powered_by = fixture.footer.powered_by;
    config.footer.hosted_by = fixture.footer.hosted_by;
    config.footer.technologies.clear();
}
