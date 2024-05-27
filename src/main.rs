//! Personal LS command implementation
use rust_ls::argument_parsing::{ArgReadingError, Arguments};
use rust_ls::output::Output;
use std::env::args;
use std::fs::{read_dir, DirEntry};

/// Sets up args from command line.
fn setup_args() -> Result<Arguments, ArgReadingError> {
    let all_raw_args = args().collect::<Vec<String>>();
    let raw_arg_array = all_raw_args.get(1..);
    raw_arg_array.map_or_else(
        || Arguments::new(&vec![]),
        |arg_array| Arguments::new(&arg_array.to_vec()),
    )
}

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
