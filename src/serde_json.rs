use serde::de::DeserializeOwned;

use crate::CustomDeserializer;

#[derive(Debug, Clone)]
pub struct JsonDeserializer;

impl CustomDeserializer for JsonDeserializer {
    type Error = serde_json::Error;

    const NAME: &'static str = "json";

    fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, Self::Error> {
        serde_json::from_str(s)
    }
}

#[cfg(test)]
mod test {
    use clap::{Args, Parser};
    use serde::Deserialize;

    use super::*;
    use crate::{Deser, MaybeDeser};

    #[derive(Deserialize, Debug, Args, Clone)]
    struct Foo {
        /// A flag for foo
        #[clap(long)]
        bar: u8,
        /// A flag for baz
        #[clap(long)]
        baz: String,
    }

    #[derive(Parser)]
    struct AppDeser {
        #[clap(long)]
        deser: Deser<Foo, JsonDeserializer>,
    }

    #[derive(Parser)]
    struct AppMaybeDeser {
        #[clap(flatten)]
        maybe_deser: MaybeDeser<Foo, JsonDeserializer>,
    }

    #[test]
    fn json_deser() {
        let args = AppDeser::try_parse_from(["test", "--deser", r#"{"bar": 1, "baz": "hello"}"#]).unwrap();
        assert_eq!(args.deser.data.bar, 1);
        assert_eq!(args.deser.data.baz, "hello");
    }

    #[test]
    fn json_maybe_deser() {
        // Test with json
        let args = AppMaybeDeser::try_parse_from(["test", "--json", r#"{"bar": 1, "baz": "hello"}"#]).unwrap();

        assert_eq!(args.maybe_deser.data.bar, 1);
        assert_eq!(args.maybe_deser.data.baz, "hello");

        // Test with fields
        let args = AppMaybeDeser::try_parse_from(["test", "--bar", "1", "--baz", "hello"]).unwrap();
        assert_eq!(args.maybe_deser.data.bar, 1);
        assert_eq!(args.maybe_deser.data.baz, "hello");
    }

    #[test]
    #[should_panic = "ArgumentConflict"]
    fn deser_cant_have_both() {
        AppMaybeDeser::try_parse_from([
            "test",
            "--json",
            r#"{"bar": 1, "baz": "hello"}"#,
            "--bar",
            "1",
            "--baz",
            "hello",
        ])
        .unwrap();
    }
}
