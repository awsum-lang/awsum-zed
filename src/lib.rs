//! Awsum extension for Zed.
//!
//! Tiny LSP-client glue: locate the `awsum` binary on the user's PATH
//! and spawn it as `awsum lsp`. The compiler ships the language server
//! as a subcommand of the same binary, so there is no separate
//! `awsum-lsp` to install — and no version-skew possible between the
//! editor and the compiler.
//!
//! Every feature the user sees (diagnostics, quick fixes, format, outline,
//! workspace symbols) is delivered by the `awsum lsp` server itself; this
//! crate exists only to register the language with Zed and tell it how
//! to start the server.

use zed_extension_api::{self as zed, serde_json, Result};

struct AwsumExtension;

impl zed::Extension for AwsumExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Resolve through the worktree's view of PATH so user-level
        // installs (`stack install`, Homebrew, manual `~/bin/awsum`)
        // all work without per-user config. If the binary is missing,
        // Zed surfaces the `Err` as a non-blocking notification — the
        // user sees a clear "awsum not found in PATH" rather than a
        // mysterious silent no-op.
        let command = worktree
            .which("awsum")
            .ok_or_else(|| {
                "couldn't find the `awsum` binary on PATH. Install it from \
                 https://github.com/awsum-lang/awsum (or via `stack install` \
                 from a checkout) and ensure the install directory is on PATH."
                    .to_string()
            })?;

        // `--stdio` is part of the contract — `awsum lsp` requires the
        // transport flag explicitly (no defaulting), same shape the
        // VS Code extension produces via `TransportKind.stdio`.
        Ok(zed::Command {
            command,
            args: vec!["lsp".to_string(), "--stdio".to_string()],
            env: Vec::new(),
        })
    }

    /// Tell the server which `awsum` version this extension was built
    /// against. The server compares against the compiler's own version
    /// and pushes a `window/showMessage` warning on mismatch — same
    /// lockstep guarantee as `awsum-vscode`, expressed via the
    /// LSP-standard `initializationOptions` payload.
    ///
    /// `env!("CARGO_PKG_VERSION")` resolves at compile time to the
    /// version string in [`Cargo.toml`], which the release process
    /// keeps in lockstep with the awsum compiler version.
    fn language_server_initialization_options(
        &mut self,
        _server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // `expectedAwsumVersion` triggers the lockstep version check.
        // `preferButtonsOverLinks: false` tells the server to render
        // the mismatch warning via `window/showMessage` with the URL
        // inline — Zed auto-linkifies notification URLs but currently
        // doesn't open external https URLs from `window/showDocument`,
        // so a button-driven path would surface a non-functional
        // button. The default is `false` already; we set it
        // explicitly to document the editor's UX choice.
        Ok(Some(serde_json::json!({
            "expectedAwsumVersion": env!("CARGO_PKG_VERSION"),
            "preferButtonsOverLinks": false,
        })))
    }
}

zed::register_extension!(AwsumExtension);
