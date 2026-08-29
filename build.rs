use chrono::{Datelike, Utc};

fn main() {
    println!("cargo:rerun-if-changed=config/site.toml");
    println!("cargo:rustc-env=ARCADEMIC_BUILD_YEAR={}", Utc::now().year());
}
