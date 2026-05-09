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

use zed_extension_api::{self as zed, Result};

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
}

zed::register_extension!(AwsumExtension);
