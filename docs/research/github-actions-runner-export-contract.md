# GitHub Actions runner export contract

This note records the official GitHub documentation used for the composite-step export implementation. The application-facing model stays provider-neutral; GitHub-specific environment-file names and parsing belong to infrastructure adapters.

## Official rules

- GitHub Actions uses runner-created environment files for step-to-step communication. `GITHUB_ENV` makes variables available to later steps, `GITHUB_PATH` adds directories to `PATH` for later steps, and `GITHUB_OUTPUT` exposes step outputs to later steps. The step that writes `GITHUB_ENV` does not see its own new value.
  - Source: [Workflow commands for GitHub Actions](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-commands)
- Environment files support multiline values using `NAME<<DELIMITER`, followed by the value and the delimiter on its own line. GitHub warns that the delimiter must not occur alone in the value; for arbitrary content, a file is safer.
  - Source: [Workflow commands for GitHub Actions](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-commands#multiline-strings)
- Composite action steps may write outputs to `GITHUB_OUTPUT`. A later step references them as `steps.<step-id>.outputs.<output-name>`. Composite action outputs map their public values to a step output expression.
  - Source: [Metadata syntax reference](https://docs.github.com/en/actions/reference/workflows-and-actions/metadata-syntax#outputs-for-composite-actions)
- `github.action_path` is the directory containing the currently running composite action. It is available only in composite actions and is also exposed as `GITHUB_ACTION_PATH`; action files/scripts are resolved from that directory.
  - Source: [Contexts reference](https://docs.github.com/en/actions/reference/workflows-and-actions/contexts#github-context)
  - Source: [Creating a composite action](https://docs.github.com/en/actions/tutorials/create-actions/create-a-composite-action)
- `GITHUB_ACTION_PATH` is a default environment variable containing the filesystem path where the currently running composite action is located.
  - Source: [Variables reference](https://docs.github.com/en/actions/reference/workflows-and-actions/variables)
- Composite `run` steps require a shell and may use either `${{ github.action_path }}` or `$GITHUB_ACTION_PATH` to address files shipped with the action.
  - Source: [Metadata syntax reference](https://docs.github.com/en/actions/reference/workflows-and-actions/metadata-syntax#runs-for-composite-actions)

## Design boundary

The application service should consume a neutral `StepExportsResponse`/outbound reader contract containing path additions, environment values, and step outputs. It should not know `GITHUB_ENV`, `GITHUB_PATH`, `GITHUB_OUTPUT`, `cat`, or provider-specific file names.

The infrastructure adapter owns the GitHub runner contract:

1. map runner file locations into the adapter;
2. read the files after a successful step;
3. parse single-line and documented multiline records;
4. return neutral path/environment/output data;
5. let the composite orchestrator merge that data into the next-step environment and `steps` context.

`github.action_path` is likewise computed at the infrastructure boundary from the container-side copied action directory and passed into the neutral evaluation context as the provider context required by the workflow expression model.
