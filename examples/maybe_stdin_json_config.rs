use clap::{Args, Parser};
use clap_maybe_deser::{JsonDeserializer, MaybeStdinDeser};
use serde::Deserialize;

#[derive(Args, Deserialize, Debug, Clone)]
struct Config {
    key:   String,
    value: String,
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(flatten)]
    config: MaybeStdinDeser<Config, JsonDeserializer>,
}

fn main() {
    let args = Cli::parse();
    match args.config {
        MaybeStdinDeser::Data(config) => {
            println!("key from json: {}", config.data.key);
            println!("value  from json: {}", config.data.value);
        }
        MaybeStdinDeser::Fields(fields) => {
            println!("key from fields: {}", fields.key);
            println!("value  from fields: {}", fields.value);
        }
    }
}
