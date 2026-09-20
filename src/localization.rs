use std::collections::BTreeMap;
use std::sync::OnceLock;

#[cfg(target_arch = "wasm32")]
use dioxus::prelude::WritableExt;
use dioxus::prelude::{Signal, try_consume_context};

const ZH_CN_JSON: &str = include_str!("../config/locales/zh-CN.json");

pub const REQUIRED_ZH_CN_KEYS: &[&str] = &[
    "site.title",
    "site.description",
    "site.social_image_alt",
    "site.skip_to_content",
    "navbar.primary_navigation",
    "navbar.brand",
    "navbar.cv",
    "navbar.mobile_open",
    "navbar.mobile_close",
    "navbar.home",
    "navbar.about",
    "navbar.experience",
    "navbar.activity",
    "hero.name",
    "hero.role",
    "hero.location",
    "hero.company",
    "hero.bio",
    "hero.bio_italic.cuhk",
    "hero.bio_italic.slai",
    "hero.bio_italic.netease",
    "hero.bio_link.1.text",
    "hero.bio_link.2.text",
    "hero.image_alt",
    "hero.interests_title",
    "hero.available",
    "hero.offline",
    "hero.interest.ai_science",
    "hero.interest.energy",
    "hero.interest.multi_agent",
    "hero.interest.trustworthy",
    "hero.interest.quantum",
    "hero.social_link_template",
    "news.heading",
    "news.item.1.date",
    "news.item.1.title",
    "news.item.1.description",
    "news.item.1.tag",
    "news.item.2.date",
    "news.item.2.title",
    "news.item.2.description",
    "news.item.2.tag",
    "news.item.3.date",
    "news.item.3.title",
    "news.item.3.description",
    "news.item.3.tag",
    "news.item.4.date",
    "news.item.4.title",
    "news.item.4.description",
    "news.item.4.tag",
    "news.item.4.link.1.text",
    "news.item.4.link.2.text",
    "activity.heading",
    "activity.vibe_alt",
    "activity.profile_label",
    "activity.loading",
    "activity.error",
    "activity.total_template",
    "activity.less",
    "activity.more",
    "activity.month.01",
    "activity.month.02",
    "activity.month.03",
    "activity.month.04",
    "activity.month.05",
    "activity.month.06",
    "activity.month.07",
    "activity.month.08",
    "activity.month.09",
    "activity.month.10",
    "activity.month.11",
    "activity.month.12",
    "activity.day.mon",
    "activity.day.wed",
    "activity.day.fri",
    "activity.contribution_template",
    "scroll_to_top.title",
    "footer.owner",
    "footer.powered_by",
    "footer.hosted_by",
    "footer.template",
    "footer.copyright_prefix",
    "footer.separator",
    "footer.and_separator",
    "footer.powered_suffix",
    "footer.hosted_prefix",
    "footer.hosted_suffix",
];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Locale {
    #[default]
    English,
    ChineseSimplified,
}

impl Locale {
    pub const fn html_language(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::ChineseSimplified => "zh-CN",
        }
    }
}

pub fn locale_from_languages<I, S>(languages: I) -> Locale
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    for language in languages {
        let language = language.as_ref().to_ascii_lowercase();
        if language == "zh" || language.starts_with("zh-") {
            return Locale::ChineseSimplified;
        }
        if language == "en" || language.starts_with("en-") {
            return Locale::English;
        }
    }
    Locale::English
}

pub fn use_locale() -> Locale {
    try_consume_context::<Signal<Locale>>().map_or(Locale::English, |locale| locale())
}

pub fn translate(locale: Locale, key: &str, fallback: &str) -> String {
    match locale {
        Locale::English => fallback.to_owned(),
        Locale::ChineseSimplified => chinese_catalog()
            .get(key)
            .map_or_else(|| fallback.to_owned(), Clone::clone),
    }
}

pub fn translate_template(
    locale: Locale,
    key: &str,
    fallback: &str,
    replacements: &[(&str, &str)],
) -> String {
    let mut output = translate(locale, key, fallback);
    for (name, value) in replacements {
        output = output.replace(&format!("{{{name}}}"), value);
    }
    output
}

fn chinese_catalog() -> &'static BTreeMap<String, String> {
    static CATALOG: OnceLock<BTreeMap<String, String>> = OnceLock::new();
    CATALOG.get_or_init(|| serde_json::from_str(ZH_CN_JSON).expect("valid zh-CN translation JSON"))
}

#[cfg(target_arch = "wasm32")]
pub struct SystemLanguageTracker {
    window: web_sys::Window,
    listener: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>,
}

#[cfg(target_arch = "wasm32")]
impl SystemLanguageTracker {
    pub fn install(locale: Signal<Locale>) -> Option<Self> {
        use wasm_bindgen::JsCast as _;

        let window = web_sys::window()?;
        let listener = wasm_bindgen::closure::Closure::wrap(Box::new(move |_event| {
            apply_browser_locale(locale);
        })
            as Box<dyn FnMut(web_sys::Event)>);
        window
            .add_event_listener_with_callback("languagechange", listener.as_ref().unchecked_ref())
            .ok()?;
        Some(Self { window, listener })
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for SystemLanguageTracker {
    fn drop(&mut self) {
        use wasm_bindgen::JsCast as _;

        let _ = self.window.remove_event_listener_with_callback(
            "languagechange",
            self.listener.as_ref().unchecked_ref(),
        );
    }
}

#[cfg(target_arch = "wasm32")]
pub fn apply_browser_locale(mut locale: Signal<Locale>) {
    let next = browser_locale();
    if locale() != next {
        locale.set(next);
    }
    if let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
    {
        let _ = root.set_attribute("lang", next.html_language());
        let _ = root.set_attribute("dir", "ltr");
    }
}

#[cfg(target_arch = "wasm32")]
fn browser_locale() -> Locale {
    let Some(window) = web_sys::window() else {
        return Locale::English;
    };
    let navigator = window.navigator();
    let mut languages = navigator
        .languages()
        .iter()
        .filter_map(|value| value.as_string())
        .collect::<Vec<_>>();
    if languages.is_empty()
        && let Some(language) = navigator.language()
    {
        languages.push(language);
    }
    locale_from_languages(languages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_negotiation_uses_the_first_supported_browser_language() {
        assert_eq!(
            locale_from_languages(["zh-Hans-CN", "en-US"]),
            Locale::ChineseSimplified
        );
        assert_eq!(locale_from_languages(["en-GB", "zh-CN"]), Locale::English);
        assert_eq!(locale_from_languages(["fr-FR"]), Locale::English);
    }

    #[test]
    fn chinese_catalog_contains_every_required_translation() {
        let catalog = chinese_catalog();
        for key in REQUIRED_ZH_CN_KEYS {
            assert!(
                catalog
                    .get(*key)
                    .is_some_and(|value| !value.trim().is_empty()),
                "missing Chinese translation: {key}"
            );
        }
    }

    #[test]
    fn templates_replace_named_values_and_keep_english_fallbacks() {
        assert_eq!(
            translate_template(
                Locale::ChineseSimplified,
                "activity.total_template",
                "{total} contributions",
                &[("total", "12")],
            ),
            "过去一年共 12 次贡献"
        );
        assert_eq!(
            translate(Locale::English, "missing", "English fallback"),
            "English fallback"
        );
    }
}
