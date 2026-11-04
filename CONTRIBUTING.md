# Contributing to xsd-parser

Thanks for your interest in improving `xsd-parser`! Bug reports, schemas that do not work, documentation fixes and pull requests are all welcome.

## Reporting Issues

Most problems depend on the schema and the configuration, so please include:

- the `xsd-parser` and `xsd-parser-types` versions,
- the schema (or a minimal schema that reproduces the problem, or a link to it),
- the `Config` you use (flags, render steps, custom types),
- the full error message, or the generated code that is wrong or does not compile.

Please use the issue templates, they ask for exactly this.

## Repository Layout

- `xsd-parser`: the code generator. The pipeline lives in `src/pipeline` (parser, interpreter, optimizer, generator, renderer), the models in `src/models`.
- `xsd-parser-types`: the runtime crate that is used by the generated code.
- `examples`: standalone example crates (`read_write`, `bpmn`, `vsme`). Smaller examples are in `xsd-parser/examples`.
- `benchmark`: performance and memory benchmarks of the generated code.

The README gives an overview of the pipeline stages and the data models.

## Development Setup

The toolchain is pinned in `rust-toolchain.toml`, `rustup` picks it up automatically. A dev container configuration is available in `.devcontainer`.

## Tests

The tests of `xsd-parser` are snapshot tests: each test generates code for a schema and compares it with an expected file. Most of the tests also compile the expected code and use it to read and write example XML documents.

- `xsd-parser/tests/feature`: one directory per feature (`schema.xsd`, `mod.rs`, `expected/*.rs`, `example/*.xml`)
- `xsd-parser/tests/schema`: real-world schemas
- `xsd-parser/tests/optimizer`, `generator`, `renderer`: tests of single pipeline stages

See [`xsd-parser/tests/README.md`](xsd-parser/tests/README.md) for how to add a new test case.

If you change the generated code, update the expected files with the `update-expectations` feature and review the diff:

```sh
cargo test -p xsd-parser --features update-expectations
cargo test -p xsd-parser --features update-expectations,base64
```

Then run the tests without the feature, they compile the updated expectations and run the read/write tests against them. Do not use `cargo test --all-features`, it enables `update-expectations` and rewrites the expected files instead of checking them.

A bug fix should come with a test that fails without the fix.

## Checks

The CI runs the following commands, please run them before opening a pull request:

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features -- --deny warnings
cargo test
cargo test -p xsd-parser-types --all-features
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps
```

If you touched code generation, also run the example crates (`cd examples/read_write && cargo run --release`, same for `bpmn` and `vsme`) and build the benchmark (`cargo build -p benchmark --release`). The `read_write` example rewrites its `output.xml`, do not commit that change.

## Code Style

`rustfmt` and `clippy` (with the pedantic lints configured in the workspace `Cargo.toml`) cover most of the style. In addition:

- Public items need doc comments (`missing_docs` is enabled).
- Do not panic on user input or schema content. Return an error instead, ideally one that names the affected type or file.
- Keep changes focused: one fix or feature per pull request, without unrelated reformatting or dependency updates.
- Changes to the public API of `xsd-parser` or `xsd-parser-types`, or to the generated code, should be mentioned in the pull request description, so they can be noted in the changelog.

## Commits and Pull Requests

- We prefer [Conventional Commits](https://www.conventionalcommits.org) for commit messages, e.g. `fix(interpreter): ...`, `feat(renderer): ...`, `docs: ...`. It is not required, pull requests are usually squashed when they are merged.
- Explain in the commit message or the pull request why the change is needed, not only what it does.
- Reference the issue in the pull request description (`Closes #123`), not in the commit message.
- If you used AI tools to create the change, please review the result yourself before opening the pull request. [`AGENTS.md`](AGENTS.md) contains hints for these tools.

## License

By contributing, you agree that your contributions are licensed under the [MIT License](LICENSE) of this project.
