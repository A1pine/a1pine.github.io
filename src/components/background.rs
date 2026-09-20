use dioxus::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::components::model::{grid_axis_coordinate, target_cell_count};
use crate::config::BackgroundConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
struct GridCell {
    id: u64,
    x: u32,
    y: u32,
}

#[component]
pub fn InteractiveBackground(config: &'static BackgroundConfig) -> Element {
    #[cfg(target_arch = "wasm32")]
    let _pointer_tracker =
        use_hook(|| std::rc::Rc::new(pointer_tracker::PointerTracker::install()));

    let style = format!(
        "--glow-size: {}px; --glow-blur: {}px; --glow-color: {}; --glow-fade: {};",
        config.pointer_glow_size_px,
        config.pointer_glow_blur_px,
        config.pointer_glow_color,
        config.pointer_glow_fade,
    );

    rsx! {
        div { class: "background-layer", aria_hidden: "true", style,
            div { class: "background-color" }
            if config.blinking_grid_enabled {
                BlinkingGrid { config }
            }
            div { id: "pointer-glow", class: "pointer-glow" }
        }
    }
}

#[component]
fn BlinkingGrid(config: &'static BackgroundConfig) -> Element {
    let cells = use_signal(Vec::<GridCell>::new);

    #[cfg(target_arch = "wasm32")]
    use_future(move || blinking_loop(cells, config));

    rsx! {
        div { class: "blinking-grid",
            for cell in cells.read().iter() {
                div {
                    key: "{cell.id}",
                    class: "blinking-cell",
                    style: format!(
                        "transform: translate3d({}px, {}px, 0); animation-duration: {}ms;",
                        cell.x * u32::from(config.blinking_grid_cell_size_px),
                        cell.y * u32::from(config.blinking_grid_cell_size_px),
                        config.blinking_grid_duration_ms,
                    ),
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn blinking_loop(mut cells: Signal<Vec<GridCell>>, config: &'static BackgroundConfig) {
    use gloo_timers::future::TimeoutFuture;

    let mut next_id = 0_u64;
    loop {
        TimeoutFuture::new(config.blinking_grid_interval_ms).await;

        let Some(window) = web_sys::window() else {
            continue;
        };
        let Some(document) = window.document() else {
            continue;
        };
        let prefers_dark = window
            .match_media("(prefers-color-scheme: dark)")
            .ok()
            .flatten()
            .is_some_and(|query| query.matches());
        if document.hidden() || prefers_dark {
            continue;
        }

        let width = window
            .inner_width()
            .ok()
            .and_then(|value| value.as_f64())
            .unwrap_or(1.0);
        let height = window
            .inner_height()
            .ok()
            .and_then(|value| value.as_f64())
            .unwrap_or(1.0);
        let target = target_cell_count(
            js_sys::Math::random(),
            config.blinking_grid_min_cells,
            config.blinking_grid_max_cells,
        );

        let mut next = cells.peek().clone();
        next.push(GridCell {
            id: next_id,
            x: grid_axis_coordinate(
                js_sys::Math::random(),
                width,
                config.blinking_grid_cell_size_px,
            ),
            y: grid_axis_coordinate(
                js_sys::Math::random(),
                height,
                config.blinking_grid_cell_size_px,
            ),
        });
        next_id = next_id.wrapping_add(1);
        if next.len() > target {
            let remove = next.len() - target;
            next.drain(..remove);
        }
        cells.set(next);
    }
}

#[cfg(target_arch = "wasm32")]
mod pointer_tracker {
    use std::cell::Cell;
    use std::rc::Rc;

    use wasm_bindgen::{JsCast, closure::Closure};
    use web_sys::{AddEventListenerOptions, HtmlElement, PointerEvent};

    pub struct PointerTracker {
        window: web_sys::Window,
        pointer_listener: Option<Closure<dyn FnMut(PointerEvent)>>,
        resize_listener: Option<Closure<dyn FnMut(web_sys::Event)>>,
        animation_callback: Closure<dyn FnMut(f64)>,
        frame: Rc<Cell<i32>>,
    }

    impl PointerTracker {
        pub fn install() -> Option<Self> {
            let window = web_sys::window()?;
            let glow = window
                .document()?
                .get_element_by_id("pointer-glow")?
                .dyn_into::<HtmlElement>()
                .ok()?;
            let latest_x = Rc::new(Cell::new(window.inner_width().ok()?.as_f64()? / 2.0));
            let latest_y = Rc::new(Cell::new(window.inner_height().ok()?.as_f64()? / 2.0));
            let frame = Rc::new(Cell::new(0));

            apply_position(&glow, latest_x.get(), latest_y.get());

            let callback_glow = glow.clone();
            let callback_x = Rc::clone(&latest_x);
            let callback_y = Rc::clone(&latest_y);
            let callback_frame = Rc::clone(&frame);
            let animation_callback = Closure::wrap(Box::new(move |_timestamp: f64| {
                callback_frame.set(0);
                apply_position(&callback_glow, callback_x.get(), callback_y.get());
            }) as Box<dyn FnMut(f64)>);
            let animation_function = animation_callback
                .as_ref()
                .unchecked_ref::<js_sys::Function>()
                .clone();

            let tracks_pointer = window
                .match_media("(pointer: fine)")
                .ok()
                .flatten()
                .is_some_and(|query| query.matches());

            let (pointer_listener, resize_listener) = if tracks_pointer {
                let listener_window = window.clone();
                let listener_x = Rc::clone(&latest_x);
                let listener_y = Rc::clone(&latest_y);
                let listener_frame = Rc::clone(&frame);
                let listener = Closure::wrap(Box::new(move |event: PointerEvent| {
                    listener_x.set(f64::from(event.client_x()));
                    listener_y.set(f64::from(event.client_y()));
                    schedule_frame(&listener_window, &listener_frame, &animation_function);
                }) as Box<dyn FnMut(PointerEvent)>);
                let options = AddEventListenerOptions::new();
                options.set_passive(true);
                window
                    .add_event_listener_with_callback_and_add_event_listener_options(
                        "pointermove",
                        listener.as_ref().unchecked_ref(),
                        &options,
                    )
                    .ok()?;
                (Some(listener), None)
            } else {
                let listener_window = window.clone();
                let listener_x = Rc::clone(&latest_x);
                let listener_y = Rc::clone(&latest_y);
                let listener_frame = Rc::clone(&frame);
                let listener = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                    if let (Ok(width), Ok(height)) = (
                        listener_window.inner_width(),
                        listener_window.inner_height(),
                    ) && let (Some(width), Some(height)) = (width.as_f64(), height.as_f64())
                    {
                        listener_x.set(width / 2.0);
                        listener_y.set(height / 2.0);
                        schedule_frame(&listener_window, &listener_frame, &animation_function);
                    }
                }) as Box<dyn FnMut(web_sys::Event)>);
                window
                    .add_event_listener_with_callback("resize", listener.as_ref().unchecked_ref())
                    .ok()?;
                (None, Some(listener))
            };

            Some(Self {
                window,
                pointer_listener,
                resize_listener,
                animation_callback,
                frame,
            })
        }
    }

    impl Drop for PointerTracker {
        fn drop(&mut self) {
            if let Some(listener) = &self.pointer_listener {
                let _ = self.window.remove_event_listener_with_callback(
                    "pointermove",
                    listener.as_ref().unchecked_ref(),
                );
            }
            if let Some(listener) = &self.resize_listener {
                let _ = self.window.remove_event_listener_with_callback(
                    "resize",
                    listener.as_ref().unchecked_ref(),
                );
            }
            if self.frame.get() != 0 {
                let _ = self.window.cancel_animation_frame(self.frame.get());
            }
            let _ = &self.animation_callback;
        }
    }

    fn schedule_frame(window: &web_sys::Window, frame: &Cell<i32>, callback: &js_sys::Function) {
        if frame.get() == 0
            && let Ok(id) = window.request_animation_frame(callback)
        {
            frame.set(id);
        }
    }

    fn apply_position(glow: &HtmlElement, x: f64, y: f64) {
        let transform = format!("translate3d({x}px, {y}px, 0) translate3d(-50%, -50%, 0)");
        let _ = glow.style().set_property("transform", &transform);
    }
}
