use dioxus::prelude::*;

use crate::components::{
    Activity, Footer, Hero, InteractiveBackground, Navbar, News, Research, ScrollToTop, Teaching,
    url_origin,
};
use crate::config::{SiteConfig, site_config};
use crate::localization::{Locale, translate, use_locale};
#[cfg(target_arch = "wasm32")]
use crate::localization::{SystemLanguageTracker, apply_browser_locale};

const FAVICON: Asset = asset!("/assets/favicon.png");
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
    let locale = use_signal(Locale::default);
    use_context_provider(|| locale);
    #[cfg(target_arch = "wasm32")]
    let _system_language_tracker =
        use_hook(move || std::rc::Rc::new(SystemLanguageTracker::install(locale)));
    #[cfg(target_arch = "wasm32")]
    use_effect(move || apply_browser_locale(locale));

    let current_locale = locale();
    let title = translate(current_locale, "site.title", &config.site.title);
    let description = translate(current_locale, "site.description", &config.site.description);
    let social_image_alt = translate(
        current_locale,
        "site.social_image_alt",
        &config.site.social_image_alt,
    );
    let contribution_origin = url_origin(&config.activity.api_url);

    rsx! {
        document::Title { {title.clone()} }
        document::Meta {
            name: "description",
            content: description.clone(),
        }
        document::Meta { name: "robots", content: config.site.robots.clone() }
        document::Meta {
            name: "google-site-verification",
            content: config.site.google_site_verification.clone(),
        }
        document::Meta {
            property: "og:title",
            content: title.clone(),
        }
        document::Meta {
            property: "og:description",
            content: description.clone(),
        }
        document::Meta {
            property: "og:image",
            content: config.site.social_image_url.clone(),
        }
        document::Meta { property: "og:image:alt", content: social_image_alt.clone() }
        document::Meta { property: "og:type", content: "website" }
        document::Meta { property: "og:url", content: config.site.canonical_url.clone() }
        document::Meta { name: "twitter:card", content: config.site.social_card.clone() }
        document::Meta { name: "twitter:title", content: title }
        document::Meta { name: "twitter:description", content: description }
        document::Meta { name: "twitter:image", content: config.site.social_image_url.clone() }
        document::Meta { name: "twitter:image:alt", content: social_image_alt }
        document::Link { rel: "canonical", href: config.site.canonical_url.clone() }
        if let Some(origin) = contribution_origin {
            document::Link { rel: "preconnect", href: origin.clone(), crossorigin: "" }
            document::Link { rel: "dns-prefetch", href: origin }
        }
        document::Link { rel: "icon", r#type: "image/png", sizes: "256x256", href: FAVICON }
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
    let locale = use_locale();
    let build_year = env!("ARCADEMIC_BUILD_YEAR")
        .parse::<i32>()
        .unwrap_or(config.footer.fixed_year);
    let footer_year = config.footer_year(build_year);
    let footer_text = config.footer_text(build_year);
    let scroll_top_visible = use_signal(|| false);

    #[cfg(target_arch = "wasm32")]
    let _system_theme_tracker = use_hook(|| std::rc::Rc::new(SystemThemeTracker::install()));
    #[cfg(target_arch = "wasm32")]
    use_effect(apply_system_theme);
    let design_style = format!(
        "--primary: {}; --primary-dark: {}; --secondary: {}; --light-bg: {}; \
         --dark-bg: {}; --light-text: {}; --dark-text: {}; --light-card: {}; \
         --dark-card: {}; --theme-duration: {}ms; --image-duration: {}ms; \
         --hero-image-duration: {}ms; --hero-content-duration: {}ms; \
         --hero-content-delay: {}ms; --section-duration: {}ms; \
         --news-duration: {}ms; --teaching-duration: {}ms; \
         --heatmap-duration: {}ms; --entrance-easing: {}; \
         --hover-lift: {}px; --heatmap-hover-scale: {};",
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
        config.animation.hero_image_duration_ms,
        config.animation.hero_content_duration_ms,
        config.animation.hero_content_delay_ms,
        config.animation.section_duration_ms,
        config.animation.news_item_duration_ms,
        config.animation.teaching_duration_ms,
        config.animation.heatmap_cell_duration_ms,
        config.animation.entrance_easing,
        config.animation.hover_lift_px,
        config.animation.heatmap_hover_scale,
    );

    rsx! {
        div {
            class: "app-root",
            "data-locale": locale.html_language(),
            "data-theme": "system",
            "data-animation-enabled": config.animation.enabled.to_string(),
            "data-respect-reduced-motion": config.animation.respect_reduced_motion.to_string(),
            style: design_style,
            a { class: "skip-link", href: "#main-content",
                {translate(locale, "site.skip_to_content", &config.site.skip_to_content_label)}
            }
            InteractiveBackground { config: &config.background }
            Navbar {
                config: &config.navbar,
                scroll_top_visible,
                scroll_top_threshold: config.scroll_to_top.threshold_px,
            }

            main { id: "main-content", class: "site-main", tabindex: "-1",
                Hero { config: &config.hero }
                if config.news.enabled {
                    News { config: &config.news, animation: &config.animation }
                }
                if config.publications.enabled {
                    Research { config: &config.publications, animation: &config.animation }
                }
                if config.teaching.enabled {
                    Teaching { config: &config.teaching, animation: &config.animation }
                }
                if config.activity.enabled {
                    Activity { config: &config.activity, animation: &config.animation }
                }
            }

            Footer { config: &config.footer, text: footer_text, year: footer_year }
            ScrollToTop { config: &config.scroll_to_top, visible: scroll_top_visible }
        }
    }
}

#[cfg(target_arch = "wasm32")]
struct SystemThemeTracker {
    query: web_sys::MediaQueryList,
    listener: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>,
}

#[cfg(target_arch = "wasm32")]
impl SystemThemeTracker {
    fn install() -> Option<Self> {
        use wasm_bindgen::JsCast as _;

        let query = web_sys::window()?
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()?;
        let listener = wasm_bindgen::closure::Closure::wrap(Box::new(move |_event| {
            apply_system_theme();
        })
            as Box<dyn FnMut(web_sys::Event)>);
        query
            .add_event_listener_with_callback("change", listener.as_ref().unchecked_ref())
            .ok()?;

        Some(Self { query, listener })
    }
}

#[cfg(target_arch = "wasm32")]
fn apply_system_theme() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(query) = window
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
    else {
        return;
    };
    let Some(root) = window
        .document()
        .and_then(|document| document.query_selector(".app-root").ok())
        .flatten()
    else {
        return;
    };
    let _ = root.set_attribute("data-theme", if query.matches() { "dark" } else { "light" });
}

#[cfg(target_arch = "wasm32")]
impl Drop for SystemThemeTracker {
    fn drop(&mut self) {
        use wasm_bindgen::JsCast as _;

        let _ = self
            .query
            .remove_event_listener_with_callback("change", self.listener.as_ref().unchecked_ref());
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
