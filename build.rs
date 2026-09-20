use std::fs;
use std::path::PathBuf;

use chrono::{Datelike, Utc};

fn main() {
    println!("cargo:rerun-if-changed=config/site.toml");
    println!("cargo:rerun-if-changed=config/locales/zh-CN.json");
    println!("cargo:rustc-env=ARCADEMIC_BUILD_YEAR={}", Utc::now().year());

    let source = fs::read_to_string("config/site.toml").expect("read config/site.toml");
    let value =
        toml::from_str::<toml::Value>(&source).expect("parse config/site.toml during build");
    let chinese =
        fs::read_to_string("config/locales/zh-CN.json").expect("read config/locales/zh-CN.json");
    serde_json::from_str::<serde_json::Value>(&chinese)
        .expect("parse config/locales/zh-CN.json during build");
    let json = serde_json::to_string(&value).expect("serialize site configuration as JSON");
    let output = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR")).join("site.json");
    fs::write(output, json).expect("write generated site.json");
}
