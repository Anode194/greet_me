use json::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
pub struct TodoData {
    pub urgent: Vec<String>,
    pub non_urgent: Vec<String>,
}
impl TodoData {
    pub fn new() -> TodoData {
        TodoData {
            urgent: Vec::new(),
            non_urgent: Vec::new(),
        }
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
pub struct Quote {
    pub quote: String,
    pub author: String,
}
impl Quote {
    pub fn new() -> Quote {
        Quote {
            quote: String::new(),
            author: String::new(),
        }
    }
}
pub struct greeting {
    pub text: String,
}
impl greeting {
    pub fn read() -> greeting {
        let mut path: PathBuf = dirs::home_dir().unwrap();
        path.push(".config/greet_me/greeting.txt");
        let mut greeting_file = File::open(path).unwrap();
        let mut greet = greeting {
            text: String::new(),
        };
        match greeting_file.read_to_string(&mut greet.text) {
            Ok(i) => i,
            Err(e) => panic!("something went wrong reading the file. \n{}", e),
        };

        greet
    }
}

#[allow(unused_assignments)]
pub fn read_json_quote() -> Quote {
    let mut path: PathBuf = dirs::home_dir().unwrap();
    path.push(".config/greet_me/quotes.json");
    let quote_file = File::open(path).unwrap();
    let reader = BufReader::new(quote_file);
    let mut json_string: String = " ".to_string();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        json_string.push_str(line.as_str());
    }
    let mut return_quote = Quote::new();
    let mut obj: JsonValue = json::parse(json_string.as_str()).unwrap();
    let mut json_obj = json::JsonValue::new_object();
    if obj.is_array() {
        json_obj = obj.pop();
        match json_obj {
            json::JsonValue::Object(mut object) => {
                return_quote.quote = object.remove("quote").unwrap().dump();
                return_quote.author = object.remove("author").unwrap().dump();
                return_quote.author.truncate(return_quote.author.len() - 1);
                return_quote.author = return_quote.author.replace('"', " ").to_string();
            }
            _ => {}
        }
    } else {
        println!("false");
    }
    return_quote
}
