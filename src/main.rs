use owo_colors::OwoColorize;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./test.txt").expect("Found file!");
    let first_word: &str = &contents[0..3];
    println!("The test file is:\n{}", contents.red());
    println!("The first word is:\n{}", first_word);
}
