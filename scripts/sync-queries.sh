#!/bin/sh
set -eu

# Copy the Tree-sitter queries from the sibling tree-sitter-awsum repo, at the
# commit pinned in extension.toml, into languages/awsum/. Zed reads queries
# only from the extension, never from the grammar repo, so these files are
# copies of upstream and must stay identical to it — edit queries upstream,
# then re-sync. scripts/check-queries.sh verifies the copies at release time.

script_dir=$(cd "$(dirname "$0")" && pwd)
target_dir=$(cd "$script_dir/.." && pwd)
ts_repo="$target_dir/../tree-sitter-awsum"
queries_dir="$target_dir/languages/awsum"

commit=$(sed -n 's/^commit = "\([0-9a-f]*\)"$/\1/p' "$target_dir/extension.toml")
if [ -z "$commit" ]; then
  echo "error: no 'commit = \"…\"' pin found in extension.toml" >&2
  exit 1
fi

if [ ! -d "$ts_repo/.git" ]; then
  echo "error: tree-sitter-awsum repo not found at $ts_repo" >&2
  exit 1
fi

cd "$ts_repo"
git rev-parse --verify "$commit^{commit}" > /dev/null

# Re-populate the whole set, so a query upstream removes or renames does not
# linger here.
rm -f "$queries_dir"/*.scm
for path in $(git ls-tree -r --name-only "$commit" queries | grep '\.scm$'); do
  git show "$commit:$path" > "$queries_dir/$(basename "$path")"
done

echo "Copied queries from tree-sitter-awsum@$commit"
