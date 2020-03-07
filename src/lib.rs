use std::process::Command;
use std::vec::Vec;

pub struct TodoData {
    pub urgent: Vec<String>,
    pub non_urgent: Vec<String>
}
impl TodoData {
    pub fn new() -> TodoData {
        TodoData { urgent: Vec::new(), non_urgent: Vec::new()}
    }
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
