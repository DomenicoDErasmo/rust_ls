use crate::argument_parsing::Arguments;
use crate::output_entry::OutputEntry;
use core::cmp::Ordering;
use std::env::current_dir;
use std::fs::DirEntry;

pub struct Output {
    /// The list of `OutputEntry` objects
    pub entries: Vec<OutputEntry>,
}

/// Sorts two strings with lowercase letters first.
fn alphabetic_lowercase_first(
    left: &OutputEntry,
    right: &OutputEntry,
) -> Ordering {
    let left_first_char =
        left.displayed_name().chars().next().unwrap_or_default();
    let right_first_char =
        left.displayed_name().chars().next().unwrap_or_default();
    match (left.file.is_dir(), right.file.is_dir()) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => match (left_first_char == '.', right_first_char == '.') {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => left.displayed_name().cmp(&right.displayed_name()),
        },
    }
}

/// Specifies which files to include based on the filter passed in.
#[allow(clippy::match_bool)]
fn include_based_on_args(filename: &OutputEntry, args: &Arguments) -> bool {
    match args.all {
        true => true,
        false => !filename.displayed_name().starts_with('.'),
    }
}

impl Output {
    #[inline]
    #[must_use]
    pub fn new(dir_entries: Vec<DirEntry>) -> Option<Self> {
        let mut paths: Vec<_> = dir_entries
            .into_iter()
            .map(|path| OutputEntry {
                file: path.path().as_path().to_owned(),
                ..Default::default()
            })
            .collect();

        let Ok(current_directory_buf) = current_dir() else {
            eprintln!("Failed to get current directory.");
            return None;
        };

        let current_clone = current_directory_buf.clone();
        let parent_directory = current_clone.parent().unwrap_or(&current_clone);

        paths.push(OutputEntry {
            file: current_directory_buf,
            is_current_directory: true,
            is_parent_directory: false,
        });
        paths.push(OutputEntry {
            file: parent_directory.to_path_buf(),
            is_current_directory: false,
            is_parent_directory: true,
        });
        paths.sort_by(alphabetic_lowercase_first);

        Some(Self { entries: paths })
    }

    #[inline]
    #[must_use]
    pub fn output(self, args: &Arguments) -> String {
        self.entries
            .into_iter()
            .filter(|filename| include_based_on_args(filename, args))
            .collect::<Vec<OutputEntry>>()
            .iter()
            .map(|colored_string| {
                let mut string = colored_string.to_string();
                string.push_str("  ");
                string
            })
            .collect()
    }
}
