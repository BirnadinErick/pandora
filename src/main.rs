use glob::glob;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone)]
struct Meta {
    id: String,
    title: String,
    author: String,
    tags: Vec<String>,
    loc: String,
}

impl Meta {
    fn new(loc: String) -> Self {
        Self {
            id: String::from("enigma-bits"),
            title: String::from("Enigma Bits"),
            author: String::from("Birnadin Erick<me@birn.cc>"),
            tags: vec![String::from("eb-birn-cc")],
            loc,
        }
    }

    fn add_id(self, id: String) -> Self {
        Self { id, ..self }
    }

    fn add_title(self, title: String) -> Self {
        Self { title, ..self }
    }

    fn add_author(self, author: String) -> Self {
        Self { author, ..self }
    }

    fn add_tag(self, tag: String) -> Self {
        let tags = vec![self.tags, vec![tag]];
        Self {
            tags: tags.into_iter().flatten().collect::<Vec<String>>(),
            ..self
        }
    }

    fn add_tags(self, tags: Vec<String>) -> Self {
        Self { tags, ..self }
    }

    fn parse_meta(f: String, v: String) -> Self {
        let m = Self::new(f);

        v.lines()
            .map(|l| l.split(": ").collect_tuple::<(&str, &str)>().unwrap())
            .fold(m, |acc, (mkey, mval)| match mkey {
                "id" => acc.add_id(mval.to_string()),
                "title" => acc.add_title(mval.to_string()),
                "author" => acc.add_author(mval.to_string()),
                "tags" => acc.add_tags(
                    mval.split(", ")
                        .map(|t| String::from(t))
                        .collect::<Vec<String>>(),
                ),
                _ => acc,
            })
    }
}

#[derive(Debug, Clone)]
struct Node {
    meta: Meta,
    content: String,
}

impl Node {
    fn new(f: String, c: String) -> Self {
        let (meta, content) = c.split("===").collect_tuple().unwrap();
        let meta = Meta::parse_meta(f.clone(), meta.to_string());

        Self {
            meta,
            content: content.to_string(),
        }
    }
}

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

fn parse_axons(md: &String, _map: Option<()>, host: &str) -> String {
    let regex =
        Regex::new(r"(?mU)\[{2}(?P<id>[a-zA-Z\s\-\d]+)\|{1}(?P<desc>[a-zA-Z\s\-\d_]*)\]{2}")
            .unwrap();

    let substitution = format!("[$desc]({}/$id.html)", host);

    regex.replace_all(md.as_str(), substitution).into()
}

fn main() {
    let host = "https://eb.birn.cc";
    let root = "eb";
    let files = discover_files(root);
    let content: Vec<(String, String)> = files
        .into_iter()
        .map(|f| (f.clone(), read_file(f.as_str().clone())))
        .collect();

    let dir: HashMap<String, Node> = content.into_iter().fold(HashMap::new(), |mut acc, (f, c)| {
        acc.insert(f.clone(), Node::new(f, c));
        acc
    });

    for (k, v) in dir.iter() {
        println!(
            "file: {}\n\tmeta: {:?}\n\tcontent: {}",
            k,
            v.meta,
            parse_axons(&v.content, None, host)
        );
    }
}
