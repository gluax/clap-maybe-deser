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
// You can run this example with: ` cargo run --features serde_json --example json_config --`
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

```

The help output looks like:
![Json Config Help Example](https://github.com/gluax/clap-maybe-deser/blob/main/screenshots/deser_json_config_help.png)

The usage looks like:
![Json Config Use Example](https://github.com/gluax/clap-maybe-deser/blob/main/screenshots/deser_json_config.png)


### `MaybeDeser`

To parse either flags or a deserializable object:

```rust
// You can run this example with: ` cargo run --features serde_json --example maybe_json_config --`
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
```

The help output looks like:
![Mayble Json Config Help Example](https://github.com/gluax/clap-maybe-deser/blob/main/screenshots/maybe_deser_json_config_help.png)

The usage passing json looks like:
![Maybe Json Config Json Use Example](https://github.com/gluax/clap-maybe-deser/blob/main/screenshots/maybe_deser_json_config_json.png)

The usage passing flags looks like:
![Maybe Json Config Flags Use Example](https://github.com/gluax/clap-maybe-deser/blob/main/screenshots/maybe_deser_json_config_flags.png)



### `MaybeStdinDeser`

To parse a deserializable object from maybe stdin or flags:

```rust
// You can run this example with: `cargo run --features serde_json,stdin --example maybe_stdin_json_config --`
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

```

The output and usage methods are the exact same as above but now you can pass in the `JSON` from `stdin`:
![Maybe Json Config Json Stdin Use Example](https://github.com/gluax/clap-maybe-deser/blob/main/screenshots/maybe_stdin_deser_json_config.png)


### Custom Implmentations

To support whatever Deserialize friendly implementation you want you can do:

```rust
// You can run this example with: `cargo run --example custom_yaml_config --`
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

```

You can see this in action as well:
![Custom Yaml Config Use Example](https://github.com/gluax/clap-maybe-deser/blob/main/screenshots/custom_yaml_config.png)


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
