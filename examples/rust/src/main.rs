use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut has_json = false;
    let mut seed: i64 = 0;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {
                has_json = true;
                i += 1;
            }
            "--stable" => {
                // no-op; output already deterministic
                i += 1;
            }
            "--seed" => {
                let Some(v) = args.get(i + 1) else {
                    eprintln!("main.rs: --seed requires a value");
                    return ExitCode::from(2);
                };
                seed = v.parse::<i64>().unwrap_or(0);
                i += 2;
            }
            other => {
                eprintln!("main.rs: unknown arg: {other}");
                return ExitCode::from(2);
            }
        }
    }

    if !has_json {
        eprintln!("main.rs: missing --json");
        return ExitCode::from(2);
    }

    eprintln!("main.rs: seed={seed}");

    // Keep output deterministic: fixed field order, no maps.
    println!(
        "{{\n  \"data\": {{\n    \"hello\": \"world\"\n  }},\n  \"ok\": true,\n  \"seed\": {seed},\n  \"version\": 1\n}}"
    );

    ExitCode::SUCCESS
}
