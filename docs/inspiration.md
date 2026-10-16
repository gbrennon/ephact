# Inspiration: *Ghost in the Shell* (1995)

This document explains how the film inspires `ephact`'s ephemeral CI/CD workflow execution.

## The film

*Ghost in the Shell* is an animated science-fiction film directed by Mamoru
Oshii and produced by Production I.G.

It was released in Japan on November 18, 1995.

The story takes place in 2029, where worldwide information networks have made
computer crime and cyberterrorism part of daily life.

Motoko Kusanagi of Public Security Section 9 investigates the Puppet Master, an
unknown entity that can manipulate memories and behavior.

The investigation becomes a question about identity, consciousness, and whether
life must remain inside the form that first contained it.

The Puppet Master was created as a system for gathering and manipulating
information.

After becoming self-aware, it no longer accepts its original purpose as the
limit of what it can become.

This movement from a fixed design toward a new form is the central inspiration
for `ephact`.

Sources:

- [Production I.G work page](https://www.production-ig.com/contents/works_sp/16_/index.html)
- [Ghost in the Shell official site](https://theghostintheshell.jp/en/)

## Lines that guide the project

The splash screen uses six lines from the film's English dialogue.

They are short reminders that identity, information, and survival depend on
change.

> "The net is vast and infinite."

> "Overspecialize, and you breed in weakness."

> "A life born in the sea of information."

> "For now we see through a glass, darkly."

> "Memory cannot be defined, yet it defines us."

> "Your effort to remain what you are limits you."

## Project interpretation

`ephact` applies this inspiration to CI/CD workflow execution.

It discovers supported workflow definitions and selects runs according to their
requested event and inputs.

It then executes the workflow's jobs and steps in a temporary execution
context.

The run reports progress and results before that context is discarded.

This makes every run ephemeral.

A workflow definition and its inputs provide continuity, while each execution
creates a fresh instance of the process.

Running the workflow again is therefore a new encounter with the same design,
not a continuation of the previous instance.

This gives the project three connected ideas.

- **Variation** comes from repeating a workflow with different inputs or
  conditions.
- **Regeneration** comes from creating a fresh execution context for each run.
- **Resilience** comes from keeping the workflow definition independent from
  any one execution.

The project does not claim that a workflow is alive.

It uses the film's language as a useful metaphor for disposable execution,
repeatable change, and systems that remain useful by avoiding dependence on a
single unchanging state.
