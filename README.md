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


## Contribution
This projects tracks the `stable` Rust toolchain, and assumes the `default` profile has been installed.

[`cargo-husky`](https://github.com/rhysd/cargo-husky) is used for Git hooks, and requires running `cargo test` once to set them up.