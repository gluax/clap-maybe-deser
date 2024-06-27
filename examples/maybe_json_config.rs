use clap::{Args, Parser};
use clap_maybe_deser::{JsonDeserializer, MaybeDeser};
use serde::Deserialize;

#[derive(Args, Deserialize, Debug, Clone)]
struct Config {
    key:   String,
    value: String,
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(flatten)]
    config: MaybeDeser<Config, JsonDeserializer>,
}

fn main() {
    let args = Cli::parse();
    match args.config {
        MaybeDeser::Data(config) => {
            println!("key from json: {}", config.data.key);
            println!("value  from json: {}", config.data.value);
        }
        MaybeDeser::Fields(fields) => {
            println!("key from fields: {}", fields.key);
            println!("value  from fields: {}", fields.value);
        }
    }
}
