# SushiCutters

## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```

## Profiling

To build the project in profiling mode run

```bash
cargo build --release --features=benchmark
```

This will add a performance counter to the project and kill the executable after 30 seconds

The simple benchmark can be run with

```bash
cargo run --release --features=benchmark <bench_name>
```

This will give stats like average FPS and frame_time.

Running the executable directly requires the assets and config files
A script called `./copy_assets` is provided to do so

### CPU Profiling

```bash
perf record --call-graph dwarf,16384 -e cpu-clock -F 997 target/release/sushi_cutters <bench_name>
```

The perf file can either be read with `perf report` or can be used
to generate a flame graph with

```bash
perf script | stackcollapse-perf.pl | stackcollapse-recursive.pl | c++filt | rust-unmangle | flamegraph.pl > flame.svg
```

[`perf` and flamegraph setup](https://gist.github.com/dlaehnemann/df31787c41bd50c0fe223df07cf6eb89)

### Memory Profiling

Memory profiling is done with [heaptrack](https://github.com/KDE/heaptrack/)

```bash
heaptrack target/release/sushi_cutters <bench_name>
```

### Bench specific

`enemies_bench`: has an environment variable called `ENEMY_COUNT` that sets the number of enemies. Useful for getting multiple measurements

---

## Contribution

This projects tracks the `stable` Rust toolchain, and assumes the `default` profile has been installed.

[`cargo-husky`](https://github.com/rhysd/cargo-husky) is used for Git hooks, and requires running `cargo test` once to
set them up.

## Attribution

Fonts are from Mozilla's Fira fonts (as of `fd8c8c0a3d353cd99e8ca1662942d165e6961407`), and licensed under
[SIL OPEN FONT LICENSE Version 1.1](https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL_web)
