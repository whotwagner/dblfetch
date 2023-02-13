use std::fs::File;
use std::fs;
use std::io::prelude::*;
use serde::Deserialize;
use std::path::Path;
use std::env;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    timeout: Option<String>,
    cachedir: Option<String>,
    blockaction: String,
    blacklists: Vec<Dbl>
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Dbl {
    name: String,
    url: String,
    timeout: Option<String>
}

pub fn load_file(file: &str) -> Result<(), serde_yaml::Error>  {
    let mut file = File::open(file).expect("Unable to open file");
    let mut yaml = String::new();

    file.read_to_string(&mut yaml)
        .expect("Unable to read file");

    let ymlconfig: Config = serde_yaml::from_str(&yaml)?;

    let cachedir = match ymlconfig.cachedir {
        Some(ref x) => x,
        None => ".cache/dblfetch"
    };

    let cachedir_path = default_cachedir(cachedir);

    println!("Cache-Dir: {}", cachedir_path);

    fs::create_dir_all(cachedir_path).unwrap_or_else(|e| panic!("Error creating dir: {}", e));



    for dbl in ymlconfig.blacklists {
        println!("Got: {}", dbl.name);
    }

    Ok(())
}

fn default_cachedir(file: &str) -> String {
    let home = "HOME";
    
    let homepath = match env::var(home) {
        Ok(val) => val,
        Err(e) => panic!("Error: could not find {}: {}", home, e),
    };

    let fullpath: String = Path::new(&homepath).join(file).into_os_string().into_string().unwrap();

    fullpath
}
