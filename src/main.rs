//! Personal LS command implementation
use rust_ls::argument_parsing::Arguments;
use std::env;

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
}
