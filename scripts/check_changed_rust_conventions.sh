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

if [[ -z "$base_revision" ]] \
    && git rev-parse --verify --quiet "origin/main^{commit}" >/dev/null; then
    base_revision="$(git merge-base "$head_revision" origin/main)"
fi

if [[ -z "$base_revision" ]]; then
    printf 'unable to determine the base revision for the changed-file check\n' >&2
    printf 'pass it explicitly: %s <base-revision> <head-revision>\n' "$0" >&2
    exit 1
fi

for revision in "$base_revision" "$head_revision"; do
    if ! git rev-parse --verify "$revision^{commit}" >/dev/null 2>&1; then
        printf 'base or head revision is not present in the checkout: %s\n' "$revision" >&2
        printf 'check out full history (fetch-depth: 0) so the base commit is available\n' >&2
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
