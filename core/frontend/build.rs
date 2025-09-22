use std::time::SystemTime;
use std::env::var;
use chrono::{DateTime, Utc};

fn main() {
    let profile = var("PROFILE")
        .unwrap_or("DEBUG".into())
        .to_uppercase();

    if profile != "DEBUG" && profile != "RELEASE" {
        panic!("Error PROFILE must be or either DEBUG or RELEASE not '{profile}'");
    }

    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-env=CANVADOT_PROFILE={profile}");
    println!("cargo:rustc-env=CANVADOT_BUILD_AGE={}", DateTime::<Utc>::from(SystemTime::now()));
}
