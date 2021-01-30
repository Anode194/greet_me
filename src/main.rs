mod data;
mod input;
mod output;

pub use crate::data::*;
pub use crate::input::*;
pub use crate::output::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_ref() {
            "-a" => {
                let output_text = read_alt_format(&args[2]);
                output::output(output_text)
            }
            "-d" => {
                output::print_all_quotes();
            }
            "-s" => {
                input::save_qoute();
                println!("daily quote saved");
            }
            "-q" => {
                output::quote_output();
            }
            "-h" | "--help" => {
                println!(" greet me is a simple todo tracker / greeter");
                println!("-a 'file name'\t\t for todo lists not tracked with joplin");
                println!("-s save daily quote. be careful to not save duplicates!");
                println!("-d print all saved quotes");
                println!("-q \t\t print the daily quote");
                println!("-h --help \t\t'prints this help message'");
            }
            _ => {
                input::joplin_setup();
                let output_text = read_text_and_parse();
                output::output(output_text)
            }
        }
    } else {
        input::joplin_setup();
        let output_text = read_text_and_parse();
        output::output(output_text)
    }
}
