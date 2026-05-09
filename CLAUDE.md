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

**Known limitation (v0):** unlike `awsum-vscode`, this extension does **not** check the version reported by `serverInfo` against its manifest version. `zed_extension_api` doesn't expose the LSP `initialize` response to the extension code, so we have no clean hook to compare. A user with a mismatched `awsum` on PATH will see degraded behavior (features added on either side that the other doesn't implement) without an explicit warning. If this matters in practice, options are: (1) shell out to `awsum --version` from `language_server_command` and compare manually, or (2) request `initializeResult` access from upstream `zed_extension_api`.

## Related Repositories

- Compiler: [`awsum-lang/awsum`](https://github.com/awsum-lang/awsum) (hosts `awsum lsp`)
- Tree-sitter grammar: [`awsum-lang/tree-sitter-awsum`](https://github.com/awsum-lang/tree-sitter-awsum)
- VSCode: [`awsum-lang/awsum-vscode`](https://github.com/awsum-lang/awsum-vscode)
- Website: [`awsum-lang/awsum-lang.org`](https://github.com/awsum-lang/awsum-lang.org)
