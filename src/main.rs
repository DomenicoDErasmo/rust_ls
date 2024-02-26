//! Personal LS command implementation
use std::env;
use rust_ls::argument_parsing::Arguments;

fn main() {
    let all_raw_args = env::args().collect::<Vec<String>>();
    let raw_arg_array = all_raw_args.get(1..);
    let args = raw_arg_array.map_or_else(
        || Arguments::new(&vec![]),
        |arg_array| Arguments::new(&arg_array.to_vec()),
    );
    println!("{args:#?}");
}
