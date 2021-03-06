extern crate clicolors_control;
extern crate dirs;
use crate::data::TodoData;
use crate::data::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::*;
use std::path::PathBuf;
use std::process::Command;
use std::vec::Vec;

pub fn read_text_and_parse() -> TodoData {
    let mut todo_filename = dirs::home_dir().unwrap();
    todo_filename.push(".config/greet_me/todo.txt");

    let todo_file = File::open(todo_filename).unwrap();
    let reader = BufReader::new(todo_file);

    let mut todo_text = Vec::new();
    let mut todos = crate::data::TodoData::new();

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        todo_text.push(line);
    }
    for el in todo_text.iter() {
        let mut todo_string = " ".to_string();
        if el.chars().nth(1) == Some('X') {
            break;
        }
        let s = el.clone();
        todo_string.push_str(s.trim_start_matches("[ ] ").trim());
        todo_string.push_str(" ");
        match todo_string.chars().nth(1) {
            Some('U') => todos
                .urgent
                .push(todo_string.to_uppercase().replace("_", " ")),
            Some(_) => todos.non_urgent.push(todo_string.replace("_", " ")),
            None => println!("an error occured with parsing the todo file"),
        }
        //this removes the " U " from the strings.
    }
    todos
}

/* two data structures: urgent todos and not-urgent todos. urgent todo's have U_
  in front of their name. urgents if they are below 5 are all printed to the
    screen. if you pass in a command line argument you will get a total list of all todo's.
*/
pub fn joplin_setup() {
    let joplin_setup = "joplin use TODO";
    let joplin_setup2 = "joplin ls > $HOME/.config/greet_me/todo.txt";

    let mut setup_1 = Command::new("sh");
    setup_1
        .arg("-c")
        .arg(joplin_setup)
        .output()
        .expect("something went wrong");

    let mut setup_2 = Command::new("sh");
    setup_2
        .arg("-c")
        .arg(joplin_setup2)
        .output()
        .expect("something went wrong");
}
pub fn read_alt_format(file_name: &str) -> TodoData {
    let todo_file = match OpenOptions::new().read(true).write(true).open(file_name) {
        Ok(x) => x,
        Err(e) => panic!(
            "couldn't open todo file the -a flag must include a file {:?}",
            e
        ),
    };

    let mut todos = TodoData::new();
    let reader = BufReader::new(todo_file);

    let mut flag = false;
    for line in reader.lines().enumerate() {
        if line.0 > 0 {
            let text = line.1.unwrap();
            if text == "non_urgents" {
                flag = true;
                continue;
            } else if flag {
                todos.non_urgent.push(text);
            } else {
                todos.urgent.push(text);
            }
        }
    }

    todos
}
pub fn save_qoute() {
    let quote = read_json_quote();
    let mut path: PathBuf = dirs::home_dir().unwrap();
    path.push(".config/greet_me/saved_quotes");
    let mut saved_quotes_file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(path)
        .unwrap();
    if let Err(e) = writeln!(saved_quotes_file, "{}", quote.quote_str()) {
        eprintln!("Couldn't save quote to file: {}", e);
    }
}

pub fn record_greeting() -> String {
    println!("type what you would like your greeting to be! ");
    let mut greeting = String::new();
    match stdin().read_line(&mut greeting) {
        Ok(_greeting) => {}
        Err(e) => panic!(
            "couldn't capture your greeting, did you type anything? \n{:?}",
            e
        ),
    }
    greeting.trim().to_string()
}
pub fn new_greeting() {
    let mut path: PathBuf = dirs::home_dir().unwrap();
    path.push(".config/greet_me/greeting.txt");

    let mut greeting_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .unwrap();
    greeting_file.write_all(record_greeting().as_bytes()).expect("unable to write new greeeting to a file");
    println!("greeting saved successfully!");
}
