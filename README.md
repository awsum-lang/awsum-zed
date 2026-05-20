# Awsum for Zed

Zed extension for the [Awsum](https://awsum-lang.org) programming language (`.aww` files).

## Features

- Syntax highlighting (Tree-sitter)
- Format on save
- Inline diagnostics (errors + warnings)
- Quick fixes (lightbulb code actions)
- Document outline / breadcrumbs
- Workspace symbol search (`Cmd+T` / `Ctrl+T`)

All of the above are powered by the `awsum` compiler's bundled language server — there is no separate `awsum-lsp` to install. As long as the `awsum` binary is on your `PATH`, the extension will spawn it as `awsum lsp` and route every editor request through it.

## Install

1. Install the Awsum compiler (see [awsum-lang/awsum](https://github.com/awsum-lang/awsum)) and ensure `awsum` is on your `PATH`.
2. Open Zed → `Cmd+Shift+P` → `zed: extensions` → search "Awsum" → Install.

## Restart the LSP server

Use Zed's built-in command:

```
zed: command palette → editor: restart language server
```

Stops the `awsum lsp` process for the active buffer and starts a new one with the same settings. Useful after a local `stack install` of a new `awsum` build, or to clear any in-memory state on the server. (The Zed extension API does not allow extensions to register editor commands, so `awsum-zed` rides Zed's built-in restart command rather than adding its own).

## Versioning

`awsum-zed A.B.C` is built and tested against `awsum A.B.C`. Mismatched versions are not supported — at startup the language server compares the extension's expected version against its own and shows a Zed notification on mismatch.

## Related

- Compiler (hosts `awsum lsp`): [awsum-lang/awsum](https://github.com/awsum-lang/awsum)
- Tree-sitter grammar: [awsum-lang/tree-sitter-awsum](https://github.com/awsum-lang/tree-sitter-awsum)
- VSCode extension: [awsum-lang/awsum-vscode](https://github.com/awsum-lang/awsum-vscode)
- Website: [awsum-lang.org](https://awsum-lang.org)

## License

MIT — see [LICENSE](LICENSE).
