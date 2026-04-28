use std::path::Path;

use ajentic_core::validation::static_run::validate_static_run_dir;

pub fn run(run_dir_arg: &str) -> Result<(), String> {
    let inspection =
        validate_static_run_dir(Path::new(run_dir_arg)).map_err(|error| error.to_string())?;

    println!("run directory: {}", inspection.run_dir.display());
    println!("required files present: {}", inspection.files.len());
    for file in inspection.files {
        println!("- {} ({} bytes)", file.name, file.byte_len);
    }
    println!("static status: valid static shape");

    Ok(())
}
