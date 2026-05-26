# Contributing to `awsum-zed`

Thanks for your interest in contributing.

## Development setup

See [README.md](README.md) for an overview. Quick reference:

```bash
rustup target add wasm32-wasip2
cargo build --target wasm32-wasip2 --release
```

The extension shells out to the `awsum` CLI for diagnostics, formatting, code actions, and document/workspace symbols — install it from the [compiler repo](https://github.com/awsum-lang/awsum) before testing changes that touch those features.

To try a local build inside Zed: open the command palette → `zed: install dev extension` → point at this directory. Zed compiles the wasm and registers the language server on the spot; subsequent rebuilds require running the command again.

## Developer Certificate of Origin

By contributing to `awsum-zed` you certify the [Developer Certificate of Origin](https://developercertificate.org/) (DCO) for your contribution — a short statement that you wrote the patch yourself, or otherwise have the right to submit it under the project's [Apache-2.0 license](LICENSE). The full text is at the link above.

After cloning, run once:

```bash
just setup-dev
```

This installs the `prepare-commit-msg` hook from [scripts/git-hooks/](scripts/git-hooks/) (via per-clone `core.hooksPath`), which adds a `Signed-off-by` trailer to every commit you make in this clone:

```
Signed-off-by: Your Name <you@example.com>
```

The trailer uses the name and email from your `[user]` section in `~/.gitconfig` (the same one used for signed commits below). No manual flags, no global gitconfig changes. The setup is per-clone — repeat in each clone of the repo.

## Signed commits

Separately from the DCO trailer above, the `main` branch requires signed commits — every commit you push to a PR needs a verified signature (GPG or SSH), otherwise the merge button stays grey.

Minimal `~/.gitconfig` for SSH signing:

```ini
[user]
	email = ...
	name = ...
	signingkey = ~/.ssh/id_ed25519.pub
[commit]
	gpgsign = true
[gpg]
	format = ssh
```

For GPG signing instead, set `gpg.format = openpgp` (or omit — that's the default) and point `signingkey` at your GPG key ID. The option name `gpgsign` is git's historical name for "sign this thing" and applies regardless of format.

The same key file must be added to GitHub Settings → SSH and GPG keys as a **Signing Key** (a separate category from Authentication Key, even if you reuse the same file). Verify locally:

```bash
git commit -S -m "test" --allow-empty
git log --show-signature -1
```

If you already made unsigned commits on a feature branch, retroactively sign with:

```bash
git rebase --exec 'git commit --amend --no-edit -S' <range>
```

then force-push your branch.

## Pull requests

- Open against `main`. CI (`ci.yml`) must be green before merge.
- For user-visible changes, add a bullet under `## [Unreleased]` in [CHANGELOG.md](CHANGELOG.md). Infrastructure-only changes (CI, dev tooling, internal refactors) still get an entry so the next release notes are complete.
- Versions are 1:1 with the `awsum` compiler. Bumping the version touches both `Cargo.toml` and `extension.toml` — [`build.rs`](build.rs) fails the build if they disagree.
