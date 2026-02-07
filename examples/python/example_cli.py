#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import sys


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--json", action="store_true")
    p.add_argument("--stable", action="store_true")
    p.add_argument("--seed", type=int, default=0)
    args = p.parse_args()

    if not args.json:
        print("example_cli.py: missing --json", file=sys.stderr)
        raise SystemExit(2)

    print(f"example_cli.py: seed={args.seed}", file=sys.stderr)

    out = {
        "ok": True,
        "version": 1,
        "seed": args.seed,
        "data": {"hello": "world"},
    }
    print(json.dumps(out, indent=2, sort_keys=args.stable))


if __name__ == "__main__":
    main()
