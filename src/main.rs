use regex::{Captures, Regex};

fn main() {
    let host = "https://eb.birn.cc";
    let regex =
        Regex::new(r"(?mU)\[{2}(?P<id>[a-zA-Z\s\-\d]+)\|{1}(?P<desc>[a-zA-Z\s\-\d_]*)\]{2}")
            .unwrap();
    let string = "I really love [[unix-phil|UNIX philosophy]], because it let me create software for complex scenarios also making them [[scalable-softwares|Scalable]].I also want to remind that I am about to finish my [[100-days|100 Days of Code Challenge]] coming August 17, 2023.    - item0    - item1";

    let substitution = format!("[$desc]({}/$id.html)", host);

    // result will be a String with the substituted value
    let result = regex.replace_all(string, |caps: &Captures| {
        let id = &caps["id"];
        println!("{}", id);
        substitution.clone()
    });

    println!("{}", result);
}
