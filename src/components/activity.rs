use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdGithub};
use serde::Deserialize;

use crate::components::reveal::use_reveal_observer;
use crate::config::{ActivityColor, ActivityConfig, AnimationConfig};
use crate::localization::{translate, translate_template, use_locale};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct GithubContribution {
    date: String,
    count: u32,
    level: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct GithubContributionTotal {
    #[serde(rename = "lastYear")]
    last_year: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct GithubContributionResponse {
    total: GithubContributionTotal,
    contributions: Vec<GithubContribution>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
enum ActivityState {
    Loading,
    Loaded(GithubContributionResponse),
    Error,
}

#[component]
pub fn Activity(config: &'static ActivityConfig, animation: &'static AnimationConfig) -> Element {
    use_reveal_observer("#activity [data-reveal]");
    let locale = use_locale();
    #[allow(unused_mut)]
    let mut state = use_signal(|| ActivityState::Loading);

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        let api_url = config.api_url.clone();
        spawn(async move {
            state.set(match fetch_contributions(&api_url).await {
                Ok(response) => ActivityState::Loaded(response),
                Err(()) => ActivityState::Error,
            });
        });
    });

    let expected_cells = usize::from(config.weeks) * usize::from(config.days);
    let current_state = state();
    let (cells, summary, state_name) = match &current_state {
        ActivityState::Loading => (
            None,
            translate(locale, "activity.loading", &config.loading_label),
            "loading",
        ),
        ActivityState::Loaded(response) => (
            Some(normalize_cells(
                response.contributions.clone(),
                expected_cells,
            )),
            translate_template(
                locale,
                "activity.total_template",
                &format!("{} {}", response.total.last_year, config.total_label),
                &[("total", &response.total.last_year.to_string())],
            ),
            "loaded",
        ),
        ActivityState::Error => (
            None,
            translate(locale, "activity.error", &config.error_label),
            "error",
        ),
    };
    let weeks = cells.map(|cells| {
        cells
            .chunks(usize::from(config.days))
            .map(<[_]>::to_vec)
            .collect::<Vec<_>>()
    });
    let month_labels = weeks
        .as_ref()
        .map_or_else(
            || fallback_month_labels(usize::from(config.weeks), &config.months),
            |weeks| month_labels(weeks, &config.months),
        )
        .into_iter()
        .map(|label| localized_month(locale, &label))
        .collect::<Vec<_>>();
    let heading = translate(locale, "activity.heading", &config.heading);
    let vibe_alt = translate(locale, "activity.vibe_alt", &config.vibe_badge_alt);

    rsx! {
        section { id: config.anchor.clone(), class: "activity-section content-section",
            div { class: "activity-container",
                a {
                    class: "vibe-usage-link reveal reveal-up",
                    "data-reveal": "",
                    href: config.vibe_profile_url.clone(),
                    target: "_blank",
                    rel: "noopener noreferrer",
                    aria_label: vibe_alt.clone(),
                    img {
                        class: "vibe-usage-badge",
                        src: config.vibe_badge_url.clone(),
                        alt: vibe_alt,
                        width: "320",
                        height: "22",
                        loading: "lazy",
                        decoding: "async",
                        fetchpriority: "low",
                    }
                }
                div { class: "activity-heading-row",
                    h2 { class: "activity-heading reveal reveal-up", "data-reveal": "", style: format!("--reveal-duration: {}ms", animation.section_duration_ms), {heading.clone()} }
                    a {
                        class: "activity-profile-link reveal reveal-up",
                        "data-reveal": "",
                        href: config.profile_url.clone(),
                        target: "_blank",
                        rel: "noopener noreferrer",
                        aria_label: translate(locale, "activity.profile_label", &config.profile_label),
                        Icon { icon: LdGithub, width: 16, height: 16 }
                        span { {config.github_username.clone()} }
                    }
                }
                div { class: "glass-card activity-card reveal reveal-card", "data-reveal": "", style: format!("--reveal-duration: {}ms", animation.section_duration_ms),
                    p {
                        class: "activity-summary",
                        aria_live: "polite",
                        "data-live-state": state_name,
                        {summary.clone()}
                    }
                    div {
                        id: "activity-scroll",
                        class: "activity-scroll",
                        tabindex: "0",
                        aria_label: heading,
                        onkeydown: move |event| {
                            let delta = match event.key() {
                                Key::ArrowRight => 40,
                                Key::ArrowLeft => -40,
                                _ => 0,
                            };
                            if delta != 0 {
                                event.prevent_default();
                                scroll_activity_region(delta);
                            }
                        },
                        div { class: "activity-grid-wrap",
                            div { class: "month-labels", aria_hidden: "true",
                                for (index, label) in month_labels.into_iter().enumerate() {
                                    if !label.is_empty() {
                                        span {
                                            key: "month-{index}",
                                            style: format!("left: {}px", index * 15),
                                            {label}
                                        }
                                    }
                                }
                            }
                            div { class: "heatmap-row",
                                div { class: "day-labels", aria_hidden: "true",
                                    for day in 0..usize::from(config.days) {
                                        span { key: "day-{day}", {day_label(config, locale, day)} }
                                    }
                                }
                                div { class: "heatmap-visual",
                                    if let Some(weeks) = weeks {
                                        div { class: "heatmap-grid", role: "grid",
                                            for (week_index, week) in weeks.into_iter().enumerate() {
                                                div { class: "heatmap-week", role: "row",
                                                    for (day_index, contribution) in week.into_iter().enumerate() {
                                                        HeatmapCell {
                                                            key: "{week_index}-{day_index}",
                                                            config,
                                                            animation,
                                                            contribution,
                                                            index: week_index * usize::from(config.days) + day_index,
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        div {
                                            class: "heatmap-skeleton",
                                            role: "img",
                                            aria_label: summary,
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "activity-legend",
                            span { {translate(locale, "activity.less", &config.less_label)} }
                            for color in &config.level_colors {
                                LegendCell { color: color.clone() }
                            }
                            span { {translate(locale, "activity.more", &config.more_label)} }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn fetch_contributions(api_url: &str) -> Result<GithubContributionResponse, ()> {
    gloo_net::http::Request::get(api_url)
        .send()
        .await
        .map_err(|_| ())?
        .json::<GithubContributionResponse>()
        .await
        .map_err(|_| ())
}

fn placeholder_cells(count: usize) -> Vec<GithubContribution> {
    vec![
        GithubContribution {
            date: String::new(),
            count: 0,
            level: 0,
        };
        count
    ]
}

fn normalize_cells(
    mut contributions: Vec<GithubContribution>,
    expected: usize,
) -> Vec<GithubContribution> {
    if contributions.len() > expected {
        contributions.drain(..contributions.len() - expected);
    } else if contributions.len() < expected {
        let mut padding = placeholder_cells(expected - contributions.len());
        padding.append(&mut contributions);
        contributions = padding;
    }
    contributions
}

fn month_labels(weeks: &[Vec<GithubContribution>], fallback: &[String]) -> Vec<String> {
    if weeks.iter().flatten().any(|cell| !cell.date.is_empty()) {
        let mut labels = weeks
            .iter()
            .enumerate()
            .map(|(index, week)| {
                let date = week
                    .iter()
                    .find(|cell| cell.date.get(8..10) == Some("01"))
                    .or_else(|| (index == 0).then(|| week.first()).flatten());
                date.and_then(|cell| cell.date.get(5..7))
                    .map(month_name)
                    .unwrap_or_default()
                    .to_owned()
            })
            .collect::<Vec<_>>();
        let mut previous: Option<usize> = None;
        for index in 0..labels.len() {
            if labels[index].is_empty() {
                continue;
            }
            if let Some(previous_index) = previous
                && index - previous_index < 3
            {
                labels[previous_index].clear();
            }
            previous = Some(index);
        }
        return labels;
    }

    fallback_month_labels(weeks.len(), fallback)
}

fn fallback_month_labels(week_count: usize, fallback: &[String]) -> Vec<String> {
    let mut labels = vec![String::new(); week_count];
    let denominator = fallback.len().saturating_sub(1).max(1);
    for (index, label) in fallback.iter().enumerate() {
        let position = index * week_count.saturating_sub(1) / denominator;
        labels[position].clone_from(label);
    }
    labels
}

fn month_name(month: &str) -> &'static str {
    match month {
        "01" => "Jan",
        "02" => "Feb",
        "03" => "Mar",
        "04" => "Apr",
        "05" => "May",
        "06" => "Jun",
        "07" => "Jul",
        "08" => "Aug",
        "09" => "Sep",
        "10" => "Oct",
        "11" => "Nov",
        "12" => "Dec",
        _ => "",
    }
}

fn localized_month(locale: crate::localization::Locale, label: &str) -> String {
    let month = match label {
        "Jan" => "01",
        "Feb" => "02",
        "Mar" => "03",
        "Apr" => "04",
        "May" => "05",
        "Jun" => "06",
        "Jul" => "07",
        "Aug" => "08",
        "Sep" => "09",
        "Oct" => "10",
        "Nov" => "11",
        "Dec" => "12",
        _ => return label.to_owned(),
    };
    translate(locale, &format!("activity.month.{month}"), label)
}

fn day_label(config: &ActivityConfig, locale: crate::localization::Locale, day: usize) -> String {
    match day {
        1 => translate(
            locale,
            "activity.day.mon",
            config.day_labels.first().map_or("", String::as_str),
        ),
        3 => translate(
            locale,
            "activity.day.wed",
            config.day_labels.get(1).map_or("", String::as_str),
        ),
        5 => translate(
            locale,
            "activity.day.fri",
            config.day_labels.get(2).map_or("", String::as_str),
        ),
        _ => String::new(),
    }
}

#[cfg(target_arch = "wasm32")]
fn scroll_activity_region(delta: i32) {
    if let Some(region) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("activity-scroll"))
    {
        region.set_scroll_left(region.scroll_left() + delta);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn scroll_activity_region(_delta: i32) {}

#[component]
fn HeatmapCell(
    config: &'static ActivityConfig,
    animation: &'static AnimationConfig,
    contribution: GithubContribution,
    index: usize,
) -> Element {
    let locale = use_locale();
    let level = usize::from(contribution.level).min(config.level_colors.len().saturating_sub(1));
    let color = &config.level_colors[level];
    let delay = u32::try_from(index)
        .unwrap_or(u32::MAX)
        .saturating_mul(animation.heatmap_cell_stagger_ms);
    let title = if contribution.date.is_empty() {
        config.loading_label.clone()
    } else {
        let noun = if contribution.count == 1 {
            "contribution"
        } else {
            "contributions"
        };
        let fallback = format!("{}: {} {noun}", contribution.date, contribution.count);
        translate_template(
            locale,
            "activity.contribution_template",
            &fallback,
            &[
                ("date", &contribution.date),
                ("count", &contribution.count.to_string()),
            ],
        )
    };

    rsx! {
        div {
            class: "heatmap-cell",
            role: "gridcell",
            title: title.clone(),
            aria_label: title,
            "data-date": contribution.date,
            "data-count": contribution.count.to_string(),
            "data-level": level.to_string(),
            style: format!(
                "--cell-light: {}; --cell-dark: {}; --cell-delay: {}ms; --cell-duration: {}ms",
                color.light, color.dark, delay, animation.heatmap_cell_duration_ms
            ),
        }
    }
}

#[component]
fn LegendCell(color: ActivityColor) -> Element {
    rsx! {
        span {
            class: "legend-cell",
            aria_hidden: "true",
            style: format!("--cell-light: {}; --cell-dark: {}", color.light, color.dark),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contribution_cells_are_padded_and_trimmed_to_the_visible_year() {
        let one = GithubContribution {
            date: "2026-08-29".to_owned(),
            count: 3,
            level: 2,
        };
        let padded = normalize_cells(vec![one.clone()], 3);
        assert_eq!(padded.len(), 3);
        assert_eq!(padded[2], one);

        let trimmed = normalize_cells(vec![one.clone(), one.clone(), one.clone()], 2);
        assert_eq!(trimmed, vec![one.clone(), one]);
    }

    #[test]
    fn live_month_labels_follow_calendar_boundaries() {
        let weeks = vec![
            vec![GithubContribution {
                date: "2026-07-26".to_owned(),
                count: 0,
                level: 0,
            }],
            vec![GithubContribution {
                date: "2026-08-01".to_owned(),
                count: 0,
                level: 0,
            }],
        ];
        assert_eq!(month_labels(&weeks, &[]), vec!["", "Aug"]);
    }
}
