#!/usr/bin/env bash
set -euo pipefail

base_revision="${1:-${GITHUB_BASE_SHA:-}}"
head_revision="${2:-${GITHUB_SHA:-HEAD}}"

if [[ -z "$base_revision" && -n "${GITHUB_EVENT_PATH:-}" && -f "$GITHUB_EVENT_PATH" ]]; then
    base_revision="$(grep -oE '"before"[[:space:]]*:[[:space:]]*"[0-9a-f]+"' "$GITHUB_EVENT_PATH" \
        | head -n 1 | cut -d '"' -f 4)"
fi

if [[ "$base_revision" =~ ^0+$ ]]; then
    base_revision=""
fi

if [[ -z "$base_revision" ]]; then
    if ! git fetch --no-tags --quiet origin main; then
        printf 'failed to fetch origin main while resolving the base revision\n' >&2
        exit 1
    fi
    if ! base_revision="$(git merge-base "$head_revision" FETCH_HEAD)"; then
        printf 'failed to compute the merge-base of %s and origin/main\n' \
            "$head_revision" >&2
        exit 1
    fi
fi

for revision in "$base_revision" "$head_revision"; do
    if ! git rev-parse --verify "$revision^{commit}" >/dev/null 2>&1; then
        printf 'revision is unavailable: %s\n' "$revision" >&2
        exit 1
    fi
done

changed_file_list="$(mktemp)"
trap 'rm -f "$changed_file_list"' EXIT

git diff --name-only --diff-filter=ACMR "$base_revision" "$head_revision" -- '*.rs' \
    > "$changed_file_list"

if [[ ! -s "$changed_file_list" ]]; then
    exit 0
fi

mapfile -t changed_files < "$changed_file_list"
scripts/check_rust_file_conventions.sh "${changed_files[@]}"
