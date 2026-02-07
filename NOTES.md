# Notes

Patterns to steal (raw-to-jpeg).

- docs discipline + `docs:list` gate
- one-shot validation script (docs + unit + integration)
- integration harness writes artifacts + a single “open me” viewer page
- interactive picker is just a driver (no second system)
- goldens + explicit regen script
- outer timeout guard in prod entrypoint

Anti-patterns to avoid (raw-to-jpeg debt).

- absolute paths in tests/scripts (non-portable)
- mixing logs + machine output on stdout (breaks piping/agents)
- no timeouts on long subprocess calls (hang risk)
- hard-coded `.venv/bin/python` without fallback
