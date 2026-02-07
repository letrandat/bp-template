# bp (Backpressure) Template

Portable contract: one CLI, two drivers (agent + human), same checks.

LLM quickstart (one line): copy `{AGENTS.md,bp,bp-menu,bp.json,contracts}` into repo root; `chmod +x bp bp-menu`; then follow `PROMPT.md`.

## What You Copy Into A Repo

- `AGENTS.md` (agent contract)
- `bp` (engine)
- `bp-menu` (interactive driver; uses `gum`)
- `bp.json` (tasks + stages)
- `contracts/` (jq asserts + goldens)

## Smoke Test (This Template)

From repo root:

```bash
./bp cli --config examples/node/bp.example.json
./bp cli --config examples/python/bp.example.json
./bp cli --config examples/go/bp.example.json
```

## Hard Rules

- Stages locked: `check|test|cli|e2e`
- Contract checks: JSON on `stdout`, logs on `stderr`
- Deterministic flags for CLIs under test (`--json`, `--stable`, `--seed 0`, etc)
- Exit codes = truth; no “looks good” without `./bp check`

## Dependencies

- required: `bash`, `jq`
- optional (menu): `gum`
- optional (timeouts): `timeout` (linux) or `gtimeout` (mac via coreutils)

## CI

Workflow snippet: `ci/github-actions.yml`

## `bp.json` Contract (Minimal)

- `stages.<stage>` = ordered array of task ids
- `tasks.<id>.cmd` = argv array (no shell-quoting games)
- `tasks.<id>.expects` (optional):
  - `stdout: "json"`
  - `jq: "contracts/<name>.jq"`
  - `golden: "contracts/goldens/<name>.json"`
