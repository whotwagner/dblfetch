use std::fs::File;
use std::fs;
use std::io::prelude::*;
use serde::Deserialize;
use std::path::Path;
use std::env;

extern crate simplelog;

use simplelog::*;

pub mod downlister;

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
    let cp = cachedir_path.clone();

    debug!("Cache-Dir: {}", cachedir_path);

    fs::create_dir_all(cp).unwrap_or_else(|e| panic!("Error creating dir: {}", e));

    for dbl in ymlconfig.blacklists {
        debug!("Got: {}", dbl.name);
        let timeout = match dbl.timeout {
            Some(x) => x,
            None => "24h".to_string()
        };
        downlister::download(dbl.name, dbl.url, &cachedir_path, &timeout);
    }

    Ok(())
}

fn default_cachedir(file: &str) -> String {
    let homepath = match env::var("HOME") {
        Ok(val) => val,
        Err(e) => panic!("Error: could not find {}: {}", "HOME", e),
    };

    let fullpath: String = Path::new(&homepath).join(file).into_os_string().into_string().unwrap();

    fullpath
}
