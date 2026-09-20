pub mod app;
pub mod components;
pub mod config;
#[cfg(not(target_arch = "wasm32"))]
pub mod finalize;
pub mod localization;

pub use app::App;

#[cfg(feature = "server")]
/// Starts the build-time or development fullstack server.
///
/// # Panics
///
/// Panics when the current executable directory cannot be resolved. Dioxus
/// requires that directory to locate the adjacent `public` artifact.
pub fn launch() {
    dioxus::server::serve(|| async move {
        use dioxus::server::{DioxusRouterExt, IncrementalRendererConfig, ServeConfig, axum};

        let public_dir = std::env::current_exe()
            .expect("server executable path")
            .parent()
            .expect("server executable directory")
            .join("public");
        let config = ServeConfig::builder()
            .incremental(
                IncrementalRendererConfig::new()
                    .static_dir(public_dir)
                    .clear_cache(false),
            )
            .enable_out_of_order_streaming();

        // Dioxus CLI 0.7.3 always probes /api/static_routes at the root even
        // when a Pages base path is configured. The custom router preserves
        // that build-time endpoint while SSR still honors DIOXUS_ASSET_ROOT.
        Ok(axum::Router::new().serve_dioxus_application(config, App))
    });
}

#[cfg(all(feature = "web", not(feature = "server")))]
pub fn launch() {
    dioxus::launch(App);
}

#[cfg(not(any(feature = "web", feature = "server")))]
pub fn launch() {
    panic!("enable either the web or server feature");
}
