//! Build script for the `xsd_parser` crate.

use std::fs::{read, read_to_string, write};
use std::path::{Path, PathBuf};

use base64::{prelude::BASE64_STANDARD, Engine};
use regex::{Captures, Regex};

fn main() {
    let out_dir = env_path("OUT_DIR");
    let cargo_dir = env_path("CARGO_MANIFEST_DIR");

    let readme = resolve_file(&cargo_dir, "README.md");
    let doc_svg = resolve_file(&cargo_dir, "doc/overview.svg");

    println!("cargo:rerun-if-changed={}", readme.display());
    println!("cargo:rerun-if-changed={}", doc_svg.display());

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

    write(out_dir.join("README.md"), &*readme).expect("Unable to write `README.md`");
}

fn env_path(var: &str) -> PathBuf {
    let Ok(value) = std::env::var(var) else {
        panic!("Missing `{var}` environment variable!");
    };

    PathBuf::from(value)
}

fn resolve_file(cargo_dir: &Path, path: &str) -> PathBuf {
    if let Ok(path) = cargo_dir.join(path).canonicalize() {
        return path;
    }

    if let Some(Ok(path)) = cargo_dir
        .parent()
        .map(|parent| parent.join(path).canonicalize())
    {
        return path;
    }

    panic!("Unable to find {path}");
}

const KEYWORDS: &[&str] = &[
    "Config",
    "DataTypes",
    "generate",
    "Generator",
    "Interpreter",
    "MetaTypes",
    "Module",
    "Optimizer",
    "Parser",
    "Renderer",
    "RenderStep",
    "Resolver",
    "Schemas",
];
