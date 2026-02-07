#!/usr/bin/env node

function main() {
  const args = process.argv.slice(2);

  const has = (flag) => args.includes(flag);
  const argValue = (flag, def) => {
    const i = args.indexOf(flag);
    if (i === -1) return def;
    return args[i + 1] ?? def;
  };

  if (!has("--json")) {
    console.error("example_cli.js: missing --json");
    process.exit(2);
  }

  const seed = Number(argValue("--seed", "0"));
  console.error(`example_cli.js: seed=${seed}`);

  const out = {
    ok: true,
    version: 1,
    seed,
    data: { hello: "world" },
  };

  process.stdout.write(JSON.stringify(out, null, 2) + "\n");
}

main();
