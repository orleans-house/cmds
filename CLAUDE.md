# CLAUDE.md

## Project overview

Rust cargo workspace monorepo for CLI tools.

## Conventions

- New tools go under `crates/<tool-name>/`
- Use `workspace.dependencies` for shared dependencies (clap, anyhow, etc.)
- Use `clap` with derive macros for argument parsing
- Use `anyhow` for error handling
- Reference `cli-common` for shared utilities: `cli-common = { path = "../cli-common" }`
- Keep each tool focused and minimal
