# AGENTS.md (Template)

Backpressure contract.

- Do not claim success without `./bp check` passing.
- Do not invent commands. Use `./bp check|test|cli|e2e`.
- Prefer deterministic, machine-verifiable outputs (JSON + jq + goldens).

Commands.

- `./bp check`: fast gate (docs/fmt/lint/type/unit)
- `./bp test`: unit only
- `./bp cli`: contract checks (CLI surface area; JSON asserts)
- `./bp e2e`: slowest checks

Output contract (for CLIs under test).

- JSON to `stdout`
- logs to `stderr`
- deterministic flags: `--json --stable --seed 0` (or equivalent)

How to add backpressure.

- Add a task in `bp.json`
- Add `contracts/*.jq` asserts
- Add/update `contracts/goldens/*.json`
- Wire into stage arrays (`stages.check|test|cli|e2e`)
