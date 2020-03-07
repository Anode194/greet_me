use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use greetme::TodoData;
fn main() {
    greetme::joplin_setup();
    read_text_and_parse();
    println!("welcome Jo here is your todo_list");
    
        
    //let mut todo_text = Vec::new();
}
fn read_text_and_parse() {
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
            Some('U') => todos.urgent.push(todo_string),
            Some(_) => todos.non_urgent.push(todo_string),
            None => println!("an error occured with parsing the todo file"),
        }
    }
        output(todos);
}
fn output(todo: TodoData) {
    println!("{:?}", todo.urgent);
    println!("{:?}", todo.non_urgent);

}
