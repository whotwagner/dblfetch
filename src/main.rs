use clap::Parser;

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

    match _args.config {
        Some(x) => {
            match deserialize::load_file(x.as_str()) {
                Ok(s) => s,
                Err(e) => println!("Error: {}", e)
            };
        },
        None => println!("Crap! Not again.")
    }
}


