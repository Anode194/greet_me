extern crate clicolors_control;
extern crate dirs;
use std::process::Command;
use std::vec::Vec;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use rand::prelude::*;
use json::*;

pub struct Quote {
    pub quote: String,
    pub author: String
}
impl Quote {
    pub fn new () -> Quote {
        Quote { quote: String::new(), author: String::new()}
    }
}
pub struct TodoData {
    pub urgent: Vec<String>,
    pub non_urgent: Vec<String>
}
impl TodoData {
    pub fn new() -> TodoData {
        TodoData { urgent: Vec::new(), non_urgent: Vec::new()}
    }
}
impl TodoData {
    pub fn clean_output(&mut self) -> &mut TodoData {
        for el in self.urgent.iter_mut() {
            let temp_string = String::from(el.trim_start_matches(" U ").to_string());
            el.clear();
            el.push_str(temp_string.as_str());
        }
        self
    }
}
pub fn ascii_border() {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0,100);
    match x {
        0..=20=> {
            println!("\t\t\t|\x1b[96m  .-.-.  .-.-.  .-.-.  .-.-.  .-.-.  .-.-.  .-.-.  .-.-.  .-.-.  .-.-.  .-.-.  .-.-.  .-.-.  .-.-.  .-.-. \x1b[0m");
            println!("\t\t\t|\x1b[96m =`. .'==`. .'==`. .'==`. .'==`. .'==`. .'==`. .'==`. .'==`. .'==`. .'==`. .'==`. .'==`. .'==`. .'==`. .'=\x1b[0m");
            println!("\t\t\t|\x1b[96m    \"      \"      \"      \"      \"      \"      \"      \"      \"      \"      \"      \"      \"      \"      \"\x1b[0m");
        },
        21..=40=> {
            println!("\t\t\t|\x1b[96m_________________ O/_________________________________________________________________________________________\x1b[0m");
            println!("\t\t\t|\x1b[96m                  0\\ \x1b[0m");
        },
        41..=60 => {
            print!("\t\t\t|\x1b[96m .--.      .-'.      .--.      .--.      .--.      .--.      .`-.      .--.\x1b[0m");
            println!("\x1b[96m .--.      .-'.      .--.      .--.      .--.      .--.      .`-.      .--.\x1b[0m");
            print!("\t\t\t|\x1b[96m\\:::::.\\::::::::.\\::::::::.\\::::::::.\\::::::::.\\::::::::.\\::::::::.\\::::::::.\x1b[0m");
            println!("\x1b[96m\\:::::.\\::::::::.\\::::::::.\\::::::::.\\::::::::.\\::::::::.\\::::::::.\\::::::::.\x1b[0m");
            print!("\t\t\t|\x1b[96m'      `--'      `.-'      `--'      `--'      `--'      `-.'      `--'      `\x1b[0m");
            println!("\x1b[96m'      `--'      `.-'      `--'      `--'      `--'      `-.'      `--'      `\x1b[0m");
        }
        61..=70 => {
            print!("\t\t\t|\x1b[96m+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+\x1b[0m");
            println!("\x1b[96m+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+\x1b[0m");
        }
        _ => {
            print!("\t\t\t|\x1b[96m <<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>\x1b[0m");
            println!("\x1b[96m<<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>><<>>\x1b[0m");

        }
    }
}

pub fn output(mut todo: TodoData) {
    todo.clean_output();
    if clicolors_control::colors_enabled() {
            print!("\t\t\t|  ");
            println!("\x1b[36;4;1mwelcome Jo here is your todo_list.\x1b[0m");
            self::ascii_border();
            println!("\t\t\t|  \x1b[31;48;100mURGENTS\t\x1b[0m",);
            println!("\t\t\t|  ");
            print!("\t\t\t|  ");
        for element in todo.urgent.iter() {
            print!("\x1b[31;48;100m{}\t\x1b[0m",element);
        }
            println!(" ");
            println!("\t\t\t|");
        let mut x = 0; 
            print!("\t\t\t|");
            println!("\x1b[33m  non urgents\t\x1b[0m",);
            println!("\t\t\t|  ");
            print!("\t\t\t|");
        for element in todo.non_urgent.iter() {
            
            print!("\x1b[33m{}\x1b[0m",element);
            x+=1;
            if x == 4 {
                println!(" ");
                print!("\t\t\t|  ");
                x = 0; 
            }
        }
            println!(" ");
            println!("\t\t\t|  ");
            print!("\t\t\t|  ");
            let quote = read_json_quote();
            println!("\x1b[34;52;4m{} {}\x1b[0m",quote.quote,quote.author);
    }

}
#[allow(unused_assignments)]
pub fn read_json_quote() -> Quote {
    let mut path:PathBuf = dirs::home_dir().unwrap();
    path.push(".config/greet_me/quotes.json");
    let quote_file=File::open(path).unwrap();
    let reader = BufReader::new(quote_file);
    let mut json_string: String = " ".to_string();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        json_string.push_str(line.as_str());
    }
    let mut return_quote = Quote::new();
    let mut obj:JsonValue = json::parse(json_string.as_str()).unwrap();
    let mut json_obj = json::JsonValue::new_object();
    if obj.is_array() {
        json_obj = obj.pop();
        match json_obj {
            json::JsonValue::Object(mut object) => {
               return_quote.quote =  object.remove("quote").unwrap().dump();
               return_quote.author = object.remove("author").unwrap().dump();
               return_quote.author.truncate(return_quote.author.len() -1);
               return_quote.author = return_quote.author.replace('"'," ").to_string();
            }
            _ => {}
        }
    } else { 
        println!("false");
    }
    return_quote
}
pub fn read_text_and_parse() -> TodoData {
    let mut todo_filename = dirs::home_dir().unwrap();
    todo_filename.push(".config/greet_me/todo.txt"); 

    let todo_file = File::open(todo_filename).unwrap();
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

