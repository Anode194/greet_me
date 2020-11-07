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
                let mut output_text = read_alt_format(&args[2]);
                output::output(output_text)
            }
            "-t" => {}
            "-h" | "--help" => {
                println!(" greet me is a simple todo tracker / greeter");
                println!("-a 'file name'\t\t for todo lists not tracked with joplin");
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
