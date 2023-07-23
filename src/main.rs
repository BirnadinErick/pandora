use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_file(name: &str) -> String {
    let file = match File::open(name) {
        Ok(file) => file,
        Err(_) => panic!("Error opening the input file"),
    };
    let mut buf = BufReader::new(file);
    let mut content = String::new();

    if let Err(_) = buf.read_to_string(&mut content) {
        panic!("Error reading the file buffer")
    };

    content
}

fn parse_axons(md: &str, _map: Option<()>, host: &str) -> String {
    let regex =
        Regex::new(r"(?mU)\[{2}(?P<id>[a-zA-Z\s\-\d]+)\|{1}(?P<desc>[a-zA-Z\s\-\d_]*)\]{2}")
            .unwrap();

    let substitution = format!("[$desc]({}/$id.html)", host);

    regex
        .replace_all(md, |caps: &Captures| {
            let id = &caps["id"];
            println!("{}", id);
            substitution.clone()
        })
        .into()
}

fn main() {
    let host = "https://eb.birn.cc";
    let content = read_file("input.md");

    let mut dir: HashMap<&str, &str> = HashMap::new();
    dir.insert("input.md", &content);

    for (_, v) in dir.iter() {
        println!("{}", parse_axons(*v, None, host));
    }
}
