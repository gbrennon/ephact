# ephact-domain

This crate contains the core domain model for the ephact application.

## Features

- Aggregates: Workflow, Settings
- Entities: Job, Step, Repository, etc.
- Value objects: FileEntry, ProjectBranding, etc.
- Domain errors: CoreError, StepError, etc.
- Commands: ExecuteActionCommand, ExecuteJobCommand, etc.

## Purpose

The domain layer is the heart of the application, containing the pure business logic and domain concepts.

It is independent of any external infrastructure or framework, making it testable in isolation.

## Usage

Import the domain types and commands as needed:

```rust
use ephact_domain::{
    Workflow, Job, Step, ExecuteActionCommand, ExecuteJobCommand, ExecuteStepCommand
};
```
