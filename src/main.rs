//! Personal LS command implementation
use core::cmp::Ordering;
use core::fmt;
use core::fmt::Display;
use core::fmt::Error;
use rust_ls::argument_parsing::Arguments;
use std::env::args;
use std::fs::{read_dir, ReadDir};

/// Holds the RGB values of some color.
#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Color {
    /// The RGB to output
    rgb: [u8; 3],
}

impl Color {
    /// Makes the color blue.
    pub const fn blue() -> Self {
        Self {
            rgb: [u8::MIN, u8::MIN, u8::MAX],
        }
    }
    /// Makes the color white.
    pub const fn white() -> Self {
        Self {
            rgb: [u8::MAX, u8::MAX, u8::MAX],
        }
    }
}

/// A string that supports color printing out of the box.
#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct FormattableString {
    /// The text to output
    pub input: String,
    /// The RGB color to output
    pub color: Color,
    /// Whether to output text in bold
    pub bold: bool,
}

impl Display for FormattableString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let colored_string = format!(
            "\x1B[38;2;{};{};{}m{}\x1B[0m",
            self.color.rgb[0], self.color.rgb[1], self.color.rgb[2], self.input
        );
        let bold_colored_string = format!(
            "{}{}{}",
            if self.bold { "\x1b[1m" } else { "" },
            colored_string,
            if self.bold { "\x1b[22m" } else { "" },
        );
        write!(formatter, "{bold_colored_string}")
    }
}

// TODO: make custom impl Ord for ColoredString?
/// Sorts two strings with lowercase letters first.
fn alphabetic_lowercase_first(
    left: &FormattableString,
    right: &FormattableString,
) -> Ordering {
    let Some(left_first_char) = left.input.chars().next() else {
        return Ordering::Less;
    };

    let Some(right_first_char) = right.input.chars().next() else {
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
fn get_paths(paths: ReadDir) -> Result<Vec<FormattableString>, Error> {
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

        let colored_string = FormattableString {
            input: name,
            color: if file_type.is_dir() {
                Color::blue()
            } else {
                Color::white()
            },
            bold: file_type.is_dir(),
        };

        result.push(colored_string);
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
        .filter(|filename| !filename.input.starts_with('.'))
        .collect::<Vec<FormattableString>>();

    let output: String = filtered_filenames
        .iter()
        .map(|colored_string| {
            let mut string = colored_string.to_string();
            string.push_str("  ");
            string
        })
        .collect();
    println!("{output}");
}
