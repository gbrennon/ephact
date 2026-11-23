#!/usr/bin/env bash
set -euo pipefail

if [[ "${EPHACT_HUMAN_AUTHORIZED_E2E_CHANGES:-}" == "1" ]]; then
    exit 0
fi

push_updates="$(timeout 1s cat || true)"
if [[ -z "$push_updates" ]]; then
    printf 'no pre-push ref updates supplied; nothing to check\n'
    exit 0
fi

changed_files=()
while read -r local_ref local_sha _remote_ref remote_sha; do
    [[ -z "${local_sha:-}" ]] && continue
    [[ "$local_sha" =~ ^0+$ ]] && continue

    if [[ "${remote_sha:-}" =~ ^0+$ || -z "${remote_sha:-}" ]]; then
        base_revision="$(git rev-list --max-parents=0 "$local_sha" | tail -n 1)"
    else
        base_revision="$remote_sha"
    fi

    while IFS= read -r changed_file; do
        [[ -n "$changed_file" ]] && changed_files+=("$changed_file")
    done < <(git diff --name-only --diff-filter=ACMRD "$base_revision" "$local_sha" -- tests/e2e/)
done <<< "$push_updates"

if ((${#changed_files[@]} == 0)); then
    exit 0
fi

printf 'push blocked: protected E2E tests were changed:\n' >&2
printf '  %s\n' "${changed_files[@]}" | sort -u >&2
printf '\nA human must explicitly authorize this change.\n' >&2
printf 'Re-run the push with EPHACT_HUMAN_AUTHORIZED_E2E_CHANGES=1 after that request.\n' >&2
exit 1
