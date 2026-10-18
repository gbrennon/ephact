#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
checker="$repository_root/scripts/check_rust_file_conventions.sh"
fixture_directory="$(mktemp -d)"
trap 'rm -rf "$fixture_directory"' EXIT

if [[ ! -x "$checker" ]]; then
    printf 'Rust convention checker is missing or not executable\n' >&2
    exit 1
fi

mkdir -p "$fixture_directory/src"
printf 'pub struct ExampleRecord { value: String }\n' \
    > "$fixture_directory/src/example_record.rs"
"$checker" "$fixture_directory/src/example_record.rs"

printf 'pub struct GenericRecord<T> { value: T }\n' \
    > "$fixture_directory/src/generic_record.rs"
"$checker" "$fixture_directory/src/generic_record.rs"

printf 'pub struct ExampleRecord { value: String }\n' \
    > "$fixture_directory/src/wrong_name.rs"

if "$checker" "$fixture_directory/src/wrong_name.rs"; then
    printf 'checker accepted a mismatched Rust filename\n' >&2
    exit 1
fi

printf 'pub struct ModuleExport;\n' > "$fixture_directory/src/mod.rs"

if "$checker" "$fixture_directory/src/mod.rs"; then
    printf 'checker accepted implementation in mod.rs\n' >&2
    exit 1
fi

printf 'pub mod child;\npub use child::Child;\n' > "$fixture_directory/src/mod.rs"
"$checker" "$fixture_directory/src/mod.rs"

printf 'pub use crate::ExampleRecord;\n' > "$fixture_directory/src/mod.rs"
"$checker" "$fixture_directory/src/mod.rs"
printf 'Rust convention checker rejected invalid fixtures\n'
