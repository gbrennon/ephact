#!/usr/bin/env bash
set -euo pipefail

expected_sha512_for_target() {
    case "$1" in
        x86_64)
            printf '%s\n' '26d1f7c8ea2400f2ccb0b2e7f321635f600b583bf4cd1f7afaa5e8c24068ad1088a2504e0a4d9f4333d4bb20a80329327d9462634343ee3af91e1a7fbdc6f18a'
            ;;
        aarch64)
            printf '%s\n' 'fd27768ea76fea8c3ba3f3247fef2f6604988b629f15e48bf40be5eaa08b8fef2e4087cc9c8af16bc522bc19cc369611fa7692101b513f63469ad58522708600'
            ;;
        *)
            printf 'unsupported architecture: %s\n' "$1" >&2
            return 1
            ;;
    esac
}

verify_archive_checksum() {
    local archive_path="$1"
    local expected_checksum="$2"

    if ! printf '%s  %s\n' "$expected_checksum" "$archive_path" \
        | sha512sum --check --status -; then
        printf 'checksum verification failed for %s\n' "$archive_path" >&2
        return 1
    fi
}

install_git_cliff() {
    local version='2.14.2'
    local architecture
    local target
    local archive_name
    local expected_checksum
    local install_dir="${GIT_CLIFF_INSTALL_DIR:-$HOME/.cargo/bin}"
    local temporary_directory
    local archive_path

    architecture="$(uname -m)"
    case "$architecture" in
        x86_64) target='x86_64-unknown-linux-gnu' ;;
        aarch64) target='aarch64-unknown-linux-gnu' ;;
        *)
            printf 'unsupported architecture: %s\n' "$architecture" >&2
            return 1
            ;;
    esac

    archive_name="git-cliff-${version}-${target}.tar.gz"
    expected_checksum="$(expected_sha512_for_target "$architecture")"
    temporary_directory="$(mktemp -d)"
    trap 'rm -rf -- "$temporary_directory"' RETURN
    archive_path="$temporary_directory/$archive_name"

    curl -fsSL --proto '=https' --tlsv1.2 \
        -o "$archive_path" \
        "https://github.com/orhun/git-cliff/releases/download/v${version}/${archive_name}"
    verify_archive_checksum "$archive_path" "$expected_checksum"

    tar -xzf "$archive_path" -C "$temporary_directory"
    mkdir -p "$install_dir"
    install -m 0755 \
        "$temporary_directory/git-cliff-${version}/git-cliff" \
        "$install_dir/git-cliff"
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
    install_git_cliff "$@"
fi
