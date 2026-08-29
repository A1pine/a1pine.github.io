use dioxus::prelude::*;

use crate::components::model::activity_level;
use crate::components::reveal::use_reveal_observer;
use crate::config::{ActivityColor, ActivityConfig, AnimationConfig};

#[component]
pub fn Activity(config: &'static ActivityConfig, animation: &'static AnimationConfig) -> Element {
    use_reveal_observer("#activity [data-reveal]");
    let month_divisor = u32::try_from(config.months.len().saturating_sub(1).max(1)).unwrap_or(1);
    let month_width = 100.0 / f64::from(month_divisor);

    rsx! {
        section { id: config.anchor.clone(), class: "activity-section content-section",
            div { class: "activity-container",
                h2 { class: "activity-heading reveal reveal-up", "data-reveal": "", style: format!("--reveal-duration: {}ms", animation.section_duration_ms), {config.heading.clone()} }
                div { class: "glass-card activity-card reveal reveal-card", "data-reveal": "", style: format!("--reveal-duration: {}ms", animation.section_duration_ms),
                    div { class: "activity-scroll", tabindex: "0", aria_label: config.heading.clone(),
                        div { class: "activity-grid-wrap",
                            div { class: "month-labels",
                                for month in &config.months {
                                    span { style: format!("width: {month_width}%"), {month.clone()} }
                                }
                            }
                            div { class: "heatmap-row",
                                div { class: "day-labels",
                                    for (index, label) in config.day_labels.iter().enumerate() {
                                        span { class: if index == 0 { "" } else { "spaced" }, {label.clone()} }
                                    }
                                }
                                div { class: "heatmap-grid", role: "grid",
                                    for week in 0..u32::from(config.weeks) {
                                        div { class: "heatmap-week", role: "row",
                                            for day in 0..u32::from(config.days) {
                                                HeatmapCell { config, animation, week, day }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "activity-legend",
                            span { {config.less_label.clone()} }
                            for color in &config.level_colors {
                                LegendCell { color: color.clone() }
                            }
                            span { {config.more_label.clone()} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HeatmapCell(
    config: &'static ActivityConfig,
    animation: &'static AnimationConfig,
    week: u32,
    day: u32,
) -> Element {
    let level = activity_level(
        week,
        day,
        config.seed_week_multiplier,
        config.seed_day_multiplier,
        config.seed_cross_multiplier,
        config.seed_offset,
        config.level_thresholds,
    );
    let color = &config.level_colors[usize::from(level)];
    let delay = (week * u32::from(config.days) + day) * animation.heatmap_cell_stagger_ms;
    let title = format!("{}: {level}", config.level_title_prefix);

    rsx! {
        div {
            key: "{week}-{day}",
            class: "heatmap-cell",
            role: "gridcell",
            title: title.clone(),
            aria_label: title,
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
