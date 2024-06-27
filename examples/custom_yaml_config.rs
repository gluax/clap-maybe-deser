use clap::Parser;
use clap_maybe_deser::{CustomDeserializer, Deser};
use serde::{de::DeserializeOwned, Deserialize};

#[derive(Debug, Clone)]
struct YamlDeserializer;

impl CustomDeserializer for YamlDeserializer {
    type Error = serde_yml::Error;

    const NAME: &'static str = "yaml";

    fn from_str<Data: DeserializeOwned>(s: &str) -> Result<Data, Self::Error> {
        serde_yml::from_str(s)
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Config {
    key:   String,
    value: String,
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(long, short)]
    config: Deser<Config, YamlDeserializer>,
}

fn main() {
    let args = Cli::parse();
    println!("key: {}", args.config.data.key);
    println!("value: {}", args.config.data.value);
}
