extern crate simplelog;
use clap::Parser;
use simplelog::*;

pub mod deserialize;

#[derive(Parser, Debug)]
#[command(author = "Wolfgang Hotwagner", version = "0.1", about, long_about = None)]
struct Args {
    #[arg(short = 'c', long, default_value = "./dblfetch.yaml")]
    /// Use this config-file
    config: Option<String>
}

fn main() {
    let _args = Args::parse();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
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
