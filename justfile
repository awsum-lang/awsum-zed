_default:
  @ just --list --unsorted

# One-time post-clone setup: installs the prepare-commit-msg hook from
# scripts/git-hooks/ so every commit in this clone auto-adds the DCO
# Signed-off-by trailer. See CONTRIBUTING.md ("Developer Certificate of Origin").
setup-dev:
  #!/bin/sh
  set -eu
  git config core.hooksPath scripts/git-hooks
  chmod +x scripts/git-hooks/prepare-commit-msg
  echo "✅ DCO prepare-commit-msg hook installed for this clone"

# Build the extension wasm (release profile, same target as CI)
build:
  cargo build --target wasm32-wasip2 --release

# Check formatting and run clippy with warnings denied — same checks CI runs
lint:
  cargo fmt --check
  cargo clippy --target wasm32-wasip2 --all-targets -- -D warnings

# Apply rustfmt in place
fmt:
  cargo fmt

# Confirm potentially dangerous actions with a specific confirmation input (e.g. version, environment name)
[private]
manual-confirmation-input message required_confirmation:
  #!/bin/sh
  set -eu

  message="{{ message }}"
  required_confirmation="{{ required_confirmation }}"

  echo "$message"
  echo "Type '$required_confirmation' to confirm:"
  read response

  if [ "$response" != "$required_confirmation" ]; then
    echo "Confirmation failed. Exiting..."
    exit 1
  fi

# Tag and push the version currently in Cargo.toml. Run after the prep PR is merged into main.
release:
  #!/bin/sh
  set -eu
  git checkout main
  git pull
  version=$(grep -m1 '^version *= *' Cargo.toml | cut -d'"' -f2)
  just manual-confirmation-input "About to tag and push v$version" "$version"
  git tag -a "v$version" -m "Release $version"
  git push origin "v$version"
