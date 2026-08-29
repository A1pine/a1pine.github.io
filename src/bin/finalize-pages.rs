#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args().skip(1);
    let index_path = arguments.next().ok_or("missing index path")?;
    let base_path = arguments.next().unwrap_or_default();
    let build_year = env!("ARCADEMIC_BUILD_YEAR").parse::<i32>()?;

    arcademic_rust::finalize::finalize_pages(
        Path::new(&index_path),
        &base_path,
        arcademic_rust::config::site_config(),
        build_year,
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {}
