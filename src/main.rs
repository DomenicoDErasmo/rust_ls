//! Personal LS command implementation
use colored::Colorize;
use core::fmt::Error;
use rust_ls::argument_parsing::Arguments;
use std::env;
use std::fs::{read_dir, ReadDir};

/// Gets all paths in the directory.
fn get_paths(paths: ReadDir) -> Result<Vec<String>, Error> {
    let mut result = vec![];

    for path_entry in paths {
        let Ok(path) = path_entry else {
            eprintln!("Failed to open {path_entry:?}.");
            return Err(Error);
        };

        let Ok(mut name) = path.file_name().into_string() else {
            eprintln!("Failed to convert {path:?} into a string.");
            return Err(Error);
        };

        if name.starts_with('.') {
            continue;
        }

        let Ok(file_type) = path.file_type() else {
            eprintln!("Failed to get file type for {path:?}.");
            return Err(Error);
        };

        if file_type.is_dir() {
            name = name.blue().bold().to_string();
        }

        result.push(name);
    }

    // TODO: folders show up before files but should be opposite
    result.sort();

    Ok(result)
}

fn main() {
    let all_raw_args = env::args().collect::<Vec<String>>();
    let raw_arg_array = all_raw_args.get(1..);
    let argument_parsing_outcome = raw_arg_array.map_or_else(
        || Arguments::new(&vec![]),
        |arg_array| Arguments::new(&arg_array.to_vec()),
    );
    let _: Arguments = match argument_parsing_outcome {
        Ok(arguments) => arguments,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };

    let directory = "./";
    let Ok(paths) = read_dir(directory) else {
        eprintln!("Failed to read directory {directory}.");
        return;
    };

    let Ok(filenames) = get_paths(paths) else {
        eprintln!("Failed to get all paths in the directory.");
        return;
    };

    let output = filenames.join("  ");

    println!("  {output}");
}
