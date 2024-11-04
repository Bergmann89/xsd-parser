//! Build script for the `xsd_parser` crate.

use std::env::var;
use std::fs::{read_to_string, write};
use std::path::PathBuf;

use regex::{Captures, Regex};

fn main() {
    let cargo_dir =
        var("CARGO_MANIFEST_DIR").expect("Missing `CARGO_MANIFEST_DIR` environment variable!");
    let cargo_dir = PathBuf::from(cargo_dir);

    let readme = cargo_dir.join("README.md");

    println!("cargo:rerun-if-changed={}", readme.display());

    let rx = Regex::new(r">\s*\[!WARNING\]\n((?:>\s*(?:[^\n]*\n))+)").unwrap();

    let readme = read_to_string(readme).expect("Unable to read `README.md`");
    let readme = readme.replace("(doc/overview.svg", "(../doc/overview.svg");
    let readme = rx.replace_all(&readme, |c: &Captures<'_>| {
        let message = c[1]
            .lines()
            .map(|s| s.strip_prefix(">").unwrap_or(s).trim())
            .collect::<Vec<_>>()
            .join("\n");

        format!("<div class=\"warning\">\n{message}\n</div>")
    });

    let out_dir = var("OUT_DIR").expect("Missing `OUT_DIR` environment variable!");
    let out_dir = PathBuf::from(out_dir);

    write(out_dir.join("README.md"), &*readme).expect("Unable to write `README.md`");
}
