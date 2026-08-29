use dioxus::prelude::*;

use crate::components::model::activity_level;
use crate::components::reveal::use_reveal_observer;
use crate::config::{ActivityColor, ActivityConfig, AnimationConfig};

#[cfg(target_arch = "wasm32")]
const CELL_SIZE: u32 = 12;
#[cfg(target_arch = "wasm32")]
const CELL_GAP: u32 = 3;
const CANVAS_WIDTH: u32 = 777;
const CANVAS_HEIGHT: u32 = 102;

#[component]
pub fn Activity(config: &'static ActivityConfig, animation: &'static AnimationConfig) -> Element {
    use_reveal_observer("#activity [data-reveal]");
    #[cfg(target_arch = "wasm32")]
    let _canvas_runtime = use_hook(|| {
        std::rc::Rc::new(canvas_runtime::ActivityCanvasRuntime::install(
            config, animation,
        ))
    });
    let month_divisor = u32::try_from(config.months.len().saturating_sub(1).max(1)).unwrap_or(1);
    let month_width = 100.0 / f64::from(month_divisor);

    rsx! {
        section { id: config.anchor.clone(), class: "activity-section content-section",
            div { class: "activity-container",
                h2 { class: "activity-heading reveal reveal-up", "data-reveal": "", style: format!("--reveal-duration: {}ms", animation.section_duration_ms), {config.heading.clone()} }
                div { class: "glass-card activity-card reveal reveal-card", "data-reveal": "", style: format!("--reveal-duration: {}ms", animation.section_duration_ms),
                    div {
                        id: "activity-scroll",
                        class: "activity-scroll",
                        tabindex: "0",
                        aria_label: config.heading.clone(),
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
                                div { id: "activity-heatmap-visual", class: "heatmap-visual",
                                    div { class: "heatmap-grid", role: "grid",
                                        for week in 0..u32::from(config.weeks) {
                                            div { class: "heatmap-week", role: "row",
                                                for day in 0..u32::from(config.days) {
                                                    HeatmapCell { config, animation, week, day }
                                                }
                                            }
                                        }
                                    }
                                    canvas {
                                        id: "activity-canvas",
                                        class: "activity-canvas",
                                        width: CANVAS_WIDTH.to_string(),
                                        height: CANVAS_HEIGHT.to_string(),
                                        aria_hidden: "true",
                                    }
                                    div {
                                        id: "activity-canvas-tooltip",
                                        class: "activity-canvas-tooltip",
                                        aria_hidden: "true",
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

#[cfg(target_arch = "wasm32")]
mod canvas_runtime {
    use wasm_bindgen::{JsCast as _, closure::Closure};
    use web_sys::{
        CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement, MutationObserver,
        MutationObserverInit, PointerEvent,
    };

    use super::{CANVAS_HEIGHT, CANVAS_WIDTH, CELL_GAP, CELL_SIZE, activity_level};
    use crate::config::{ActivityConfig, AnimationConfig};

    pub struct ActivityCanvasRuntime {
        canvas: HtmlCanvasElement,
        pointer_move: Closure<dyn FnMut(PointerEvent)>,
        pointer_leave: Closure<dyn FnMut(PointerEvent)>,
        mutation_callback: Closure<dyn FnMut(js_sys::Array, MutationObserver)>,
        observer: MutationObserver,
        ready_callback: Option<Closure<dyn FnMut()>>,
        ready_timeout: Option<i32>,
        window: web_sys::Window,
    }

    impl ActivityCanvasRuntime {
        pub fn install(
            config: &'static ActivityConfig,
            animation: &'static AnimationConfig,
        ) -> Option<Self> {
            let window = web_sys::window()?;
            let document = window.document()?;
            let canvas = document
                .get_element_by_id("activity-canvas")?
                .dyn_into::<HtmlCanvasElement>()
                .ok()?;
            let tooltip = document
                .get_element_by_id("activity-canvas-tooltip")?
                .dyn_into::<HtmlElement>()
                .ok()?;
            let context = canvas
                .get_context("2d")
                .ok()
                .flatten()?
                .dyn_into::<CanvasRenderingContext2d>()
                .ok()?;
            let ratio = window.device_pixel_ratio().clamp(1.0, 2.0);
            canvas.set_width((f64::from(CANVAS_WIDTH) * ratio).round() as u32);
            canvas.set_height((f64::from(CANVAS_HEIGHT) * ratio).round() as u32);
            context
                .set_transform(ratio, 0.0, 0.0, ratio, 0.0, 0.0)
                .ok()?;
            draw_grid(&context, config, document_is_dark(&document));

            let move_canvas = canvas.clone();
            let move_tooltip = tooltip.clone();
            let pointer_move = Closure::wrap(Box::new(move |event: PointerEvent| {
                let bounds = move_canvas.get_bounding_client_rect();
                if bounds.width() <= 0.0 || bounds.height() <= 0.0 {
                    return;
                }
                let x = (f64::from(event.client_x()) - bounds.left())
                    * (f64::from(CANVAS_WIDTH) / bounds.width());
                let y = (f64::from(event.client_y()) - bounds.top())
                    * (f64::from(CANVAS_HEIGHT) / bounds.height());
                let pitch = f64::from(CELL_SIZE + CELL_GAP);
                let week = (x / pitch).floor() as u32;
                let day = (y / pitch).floor() as u32;
                let within_x = x - f64::from(week * (CELL_SIZE + CELL_GAP));
                let within_y = y - f64::from(day * (CELL_SIZE + CELL_GAP));
                let hovered = (week < u32::from(config.weeks)
                    && day < u32::from(config.days)
                    && within_x <= f64::from(CELL_SIZE)
                    && within_y <= f64::from(CELL_SIZE))
                .then_some((week, day));

                if let Some((week, day)) = hovered {
                    let level = cell_level(config, week, day);
                    move_tooltip
                        .set_text_content(Some(&format!("{}: {level}", config.level_title_prefix)));
                    let left = week * (CELL_SIZE + CELL_GAP) + CELL_SIZE / 2;
                    let top = day * (CELL_SIZE + CELL_GAP);
                    let _ = move_tooltip
                        .style()
                        .set_property("left", &format!("{left}px"));
                    let _ = move_tooltip
                        .style()
                        .set_property("top", &format!("{top}px"));
                    let _ = move_tooltip.class_list().add_1("is-visible");
                } else {
                    let _ = move_tooltip.class_list().remove_1("is-visible");
                }
            }) as Box<dyn FnMut(PointerEvent)>);

            let leave_tooltip = tooltip;
            let pointer_leave = Closure::wrap(Box::new(move |_event: PointerEvent| {
                let _ = leave_tooltip.class_list().remove_1("is-visible");
            }) as Box<dyn FnMut(PointerEvent)>);

            canvas
                .add_event_listener_with_callback(
                    "pointermove",
                    pointer_move.as_ref().unchecked_ref(),
                )
                .ok()?;

            let observer_context = context.clone();
            let observer_document = document.clone();
            let mutation_callback = Closure::wrap(Box::new(
                move |_records: js_sys::Array, _observer: MutationObserver| {
                    draw_grid(
                        &observer_context,
                        config,
                        document_is_dark(&observer_document),
                    );
                },
            )
                as Box<dyn FnMut(js_sys::Array, MutationObserver)>);
            let observer =
                MutationObserver::new(mutation_callback.as_ref().unchecked_ref()).ok()?;
            let options = MutationObserverInit::new();
            options.set_attributes(true);
            let root = document.query_selector(".app-root").ok().flatten()?;
            observer.observe_with_options(&root, &options).ok()?;
            canvas
                .add_event_listener_with_callback(
                    "pointerleave",
                    pointer_leave.as_ref().unchecked_ref(),
                )
                .ok()?;

            let ready_canvas = canvas.clone();
            let ready = move || {
                let _ = ready_canvas.class_list().add_1("is-ready");
                if let Some(wrapper) = ready_canvas.parent_element() {
                    let _ = wrapper.class_list().add_1("canvas-ready");
                }
            };
            let reduced_motion = window
                .match_media("(prefers-reduced-motion: reduce)")
                .ok()
                .flatten()
                .is_some_and(|query| query.matches());
            let (ready_callback, ready_timeout) = if reduced_motion || !animation.enabled {
                ready();
                (None, None)
            } else {
                let callback = Closure::wrap(Box::new(ready) as Box<dyn FnMut()>);
                let cell_count = u32::from(config.weeks) * u32::from(config.days);
                let delay = cell_count
                    .saturating_sub(1)
                    .saturating_mul(animation.heatmap_cell_stagger_ms)
                    .saturating_add(animation.heatmap_cell_duration_ms);
                let timeout = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        callback.as_ref().unchecked_ref(),
                        i32::try_from(delay).unwrap_or(i32::MAX),
                    )
                    .ok()?;
                (Some(callback), Some(timeout))
            };

            Some(Self {
                canvas,
                pointer_move,
                pointer_leave,
                mutation_callback,
                observer,
                ready_callback,
                ready_timeout,
                window,
            })
        }
    }

    impl Drop for ActivityCanvasRuntime {
        fn drop(&mut self) {
            let _ = self.canvas.remove_event_listener_with_callback(
                "pointermove",
                self.pointer_move.as_ref().unchecked_ref(),
            );
            let _ = self.canvas.remove_event_listener_with_callback(
                "pointerleave",
                self.pointer_leave.as_ref().unchecked_ref(),
            );
            if let Some(timeout) = self.ready_timeout {
                self.window.clear_timeout_with_handle(timeout);
            }
            self.observer.disconnect();
            let _ = (&self.ready_callback, &self.mutation_callback);
        }
    }

    fn cell_level(config: &ActivityConfig, week: u32, day: u32) -> u8 {
        activity_level(
            week,
            day,
            config.seed_week_multiplier,
            config.seed_day_multiplier,
            config.seed_cross_multiplier,
            config.seed_offset,
            config.level_thresholds,
        )
    }

    fn draw_grid(context: &CanvasRenderingContext2d, config: &ActivityConfig, dark: bool) {
        context.clear_rect(0.0, 0.0, f64::from(CANVAS_WIDTH), f64::from(CANVAS_HEIGHT));
        for week in 0..u32::from(config.weeks) {
            for day in 0..u32::from(config.days) {
                let level = cell_level(config, week, day);
                let size = f64::from(CELL_SIZE);
                let offset = (f64::from(CELL_SIZE) - size) / 2.0;
                let x = f64::from(week * (CELL_SIZE + CELL_GAP)) + offset;
                let y = f64::from(day * (CELL_SIZE + CELL_GAP)) + offset;
                context.set_global_alpha(1.0);
                let color = &config.level_colors[usize::from(level)];
                context.set_fill_style_str(if dark { &color.dark } else { &color.light });
                context.begin_path();
                let _ = context.round_rect_with_f64(x, y, size, size, 2.0_f64.min(size / 4.0));
                context.fill();
            }
        }
        context.set_global_alpha(1.0);
    }

    fn document_is_dark(document: &Document) -> bool {
        let theme = document
            .query_selector(".app-root")
            .ok()
            .flatten()
            .and_then(|root| root.get_attribute("data-theme"));
        match theme.as_deref() {
            Some("dark") => true,
            Some("light") => false,
            _ => web_sys::window()
                .and_then(|window| {
                    window
                        .match_media("(prefers-color-scheme: dark)")
                        .ok()
                        .flatten()
                })
                .is_some_and(|query| query.matches()),
        }
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
