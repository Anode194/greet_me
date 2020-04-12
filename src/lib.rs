use std::process::Command;
use std::vec::Vec;
use crossterm::style::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
pub struct TodoData {
    pub urgent: Vec<String>,
    pub non_urgent: Vec<String>
}
impl TodoData {
    pub fn new() -> TodoData {
        TodoData { urgent: Vec::new(), non_urgent: Vec::new()}
    }
}

pub fn output(todo: TodoData) {
    for element in todo.urgent.iter() {
        print!("{}\t",element);
    }
        println!(" ");
        println!(" ");
    for element in todo.non_urgent.iter() {
        print!("{}\t",element);
    }

}
pub fn read_text_and_parse() -> TodoData {
    let _mac_todo_filename = "/Users/anode/.config/greet_me/todo.txt";
    let linux_todo_filename = "/home/anode/.config/greet_me/todo.txt"; 

    //replace with linux when on linux til config file function is built.
    let todo_file = File::open(linux_todo_filename).unwrap();
    let reader = BufReader::new(todo_file);

    let mut todo_text = Vec::new();
    let mut todos = TodoData::new();
    
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        todo_text.push(line);
    }
        for el in todo_text.iter() {
        let mut todo_string = " ".to_string();
        if el.chars().nth(1) == Some('X') { break;}
        let s = el.clone();
        todo_string.push_str(s.trim_start_matches("[ ] ").trim());
        todo_string.push_str(" ");
        match todo_string.chars().nth(1) {
            Some('U') => todos.urgent.push(todo_string
                .to_uppercase()
                .replace("_"," ")),
            Some(_) => todos.non_urgent.push(todo_string.replace("_"," ")),
            None => println!("an error occured with parsing the todo file"),
        }
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
    setup_1.arg("-c")
        .arg(joplin_setup)
        .output()
        .expect("something went wrong");

    let mut setup_2 = Command::new("sh");
    setup_2.arg("-c")
        .arg(joplin_setup2)
        .output()
        .expect("something went wrong");

}
