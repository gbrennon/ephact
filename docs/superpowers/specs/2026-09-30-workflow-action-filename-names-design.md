# Workflow and Action Filename Names

This design gives every supported workflow and action format a stable display name.

## Scope

The name-resolution rule applies to workflows and local action definitions from every supported
format:

1. Use the parsed `name` when it is present and non-empty.
2. Otherwise use the source filename, including its extension.

This affects workflow listing, workflow selection, workflow execution messages, and action
execution metadata. The existing `uses` reference list remains a list of references; it does not
become a list of action display names.

## Architecture

Source adapters will return file metadata together with file contents. The metadata will preserve
the source filename while keeping filesystem concerns outside the domain model's naming rule.

Workflow loading will attach the source filename to the `Workflow` aggregate. The aggregate will
expose one resolved display-name behavior that prefers its parsed name and falls back to its file.
Callers that display or report workflow names will use that behavior instead of treating an absent
YAML name as `unnamed`.

Action YAML will accept an optional parsed name. The action loader will resolve the action name from
the parsed value or the selected `action.yml`/`action.yaml` filename before constructing the domain
action definition.

The fallback logic will be shared and tested independently of Woodpecker-specific parsing. Vendor
adapters will only parse their own formats and provide source metadata.

## Data flow

1. A workflow source discovers a file and reads its contents.
2. The source returns contents plus the source filename.
3. The application loader parses the contents and attaches the filename to the workflow.
4. Workflow consumers request the resolved display name.
5. The action loader selects `action.yml` or `action.yaml`, parses its optional name, and resolves
   the name against that selected filename.

## Error handling

Malformed workflow or action YAML continues to return the existing parse errors. Missing source
filenames are not silently converted into a generic label; the existing source or loading error is
preserved. Empty or whitespace-only parsed names use the filename fallback.

## Testing

Add tests for:

- named workflows retaining their YAML name;
- unnamed and blank-name workflows using the filename;
- workflow lookup by both resolved names;
- workflow execution reports using the resolved name;
- named actions retaining their YAML name;
- unnamed and blank-name `action.yml` and `action.yaml` definitions using the selected filename;
- all existing vendor formats continuing to parse and list correctly.

Run the full Cargo test suite and repository quality checks before completion.
