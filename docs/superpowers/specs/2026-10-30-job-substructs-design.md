# Job sub-struct decomposition

## Context

Issue #117 asks to split the domain `Job` entity into smaller logically grouped
sub-structs while preserving behavior. `Job` currently owns sixteen fields that
cover identity, dependency planning, execution policy, runtime resources, and
workflow-call data. Its public constructor, fluent builders, accessors, and
behavior methods are used throughout the domain, application, infrastructure,
and tests.

## Chosen design

Keep `Job` as the only public domain entity and compose it from four private
entity submodules:

```rust
pub struct Job {
    identity: JobIdentity,
    policy: JobExecutionPolicy,
    runtime: JobRuntime,
    contract: JobContract,
}
```

The sub-structs remain private implementation details. No new grouped getters
or public domain types are introduced.

### `JobIdentity`

Owns the job display name and declared dependency identifiers:

- `name: Option<String>`
- `needs: Vec<String>`

The workflow map remains the source of the canonical job ID; this refactor does
not add an ID to `Job`.

### `JobExecutionPolicy`

Owns scheduling, failure, security, and concurrency policy:

- `r#if: Option<String>`
- `strategy: Option<JobStrategy>`
- `timeout_minutes: Option<f64>`
- `continue_on_error: Option<String>`
- `permissions: Option<TokenPermissions>`
- `concurrency: Option<ConcurrencyGroup>`

### `JobRuntime`

Owns the executable workload and runtime environment:

- `runs_on: Option<String>`
- `steps: Vec<Step>`
- `env: HashMap<String, String>`
- `container: Option<ContainerSpecification>`
- `services: HashMap<String, ContainerSpecification>`

### `JobContract`

Owns data crossing the job boundary:

- `outputs: HashMap<String, String>`
- `with: Option<ContextValue>`
- `secrets: Option<ContextValue>`

The internal field remains `with` so the domain terminology and compatibility
accessor remain unchanged.

## Compatibility and migration

The existing public `Job` API remains unchanged:

- `Job::new(name, runs_on, steps, needs)` keeps its signature and defaults.
- Every `with_*` builder keeps its name, argument type, consuming return type,
  replacement semantics, and chaining behavior.
- Every accessor keeps its name and return type, including the `r#if()` and
  `if_condition()` aliases.
- `depends_on` and `continues_after_failure` remain on `Job` and delegate to
  their owning groups.
- `Job` retains `Debug`, `Clone`, `PartialEq`, and `Default`.

`JobYaml` remains a flat infrastructure representation of authored YAML and
continues constructing `Job` through its existing constructor and builders.
Woodpecker conversion and all other callers continue using the `Job` façade;
no adapter is coupled to the private groups.

## Invariants

The refactor must not change these behaviors:

1. `Job::new` initializes optional policy and contract values to `None` and
   collection fields to empty collections.
2. Calling a builder twice replaces the previous value; maps are not merged.
3. `r#if()` and `if_condition()` return the same condition.
4. `continues_after_failure()` returns true only for a case-insensitive
   `"true"` value in `continue_on_error`.
5. `Workflow::plan()` observes the same dependency list through `needs()`.
6. No YAML schema, execution ordering, failure handling, or runtime behavior
   changes.

## Verification

The existing `Job` tests remain the primary behavior contract and will be
extended only where needed to exercise the grouped delegation. Focused domain,
YAML conversion, Woodpecker conversion, workflow planning, and application/
infrastructure tests will run before the canonical workspace gates.
