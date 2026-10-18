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
        invalid_line="$(
            awk '
                /^[[:space:]]*$/ || /^[[:space:]]*#/ || /^[[:space:]]*\/\// { next }
                /^[[:space:]]*(pub([[:space:]]*\([^)]*\))?[[:space:]]+)?mod[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[[:space:]]*;/ { next }
                /^[[:space:]]*pub([[:space:]]*\([^)]*\))?[[:space:]]+use[[:space:]]+/ { next }
                { print NR ":" $0; exit }
            ' "$file"
        )"
        if [[ -n "$invalid_line" ]]; then
            printf '%s: %s: mod.rs may only declare modules or re-export symbols\n' \
                "$file" "$invalid_line" >&2
            status=1
        fi
        continue
    fi

    unbound_function="$(
        grep -nE '^(pub([[:space:]]*\([^)]*\))?[[:space:]]+)?(unsafe[[:space:]]+)?(async[[:space:]]+)?fn[[:space:]]+[A-Za-z_][A-Za-z0-9_]*' "$file" \
            | awk 'NR == 1 { print; exit }' || true
    )"
    if [[ "$filename" != main.rs && "$file" != */tests/* \
        && -n "$unbound_function" ]]; then
        function_name="$(printf '%s\n' "$unbound_function" \
            | sed -E 's/^[0-9]+:.*fn[[:space:]]+([A-Za-z_][A-Za-z0-9_]*).*/\1/')"
        printf '%s: %s: function %s is not inside an impl block; production functions must be bound to an implementation type\n' \
            "$file" "$unbound_function" "$function_name" >&2
        status=1
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

    struct_name="$(printf '%s\n' "${declarations[0]}" \
        | sed -E 's/.*[[:space:]]struct[[:space:]]+([A-Za-z_][A-Za-z0-9_]*).*/\1/')"
    expected_filename="$(printf '%s\n' "$struct_name" | snake_case).rs"

    if [[ "$filename" != "$expected_filename" ]]; then
        printf '%s: expected filename %s for struct %s\n' \
            "$file" "$expected_filename" "$struct_name" >&2
        status=1
    fi
done

exit "$status"
