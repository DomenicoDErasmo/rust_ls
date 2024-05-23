//! Personal LS command implementation
use core::cmp::Ordering;
use core::fmt::Error;
use rust_ls::argument_parsing::Arguments;
use std::env::args;
use std::fs::{read_dir, ReadDir};

/// Holds the RGB values of some color.
struct Color {
    /// The RGB to output
    rgb: [u8; 3],
}

/// A string that supports color printing out of the box.
struct ColoredString {
    /// The text to output
    input: String,
    /// The RGB color to output
    rgb: Color,
}

impl ColoredString {}

/// Sorts two strings with lowercase letters first.
fn alphabetic_lowercase_first(left: &String, right: &String) -> Ordering {
    let Some(left_first_char) = left.chars().next() else {
        return Ordering::Less;
    };

    let Some(right_first_char) = right.chars().next() else {
        return Ordering::Less;
    };

    match (
        left_first_char.is_lowercase(),
        right_first_char.is_lowercase(),
    ) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => left.cmp(right),
    }
}

/// Gets all paths in the directory.
fn get_paths(paths: ReadDir) -> Result<Vec<String>, Error> {
    let mut result = vec![];

    for path_entry in paths {
        let Ok(path) = path_entry else {
            eprintln!("Failed to open {path_entry:?}.");
            return Err(Error);
        };

        let Ok(name) = path.file_name().into_string() else {
            eprintln!("Failed to convert {path:?} into a string.");
            return Err(Error);
        };

        let Ok(file_type) = path.file_type() else {
            eprintln!("Failed to get file type for {path:?}.");
            return Err(Error);
        };

        result.push(name);
    }

    Ok(result)
}

fn main() {
    let all_raw_args = args().collect::<Vec<String>>();
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

    let Ok(mut filenames) = get_paths(paths) else {
        eprintln!("Failed to get all paths in the directory.");
        return;
    };

    filenames.sort_by(alphabetic_lowercase_first);

    let filtered_filenames = filenames
        .into_iter()
        .filter(|filename| !filename.starts_with('.'))
        .collect::<Vec<String>>();

    let output = filtered_filenames.join("  ");
    println!("{output}");
}
