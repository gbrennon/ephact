# E2E Test Protection

The files under `tests/e2e/` are protected verification scenarios for the parent crate.

## Agent authorization

- Agents may inspect and run these tests without additional authorization.
- Agents must not create, modify, delete, rename, or weaken files under `tests/e2e/` autonomously.
- A human must explicitly request the E2E test change in the current task
  before an agent edits it.
- When a human has explicitly requested the change, preserve the existing
  scenarios and run the complete E2E test target before reporting completion.

The pre-push hook enforces this policy. An explicitly authorized human-driven change must set:

```text
EPHACT_HUMAN_AUTHORIZED_E2E_CHANGES=1
```

Do not use that override without the human request.
