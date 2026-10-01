# Domain responsibilities and purity boundary

## Decision

The domain crate is the innermost policy boundary. It MUST NOT perform filesystem I/O,
resolve paths, inspect repository markers, canonicalize paths, invoke a client, or know
which forge or source-control tool supplies an input.

A domain service is not a generic home for code that does not fit elsewhere. It is a
stateless domain operation whose inputs and outputs are domain concepts and whose rule
cannot naturally belong to one entity or value object.

For this workflow runner, the domain crate now contains aggregates, entities, value
objects, domain errors, and domain messages. It contains no domain-service module. The
workflow-expression evaluator has moved to the application layer because expression
interpolation is execution orchestration, not a canonical business concept of the
workflow aggregate.

## Findings

### `Workflow::plan`

Job dependency planning is behavior of one `Workflow` aggregate. It reads only that
aggregate's jobs, detects missing dependencies/cycles, and produces execution stages.
It lives on `Workflow::plan`; a separate `ExecutionPlanner` type was an anemic aggregate
symptom and an unnecessary domain-service dependency.

### Workflow-expression evaluation

`StepInterpolator` and its lexer/parser/evaluator now live under
`crates/application/src/services/step_interpolator`.

The application execution flow owns the point at which a symbolic workflow step becomes
a concrete execution request:

```text
workflow + runtime context
        -> application StepInterpolator
        -> concrete step/action request
        -> outbound runner port
```

This keeps the domain crate free of expression-language implementation and avoids making
workflow execution syntax a domain service. The application layer still does not own
filesystem, provider, container, or source-control behavior. It only coordinates pure
interpolation before dispatching concrete work to outbound ports.

The expression parser, evaluator, supported functions, and interpolation errors moved
with `StepInterpolator`. `EvaluationContext` and `ContextValue` remain passive data
structures used to carry runtime values; they do not perform evaluation or external I/O.

### Removed wrappers

- `WorkflowRunConfigFactory` only delegated to construction of the existing config.
- `WorkflowRunConfigInput` was not a second domain model; it was a raw request mapper.
  It now lives privately in infrastructure beside the workflow execution adapters.
- `RepositoryFactory` only assembled existing constructors and obscured the boundary.
- `EphemeralRepository` was unused in production and coupled a domain entity to
  worktree conversion vocabulary.

## Filesystem and repository boundary

`RepoPath` previously called `canonicalize`, `is_dir`, `is_file`, and inspected `.git`.
Those operations are infrastructure behavior, not value-object invariants. `RepoPath` is
now a passive boundary-provided path value with only empty-value validation.

`RepositoryResolver` in infrastructure owns:

- canonicalizing the supplied location;
- checking that it is a directory;
- checking the repository marker;
- deriving a name when the caller has not supplied one; and
- constructing the passive domain values after those checks.

Presentation and infrastructure execution paths use that resolver. Application services
only assemble domain values from request data and depend on outbound ports for external
work.

The terms `.git`, standalone checkout, and linked worktree no longer occur in the
repository domain model. `GitDirKind` and the related entity methods were deleted.

## Remote action boundary

Remote action coordinates remain domain data: source scheme/host/owner/repository,
revision, and optional action directory. Acquisition mechanics no longer live there:
`RemoteActionReference` does not expose `clone_url` or `cache_key`, and calls the neutral
field `revision` rather than `git_ref`.

The infrastructure `GitActionFetcher` now owns URL construction, cache-key policy, source
client invocation, checkout, and source-client error details. This is the correct place
for Git, Forgejo, GitHub, Codeberg, or a local mirror to be selected. The domain only
parses and preserves the action coordinate supplied by the workflow language.

A remaining migration is `ExecuteWorkflowCommand`, which still carries the repository
carrier used by the current execution message flow. Removing that carrier from the
innermost messages requires a separate application-message migration; it should not be
replaced with another path or source-control abstraction in the domain.

## Dead-code audit

`TempDirTemplate` was removed because repository-wide reference search found only its
definition and public re-exports; no production or test caller constructed it.

`ContainerCredentials` is not dead: YAML infrastructure maps registry credentials into
`ContainerSpecification`, and the domain specification retains them. However, the
current runtime path does not yet consume those credentials when starting a container.
That is an incomplete container-runtime capability, not an unused type. It should either
be wired into the container adapter when private-image support is implemented or removed
as a separate product decision; it should not be silently mistaken for a duplicate model.

The compiler dead-code lint (`cargo rustc -p ephact-domain --lib -- -D dead_code`) passes.
Public domain types still require repository-wide reference review because Rust does not
report unused public API as dead code.

## Enforcement

`.semgrep/architecture.yml` enforces:

- domain cannot import infrastructure, presentation, or application;
- domain services cannot import sibling domain services; and
- application services cannot inject inbound ports or import sibling application services.

These rules enforce dependency direction, but they do not determine whether a class is a
meaningful domain service. Responsibility remains a modeling decision documented here.
