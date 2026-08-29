use crate::config::ThemeChoice;

pub fn initialize_theme(
    configured: ThemeChoice,
    stored: Option<&str>,
    prefers_dark: bool,
) -> ThemeChoice {
    match stored {
        Some("dark") => ThemeChoice::Dark,
        Some("light") => ThemeChoice::Light,
        _ => resolve_system_theme(configured, prefers_dark),
    }
}

const fn resolve_system_theme(configured: ThemeChoice, prefers_dark: bool) -> ThemeChoice {
    match configured {
        ThemeChoice::Dark => ThemeChoice::Dark,
        ThemeChoice::System if prefers_dark => ThemeChoice::Dark,
        ThemeChoice::Light | ThemeChoice::System => ThemeChoice::Light,
    }
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
pub fn is_working_hour(utc_hour: i32, offset: i8, start: u8, end: u8) -> bool {
    let local_hour = (utc_hour + i32::from(offset)).rem_euclid(24);
    local_hour >= i32::from(start) && local_hour < i32::from(end)
}

pub fn unsplash_with_width(source: &str, width: u16) -> String {
    let Ok(mut url) = url::Url::parse(source) else {
        return source.to_owned();
    };
    if url.host_str() != Some("images.unsplash.com") {
        return source.to_owned();
    }

    {
        let mut query = url.query_pairs_mut();
        query.append_pair("auto", "format");
        query.append_pair("fit", "crop");
        query.append_pair("q", "75");
        query.append_pair("w", &width.to_string());
    }
    url.to_string()
}

pub fn image_srcset(source: &str, widths: &[u16]) -> String {
    widths
        .iter()
        .map(|width| format!("{} {width}w", unsplash_with_width(source, *width)))
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn url_origin(source: &str) -> Option<String> {
    let url = url::Url::parse(source).ok()?;
    let host = url.host_str()?;
    Some(format!("{}://{host}", url.scheme()))
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
pub fn scroll_progress(scroll_y: f64, max_scroll: f64) -> f64 {
    if max_scroll <= 0.0 {
        0.0
    } else {
        (scroll_y / max_scroll).clamp(0.0, 1.0)
    }
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn grid_axis_coordinate(random: f64, viewport_px: f64, cell_size_px: u16) -> u32 {
    let cells = (viewport_px / f64::from(cell_size_px)).ceil().max(1.0) as u32;
    let normalized = random.clamp(0.0, 1.0 - f64::EPSILON);
    (normalized * f64::from(cells)).floor() as u32
}

#[cfg_attr(not(any(target_arch = "wasm32", test)), allow(dead_code))]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn target_cell_count(random: f64, minimum: u8, maximum: u8) -> usize {
    let span = maximum.saturating_sub(minimum) + 1;
    let normalized = random.clamp(0.0, 1.0 - f64::EPSILON);
    usize::from(minimum) + (normalized * f64::from(span)).floor() as usize
}

pub fn normalize_counts(counts: &[u16]) -> Vec<f64> {
    let maximum = counts.iter().copied().max().unwrap_or(0);
    if maximum == 0 {
        return vec![0.0; counts.len()];
    }
    counts
        .iter()
        .map(|count| f64::from(*count) / f64::from(maximum))
        .collect()
}

pub fn activity_level(
    week: u32,
    day: u32,
    week_multiplier: u32,
    day_multiplier: u32,
    cross_multiplier: u32,
    offset: u32,
    thresholds: [u8; 4],
) -> u8 {
    let seed =
        (week * week_multiplier + day * day_multiplier + week * day * cross_multiplier + offset)
            % 100;
    let level = thresholds
        .iter()
        .filter(|threshold| seed > u32::from(**threshold))
        .count();
    u8::try_from(level).unwrap_or(4)
}

pub fn is_external_link(url: &str) -> bool {
    url.starts_with("https://") || url.starts_with("http://")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationActionKind {
    Disabled,
    Internal,
    External,
}

pub fn publication_action_kind(url: &str) -> PublicationActionKind {
    if url.is_empty() {
        PublicationActionKind::Disabled
    } else if is_external_link(url) {
        PublicationActionKind::External
    } else {
        PublicationActionKind::Internal
    }
}

pub fn should_show_scroll_to_top(scroll_y: f64, threshold: u32) -> bool {
    scroll_y > f64::from(threshold)
}

#[cfg(target_arch = "wasm32")]
pub fn persist_theme(storage_key: &str, theme: ThemeChoice) {
    if let Some(window) = web_sys::window()
        && let Ok(Some(storage)) = window.local_storage()
    {
        let _ = storage.set_item(storage_key, theme.as_str());
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn persist_theme(_storage_key: &str, _theme: ThemeChoice) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_initialization_prefers_valid_storage() {
        assert_eq!(
            initialize_theme(ThemeChoice::System, Some("dark"), false),
            ThemeChoice::Dark
        );
        assert_eq!(
            initialize_theme(ThemeChoice::System, Some("light"), true),
            ThemeChoice::Light
        );
    }

    #[test]
    fn theme_initialization_resolves_system_preference() {
        assert_eq!(
            initialize_theme(ThemeChoice::System, None, true),
            ThemeChoice::Dark
        );
        assert_eq!(
            initialize_theme(ThemeChoice::System, Some("invalid"), false),
            ThemeChoice::Light
        );
    }

    #[test]
    fn working_hours_include_start_and_exclude_end() {
        assert!(is_working_hour(1, 8, 9, 18));
        assert!(is_working_hour(9, 8, 9, 18));
        assert!(!is_working_hour(10, 8, 9, 18));
        assert!(!is_working_hour(0, 8, 9, 18));
    }

    #[test]
    fn working_hours_wrap_timezone_across_midnight() {
        assert!(is_working_hour(23, 10, 9, 18));
        assert!(!is_working_hour(2, -8, 9, 18));
    }

    #[test]
    fn unsplash_sources_receive_responsive_parameters() {
        let source = "https://images.unsplash.com/photo-1?q=80&w=800";
        let result = unsplash_with_width(source, 576);
        let parsed = url::Url::parse(&result).expect("generated URL");
        let pairs = parsed
            .query_pairs()
            .collect::<std::collections::HashMap<_, _>>();
        assert_eq!(pairs.get("w").expect("width").as_ref(), "576");
        assert_eq!(pairs.get("q").expect("quality").as_ref(), "75");
        assert_eq!(pairs.get("fit").expect("fit").as_ref(), "crop");
    }

    #[test]
    fn non_unsplash_sources_are_unchanged() {
        let source = "/assets/profile.jpg";
        assert_eq!(unsplash_with_width(source, 576), source);
    }

    #[test]
    fn url_origin_keeps_only_scheme_and_host() {
        assert_eq!(
            url_origin("https://images.example.com/path?q=1").as_deref(),
            Some("https://images.example.com")
        );
        assert_eq!(url_origin("/assets/profile.jpg"), None);
    }

    #[test]
    fn scroll_progress_is_clamped_and_handles_zero_distance() {
        assert!((scroll_progress(10.0, 0.0) - 0.0).abs() < f64::EPSILON);
        assert!((scroll_progress(-10.0, 100.0) - 0.0).abs() < f64::EPSILON);
        assert!((scroll_progress(50.0, 100.0) - 0.5).abs() < f64::EPSILON);
        assert!((scroll_progress(150.0, 100.0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn generated_grid_coordinates_stay_inside_the_grid() {
        assert_eq!(grid_axis_coordinate(0.0, 390.0, 40), 0);
        assert_eq!(grid_axis_coordinate(1.0, 390.0, 40), 9);
        assert_eq!(grid_axis_coordinate(0.5, 0.0, 40), 0);
    }

    #[test]
    fn target_cell_count_includes_both_configured_bounds() {
        assert_eq!(target_cell_count(0.0, 6, 10), 6);
        assert_eq!(target_cell_count(1.0, 6, 10), 10);
        assert!((6..=10).contains(&target_cell_count(0.45, 6, 10)));
    }

    #[test]
    fn count_normalization_handles_empty_and_zero_data() {
        assert!(normalize_counts(&[]).is_empty());
        assert_eq!(normalize_counts(&[0, 0]), vec![0.0, 0.0]);
    }

    #[test]
    fn count_normalization_preserves_relative_height() {
        let normalized = normalize_counts(&[5, 10, 20]);
        assert!((normalized[0] - 0.25).abs() < f64::EPSILON);
        assert!((normalized[1] - 0.5).abs() < f64::EPSILON);
        assert!((normalized[2] - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn activity_level_matches_source_golden_cases() {
        let thresholds = [20, 40, 60, 80];
        assert_eq!(activity_level(0, 0, 17, 31, 7, 13, thresholds), 0);
        assert_eq!(activity_level(1, 0, 17, 31, 7, 13, thresholds), 1);
        assert_eq!(activity_level(2, 0, 17, 31, 7, 13, thresholds), 2);
        assert_eq!(activity_level(3, 0, 17, 31, 7, 13, thresholds), 3);
        assert_eq!(activity_level(4, 0, 17, 31, 7, 13, thresholds), 4);
    }

    #[test]
    fn activity_grid_has_expected_size_and_level_range() {
        let levels = (0..52)
            .flat_map(|week| {
                (0..7).map(move |day| activity_level(week, day, 17, 31, 7, 13, [20, 40, 60, 80]))
            })
            .collect::<Vec<_>>();
        assert_eq!(levels.len(), 364);
        assert!(levels.iter().all(|level| *level <= 4));
    }

    #[test]
    fn link_classification_distinguishes_external_targets() {
        assert!(is_external_link("https://example.com/paper.pdf"));
        assert!(is_external_link("http://localhost/code"));
        assert!(!is_external_link("/papers/local.pdf"));
        assert!(!is_external_link("#publication"));
    }

    #[test]
    fn publication_actions_cover_disabled_internal_and_external_states() {
        assert_eq!(publication_action_kind(""), PublicationActionKind::Disabled);
        assert_eq!(
            publication_action_kind("/papers/local.pdf"),
            PublicationActionKind::Internal
        );
        assert_eq!(
            publication_action_kind("https://example.com/code"),
            PublicationActionKind::External
        );
    }

    #[test]
    fn scroll_to_top_threshold_is_strict() {
        assert!(!should_show_scroll_to_top(300.0, 300));
        assert!(should_show_scroll_to_top(300.1, 300));
    }
}
