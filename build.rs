use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let config = cbindgen::Config {
        language: cbindgen::Language::C,
        pragma_once: true,
        ..Default::default()
    };
    _ = cbindgen::generate_with_config(&crate_dir, config)
        .map(|p| p.write_to_file(crate_dir.join("include/oml/oml.h")));
    _ = std::fs::remove_file(format!(
        "target/package/{}-{}/Cargo.lock",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    ));
}
