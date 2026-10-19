#!/usr/bin/env bash
set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture_directory="$(mktemp -d)"
trap 'rm -rf -- "$fixture_directory"' EXIT

mkdir -p "$fixture_directory"
printf 'fn parse(&mut self) {}\n' > "$fixture_directory/parser.rs"
just --justfile "$repository_root/justfile" immutable-domain-receivers "$fixture_directory"

printf 'pub fn with_value(mut self) {}\n' > "$fixture_directory/builder.rs"
if just --justfile "$repository_root/justfile" immutable-domain-receivers \
    "$fixture_directory"; then
    printf 'immutable receiver check accepted a mutable consuming receiver\n' >&2
    exit 1
fi

printf '%s\n' 'immutable receiver enforcement passed'
