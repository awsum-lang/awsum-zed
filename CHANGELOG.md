# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

`awsum-zed` is versioned 1:1 with the `awsum` compiler — the extension's `A.B.C` is exactly the `awsum` `A.B.C` it targets. The Zed extension registry only accepts plain semver, so we collapse the two version axes into one: every `awsum` release ships a matching extension release, the extension is never released ahead of the compiler, and only the latest `awsum` release is supported.

Until `awsum 1.0.0`, the project does not follow SemVer — every release increments only the patch (`0.0.1 → 0.0.2 …`), and any release may break. The 1:1 lockstep above is the contract that does hold: within a single `0.0.x`, the extension and the `awsum` it ships against are mutually compatible.

## [Unreleased]

### Changed

- `rust-toolchain.toml` adds the `rust-analyzer` component, so editors run the server that matches the pinned toolchain.

## [0.0.8] - 2026-11-07

### Fixed

- **Nested block comments highlight correctly.** The grammar pin moves to `tree-sitter-awsum` 0.0.8, which parses `{- outer {- inner -} more -}` as one comment; before, a nested comment broke highlighting of the declaration after it.
- **Bare type names in signatures are highlighted as types.** The extension's copy of the highlight query had fallen behind `tree-sitter-awsum`'s and coloured only applied types (`Maybe Int32`), so `Pick` and `Int32` in `pick : Pick -> Int32` stayed plain. Both query files (`highlights.scm`, `outline.scm`) are now exact copies of `tree-sitter-awsum` at the pinned commit.

### Changed

- Lockstep release with `awsum` 0.0.8.
- Rust toolchain 1.93.1 → 1.99.1 (local `rust-toolchain.toml` and CI); `Cargo.lock` regenerated, holding back the two crates published within the last 7 days (`flate2` 1.1.10, `smallvec` 1.16.4).
- CI: every GitHub Action pinned to the commit SHA of a release at least 7 days old.
- Format on save is no longer advertised as built in. Since Zed turned format-on-save off by default (June 2026), it applies to Awsum only once enabled in the user's settings; the README now shows the setting. The `[format_on_save]` table in `languages/awsum/config.toml` is removed — Zed never read it (its language config has no such field); format-on-save used to work only because of Zed's former global default.
- `just sync-queries` copies `tree-sitter-awsum`'s queries at the commit pinned in `extension.toml`, and `just check-queries` verifies them. The release workflow runs the check on every tag, so the copies can no longer drift from the pinned grammar.
- The README lists hover (type and doc comment of the name under the cursor). `awsum lsp` has served it since 0.0.5; only the documentation was missing it.

## [0.0.7] - 2026-06-22

### Changed

- Lockstep release with `awsum` 0.0.7. Tree-sitter grammar pin bumped to `tree-sitter-awsum` 0.0.7.

## [0.0.6] - 2026-06-03

### Changed

- Lockstep release with `awsum` 0.0.6. Tree-sitter grammar pin bumped to `tree-sitter-awsum` 0.0.6.

## [0.0.5] - 2026-05-31

### Changed

- Lockstep release with `awsum` 0.0.5. Tree-sitter grammar pin bumped to `tree-sitter-awsum` 0.0.5.

## [0.0.4] - 2026-05-13

### Added

- Initial release. Thin LSP client to `awsum lsp --stdio` (subcommand of the `awsum` compiler binary). Every editor feature is computed inside the compiler and pushed over LSP:
  - **Syntax highlighting** via the Tree-sitter grammar from [`awsum-lang/tree-sitter-awsum`](https://github.com/awsum-lang/tree-sitter-awsum) (commit-pinned in `extension.toml`).
  - **Format on save** via `textDocument/formatting` — same algorithm as `awsum format`.
  - **Inline diagnostics** via `textDocument/publishDiagnostics` (debounced 500 ms server-side; `error` / `warning` severity honoured by Zed's theme).
  - **Quick fixes** via `textDocument/codeAction` — compiler-supplied fixes only; the extension does no language-aware reasoning.
  - **Document outline / breadcrumbs** via `textDocument/documentSymbol`.
  - **Workspace symbol search** (`Cmd+T` / `Ctrl+T`) via `workspace/symbol`.
- Declarative lockstep version check: the extension passes `initializationOptions: { expectedAwsumVersion, preferButtonsOverLinks: false }` and the server warns on mismatch via `window/showMessage`. The expected version comes from `env!("CARGO_PKG_VERSION")` baked into the wasm at build time. `preferButtonsOverLinks: false` is set explicitly because Zed currently doesn't open external URLs from `window/showDocument` — an inline URL in the notification is the working UX.
- `build.rs` version-sync guard: panics at `cargo build` / `cargo check` if `Cargo.toml`'s `CARGO_PKG_VERSION` and `extension.toml`'s top-level `version` disagree. Drift would mean the marketplace UI shows one version while the LSP handshake claims another.
- Release workflow: pushing a `v*` tag builds the extension and publishes a GitHub Release with the `.wasm` attached. Tag and `Cargo.toml` version must match, or the run fails before the build; the `extension.toml` ↔ `Cargo.toml` consistency is enforced by `build.rs` during the build step itself.
- Build provenance via `actions/attest-build-provenance@v4` on the published `.wasm` — each release asset gets a Sigstore-signed attestation tying it to the release workflow run and the tagged commit. Users verify with `gh attestation verify <file>.wasm --repo awsum-lang/awsum-zed`.
- `CONTRIBUTING.md` — covers the dev-loop commands, the signed-commits requirement on `main`, and the PR / CHANGELOG conventions.
- `justfile` with a single user-facing `just release` recipe — checks out `main`, pulls, reads the version from `Cargo.toml`, asks the operator to type the version back as confirmation, then creates an annotated tag and pushes it. Mirrors the same recipe in `awsum/justfile` and `awsum-vscode/justfile`.
