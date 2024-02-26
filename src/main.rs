//! Personal LS command implementation
use rust_ls::argument_parsing::Arguments;
use std::env;
use std::fs::{metadata, read_dir};
use std::os::windows::fs::MetadataExt;

fn main() {
    let all_raw_args = env::args().collect::<Vec<String>>();
    let raw_arg_array = all_raw_args.get(1..);
    let argument_parsing_outcome = raw_arg_array.map_or_else(
        || Arguments::new(&vec![]),
        |arg_array| Arguments::new(&arg_array.to_vec()),
    );
    let args = match argument_parsing_outcome {
        Ok(arguments) => arguments,
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };

    println!("{args:#?}");

    let directory = "./";
    let Ok(paths) = read_dir(directory) else {
        eprintln!("Failed to read directory {directory}.");
        return;
    };

    for path_entry in paths {
        let Ok(path) = path_entry else {
            eprintln!("Failed to open {path_entry:?}.");
            return;
        };
        println!("Name: {}", path.path().display());
        let Ok(metadata) = metadata(path.path()) else {
            eprintln!("Failed to get metadata from {}.", path.path().display());
            return;
        };
        println!(
            "File Type: {:?}, Permissions: {:?}, Owner: {:?}",
            metadata.file_type(),
            metadata.permissions(),
            metadata.file_attributes()
        );
        // TODO: get group and user owners with libc
        // TODO: get file size, last modified
    }
}
