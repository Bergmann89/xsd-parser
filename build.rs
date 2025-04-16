//! Build script for the `xsd_parser` crate.

use std::env::var;
use std::fs::{read, read_to_string, write};
use std::path::PathBuf;

use base64::{prelude::BASE64_STANDARD, Engine};
use regex::{Captures, Regex};

fn main() {
    let cargo_dir =
        var("CARGO_MANIFEST_DIR").expect("Missing `CARGO_MANIFEST_DIR` environment variable!");
    let cargo_dir = PathBuf::from(cargo_dir);

    let readme = cargo_dir.join("README.md");
    let doc_svg = cargo_dir.join("doc/overview.svg");

    println!("cargo:rerun-if-changed={}", readme.display());

    let rx = Regex::new(r"`(.*?)`").unwrap();

    let doc_svg = read(doc_svg).expect("Unable to load `doc/overview.svg`");
    let doc_svg = BASE64_STANDARD.encode(doc_svg);
    let doc_svg = format!("data:image/svg+xml;base64,{doc_svg}");

    let readme = read_to_string(readme).expect("Unable to read `README.md`");
    let readme = readme.replace("doc/overview.svg", &doc_svg);
    let readme = rx.replace_all(&readme, |c: &Captures<'_>| {
        let keyword = &c[1];
        if KEYWORDS.contains(&keyword) {
            format!("[`{keyword}`]")
        } else {
            format!("`{keyword}`")
        }
    });

    let out_dir = var("OUT_DIR").expect("Missing `OUT_DIR` environment variable!");
    let out_dir = PathBuf::from(out_dir);

    write(out_dir.join("README.md"), &*readme).expect("Unable to write `README.md`");
}

const KEYWORDS: &[&str] = &[
    "Parser",
    "Schemas",
    "Resolver",
    "Interpreter",
    "Types",
    "Optimizer",
    "Generator",
    "Renderer",
    "generate",
    "Config",
];
