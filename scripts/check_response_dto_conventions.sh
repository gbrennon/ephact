#!/usr/bin/env bash
set -euo pipefail

response_directory="${1:-crates/application/src/dtos/responses}"

if [[ ! -d "$response_directory" ]]; then
    printf 'response DTO directory does not exist: %s\n' "$response_directory" >&2
    exit 1
fi

status=0
while IFS= read -r file; do
    filename="$(basename "$file")"
    stem="${filename%.rs}"
    expected_name="$(printf '%s\n' "$stem" | awk -F_ '{for (i = 1; i <= NF; i++) printf toupper(substr($i, 1, 1)) substr($i, 2)}')"
    if ! grep -Eq "^[[:space:]]*pub (struct|enum) ${expected_name}([[:space:]]|\{|<|;)" "$file"; then
        printf '%s: expected public response type %s\n' "$file" "$expected_name" >&2
        status=1
    fi
done < <(find "$response_directory" -maxdepth 1 -type f -name '*_response.rs' -print | sort)

exit "$status"
