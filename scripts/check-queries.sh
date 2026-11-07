#!/bin/sh
set -eu

# Verify languages/awsum/*.scm are identical to tree-sitter-awsum's queries at
# the commit pinned in extension.toml. The release workflow gates on this.

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

exit_code=0

# every upstream query at the pin must exist here and match
for path in $(git ls-tree -r --name-only "$commit" queries | grep '\.scm$'); do
  fname=$(basename "$path")
  if ! git show "$commit:$path" | diff -q - "$queries_dir/$fname" > /dev/null 2>&1; then
    echo "DRIFT: languages/awsum/$fname differs from tree-sitter-awsum@$commit:$path"
    exit_code=1
  fi
done

# any query here that does not exist upstream is also drift
for local_q in "$queries_dir"/*.scm; do
  [ -e "$local_q" ] || continue
  fname=$(basename "$local_q")
  if ! git cat-file -e "$commit:queries/$fname" 2>/dev/null; then
    echo "DRIFT: languages/awsum/$fname exists here but not at tree-sitter-awsum@$commit"
    exit_code=1
  fi
done

if [ "$exit_code" -ne 0 ]; then
  echo
  echo "Run 'just sync-queries' to re-sync." >&2
  exit "$exit_code"
fi

echo "OK — queries match tree-sitter-awsum@$commit"
