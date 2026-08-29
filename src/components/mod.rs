mod activity;
mod background;
mod footer;
mod hero;
mod model;
mod navbar;
mod news;
mod research;
mod reveal;
mod scroll_to_top;
mod teaching;

pub use activity::Activity;
pub use background::InteractiveBackground;
pub use footer::Footer;
pub use hero::Hero;
pub use navbar::Navbar;
pub use news::News;
pub use research::Research;
pub use scroll_to_top::ScrollToTop;
pub use teaching::Teaching;

pub use model::{initialize_theme, persist_theme};
