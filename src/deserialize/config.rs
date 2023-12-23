use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
pub    timeout: Option<String>,
pub    cachedir: Option<String>,
pub    blockaction: String,
pub    blockaction_v6: Option<String>,
pub    blacklists: Vec<Dbl>
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Dbl {
pub    name: String,
pub    url: String,
pub    timeout: Option<String>
}
