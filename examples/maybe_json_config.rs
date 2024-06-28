use clap::{Args, Parser};
use clap_maybe_deser::{JsonDeserializer, MaybeDeser};
use serde::Deserialize;

#[derive(Args, Deserialize, Debug, Clone)]
struct Config {
    #[clap(long, short)]
    key:   String,
    #[clap(long, short)]
    value: String,
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(flatten)]
    config: MaybeDeser<Config, JsonDeserializer>,
}

fn main() {
    let args = Cli::parse();
    println!("key: {}", args.config.data.key);
    println!("value: {}", args.config.data.value);
}
