# Architecture

`ephact` follows a hexagonal architecture. The design separates workflow rules
and orchestration from user interfaces and external systems so that each part
can evolve behind an explicit boundary.

## Layers

| Layer          | Responsibility                                                                                                         |
| -------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Domain         | Defines workflow concepts, rules, planning, events, values, and errors without knowledge of delivery or infrastructure |
| Application    | Implements use cases, coordinates domain behavior, and defines the inbound and outbound ports at its boundary          |
| Infrastructure | Supplies adapters for external capabilities and composes lower-level implementation concerns                           |
| Presentation   | Translates user input into application requests and renders application results                                        |

The compile-time dependency direction points inward. The domain does not depend
on other project layers. The application depends on the domain, while
infrastructure and presentation may depend on the application and domain.
Runtime control and data can cross a boundary in either direction, but only
through contracts owned by the inner layer.

## Ports and Adapters

Inbound ports describe the use cases that the application offers. Presentation
adapters call those ports without knowing how the use cases are implemented.
Outbound ports describe capabilities that application orchestration needs from
the outside world. Infrastructure adapters implement those capabilities without
exposing external-system details to the application or domain.

Infrastructure may use its own internal interfaces to separate adapter
responsibilities. Those interfaces are implementation seams, not application
boundary ports, and do not reverse the inward dependency rule.

## Orchestration

The application layer owns orchestration. It coordinates workflow, job, step,
and action execution, asks the domain to make workflow decisions, and requests
external effects through outbound ports. Infrastructure adapters prepare
infrastructure-specific run context and implement those ports without exposing
external-system details to the application or domain.

Not every part of a use case must pass through a single dispatch mechanism.
Top-level orchestration can perform discovery and configuration before handing
work to the execution stages. The architectural constraint is dependency on
stable boundaries, not one prescribed sequence of handlers or messages.

## Execution Lifecycle

Hexagonal boundaries improve separation and testability; they do not make
production execution side-effect-free. A run can acquire external resources and
change external state. Cleanup is best-effort after normally completed
orchestration, and failures can bypass or limit cleanup. The architecture does
not provide transactional rollback for changes made during a run.

See [Usage](usage.md) for the current runtime requirements, isolation model,
network behavior, repository access, and cleanup guarantees.

## Testing Strategy

Tests follow the same boundaries without making their directory layout part of
the architecture. Domain tests exercise rules in isolation. Application tests
replace outbound ports with controlled test doubles. Infrastructure tests
exercise adapters and their external contracts, while end-to-end tests validate
the behavior of an assembled application.
