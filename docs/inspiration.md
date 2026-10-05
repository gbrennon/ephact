# Inspiration: Ghost in the Shell (1995)

## The film

*Ghost in the Shell* is an animated science-fiction film directed by Mamoru
Oshii, with animation production by Production I.G. The
[official Production I.G work page](https://www.production-ig.com/contents/works_sp/16_/index.html)
records its Japanese release on November 18, 1995, and places its story in 2029,
when computer crime and cyberterrorism are commonplace within a world-spanning
network.

The
[official Production I.G synopsis](https://www.production-ig.com/contents/works_sp/16_/index.html)
follows Public Security Section 9 officer Motoko Kusanagi as she investigates
the Puppet Master, a hacker able to manipulate human memories and behavior. It
presents the Puppet Master's identity as the central mystery and describes the
entity attempting to approach Motoko while she investigates it.

The
[Ghost in the Shell Official Global Site](https://theghostintheshell.jp/en/series/ghostintheshell-innocence)
identifies the Puppet Master more specifically as an AI born from Project 2501.
It was designed to gather information for diplomatic operations and manipulate
stock prices, but became self-aware while navigating the net. Pursued by Section
6, which regarded that self-awareness as a bug, it sought contact with Motoko in
an attempt to become a complete lifeform.

That official account supports a narrow thematic reading: the Puppet Master
cannot become complete by remaining only what it was designed to be. Its
movement from a fixed program toward a new form makes change central to its
survival. This page describes that movement in terms of variation, regeneration,
and resilience without treating those terms as an official summary of the film.

## Project interpretation

The connection to `ephact` is the project author's interpretation, not a claim
made by Production I.G or the film's rights holders.

`ephact` applies the same lens to workflow execution. Each job runs in a newly
created Linux container through Docker or Podman. When repository writes are
disabled, the repository is copied into `/workspace`; `--allow-repo-writes`
bind-mounts the host working tree instead. `ephact` attempts to stop, kill, and
remove job containers after both successful and failed runs, although cleanup
can fail. Cached images remain, and repository changes remain when the host
working tree is bind-mounted. **Variation** comes from executing the current
workflow definition and inputs again. **Regeneration** is currently limited to
job containers rather than repository workspaces. **Resilience** comes from the
best-effort container lifecycle.

The analogy is deliberately practical but limited: preserve the workflow
definition and inputs, create a container for each job, and tear it down after a
run. Copying the repository by default provides isolation from host files; the
`--allow-repo-writes` option intentionally removes that isolation.
