//! Personal LS command implementation
use glob::{glob_with, MatchOptions};
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

    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };
    let Ok(globs) = glob_with(&args.path, options) else {
        eprintln!("Failed to glob {}.", &args.path);
        return;
    };
    for glob in globs {
        let Ok(path) = glob else {
            eprintln!("Failed to get path for {glob:#?}.");
            return;
        };

        // TODO: 
        // read_dir on the target directory for the regex pattern
        // use its canonical path
        // if we have a file that matches the pattern, add to output
        // if we have a directory that matches the pattern, read_dir it
        let Ok(directory) = read_dir(path.clone()) else {
            eprintln!("Failed to read directory {path:#?}.");
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
}
