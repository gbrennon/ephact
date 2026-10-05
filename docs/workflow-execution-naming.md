# Workflow Execution Planning

This document describes the current domain behavior that orders workflow jobs
by their declared dependencies.

## Current behavior

`Workflow::plan` consumes an already parsed workflow and:

- reads each job's `needs` dependencies;
- rejects missing job dependencies;
- rejects cyclic dependencies;
- orders jobs according to their dependencies; and
- groups dependency-ready jobs into execution stages.

The transformation is:

```text
Workflow definition -> validated dependency-ordered execution plan
```

Stages describe dependency levels. The execution service currently processes
the runs in those stages sequentially and skips jobs blocked by failed
dependencies.

## Boundary

YAML parsing remains an infrastructure concern. Dependency planning belongs to
the `Workflow` aggregate because it operates only on vendor-neutral workflow
jobs and their declared relationships.

The application execution flow invokes `Workflow::plan`. The execution service
consumes the resulting `ExecutionPlan` and does not need to parse YAML or
reimplement dependency resolution.
