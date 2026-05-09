//! Build-time guard: `Cargo.toml` and `extension.toml` must agree on `version`.
//!
//! Two independent files, two independent fields, two independent ways for
//! a release-time mistake to silently land:
//!
//!   * `Cargo.toml` ⇒ `env!("CARGO_PKG_VERSION")` ⇒ baked into the wasm,
//!     used by `language_server_initialization_options` to populate
//!     `expectedAwsumVersion`. This is what the LSP server compares
//!     against its own version to decide whether to warn the user.
//!
//!   * `extension.toml` ⇒ what Zed's UI / marketplace displays as "you
//!     have version X installed".
//!
//! If the two drift, the user sees one number in the marketplace and the
//! server quietly thinks the extension targets a different one. The
//! version-mismatch warning becomes wrong (false-negative when the user
//! bumped only `extension.toml`; false-positive the other way around).
//!
//! Releases should bump both values in lockstep. This script enforces
//! that mechanically — `cargo build` fails fast with a clear message.

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=extension.toml");
    println!("cargo:rerun-if-changed=Cargo.toml");

    let cargo_version =
        env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION set by Cargo");

    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set by Cargo");
    let ext_path = Path::new(&manifest_dir).join("extension.toml");
    let ext_content = fs::read_to_string(&ext_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", ext_path.display()));

    // Hand-roll parsing the single line we care about — a `toml` crate
    // pulled in just to read one field would be overkill. Looks for a
    // line of the form `version = "X.Y.Z"` outside any [section]: we
    // only accept it before the first `[`.
    let mut ext_version: Option<String> = None;
    for line in ext_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            // Reached the first sub-table; the top-level `version`
            // field, if it exists, is above us already.
            break;
        }
        if let Some(rest) = trimmed.strip_prefix("version") {
            let rest = rest.trim_start();
            if let Some(rest) = rest.strip_prefix('=') {
                let value = rest.trim().trim_matches('"').to_string();
                ext_version = Some(value);
                break;
            }
        }
    }

    let ext_version = ext_version
        .expect("extension.toml is missing a top-level `version = \"...\"` field");

    if ext_version != cargo_version {
        panic!(
            "awsum-zed version mismatch between Cargo.toml ({cargo_version}) and \
             extension.toml ({ext_version}). Both must agree — release-time bumps \
             touch both files in lockstep, and the LSP `expectedAwsumVersion` \
             machinery relies on Cargo.toml ⇒ env!(\"CARGO_PKG_VERSION\") matching \
             what Zed's UI displays from extension.toml."
        );
    }
}
