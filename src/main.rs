use std::env;
use std::fs;
use std::io::prelude::*;

fn truncate_content(contents: &str, max_length: usize) -> String {
    if contents.len() > max_length {
        let truncated = &contents[..max_length.saturating_sub(3)];
        format!("{}...", truncated)
    } else {
        contents.to_string()
    }
}

fn main() {
    let argues: Vec<String> = env::args().collect();
    let query = &argues[1];
    let filename = &argues[2];

    println!("Searching for : {}", query);
    let mut f = fs::File::open(filename)
        .expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong when reading the file");
    contents = truncate_content(&contents, 240);
    println!("With text:\n{}", contents);
}
