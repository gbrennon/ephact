# ephact-infrastructure

This crate contains the infrastructure implementation for the ephact application.

## Features

- Container runtime implementation
- File system access
- Network communication
- Database access (if needed)
- External service integrations

## Purpose

The infrastructure layer implements the ports defined in the application layer, providing concrete implementations for external dependencies.

This layer is responsible for interacting with the outside world and should not contain any business logic.

## Usage

Import the infrastructure components as needed:

```rust
use ephact_infrastructure::{
    ContainerRuntime, Filesystem, NetworkClient
};
```
