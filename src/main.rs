use std::{env, fs, io::{Read}};

fn main() {
    let a = env::args().collect::<Vec<String>>();
    let (q, f) = (a.get(1).map(|s| &s[..]).unwrap_or(""), a.get(2).map(|s| &s[..]).unwrap_or(""));
    if q.is_empty() || f.is_empty() {
        println!("not enough arguments");
    } else {
        let mut b = fs::File::open(f).unwrap_or_else(|_| panic!("file not found"));
        let mut c = String::new();
        b.read_to_string(&mut c).unwrap_or_else(|_| panic!("read error"));
        println!("Searching for: {}", q);
        println!("With text:\n{}", c.chars().take(240).collect::<String>().replace(&c[240..], "..."));
    }
}
