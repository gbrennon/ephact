# Workflow Execution Naming

This document explains the naming options for the domain service that converts workflow job dependencies into executable stages.

## What the service does

The service does not parse YAML and does not decide what a workflow should do. It consumes an already parsed workflow and:

- Reads each job's `needs` dependencies.
- Rejects missing job dependencies.
- Rejects cyclic dependencies.
- Orders jobs according to their dependencies.
- Groups independent jobs into stages that may run in parallel.

The transformation is:

```text
Workflow definition -> validated executable job stages
```

## Why "topology" was proposed

In graph theory, topology describes the structural relationships between nodes and edges. A workflow's jobs are nodes, and `needs` relationships are directed edges.

For example:

```text
prepare -> build -> test
             └──> lint
```

The service derives the topology's executable layers:

```text
Stage 1: prepare
Stage 2: build
Stage 3: test, lint
```

"Topology" emphasizes that the service is resolving graph structure rather than creating a business plan, choosing a deployment strategy, or scheduling time-based events.

## Why "topology" may still be wrong

"Topology" is technically accurate but may be too abstract for the workflow domain. It is not a term users see in workflow files, and it may force maintainers to understand graph terminology before understanding the service.

The domain vocabulary should prefer the simplest name that clearly describes the business behavior.

## Naming options

### `WorkflowExecutionTopologyBuilder`

Advantages:

- Precise about dependency structure.
- Distinguishes job dependencies from trigger schedules.
- Describes the transformation as a build operation.

Disadvantages:

- Technical and graph-oriented.
- Less familiar to workflow users.

### `WorkflowExecutionGraphBuilder`

Advantages:

- More familiar than topology.
- Clearly communicates dependency relationships.

Disadvantages:

- The service does not return only a graph; it returns ordered parallel stages.
- Still exposes implementation terminology.

### `WorkflowExecutionStagesBuilder`

Advantages:

- Describes the actual output.
- Makes parallel execution behavior obvious.
- Avoids graph terminology.

Disadvantages:

- Does not communicate that dependency validation also occurs.

### `WorkflowDependencyResolver`

Advantages:

- Describes the main input relationship: job dependencies.
- Communicates validation and dependency ordering.

Disadvantages:

- "Resolver" can imply name or reference lookup.
- Does not clearly communicate stage construction.

### `WorkflowExecutionScheduleBuilder`

Advantages:

- Describes the ordered execution result.
- Familiar scheduling vocabulary.

Disadvantages:

- Can be confused with the `Schedule` trigger kind.
- May imply wall-clock scheduling rather than dependency ordering.

## Recommendation

Use `WorkflowExecutionStagesBuilder` if the domain should prioritize clear workflow vocabulary.

Use `WorkflowExecutionTopologyBuilder` only if graph terminology is already established in the project or if dependency topology is an important domain concept elsewhere.

The preferred public API would be:

```rust
pub struct WorkflowExecutionStagesBuilder;

impl WorkflowExecutionStagesBuilder {
    pub fn build(
        &self,
        workflow: &Workflow,
    ) -> Result<ExecutionStages, WorkflowDependencyError>;
}
```

The implementation can still use topological sorting internally. Internal algorithm terminology does not need to determine the public domain name.

## Boundary decision

YAML parsing remains an infrastructure concern. The stages builder belongs in the domain because it operates only on vendor-neutral workflow jobs and dependency relationships.

The application service invokes the builder before execution. The executor consumes the resulting stages and does not need to understand YAML or dependency resolution.
