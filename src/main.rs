use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    let argues: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&argues);
    if query == "" && filename == "" {
        println!("not enough arguments");
    } else {
        println!("Searching for : {}", query);
        let mut f = fs::File::open(filename)
            .expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong when reading the file");
        contents = truncate_content(&contents, 240);
        println!("With text:\n{}", contents);
    }
}

fn truncate_content(contents: &str, max_length: usize) -> String {
    if contents.len() > max_length {
        let truncated = &contents[..max_length.saturating_sub(3)];
        format!("{}...", truncated)
    } else {
        contents.to_string()
    }
}

fn parse_config(args: &[String]) -> (&str, &str) {
    if args.len() < 3 {
        ("", "")
    } else {
        let query = &args[1];
        let filename = &args[2];
        (query, filename)
    }
}
