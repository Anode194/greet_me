use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
//use std::path::Path;
//use std::ffi::OsStr;
use std::vec::Vec;
fn main() {
    let joplin_setup = "joplin use TODO";
    let joplin_setup2 = "joplin ls > $HOME/todo.txt";

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

    let todo_filename = "/home/anode/todo.txt";
    let todo_file = File::open(todo_filename).unwrap();
    let reader = BufReader::new(todo_file);

    let mut todo_text = Vec::new();
    
    println!("welcome Jo here is your todo_list");
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        todo_text.push(line);
    }
    let mut todo_string = " ".to_string();
    let mut todos: Vec<String> = Vec::new();
    for mut el in todo_text.iter() {
        if el.chars().nth(1) == Some('X') { break;}
        let mut s = el.clone();
        todo_string.push_str(s.trim_start_matches("[ ] ").trim());
        todo_string.push_str(" ");
    }
        println!("{:?}", todo_string);
}
struct todo {
    urgent: Vec<String>,
    non_urgent: Vec<String>
}
impl todo  {
    fn new() -> todo {
        todo { urgent: Vec::new(), non_urgent: Vec::new()}
    }
}
/* two data structures: urgent todos and not-urgent todos. urgent todo's have U_ 
  in front of their name. urgents if they are below 5 are all printed to the 
    screen. if you pass in a command line argument you will get a total list of all todo's.
*/ 

