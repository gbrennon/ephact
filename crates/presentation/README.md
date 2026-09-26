# ephact-presentation

This crate contains the presentation layer for the ephact application.

## Features

- Command-line interface (CLI)
- Web interface (if needed)
- User interface components
- Input validation and formatting
- Output rendering

## Purpose

The presentation layer handles user interaction and presentation concerns.

It should not contain any business logic or domain concepts, only UI/UX related code.
It depends on the application layer for business logic and the domain layer for data models.

## Usage

Import the presentation components as needed:

```rust
use ephact_presentation::{
    Cli, WebInterface, UserInterface
};
```
