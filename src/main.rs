use glob::glob;
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

fn discover_files(root: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for p in glob(&format!("{}/**/*.md", root))
        .unwrap()
        .filter_map(std::result::Result::ok)
    {
        files.push(p.display().to_string())
    }

    files
}

fn parse_axons(md: &str, _map: Option<()>, host: &str) -> String {
    let regex =
        Regex::new(r"(?mU)\[{2}(?P<id>[a-zA-Z\s\-\d]+)\|{1}(?P<desc>[a-zA-Z\s\-\d_]*)\]{2}")
            .unwrap();

    let substitution = format!("[$desc]({}/$id.html)", host);

    // regex
    //     .replace_all(md, |caps: &Captures| {
    //         let id = &caps["id"];
    //         println!("{}", id);
    //         substitution.clone()
    //     })
    //     .into()
    // let res: String = regex.replace_all(md, substitution).into();
    // println!("{}", res);
    // res

    regex.replace_all(md, substitution).into()
}

fn main() {
    let host = "https://eb.birn.cc";
    let root = "eb";
    let files = discover_files(root);
    let content: Vec<(String, String)> = files
        .into_iter()
        .map(|f| (f.clone(), read_file(f.as_str().clone())))
        .collect();
    // let content = read_file("input.md");

    let dir: HashMap<String, String> =
        content.into_iter().fold(HashMap::new(), |mut acc, (f, c)| {
            acc.insert(f, c);
            acc
        });
    // dir.insert("input.md", &content);

    for (f, v) in dir.iter() {
        println!("{}:", f);
        println!("{}", parse_axons(&v, None, host));
        println!("");
    }
}
