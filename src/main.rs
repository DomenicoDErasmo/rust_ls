//! Personal LS command implementation
use rust_ls::argument_parsing::setup_args;
use rust_ls::output::Output;
use std::fs::{read_dir, DirEntry};

fn main() {
    let args = match setup_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{err}");
            return;
        }
    };

    let Ok(directory) = read_dir(args.path.clone()) else {
        eprintln!("Failed to read directory {}.", args.path);
        return;
    };

    let Ok(dir_entries): Result<Vec<DirEntry>, _> =
        directory.into_iter().collect()
    else {
        eprintln!("Failed to collect paths");
        return;
    };

    let Some(paths) = Output::new(dir_entries) else {
        eprintln!("Failed to create OutputEntry from dir_entries.");
        return;
    };

    let output = paths.output(&args);
    println!("{output}");
}
