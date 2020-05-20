fn main() {
    greetme::joplin_setup();
    let output_text = greetme::read_text_and_parse();
    greetme::output(output_text)
        
}
