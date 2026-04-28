//! AJENTIC CLI entry point.

mod commands;

fn main() {
    let exit_code = run(std::env::args().collect());
    std::process::exit(exit_code);
}

fn run(args: Vec<String>) -> i32 {
    match parse_args(&args) {
        Ok(("validate", run_dir_arg)) => match commands::validate::run(run_dir_arg) {
            Ok(()) => 0,
            Err(message) => {
                eprintln!("Validation failed: {message}");
                1
            }
        },
        Ok(("inspect", run_dir_arg)) => match commands::inspect::run(run_dir_arg) {
            Ok(()) => 0,
            Err(message) => {
                eprintln!("Inspect failed: {message}");
                1
            }
        },
        Err(message) => {
            eprintln!("{message}");
            eprintln!("{}", usage());
            1
        }
        Ok((_, _)) => {
            eprintln!("unknown command");
            eprintln!("{}", usage());
            1
        }
    }
}

fn parse_args(args: &[String]) -> Result<(&str, &str), String> {
    if args.len() < 2 {
        return Err("missing command".to_string());
    }

    let command = args[1].as_str();
    if command != "validate" && command != "inspect" {
        return Err(format!("unknown command: {command}"));
    }

    if args.len() < 3 {
        return Err("missing run directory path".to_string());
    }

    if args.len() > 3 {
        return Err("unexpected extra arguments".to_string());
    }

    Ok((command, args[2].as_str()))
}

fn usage() -> &'static str {
    "Usage: ajentic <validate|inspect> <run-dir>"
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn no_args_returns_non_zero() {
        assert_ne!(run(vec!["ajentic".to_string()]), 0);
    }

    #[test]
    fn unknown_command_returns_non_zero() {
        assert_ne!(
            run(vec![
                "ajentic".to_string(),
                "unknown".to_string(),
                concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/minimal_run").to_string(),
            ]),
            0
        );
    }

    #[test]
    fn missing_run_directory_returns_non_zero() {
        assert_ne!(run(vec!["ajentic".to_string(), "validate".to_string()]), 0);
    }

    #[test]
    fn extra_argument_returns_non_zero() {
        assert_ne!(
            run(vec![
                "ajentic".to_string(),
                "validate".to_string(),
                concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/minimal_run").to_string(),
                "extra".to_string(),
            ]),
            0
        );
    }

    #[test]
    fn inspect_valid_run_directory_returns_zero() {
        assert_eq!(
            run(vec![
                "ajentic".to_string(),
                "inspect".to_string(),
                concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/minimal_run").to_string(),
            ]),
            0
        );
    }
}
