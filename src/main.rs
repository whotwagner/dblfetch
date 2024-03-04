extern crate simplelog;
use clap::Parser;
use simplelog::*;
use std::fs::File;

pub mod deserialize;

#[derive(Parser, Debug)]
#[command(author = "Wolfgang Hotwagner", version = "0.5", about, long_about = None)]
struct Args {
    #[arg(short = 'c', long, default_value = "/etc/dblfetch.yaml")]
    /// Use this config-file
    config: Option<String>
}

fn main() {
    let _args = Args::parse();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Error, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("/var/log/dblfetch.log").unwrap())
        ]
    ).unwrap();

    match _args.config {
        Some(x) => {
            match deserialize::load_file(x.as_str()) {
                Ok(s) => s,
                Err(e) => error!("Error: {}", e)
            };
        },
        None => error!("Crap! Not again.")
    }
}
