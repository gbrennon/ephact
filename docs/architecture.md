# Architecture

`ephact` is organized as a hexagon with four layers under `src/`:

| Layer | Responsibility |
|---|---|
| `domain` | Entities, value objects, workflow and expression models, planner, domain events, and errors |
| `application` | Inbound and outbound ports, DTOs, one service per use case, and application commands (`ExecuteWorkflowCommand`, `ExecuteJobCommand`, `ExecuteStepCommand`, `ExecuteActionCommand`) dispatched via `CommandBusPort` |
| `infrastructure` | Adapters: action fetching, container runtime (Bollard), image handling, runners, workflow loading, events, and dependency injection container |
| `presentation` | CLI entrypoint, subcommands (`run`, `list-workflows`, `list-actions`), composition root wiring, and terminal UI components (`Banner`, `BoxComponent`, `RunSummary`) |

The dependency rule points inwards: `domain` depends on nothing, `application` depends only on `domain`, and outer layers implement the ports declared by inner layers. All interactions with the outside world - fetching actions, creating containers, reading and writing files - go through injectable ports, which keeps the default configuration side-effect-free.

### Test Suites

The test suite mirrors the layers and integration concerns:

- `tests/application`: Unit tests for application services, command handling, and DTO mappings using domain fakes.
- `tests/infrastructure`: Unit and contract tests for infrastructure adapters (workflow parsers, image mappers, event bus, runners).
- `tests/presentation`: CLI argument parsing, subcommand handlers, and terminal UI component formatting.
- `tests/container_integration`: Real Docker and Podman integration tests via Bollard, gated behind the `container-integration` Cargo feature.
- `tests/e2e`: End-to-end execution testing full workflow runs with ephemeral repositories.
- `tests/common`: Shared test fakes, in-memory adapters, stubs, and test fixtures used across all test suites.

## Application Services Orchestration & Coordination

Application services coordinate decisions made by the domain, but **never depend directly on one another or on inbound ports**. All level-to-level coordination flows strictly through commands published via `CommandBusPort`. Infrastructure command handlers receive these commands and invoke the corresponding coordination service through its inbound port (`workflow -> job -> step -> action`).

```text
                        ┌────────────────────────┐
                        │ Presentation: CLI / Run│
                        └───────────┬────────────┘
                                    │
                    ┌───────────────┴───────────────┐
                    │ --all-workflows               │ single workflow
                    ▼                               ▼
        ┌───────────────────────┐       ┌───────────────────────┐
        │ RunAllWorkflowsService│       │   RunWorkflowService  │
        └───────────┬───────────┘       └───────────┬───────────┘
                    │ (1) dispatch_workflow         │ (1) dispatch_workflow
                    └───────────────┬───────────────┘
                                    ▼
╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║                        SINGLE SHARED COMMAND BUS                          ║
║                             (CommandBusPort)                              ║
║                                                                           ║
║  (1) ExecuteWorkflowCommand                                               ║
║       │                                                                   ║
║       ▼                                                                   ║
║      WorkflowCommandHandler ──► ExecuteWorkflowService                    ║
║                                        │                                  ║
║  (2) ExecuteJobCommand                 │ (dispatches)                     ║
║       │ ◄──────────────────────────────┘                                  ║
║       ▼                                                                   ║
║      JobCommandHandler      ──► ExecuteJobService                         ║
║                                        │                                  ║
║  (3) ExecuteStepCommand                │ (dispatches)                     ║
║       │ ◄──────────────────────────────┘                                  ║
║       ▼                                                                   ║
║      StepCommandHandler     ──► ExecuteStepService                        ║
║                                        │                                  ║
║  (4) ExecuteActionCommand              │ (dispatches)                     ║
║       │ ◄──────────────────────────────┘                                  ║
║       ▼                                                                   ║
║      ActionCommandHandler   ──► ExecuteActionService (Node / Composite)  ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

### Coordination Flow Summary

1. **Entrypoints**: The CLI invokes `RunWorkflowPort` (for a single workflow), `RunAllWorkflowsPort` (for all workflows in repository), `ListWorkflowsPort` (to list discovered workflows), or `ListActionsPort` (to list referenced actions). The execution services load workflow YAML from `WorkflowSourcePort` and dispatch `ExecuteWorkflowCommand` over `CommandBusPort`.
2. **Workflow Execution**: `WorkflowCommandHandler` parses the run configuration and context, calling `ExecuteWorkflowService` via `ExecuteWorkflowPort`. The service plans stage runs using the domain `Planner` and dispatches `ExecuteJobCommand` for each job.
3. **Job Execution**: `JobCommandHandler` invokes `ExecuteJobService` via `ExecuteJobPort`. The service sets up the job container via `PrepareJobContainerPort`, prepares environment variables, and dispatches `ExecuteStepCommand` for each step.
4. **Step Execution**: `StepCommandHandler` delegates to `ExecuteStepService` via `ExecuteStepPort`. Steps are evaluated and interpolated with `StepInterpolator`. If the step executes a shell command (`run:`), it runs via `RunShellStepPort`. If the step references an action (`uses:`), it dispatches `ExecuteActionCommand`.
5. **Action Execution**: `ActionCommandHandler` delegates to `ExecuteActionService` via `ExecuteActionPort`, resolving inputs, fetching actions, and executing either composite steps (which may recursively dispatch nested action commands) or Node.js actions within the job container.
6. **Cleanup**: Upon workflow completion, both `RunWorkflowService` and `RunAllWorkflowsService` publish `DomainEvent::ActRunCompleted` over `EventBusPort`, triggering `ContainerCleanupHandler` to stop and clean up containers.
