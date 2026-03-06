# cmds

A collection of CLI tools built with Rust.

## Build

```bash
cargo build --release
```

## Install all tools

```bash
make install
```

## Add a new tool

1. Create a new directory under `crates/`:

```bash
mkdir -p crates/my-tool/src
```

2. Add `crates/my-tool/Cargo.toml`:

```toml
[package]
name = "my-tool"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { workspace = true }
anyhow = { workspace = true }
cli-common = { path = "../cli-common" }
```

3. Add `crates/my-tool/src/main.rs` and start coding.
