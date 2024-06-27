use serde::de::DeserializeOwned;

use crate::CustomDeserializer;

#[derive(Debug, Clone)]
pub struct JsonDeserializer;

impl CustomDeserializer for JsonDeserializer {
    const NAME: &'static str = "json";
    type Error = serde_json::Error;

    fn from_str<T: DeserializeOwned>(s: &str) -> Result<T, Self::Error> {
        serde_json::from_str(s)
    }
}

#[cfg(test)]
mod test {
    use clap::{Args, Parser};
    use serde::Deserialize;

    use crate::{Deser, MaybeDeser};

    use super::*;

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
        let args =
            AppDeser::try_parse_from(["test", "--deser", r#"{"bar": 1, "baz": "hello"}"#]).unwrap();
        assert_eq!(args.deser.data.bar, 1);
        assert_eq!(args.deser.data.baz, "hello");
    }

    #[test]
    fn json_maybe_deser() {
        // Test with json
        let args =
            AppMaybeDeser::try_parse_from(["test", "--json", r#"{"bar": 1, "baz": "hello"}"#])
                .unwrap();
        if let MaybeDeser::Data(deser) = args.maybe_deser {
            assert_eq!(deser.data.bar, 1);
            assert_eq!(deser.data.baz, "hello");
        } else {
            panic!("Expected MaybeDeser::Data, got {:?}", args.maybe_deser);
        }

        // Test with fields
        let args = AppMaybeDeser::try_parse_from(["test", "--bar", "1", "--baz", "hello"]).unwrap();
        if let MaybeDeser::Fields(fields) = args.maybe_deser {
            assert_eq!(fields.bar, 1);
            assert_eq!(fields.baz, "hello");
        } else {
            panic!("Expected MaybeDeser::Fields, got {:?}", args.maybe_deser);
        }
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
