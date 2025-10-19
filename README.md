<div align="center" markdown="1">

# LLMWereWolf

[![Crates.io](https://img.shields.io/crates/v/llmwerewolf?logo=rust&style=flat-square&color=E05D44)](https://crates.io/crates/llmwerewolf)
[![Crates.io Downloads](https://img.shields.io/crates/d/llmwerewolf?logo=rust&style=flat-square)](https://crates.io/crates/llmwerewolf)
[![npm version](https://img.shields.io/npm/v/llmwerewolf?logo=npm&style=flat-square&color=CB3837)](https://www.npmjs.com/package/llmwerewolf)
[![npm downloads](https://img.shields.io/npm/dt/llmwerewolf?logo=npm&style=flat-square)](https://www.npmjs.com/package/llmwerewolf)
[![PyPI version](https://img.shields.io/pypi/v/llmwerewolf?logo=python&style=flat-square&color=3776AB)](https://pypi.org/project/llmwerewolf/)
[![PyPI downloads](https://img.shields.io/pypi/dm/llmwerewolf?logo=python&style=flat-square)](https://pypi.org/project/llmwerewolf/)
[![rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust&logoColor=white&style=flat-square)](https://www.rust-lang.org/)
[![tests](https://img.shields.io/github/actions/workflow/status/Mai0313/LLMWereWolf/test.yml?label=tests&logo=github&style=flat-square)](https://github.com/Mai0313/LLMWereWolf/actions/workflows/test.yml)
[![code-quality](https://img.shields.io/github/actions/workflow/status/Mai0313/LLMWereWolf/code-quality-check.yml?label=code-quality&logo=github&style=flat-square)](https://github.com/Mai0313/LLMWereWolf/actions/workflows/code-quality-check.yml)
[![license](https://img.shields.io/badge/License-MIT-green.svg?labelColor=gray&style=flat-square)](https://github.com/Mai0313/LLMWereWolf/tree/master?tab=License-1-ov-file)
[![PRs](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/Mai0313/LLMWereWolf/pulls)

</div>

🚀 A Rust rewrite of **LLM Werewolf**—the social deduction game powered by LLM agents. This repo hosts the core engine scaffold, release tooling, and multi-language package surfaces that the upcoming implementation will build upon.

Project languages: [English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md)

## ✨ Highlights

- Modern Cargo layout (`src/lib.rs`, `src/main.rs`, `tests/`)
- Dynamic version information with git metadata (tag, commit hash, build tools)
- Lint & format with clippy and rustfmt
- GitHub Actions: tests, quality, package build, Docker publish, release drafter, Rust-aware labeler, secret scans, semantic PR, weekly dependency update
- Multi-stage Dockerfile producing a minimal runtime image

## 🚀 Quick Start

**Requirements:**

- Rust 1.85 or higher (using Edition 2024)
- Docker (optional)

Install Rust via `rustup` if you haven't already.

```bash
make fmt            # rustfmt + clippy
make test           # cargo test (all targets)
make test-verbose   # cargo test (all targets with verbose output)
make coverage       # generate LCOV coverage report
make build          # cargo build (release mode)
make build-release  # cargo build --release
make run            # run the release binary
make clean          # clean build artifacts and caches
make package        # build crate package (allow dirty)
make help           # list targets
```

## 📌 Version Information

The binary automatically displays dynamic version information including:

- Git tag version (or `Cargo.toml` version if no tags)
- Commit count since last tag
- Short commit hash
- Dirty working directory indicator
- Rust and Cargo versions used for building

Example output:

```
llmwerewolf v0.1.25-2-gf4ae332-dirty
Built with Rust 1.90.0 and Cargo 1.90.0
```

This version information is embedded at build time through `build.rs` and automatically updated based on your git state.

## 🐳 Docker

```bash
docker build -f docker/Dockerfile --target prod -t ghcr.io/<owner>/<repo>:latest .
docker run --rm ghcr.io/<owner>/<repo>:latest
```

Or using the actual binary name:

```bash
docker build -f docker/Dockerfile --target prod -t llmwerewolf:latest .
docker run --rm llmwerewolf:latest
```

## 📦 Packaging

```bash
make package        # build crate package (allow dirty)
# or use cargo directly:
cargo package --locked --allow-dirty
# CARGO_REGISTRY_TOKEN=... cargo publish
```

CI builds run automatically on tags matching `v*` and upload the `.crate` file. Uncomment the publish step in `build_package.yml` to automate crates.io releases.

Distribution identifiers:

- Crate: `llmwerewolf`
- npm: `llmwerewolf` (and scoped `@mai0313/llmwerewolf`)
- PyPI: `llmwerewolf`

## 🧩 Cross Builds

This setup does not ship cross-compile tooling by default. If you need cross or zig-based builds locally, install and configure them per your environment.

GitHub Actions `build_release.yml` builds multi-platform release binaries on tags matching `v*` and uploads them to the GitHub Release assets.

Targets:

- x86_64-unknown-linux-gnu, x86_64-unknown-linux-musl
- aarch64-unknown-linux-gnu, aarch64-unknown-linux-musl
- x86_64-apple-darwin, aarch64-apple-darwin
- x86_64-pc-windows-msvc, aarch64-pc-windows-msvc

Assets naming:

- `<bin>-v<version>-<target>.tar.gz` (all platforms)
- `<bin>-v<version>-<target>.zip` (Windows additionally)

## 🔁 CI/CD Workflows

### Main Workflows

- Tests (`test.yml`): cargo build/test + generate LCOV coverage report and upload artifact
- Code Quality (`code-quality-check.yml`): rustfmt check + clippy (deny warnings)
- Build Package (`build_package.yml`): package on tag `v*`, optional crates.io publish
- Publish Docker Image (`build_image.yml`): push to GHCR on `main/master` and tags `v*`
- Build Release (`build_release.yml`): Linux release binaries uploaded on tags `v*`

### Additional Automation

- Auto Labeler (`auto_labeler.yml`): automatically label PRs based on branch names and file changes
- Code Scan (`code_scan.yml`): multi-layer security scanning (GitLeaks, Trufflehog secret scanning, CodeQL code analysis, Trivy vulnerability scanning)
- Release Drafter (`release_drafter.yml`): auto-generate release notes
- Semantic PR (`semantic-pull-request.yml`): enforce PR title format
- Dependabot weekly dependency updates

## 🤝 Contributing

- Open issues/PRs

- Use Conventional Commits for PR titles

- Keep code formatted and clippy‑clean

- After every edit, run `cargo build` to confirm compilation is successful

- Before opening a PR, please run locally:

  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`

## 📄 License

MIT — see `LICENSE`.
