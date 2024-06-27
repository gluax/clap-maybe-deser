# clap-maybe-deser

Provides wrapper types to allow:
- Parse `serde` deserializable objects as a flag via the `Deser` type for `clap`.
- You can also have your app parse either the flags or the deserializable object by using the `MaybeDeser` type.
- And with the `MaybeStdinDeser` type you can do the above, but the deserializable object can come from `stdin` via the `clap-stdin` crate.
- Also exposes the `CustomDeserializer` trait so you can implement with your own `Deserialize` type.

[![Crates.io](https://img.shields.io/crates/v/clap-maybe-deser?style=flat-square)](https://crates.io/crates/clap-maybe-deser)
[![API Reference](https://img.shields.io/docsrs/clap-maybe-deser?style=flat-square)](https://docs.rs/clap-maybe-deser)

## Usage

### `Deser`

To parse a serde deserializable object as a flag:
```rust
use clap::Parser;
use serde::Deserialize;
use clap_maybe_deser::Deser;

#[derive(Deserialize, Debug)]
struct Config {
    key: String,
    value: String,
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(flatten)]
    config: Deser<Config, JsonDeserializer>,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args.config.data);
}
```

### `MaybeDeser`

To parse either flags or a deserializable object:

```rust
use clap::Parser;
use serde::Deserialize;
use clap_maybe_deser::MaybeDeser;

#[derive(Deserialize, Debug, clap::Args)]
struct Config {
    key: String,
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
        MaybeDeser::Data(deser) => println!("{:?}", deser.data),
        MaybeDeser::Fields(fields) => println!("{:?}", fields),
    }
}
```

### `MaybeStdinDeser`

To parse a deserializable object from maybe stdin or flags:

```rust
use clap::Parser;
use serde::Deserialize;
use clap_maybe_deser::MaybeStdinDeser;


#[derive(Deserialize, Debug)]
struct Config {
    key: String,
    value: String,
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(flatten)]
    config: MaybeStdinDeser<Config, JsonDeserializer>,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args.config.data);
}
```

### Custom Implmentations

To support whatever Deserialize friendly implementation you want you can do:

```rust
use serde::Deserialize;
use serde_yaml;
use std::fmt;
use std::error::Error;
use clap_maybe_deser::CustomDeserializer;

// Implement the trait for YAML deserialization
struct YamlDeserializer;

impl CustomDeserializer for YamlDeserializer {
    const NAME: &'static str = "yaml";
    type Error = serde_yaml::Error;

    fn from_str<Data: DeserializeOwned>(s: &str) -> Result<Data, Self::Error> {
        serde_yaml::from_str(s)
    }
}

// Example usage
#[derive(Deserialize, Debug)]
struct Config {
    key: String,
    value: String,
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(flatten)]
    config: Deser<Config, YamlDeserializer>,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args.config.data);
}
```

## TODO's

- [ ] Support more serde crates out of the box.
- [ ] Dynamic naming of the flag for `MaybeDeser` and `MaybeStdinDeser`.

## Licensing

This project is licensed under both the MIT License and the Apache 2.0 License. See the LICENSE-MIT and LICENSE-APACHE files for details.

This project includes dependencies that are licensed under permissive licenses:

- `clap`: [MIT License or Apache 2.0 License](https://github.com/clap-rs/clap/blob/master/LICENSE-MIT)
- `clap-stdin`: [MIT License or Apache 2.0 License](https://github.com/thepacketgeek/clap-stdin/blob/main/LICENSE-MIT)
- `serde`: [MIT License or Apache 2.0 License](https://github.com/serde-rs/serde/blob/master/LICENSE-MIT)
- `serde_json`: [MIT License or Apache 2.0 License](https://github.com/serde-rs/json/blob/master/LICENSE-MIT)
