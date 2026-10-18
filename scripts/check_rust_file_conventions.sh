#!/usr/bin/env bash
set -euo pipefail

if (( $# == 0 )); then
    printf 'usage: %s FILE...\n' "$0" >&2
    exit 2
fi

snake_case() {
    sed -E 's/([A-Z]+)([A-Z][a-z])|([a-z0-9])([A-Z])/\1\3_\2\4/g' \
        | tr '[:upper:]' '[:lower:]'
}

status=0

for file in "$@"; do
    [[ -f "$file" ]] || continue
    filename="$(basename "$file")"

    if [[ "$filename" == mod.rs ]]; then
        if grep -Eq '^[[:space:]]*(pub([[:space:]]*\([^)]*\))?[[:space:]]+)?(struct|enum|trait|fn|impl|const|static|type)[[:space:]]' "$file"; then
            printf '%s: mod.rs may only declare modules and re-export symbols\n' "$file" >&2
            status=1
        fi
        continue
    fi

    mapfile -t declarations < <(
        grep -E '^[[:space:]]*(pub([[:space:]]*\([^)]*\))?[[:space:]]+)?struct[[:space:]]+[A-Za-z_][A-Za-z0-9_]*' "$file" \
            || true
    )
    (( ${#declarations[@]} == 0 )) && continue

    if (( ${#declarations[@]} != 1 )); then
        printf '%s: files declaring structs must contain exactly one struct\n' "$file" >&2
        status=1
        continue
    fi

    struct_name="$(printf '%s\n' "${declarations[0]}" | awk '{for (i = 1; i <= NF; i++) if ($i == "struct") { print $(i + 1); exit }}')"
    expected_filename="$(printf '%s\n' "${struct_name}" | snake_case).rs"

    if [[ "$filename" != "$expected_filename" ]]; then
        printf '%s: expected filename %s for struct %s\n' \
            "$file" "$expected_filename" "$struct_name" >&2
        status=1
    fi
done

exit "$status"
