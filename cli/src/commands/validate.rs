use std::path::Path;

use ajentic_core::validation::static_run::validate_static_run_dir;

pub fn run(run_dir_arg: &str) -> Result<(), String> {
    validate_static_run_dir(Path::new(run_dir_arg)).map_err(|error| error.to_string())?;

    println!("static validation passed for run directory: {run_dir_arg}");
    Ok(())
}
