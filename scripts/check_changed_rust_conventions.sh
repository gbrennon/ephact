#!/usr/bin/env bash
set -euo pipefail

base_revision="${GITHUB_BASE_SHA:-HEAD^}"
head_revision="${GITHUB_SHA:-HEAD}"

if ! git rev-parse --verify "$base_revision^{commit}" >/dev/null 2>&1; then
    git fetch --no-tags origin "$base_revision"
fi

if ! git rev-parse --verify "$base_revision^{commit}" >/dev/null 2>&1; then
    printf 'base revision is unavailable: %s\n' "$base_revision" >&2
    exit 1
fi

mapfile -t changed_files < <(
    git diff --name-only "$base_revision" "$head_revision" -- '*.rs'
)

if (( ${#changed_files[@]} == 0 )); then
    exit 0
fi

scripts/check_rust_file_conventions.sh "${changed_files[@]}"
