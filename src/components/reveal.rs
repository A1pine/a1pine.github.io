#[cfg(target_arch = "wasm32")]
mod web {
    use std::rc::Rc;

    use wasm_bindgen::{JsCast, JsValue, closure::Closure};
    use web_sys::{
        Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
    };

    pub struct RevealObserver {
        observer: IntersectionObserver,
        _callback: Closure<dyn FnMut(js_sys::Array, IntersectionObserver)>,
    }

    impl RevealObserver {
        fn install(selector: &str) -> Option<Self> {
            let document = web_sys::window()?.document()?;
            let callback = Closure::wrap(Box::new(
                move |entries: js_sys::Array, observer: IntersectionObserver| {
                    for entry in entries.iter() {
                        let Ok(entry) = entry.dyn_into::<IntersectionObserverEntry>() else {
                            continue;
                        };
                        if !entry.is_intersecting() {
                            continue;
                        }
                        let target = entry.target();
                        let Ok(element) = target.dyn_into::<Element>() else {
                            continue;
                        };
                        let _ = element.class_list().add_1("is-visible");
                        observer.unobserve(&element);
                    }
                },
            )
                as Box<dyn FnMut(js_sys::Array, IntersectionObserver)>);
            let options = IntersectionObserverInit::new();
            options.set_root_margin("0px 0px -8% 0px");
            options.set_threshold(&JsValue::from_f64(0.08));
            let observer =
                IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &options)
                    .ok()?;

            let elements = document.query_selector_all(selector).ok()?;
            for index in 0..elements.length() {
                let Some(node) = elements.item(index) else {
                    continue;
                };
                let Ok(element) = node.dyn_into::<Element>() else {
                    continue;
                };
                let _ = element.class_list().add_1("reveal-pending");
                observer.observe(&element);
            }

            Some(Self {
                observer,
                _callback: callback,
            })
        }
    }

    impl Drop for RevealObserver {
        fn drop(&mut self) {
            self.observer.disconnect();
        }
    }

    pub fn use_reveal_observer(selector: &'static str) {
        let _observer = dioxus::prelude::use_hook(|| Rc::new(RevealObserver::install(selector)));
    }
}

#[cfg(target_arch = "wasm32")]
pub use web::use_reveal_observer;

#[cfg(not(target_arch = "wasm32"))]
pub fn use_reveal_observer(_selector: &'static str) {
    dioxus::prelude::use_hook(|| ());
}
