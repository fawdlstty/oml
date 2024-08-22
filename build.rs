use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let config = cbindgen::Config::from_file("cbindgen.toml").unwrap();
    _ = cbindgen::generate_with_config(&crate_dir, config)
        .map(|p| p.write_to_file(crate_dir.join("include/oml/oml.h")));
}
