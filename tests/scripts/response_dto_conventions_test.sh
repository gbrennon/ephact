#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
checker="$repository_root/scripts/check_response_dto_conventions.sh"
fixture_directory="$(mktemp -d)"
trap 'rm -rf "$fixture_directory"' EXIT

if [[ ! -x "$checker" ]]; then
    printf 'response DTO convention checker is missing or not executable\n' >&2
    exit 1
fi

mkdir -p "$fixture_directory/responses"
printf 'pub struct ExampleResponse;\n' > "$fixture_directory/responses/example_response.rs"
"$checker" "$fixture_directory/responses"

printf 'pub struct WrongNameResponse;\n' > "$fixture_directory/responses/example_response.rs"

if "$checker" "$fixture_directory/responses"; then
    printf 'checker accepted a mismatched response DTO name\n' >&2
    exit 1
fi

printf 'response DTO convention checker rejected invalid fixture\n'
