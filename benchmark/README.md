# xsd-parser Benchmarks

This crate contains end-to-end performance and memory benchmarks for `xsd-parser` and the generated types.

It is intended for:
- comparing different generated backends (e.g. quick-xml vs serde-based implementations),
- tracking performance and memory regressions,

The goal is to benchmark realistic end-to-end flows.

## Quickstart

Build and run in release mode:

```bash
cargo run -p benchmark --release
```

You can also pass CLI flags:

```bash
cargo run -p benchmark --release -- --help
```

If you are interested in benchmarks of the debug build you can also run the following command. This might be interesting because if you execute `xsd-parser` inside the build script of your crate, it is usually compiled using
debug build.

```bash
cargo run -p benchmark
```

## Outputs

The benchmark runner prints a summary table to the terminal

Typical metrics include:
- Runtime (min / max / avg / median across runs),
- Stack usage.

The reported stack usage is a high-water-mark estimate based on stack painting and scanning. It is Linux-only and should be treated as an approximation.

## Adding a new benchmark case

1. Extend the build script to generate the code from the schema
2. Add a new module in `benchmark/src/schemas/`
3. Register it in `benchmark/src/schemas/mod.rs`
4. Implement the benchmark entrypoints used by the runner (`benchmark/src/main.rs`).

Guidelines:
- Prefer end-to-end benchmarks that reflect real-world usage:
  - parse XML from bytes,
  - deserialize into the generated structs,
  - (optionally) re-serialize to XML.
- Ensure test cases are reproducible:
  - fixed seeds,
  - fixed input payloads,
  - avoid reading from the network.
- Use `--release` when comparing results.
- Consider multiple payload sizes (small/medium/large) to get throughput curves.
