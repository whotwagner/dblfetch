use reqwest;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn download(name: String, url: String, cachedir: &String) {
    println!("download...{}", name);
    let filepath = Path::new(cachedir).join(name);
    let mut result = reqwest::blocking::get(url).unwrap();
    let mut output = File::create(filepath).expect("whoopsi");
    write!(output, "{}", result.text().unwrap());
}
