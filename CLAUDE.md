# `awsum-zed`

`awsum-zed` is the Zed extension for Awsum (`.aww` files). It is a thin client to the bundled `awsum lsp` server — every diagnostic, code action, format edit, document symbol, and workspace symbol is computed inside the `awsum` compiler binary and pushed over LSP.

## Quick reference

```bash
cargo build --target wasm32-wasip2 --release   # compile the extension
```

Zed loads the extension as a WebAssembly module; install it locally via Zed's "Install Dev Extension" command pointing at this directory.

## Structure

```
extension.toml                       # Manifest: id, version, grammar pin, language servers
Cargo.toml                           # Rust crate (cdylib targeting wasm32-wasip2)
src/lib.rs                           # AwsumExtension impl — locates `awsum` on PATH, runs `awsum lsp`
languages/awsum/config.toml          # Language config (file extensions, comments, brackets)
languages/awsum/highlights.scm       # Syntax-highlight query (tree-sitter)
languages/awsum/outline.scm          # Outline / breadcrumb query (tree-sitter)
```

The tree-sitter grammar itself lives in [`awsum-lang/tree-sitter-awsum`](https://github.com/awsum-lang/tree-sitter-awsum) and is referenced by `extension.toml`'s `[grammars.awsum]` section with a commit pin updated each release.

## Features (all delivered through `awsum lsp`)

- **Syntax highlighting** — Tree-sitter grammar from `tree-sitter-awsum` + `highlights.scm`.
- **Format on save** — `textDocument/formatting`. The same `awsum format` algorithm the CLI uses.
- **Diagnostics** — `textDocument/publishDiagnostics`. Pushed on open / save / change (debounced 500 ms server-side). Severity (`error` vs `warning`) honoured by Zed's theme.
- **Quick fixes (lightbulb)** — `textDocument/codeAction`. Compiler-supplied fixes only; the extension does no language-aware reasoning.
- **Document symbols (outline / breadcrumbs)** — `textDocument/documentSymbol`. Top-level functions, constants, types.
- **Workspace symbols (`Cmd+T`)** — `workspace/symbol`. Walks every `.aww` under the workspace folders received at `initialize`.

## Versioning

`awsum-zed A.B.C` ↔ `awsum A.B.C` ↔ `tree-sitter-awsum A.B.C` ↔ `awsum-vscode A.B.C`. One version, four artefacts. The lockstep convention is enforced because the LSP server lives inside the `awsum` binary itself, not in a separate `awsum-lsp` artefact.

**The version is duplicated in two places inside this repo.** Zed's manifest format requires both:

- `extension.toml` ⇒ what Zed's UI / marketplace displays as the installed version.
- `Cargo.toml` ⇒ baked into the wasm via `env!("CARGO_PKG_VERSION")`, used by `language_server_initialization_options` to populate `expectedAwsumVersion`.

A release-time bump must touch both. [`build.rs`](build.rs) runs at every `cargo build` / `cargo check`, parses `extension.toml`, and panics if its `version` field disagrees with `CARGO_PKG_VERSION`. So drift fails the build with a clear message — never silently lands a wrong-version release.

**Mismatch detection (against the compiler).** `language_server_initialization_options` ships `{"expectedAwsumVersion": env!("CARGO_PKG_VERSION")}` in the LSP `initialize` request. The server compares against its own (compiler) version and pushes a `window/showMessage` warning on mismatch. Same code path runs for `awsum-vscode` and any other LSP client that opts in.

## Related Repositories

- Compiler: [`awsum-lang/awsum`](https://github.com/awsum-lang/awsum) (hosts `awsum lsp`)
- Tree-sitter grammar: [`awsum-lang/tree-sitter-awsum`](https://github.com/awsum-lang/tree-sitter-awsum)
- VSCode: [`awsum-lang/awsum-vscode`](https://github.com/awsum-lang/awsum-vscode)
- Website: [`awsum-lang/awsum-lang.org`](https://github.com/awsum-lang/awsum-lang.org)
