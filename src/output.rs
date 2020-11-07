extern crate termion;
use rand::prelude::*;
use crate::data::*;
pub fn ascii_border() { 
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0,100);
    match x {
        0..=20 => {
            let term_construct = termion::terminal_size().unwrap();
            let mut term_size = term_construct.0;
            term_size = term_size / 10;
            print!("\t\t\t|");
            for _x in 0..term_size {
                print!("\x1b[96m  .-.-. \x1b[0m");
            }
            println!("");
            print!("\t\t\t|");
            for _x in 0..term_size {
                print!("\x1b[96m =`. .'=\x1b[0m");
            }
            println!("");
            print!("\t\t\t|");
            for _x in 0..term_size {
                print!("\x1b[96m    \"   \x1b[0m");
            }
            println!("");
        }
        21..=40 => {
            println!("\t\t\t|\x1b[96m_________________ O/________________________________________________________________________\x1b[0m");
            println!("\t\t\t|\x1b[96m                  0\\ \x1b[0m");
        }
        41..=60 => {
            let term_construct = termion::terminal_size().unwrap();
            let mut term_size = term_construct.0;
            term_size = term_size / 30;
            print!("\t\t\t|");
            for _x in 0..term_size {
                print!("\x1b[96m  .--.       .-'.    \x1b[0m");
            }
                println!("");
                print!("\t\t\t|");
                for _x in 0..term_size {
                    print!("\x1b[96m:::::.\\:::::::::\\::::\x1b[0m");
                }
                println!("");
                print!("\t\t\t|");
                for _x in 0..term_size {
                    print!("\x1b[96m       `--'      `.-'\x1b[0m");
                }
                println!("");
            }
        61..=70 => {
            let term_construct = termion::terminal_size().unwrap();
            let mut term_size = term_construct.0;
            term_size = term_size / 10;

            print!("\t\t\t|");
            for _x in 0..term_size {
                print!("\x1b[96m+-+-+-\x1b[0m");
            }
            println!("");
        }
        71..=90 => {
            let term_construct = termion::terminal_size().unwrap();
            let mut term_size = term_construct.0;
            term_size = term_size / 10;
            print!("\t\t\t|");
            for _x in 0..term_size {
                print!("\x1b[96m___  ___\x1b[0m");
            }
            println!("");
            print!("\t\t\t|");
            for _x in 0..term_size {
                print!("\x1b[96m __)(__ \x1b[0m");
            }
            println!("");
            print!("\t\t\t|");
            for _x in 0..term_size {
                print!("\x1b[96m(______)\x1b[0m");
            }
            println!("");
        }
        91..=100 => {
            let term_construct = termion::terminal_size().unwrap();
            let mut term_size = term_construct.0;
            term_size = term_size / 5;
            print!("\t\t\t|");
            for _x in 0..term_size {
                print!("\x1b[96m * *\x1b[0m");
            }
            println!("");
        }
        _ => {
            let term_construct = termion::terminal_size().unwrap();
            let mut term_size = term_construct.0;
            term_size = term_size / 10;
            print!("\t\t\t|");
            for _x in 0..term_size {
                print!("\x1b[96m<<>><<>>\x1b[0m");
            }
            println!("");
        }
    }
}
pub fn output(mut todo: TodoData) {
    todo.clean_output();
    let greeting = greeting::read();
    if clicolors_control::colors_enabled() {
        print!("\t\t\t|  ");
        println!("\x1b[36;4;1m{}\x1b[0m",greeting.text);
        ascii_border();
        println!("\t\t\t|  \x1b[31;48;100mURGENTS\t\x1b[0m",);
        println!("\t\t\t|  ");
        print!("\t\t\t|  ");
        for element in todo.urgent.iter() {
            print!("\x1b[31;48;100m{}\t\x1b[0m", element);
        }
        println!(" ");
        println!("\t\t\t|");
        let mut x = 0;
        print!("\t\t\t|");
        println!("\x1b[33m  non urgents\t\x1b[0m",);
        println!("\t\t\t|  ");
        print!("\t\t\t|  ");
        for element in todo.non_urgent.iter() {
            print!("\x1b[33m{}\x1b[0m|", element);
            x += 1;
            if x == 4 {
                println!(" ");
                print!("\t\t\t|  ");
                x = 0;
            }
        }
        println!(" ");
        println!("\t\t\t|  ");
        print!("\t\t\t|  ");
        quote_output();
    }
}

pub fn quote_output() {
    if clicolors_control::colors_enabled() {
        let quote = read_json_quote();
        println!("\x1b[34;52;4m{} {}\x1b[0m", quote.quote, quote.author);
    }
}
