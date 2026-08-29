use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::components::initialize_theme;
use crate::components::{
    Activity, Footer, Hero, InteractiveBackground, Navbar, News, Research, ScrollToTop, Teaching,
};
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
    let scroll_y = use_signal(|| 0.0_f64);

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
            Navbar { config: &config.navbar, theme, scroll_y }

            main { class: "site-main",
                Hero { config: &config.hero }
                News { config: &config.news }
                Research { config: &config.publications }
                Teaching { config: &config.teaching }
                Activity { config: &config.activity }
            }

            Footer { config: &config.footer, text: footer_text }
            ScrollToTop { config: &config.scroll_to_top, scroll_y }
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
