use greetme::TodoData;
fn main() {
    greetme::joplin_setup();
    let output_text = greetme::read_text_and_parse();
    println!("\t\twelcome Jo here is your todo_list");
    greetme::output(output_text)
}
