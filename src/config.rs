use std::collections::HashSet;
use std::fmt::Write as _;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

const SITE_TOML: &str = include_str!("../config/site.toml");

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to parse site configuration: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("invalid site configuration:\n{0}")]
    Validation(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteConfig {
    pub site: SiteMetaConfig,
    pub navbar: NavbarConfig,
    pub hero: HeroConfig,
    pub news: NewsConfig,
    pub publications: PublicationsConfig,
    pub teaching: TeachingConfig,
    pub activity: ActivityConfig,
    pub background: BackgroundConfig,
    pub scroll_to_top: ScrollToTopConfig,
    pub footer: FooterConfig,
    pub animation: AnimationConfig,
    pub design: DesignConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SiteMetaConfig {
    pub title: String,
    pub description: String,
    pub language: String,
    pub canonical_url: String,
    pub favicon_url: String,
    pub social_image_url: String,
    pub default_theme: ThemeChoice,
    pub theme_storage_key: String,
    pub base_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeChoice {
    System,
    Light,
    Dark,
}

impl ThemeChoice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    #[must_use]
    pub const fn toggled(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::System | Self::Light => Self::Dark,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NavbarConfig {
    pub brand: String,
    pub brand_url: String,
    pub cv_label: String,
    pub cv_url: String,
    pub show_cv: bool,
    pub theme_toggle_label: String,
    pub mobile_menu_open_label: String,
    pub mobile_menu_close_label: String,
    pub links: Vec<NavLink>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NavLink {
    pub name: String,
    pub href: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroConfig {
    pub anchor: String,
    pub about_anchor: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub location: String,
    pub company: String,
    pub bio: String,
    pub image_url: String,
    pub image_alt: String,
    pub interests_title: String,
    pub timezone_offset_hours: i8,
    pub working_hours_start: u8,
    pub working_hours_end: u8,
    pub available_title: String,
    pub offline_title: String,
    pub interests: Vec<HeroInterest>,
    pub social_links: Vec<HeroSocialLink>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroInterest {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeroSocialLink {
    pub platform: String,
    pub icon: String,
    pub url: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewsConfig {
    pub anchor: String,
    pub heading: String,
    pub items: Vec<NewsItem>,
    pub tag_styles: Vec<TagStyle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewsItem {
    pub id: String,
    pub date: String,
    pub title: String,
    pub description: String,
    pub tag: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TagStyle {
    pub name: String,
    pub light_background: String,
    pub light_text: String,
    pub dark_background: String,
    pub dark_text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationsConfig {
    pub anchor: String,
    pub heading: String,
    pub subtitle: String,
    pub total_label: String,
    pub total_value: String,
    pub citations_label: String,
    pub citations_value: String,
    pub output_heading: String,
    pub output_range: String,
    pub papers_label: String,
    pub pdf_label: String,
    pub code_label: String,
    pub highlight_author: String,
    pub stats: Vec<PublicationStat>,
    pub items: Vec<PublicationItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationStat {
    pub year: u16,
    pub count: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationItem {
    pub id: String,
    pub title: String,
    pub venue: String,
    pub year: u16,
    pub authors: Vec<String>,
    pub description: String,
    pub tags: Vec<String>,
    pub image_url: String,
    pub image_alt: String,
    pub pdf_url: String,
    pub code_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TeachingConfig {
    pub anchor: String,
    pub heading: String,
    pub semester_prefix: String,
    pub items: Vec<TeachingItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TeachingItem {
    pub id: String,
    pub code: String,
    pub title: String,
    pub semester: String,
    pub description: String,
    pub materials_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityConfig {
    pub anchor: String,
    pub heading: String,
    pub less_label: String,
    pub more_label: String,
    pub level_title_prefix: String,
    pub weeks: u8,
    pub days: u8,
    pub months: Vec<String>,
    pub day_labels: Vec<String>,
    pub seed_week_multiplier: u32,
    pub seed_day_multiplier: u32,
    pub seed_cross_multiplier: u32,
    pub seed_offset: u32,
    pub level_thresholds: [u8; 4],
    pub level_colors: Vec<ActivityColor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityColor {
    pub light: String,
    pub dark: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackgroundConfig {
    pub pointer_glow_size_px: u16,
    pub pointer_glow_blur_px: u16,
    pub pointer_glow_color: String,
    pub pointer_glow_fade: String,
    pub blinking_grid_enabled: bool,
    pub blinking_grid_cell_size_px: u16,
    pub blinking_grid_min_cells: u8,
    pub blinking_grid_max_cells: u8,
    pub blinking_grid_interval_ms: u32,
    pub blinking_grid_duration_ms: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScrollToTopConfig {
    pub title: String,
    pub threshold_px: u32,
    pub behavior: ScrollBehavior,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScrollBehavior {
    Auto,
    Smooth,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FooterConfig {
    pub owner: String,
    pub powered_by: String,
    pub hosted_by: String,
    pub year_mode: FooterYearMode,
    pub fixed_year: i32,
    pub text_template: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FooterYearMode {
    Build,
    Fixed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnimationConfig {
    pub enabled: bool,
    pub respect_reduced_motion: bool,
    pub entrance_easing: String,
    pub hero_image_duration_ms: u32,
    pub hero_content_delay_ms: u32,
    pub section_duration_ms: u32,
    pub news_item_duration_ms: u32,
    pub item_stagger_ms: u32,
    pub heatmap_cell_duration_ms: u32,
    pub heatmap_cell_stagger_ms: u32,
    pub theme_transition_ms: u32,
    pub image_transition_ms: u32,
    pub hover_lift_px: f32,
    pub heatmap_hover_scale: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DesignConfig {
    pub primary: String,
    pub primary_dark: String,
    pub secondary: String,
    pub light_background: String,
    pub dark_background: String,
    pub light_text: String,
    pub dark_text: String,
    pub light_card: String,
    pub dark_card: String,
}

impl SiteConfig {
    /// Parses and validates a complete site configuration.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Parse`] for malformed TOML and
    /// [`ConfigError::Validation`] when a cross-field invariant fails.
    pub fn from_toml(source: &str) -> Result<Self, ConfigError> {
        let config: Self = toml::from_str(source)?;
        config.validate()?;
        Ok(config)
    }

    /// Validates cross-field invariants that Serde cannot express.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Validation`] with all discovered issues.
    pub fn validate(&self) -> Result<(), ConfigError> {
        let mut issues = Vec::new();

        self.validate_required_strings(&mut issues);
        self.validate_anchors(&mut issues);
        self.validate_urls(&mut issues);
        self.validate_identifiers(&mut issues);
        self.validate_ranges(&mut issues);
        self.validate_footer_template(&mut issues);

        if issues.is_empty() {
            Ok(())
        } else {
            let mut message = String::new();
            for issue in issues {
                let _ = writeln!(message, "- {issue}");
            }
            Err(ConfigError::Validation(message.trim_end().to_owned()))
        }
    }

    pub fn anchor_ids(&self) -> [&str; 6] {
        [
            &self.hero.anchor,
            &self.hero.about_anchor,
            &self.news.anchor,
            &self.publications.anchor,
            &self.teaching.anchor,
            &self.activity.anchor,
        ]
    }

    pub fn footer_text(&self, build_year: i32) -> String {
        let year = match self.footer.year_mode {
            FooterYearMode::Build => build_year,
            FooterYearMode::Fixed => self.footer.fixed_year,
        };

        self.footer
            .text_template
            .replace("{year}", &year.to_string())
            .replace("{owner}", &self.footer.owner)
            .replace("{powered_by}", &self.footer.powered_by)
            .replace("{hosted_by}", &self.footer.hosted_by)
    }

    fn validate_required_strings(&self, issues: &mut Vec<String>) {
        let fields = [
            ("site.title", self.site.title.as_str()),
            ("site.description", self.site.description.as_str()),
            ("site.language", self.site.language.as_str()),
            (
                "site.theme_storage_key",
                self.site.theme_storage_key.as_str(),
            ),
            ("navbar.brand", self.navbar.brand.as_str()),
            ("hero.first_name", self.hero.first_name.as_str()),
            ("hero.last_name", self.hero.last_name.as_str()),
            ("hero.role", self.hero.role.as_str()),
            ("hero.bio", self.hero.bio.as_str()),
            ("news.heading", self.news.heading.as_str()),
            ("publications.heading", self.publications.heading.as_str()),
            ("teaching.heading", self.teaching.heading.as_str()),
            ("activity.heading", self.activity.heading.as_str()),
            ("footer.text_template", self.footer.text_template.as_str()),
        ];

        for (field, value) in fields {
            if value.trim().is_empty() {
                issues.push(format!("{field} must not be empty"));
            }
        }
    }

    fn validate_anchors(&self, issues: &mut Vec<String>) {
        let anchors = self.anchor_ids();
        let mut unique = HashSet::new();

        for anchor in anchors {
            if anchor.is_empty()
                || !anchor
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == '-')
            {
                issues.push(format!("section anchor `{anchor}` is invalid"));
            } else if !unique.insert(anchor) {
                issues.push(format!("duplicate section anchor `{anchor}`"));
            }
        }

        for (index, link) in self.navbar.links.iter().enumerate() {
            if let Some(target) = link.href.strip_prefix('#')
                && !target.is_empty()
                && !unique.contains(target)
            {
                issues.push(format!(
                    "navbar.links[{index}].href targets unknown anchor `{target}`"
                ));
            }
        }
    }

    fn validate_urls(&self, issues: &mut Vec<String>) {
        validate_external_url(
            "site.canonical_url",
            &self.site.canonical_url,
            false,
            issues,
        );
        validate_link("site.favicon_url", &self.site.favicon_url, false, issues);
        validate_link(
            "site.social_image_url",
            &self.site.social_image_url,
            false,
            issues,
        );
        validate_link("navbar.brand_url", &self.navbar.brand_url, true, issues);
        validate_link("navbar.cv_url", &self.navbar.cv_url, true, issues);
        validate_link("hero.image_url", &self.hero.image_url, false, issues);

        for (index, link) in self.navbar.links.iter().enumerate() {
            validate_link(
                &format!("navbar.links[{index}].href"),
                &link.href,
                true,
                issues,
            );
        }

        for (index, link) in self.hero.social_links.iter().enumerate() {
            validate_external_url(
                &format!("hero.social_links[{index}].url"),
                &link.url,
                true,
                issues,
            );
        }

        for (index, item) in self.publications.items.iter().enumerate() {
            validate_link(
                &format!("publications.items[{index}].image_url"),
                &item.image_url,
                false,
                issues,
            );
            validate_optional_link(
                &format!("publications.items[{index}].pdf_url"),
                &item.pdf_url,
                issues,
            );
            validate_optional_link(
                &format!("publications.items[{index}].code_url"),
                &item.code_url,
                issues,
            );
        }

        for (index, item) in self.teaching.items.iter().enumerate() {
            validate_optional_link(
                &format!("teaching.items[{index}].materials_url"),
                &item.materials_url,
                issues,
            );
        }
    }

    fn validate_identifiers(&self, issues: &mut Vec<String>) {
        validate_unique_ids(
            "news.items",
            self.news.items.iter().map(|item| item.id.as_str()),
            issues,
        );
        validate_unique_ids(
            "publications.items",
            self.publications.items.iter().map(|item| item.id.as_str()),
            issues,
        );
        validate_unique_ids(
            "teaching.items",
            self.teaching.items.iter().map(|item| item.id.as_str()),
            issues,
        );

        let known_tags: HashSet<_> = self
            .news
            .tag_styles
            .iter()
            .map(|style| style.name.as_str())
            .collect();
        for (index, item) in self.news.items.iter().enumerate() {
            if !known_tags.contains(item.tag.as_str()) {
                issues.push(format!(
                    "news.items[{index}].tag `{}` has no configured tag style",
                    item.tag
                ));
            }
        }
    }

    fn validate_ranges(&self, issues: &mut Vec<String>) {
        if !(-23..=23).contains(&self.hero.timezone_offset_hours) {
            issues.push("hero.timezone_offset_hours must be between -23 and 23".to_owned());
        }
        if self.hero.working_hours_start > 23 {
            issues.push("hero.working_hours_start must be between 0 and 23".to_owned());
        }
        if !(1..=24).contains(&self.hero.working_hours_end) {
            issues.push("hero.working_hours_end must be between 1 and 24".to_owned());
        }
        if self.hero.working_hours_start >= self.hero.working_hours_end {
            issues.push("hero working hour start must be before end".to_owned());
        }
        if !(1..=53).contains(&self.activity.weeks) {
            issues.push("activity.weeks must be between 1 and 53".to_owned());
        }
        if !(1..=7).contains(&self.activity.days) {
            issues.push("activity.days must be between 1 and 7".to_owned());
        }
        if self.activity.months.len() < 2 {
            issues.push("activity.months must contain at least two labels".to_owned());
        }
        if self.activity.day_labels.is_empty() {
            issues.push("activity.day_labels must not be empty".to_owned());
        }
        if !self
            .activity
            .level_thresholds
            .windows(2)
            .all(|pair| pair[0] < pair[1])
        {
            issues.push("activity.level_thresholds must be strictly increasing".to_owned());
        }
        if self.activity.level_colors.len() != 5 {
            issues.push("activity.level_colors must contain exactly five levels".to_owned());
        }
        if self.background.blinking_grid_min_cells > self.background.blinking_grid_max_cells {
            issues.push("background.blinking_grid_min_cells must not exceed max_cells".to_owned());
        }
        if self.background.blinking_grid_cell_size_px == 0 {
            issues.push("background.blinking_grid_cell_size_px must be positive".to_owned());
        }

        let durations = [
            (
                "animation.hero_image_duration_ms",
                self.animation.hero_image_duration_ms,
            ),
            (
                "animation.hero_content_delay_ms",
                self.animation.hero_content_delay_ms,
            ),
            (
                "animation.section_duration_ms",
                self.animation.section_duration_ms,
            ),
            (
                "animation.news_item_duration_ms",
                self.animation.news_item_duration_ms,
            ),
            ("animation.item_stagger_ms", self.animation.item_stagger_ms),
            (
                "animation.heatmap_cell_duration_ms",
                self.animation.heatmap_cell_duration_ms,
            ),
            (
                "animation.heatmap_cell_stagger_ms",
                self.animation.heatmap_cell_stagger_ms,
            ),
            (
                "animation.theme_transition_ms",
                self.animation.theme_transition_ms,
            ),
            (
                "animation.image_transition_ms",
                self.animation.image_transition_ms,
            ),
        ];
        for (field, duration) in durations {
            if duration > 60_000 {
                issues.push(format!("{field} must not exceed 60000"));
            }
        }
        if !self.animation.hover_lift_px.is_finite() || self.animation.hover_lift_px < 0.0 {
            issues.push("animation.hover_lift_px must be finite and non-negative".to_owned());
        }
        if !self.animation.heatmap_hover_scale.is_finite()
            || self.animation.heatmap_hover_scale <= 0.0
        {
            issues.push("animation.heatmap_hover_scale must be finite and positive".to_owned());
        }
    }

    fn validate_footer_template(&self, issues: &mut Vec<String>) {
        const REQUIRED: [&str; 4] = ["year", "owner", "powered_by", "hosted_by"];
        let placeholders = template_placeholders(&self.footer.text_template);

        for required in REQUIRED {
            if !placeholders.contains(required) {
                issues.push(format!(
                    "footer.text_template must include `{{{required}}}`"
                ));
            }
        }
        for placeholder in placeholders {
            if !REQUIRED.contains(&placeholder) {
                issues.push(format!(
                    "footer.text_template contains unknown placeholder `{{{placeholder}}}`"
                ));
            }
        }
    }
}

/// Returns the parsed configuration embedded in both server and web builds.
///
/// # Panics
///
/// Panics when the checked-in `config/site.toml` is invalid. CI validates the
/// same file before producing an artifact, so an invalid release cannot ship.
pub fn site_config() -> &'static SiteConfig {
    static CONFIG: OnceLock<SiteConfig> = OnceLock::new();
    CONFIG.get_or_init(|| {
        SiteConfig::from_toml(SITE_TOML).expect("embedded config/site.toml must be valid")
    })
}

pub fn join_base_path(base_path: &str, path: &str) -> String {
    let base = base_path.trim_matches('/');
    let relative = path.trim_start_matches('/');

    if base.is_empty() {
        format!("/{relative}")
    } else if relative.is_empty() {
        format!("/{base}/")
    } else {
        format!("/{base}/{relative}")
    }
}

fn validate_unique_ids<'a>(
    field: &str,
    ids: impl Iterator<Item = &'a str>,
    issues: &mut Vec<String>,
) {
    let mut unique = HashSet::new();
    for id in ids {
        if id.trim().is_empty() {
            issues.push(format!("{field} contains an empty id"));
        } else if !unique.insert(id) {
            issues.push(format!("{field} contains duplicate id `{id}`"));
        }
    }
}

fn validate_optional_link(field: &str, value: &str, issues: &mut Vec<String>) {
    if !value.is_empty() {
        validate_link(field, value, true, issues);
    }
}

fn validate_link(field: &str, value: &str, allow_fragment: bool, issues: &mut Vec<String>) {
    if value.starts_with('/') || (allow_fragment && value.starts_with('#')) {
        return;
    }
    validate_external_url(field, value, true, issues);
}

fn validate_external_url(field: &str, value: &str, allow_mailto: bool, issues: &mut Vec<String>) {
    let allowed = if allow_mailto {
        ["http", "https", "mailto"].as_slice()
    } else {
        ["http", "https"].as_slice()
    };

    match Url::parse(value) {
        Ok(url) if allowed.contains(&url.scheme()) => {}
        _ => issues.push(format!("{field} must use an allowed absolute URL scheme")),
    }
}

fn template_placeholders(template: &str) -> HashSet<&str> {
    let mut placeholders = HashSet::new();
    let mut remainder = template;

    while let Some(open) = remainder.find('{') {
        remainder = &remainder[open + 1..];
        let Some(close) = remainder.find('}') else {
            break;
        };
        placeholders.insert(&remainder[..close]);
        remainder = &remainder[close + 1..];
    }

    placeholders
}

#[cfg(test)]
mod tests {
    use super::*;

    fn production_source() -> &'static str {
        SITE_TOML
    }

    #[test]
    fn production_configuration_is_valid() {
        let config = SiteConfig::from_toml(production_source()).expect("production config");
        assert_eq!(config.hero.first_name, "Tony");
        assert_eq!(config.publications.items.len(), 3);
        assert_eq!(config.teaching.items.len(), 4);
    }

    #[test]
    fn unknown_fields_are_rejected() {
        let source = production_source().replace(
            "title = \"Tony Stark Academic Profile\"",
            "title = \"Tony Stark Academic Profile\"\nunknown = true",
        );
        let error = SiteConfig::from_toml(&source).expect_err("unknown key must fail");
        assert!(error.to_string().contains("unknown field `unknown`"));
    }

    #[test]
    fn missing_required_fields_are_rejected() {
        let source = production_source().replace(
            "description = \"Academic profile, research output, teaching, and activity for Tony Stark.\"\n",
            "",
        );
        let error = SiteConfig::from_toml(&source).expect_err("missing key must fail");
        assert!(error.to_string().contains("missing field `description`"));
    }

    #[test]
    fn duplicate_ids_are_rejected() {
        let source = production_source().replace("id = \"2\"\ndate", "id = \"1\"\ndate");
        let error = SiteConfig::from_toml(&source).expect_err("duplicate id must fail");
        assert!(
            error
                .to_string()
                .contains("news.items contains duplicate id `1`")
        );
    }

    #[test]
    fn unknown_navigation_anchor_is_rejected() {
        let source = production_source().replace("href = \"#news\"", "href = \"#missing\"");
        let error = SiteConfig::from_toml(&source).expect_err("unknown anchor must fail");
        assert!(
            error
                .to_string()
                .contains("targets unknown anchor `missing`")
        );
    }

    #[test]
    fn duplicate_section_anchors_are_rejected() {
        let source = production_source().replace(
            "[publications]\nanchor = \"publications\"",
            "[publications]\nanchor = \"news\"",
        );
        let error = SiteConfig::from_toml(&source).expect_err("duplicate anchor must fail");
        assert!(
            error
                .to_string()
                .contains("duplicate section anchor `news`")
        );
    }

    #[test]
    fn invalid_url_scheme_is_rejected() {
        let source = production_source().replace(
            "canonical_url = \"https://example.com/\"",
            "canonical_url = \"javascript:alert(1)\"",
        );
        let error = SiteConfig::from_toml(&source).expect_err("invalid URL must fail");
        assert!(error.to_string().contains("site.canonical_url"));
    }

    #[test]
    fn invalid_ranges_are_rejected() {
        let source =
            production_source().replace("working_hours_start = 9", "working_hours_start = 19");
        let error = SiteConfig::from_toml(&source).expect_err("hour range must fail");
        assert!(
            error
                .to_string()
                .contains("working hour start must be before end")
        );
    }

    #[test]
    fn invalid_activity_dimensions_are_rejected() {
        let source = production_source().replace("weeks = 52", "weeks = 0");
        let error = SiteConfig::from_toml(&source).expect_err("grid dimension must fail");
        assert!(
            error
                .to_string()
                .contains("activity.weeks must be between 1 and 53")
        );
    }

    #[test]
    fn invalid_animation_duration_is_rejected() {
        let source = production_source()
            .replace("section_duration_ms = 1200", "section_duration_ms = 60001");
        let error = SiteConfig::from_toml(&source).expect_err("duration must fail");
        assert!(
            error
                .to_string()
                .contains("animation.section_duration_ms must not exceed 60000")
        );
    }

    #[test]
    fn footer_template_is_checked_and_rendered() {
        let config = SiteConfig::from_toml(production_source()).expect("production config");
        assert_eq!(
            config.footer_text(2034),
            "© Copyright 2034 Tony Stark. Powered by Arc Reactor Core. Hosted by Jarvis."
        );

        let source = production_source().replace("{hosted_by}", "{mystery}");
        let error = SiteConfig::from_toml(&source).expect_err("placeholder must fail");
        assert!(
            error
                .to_string()
                .contains("unknown placeholder `{mystery}`")
        );
    }

    #[test]
    fn base_path_joining_is_deterministic() {
        assert_eq!(join_base_path("", "/assets/main.css"), "/assets/main.css");
        assert_eq!(
            join_base_path("/arcademic-rust/", "/assets/main.css"),
            "/arcademic-rust/assets/main.css"
        );
        assert_eq!(join_base_path("arcademic-rust", ""), "/arcademic-rust/");
    }

    #[test]
    fn theme_toggle_has_a_deterministic_initial_transition() {
        assert_eq!(ThemeChoice::System.toggled(), ThemeChoice::Dark);
        assert_eq!(ThemeChoice::Light.toggled(), ThemeChoice::Dark);
        assert_eq!(ThemeChoice::Dark.toggled(), ThemeChoice::Light);
    }
}
