# Rust_Practice

Welcome! This repository is a playground for my journey learning Rust. Below are quick notes on getting Rust set up, compiling and running the code, and tearing it down if you need to start over.

## Table of Contents

- [Rust\_Practice](#rust_practice)
  - [Table of Contents](#table-of-contents)
  - [Prerequisites](#prerequisites)
  - [Installing Rust](#installing-rust)
    - [Install via rustup (recommended)](#install-via-rustup-recommended)
    - [Verify the installation](#verify-the-installation)
  - [Updating Rust](#updating-rust)
  - [Uninstalling Rust](#uninstalling-rust)
  - [Building the project](#building-the-project)
  - [Running the project](#running-the-project)
  - [Cleaning build artifacts](#cleaning-build-artifacts)
  - [Next steps \& helpful commands](#next-steps--helpful-commands)
  - [Resources](#resources)

## Prerequisites

* A terminal / command prompt
* A code editor (VS Code + rust‑analyzer extension is recommended)

## Installing Rust

### Install via rustup (recommended)

`rustup` is the official installer and version manager for Rust.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* **Windows** – download and run [`rustup-init.exe`](https://rustup.rs) *or* use Chocolatey:<br/>`choco install rustup.install -y`
* **macOS** – Homebrew alternative:<br/>`brew install rustup-init && rustup-init`

During setup choose *default installation* unless you have special needs.

### Verify the installation

```bash
rustc --version
cargo --version
```

Both commands should print a version ≥ `1.80` (stable channel, July 2025).

## Updating Rust

```bash
rustup update
```

This updates the toolchain (`rustc`, `cargo`, stdlib) to the latest stable release.

## Uninstalling Rust

```bash
rustup self uninstall
```

Follow the prompt to remove Rust and all toolchains.
If you installed with a package manager, remove using the same tool, e.g. `brew uninstall rust` or `choco uninstall rustup.install`.

## Building the project

From the repo root:

```bash
cargo build --release   # optimized build
# or
cargo build             # debug build (faster compile)
```

The compiled binary lands in `target/release/` or `target/debug/`.

## Running the project

```bash
cargo run               # compile (if needed) & execute
cargo run --bin app     # run a specific binary target
```

To run a standalone file without Cargo (rarely needed):

```bash
rustc src/main.rs && ./main
```

## Cleaning build artifacts

```bash
cargo clean
```

## Next steps & helpful commands

| Task             | Command                          |
| ---------------- | -------------------------------- |
| Run unit tests   | `cargo test`                     |
| Check code style | `cargo fmt --check`              |
| Auto‑format      | `cargo fmt`                      |
| Lint (Clippy)    | `cargo clippy -- -W clippy::all` |
| Generate docs    | `cargo doc --open`               |

## Resources

* [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
* [Rustlings](https://github.com/rust-lang/rustlings)
* [Async Rust](https://rust-lang.github.io/async-book/)
* [Cargo Book](https://doc.rust-lang.org/cargo/)
