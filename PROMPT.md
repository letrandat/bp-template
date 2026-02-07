# Prompt: Adapt `bp` To A Repo

Goal: create automated backpressure that both agents and humans run.

1. Copy `AGENTS.md`, `bp`, `bp-menu`, `bp.json`, `contracts/` into repo root.
2. Edit `bp.json`:
   - lock stages: `check|test|cli|e2e`
   - map tasks to existing repo commands (no new toolchain unless needed)
3. For each feature CLI:
   - add `--json` mode (deterministic)
   - JSON on stdout, logs on stderr
   - add `contracts/<feature>.jq`
   - add `contracts/goldens/<feature>.json` (stable ordering via `jq -S .`)
4. Make `./bp check` fast. Keep `./bp e2e` slow and optional.
5. CI calls the same commands: `./bp check` then `./bp cli`.

If anything is unclear: ask only about the repo’s real commands (fmt/lint/type/test/e2e).
