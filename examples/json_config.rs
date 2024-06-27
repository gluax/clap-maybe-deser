use clap::Parser;
use clap_maybe_deser::{Deser, JsonDeserializer};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Config {
    key:   String,
    value: String,
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(long, short)]
    config: Deser<Config, JsonDeserializer>,
}

fn main() {
    let args = Cli::parse();
    println!("key: {}", args.config.data.key);
    println!("value: {}", args.config.data.value);
}
