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

printf 'pub fn helper() {}\n' > "$fixture_directory/src/helper.rs"

if "$checker" "$fixture_directory/src/helper.rs"; then
    printf 'checker accepted an unbound production function\n' >&2
    exit 1
fi

cat > "$fixture_directory/src/example_record.rs" <<'EOF'
pub struct ExampleRecord {
    value: String,
}

impl ExampleRecord {
    pub fn value(&self) -> &str {
        &self.value
    }
}
EOF
"$checker" "$fixture_directory/src/example_record.rs"
mkdir -p "$fixture_directory/tests"
printf '#[test]\nfn test_helper() {}\n' > "$fixture_directory/tests/example_tests.rs"
"$checker" "$fixture_directory/tests/example_tests.rs"



printf 'pub fn main() {}\n' > "$fixture_directory/src/main.rs"
"$checker" "$fixture_directory/src/main.rs"


printf 'pub struct ModuleExport;\n' > "$fixture_directory/src/mod.rs"

if "$checker" "$fixture_directory/src/mod.rs"; then
    printf 'checker accepted implementation in mod.rs\n' >&2
    exit 1
fi

printf 'pub mod child;\npub use child::Child;\n' > "$fixture_directory/src/mod.rs"
"$checker" "$fixture_directory/src/mod.rs"

printf 'pub use crate::ExampleRecord;\n' > "$fixture_directory/src/mod.rs"
"$checker" "$fixture_directory/src/mod.rs"

cat > "$fixture_directory/src/mod.rs" <<'EOF'
mod alpha;
mod beta;

pub(super) use self::{
    alpha::Alpha,
    beta::Beta,
};
pub use self::alpha::Alpha;
EOF
"$checker" "$fixture_directory/src/mod.rs"
printf 'Rust convention checker rejected invalid fixtures\n'
