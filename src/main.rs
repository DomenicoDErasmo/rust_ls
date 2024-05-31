//! Personal LS command implementation
use glob::{glob_with, MatchOptions};
use rust_ls::argument_parsing::setup_args;
use rust_ls::output::Output;
use std::fs::{read_dir, DirEntry};
use std::path::PathBuf;

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

    // TODO:
    // read_dir on the target directory for the regex pattern
    // use its canonical path
    // if we have a file that matches the pattern, add to output
    // if we have a directory that matches the pattern, read_dir it
    // should I just convert my DirEntries to PathBuf and use PathBuf?
    let mut path_bufs = vec![];
    for glob in globs {
        let Ok(path) = glob else {
            eprintln!("Failed to get path for {glob:#?}.");
            return;
        };

        if path.is_dir() {
            let Ok(current_directory) = read_dir(path.clone()) else {
                eprintln!("Failed to read directory {path:#?}.");
                continue;
            };

            let Ok(current_directory_entries): Result<Vec<DirEntry>, _> =
                current_directory.into_iter().collect()
            else {
                eprintln!("Failed to collect paths");
                continue;
            };

            let mut curernt_directory_paths: Vec<PathBuf> =
                current_directory_entries
                    .into_iter()
                    .map(|entry| entry.path())
                    .collect();

            path_bufs.append(&mut curernt_directory_paths);
        }

        if path.is_file() {}
    }
    let Some(paths) = Output::new(path_bufs) else {
        eprintln!("Failed to create OutputEntry from dir_entries.");
        return;
    };

    let output = paths.output(&args);
    println!("{output}");
}
