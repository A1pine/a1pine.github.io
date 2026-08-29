use dioxus::prelude::*;

use crate::components::reveal::use_reveal_observer;
use crate::config::TeachingConfig;

#[component]
pub fn Teaching(config: &'static TeachingConfig) -> Element {
    use_reveal_observer("#teaching [data-reveal]");

    rsx! {
        section { id: config.anchor.clone(), class: "teaching-section content-section",
            div { class: "teaching-container",
                h2 { class: "teaching-heading reveal reveal-up", "data-reveal": "", {config.heading.clone()} }
                div { class: "course-grid",
                    for (index, course) in config.items.iter().enumerate() {
                        article {
                            class: "glass-card course-card reveal reveal-card",
                            "data-reveal": "",
                            style: format!("--reveal-delay: {}ms", index * 100),
                            p { class: "course-code", {course.code.clone()} }
                            h3 { {course.title.clone()} }
                            p { class: "course-semester",
                                {config.semester_prefix.clone()}
                                ": "
                                {course.semester.clone()}
                            }
                            p { class: "course-description", {course.description.clone()} }
                        }
                    }
                }
            }
        }
    }
}
