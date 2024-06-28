use core::fmt;
use std::{marker::PhantomData, str::FromStr};

use clap::{Args, FromArgMatches};
use serde::de::DeserializeOwned;

#[cfg(feature = "serde_json")]
mod serde_json;
#[cfg(feature = "stdin")]
use clap_stdin::MaybeStdin;
#[cfg(feature = "serde_json")]
pub use serde_json::JsonDeserializer;

pub trait CustomDeserializer {
    const NAME: &'static str;
    type Error: fmt::Display;

    fn from_str<Data: DeserializeOwned>(s: &str) -> Result<Data, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct Deser<Data, Deserializer> {
    pub data:      Data,
    _deserializer: PhantomData<Deserializer>,
}

impl<Data, Deserializer> From<Data> for Deser<Data, Deserializer> {
    fn from(data: Data) -> Self {
        Deser {
            data,
            _deserializer: PhantomData,
        }
    }
}

impl<Data, Deserializer> fmt::Display for Deser<Data, Deserializer>
where
    Data: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<Data, Deserializer> FromStr for Deser<Data, Deserializer>
where
    Data: DeserializeOwned,
    Deserializer: CustomDeserializer,
{
    type Err = Deserializer::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = Deserializer::from_str(s)?;
        Ok(Deser {
            data,
            _deserializer: PhantomData,
        })
    }
}

#[derive(Debug)]
pub struct MaybeDeser<Data, Deserializer> {
    pub data:      Data,
    _deserializer: PhantomData<Deserializer>,
}

impl<Data, Deserializer> From<Data> for MaybeDeser<Data, Deserializer> {
    fn from(data: Data) -> Self {
        MaybeDeser {
            data,
            _deserializer: PhantomData,
        }
    }
}

impl<Data, Deserializer> fmt::Display for MaybeDeser<Data, Deserializer>
where
    Data: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<Data, Deserializer> FromArgMatches for MaybeDeser<Data, Deserializer>
where
    Data: DeserializeOwned + Args + Clone + Send + Sync + 'static,
    Deserializer: CustomDeserializer,
{
    fn from_arg_matches(matches: &clap::ArgMatches) -> std::result::Result<Self, clap::Error> {
        if let Some(data_str) = matches.get_one::<String>(Deserializer::NAME) {
            let data: Data = Deserializer::from_str(data_str)
                .map_err(|e: Deserializer::Error| clap::Error::raw(clap::error::ErrorKind::InvalidValue, e))?;
            Ok(Self::from(data))
        } else {
            let fields = Data::from_arg_matches(matches)?;
            Ok(Self::from(fields))
        }
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
        if let Some(data_str) = matches.get_one::<String>(Deserializer::NAME) {
            let data: Data = Deserializer::from_str(data_str).map_err(|e: Deserializer::Error| {
                clap::Error::raw(clap::error::ErrorKind::InvalidValue, e.to_string())
            })?;
            *self = Self::from(data);
        } else {
            *self = Self::from(Data::from_arg_matches(matches)?);
        }
        Ok(())
    }
}

impl<Data, Deserializer> Args for MaybeDeser<Data, Deserializer>
where
    Data: DeserializeOwned + Args + Clone + Send + Sync + 'static,
    Deserializer: CustomDeserializer,
{
    fn augment_args(cmd: clap::Command) -> clap::Command {
        // Create a list of field names dynamically from T
        let field_names = Data::augment_args(clap::Command::new(""))
            .get_arguments()
            .map(|arg| arg.get_id().clone())
            .collect::<Vec<_>>();

        let cmd = cmd.arg(
            clap::Arg::new(Deserializer::NAME)
                .long(Deserializer::NAME)
                .num_args(1)
                .help(format!(
                    "{} input for the request. If this is provided, all other flags will be ignored.",
                    Deserializer::NAME
                ))
                .conflicts_with_all(field_names),
        );
        Data::augment_args(cmd)
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        // Create a list of field names dynamically from T
        let field_names = Data::augment_args_for_update(clap::Command::new(""))
            .get_arguments()
            .map(|arg| arg.get_id().clone())
            .collect::<Vec<_>>();

        let cmd = cmd.arg(
            clap::Arg::new(Deserializer::NAME)
                .long(Deserializer::NAME)
                .num_args(1)
                .help(format!(
                    "{} input for the request. If this is provided, all other flags will be ignored.",
                    Deserializer::NAME
                ))
                .conflicts_with_all(field_names),
        );
        Data::augment_args_for_update(cmd)
    }
}

#[cfg(feature = "stdin")]
#[derive(Debug)]
pub struct MaybeStdinDeser<Data, Deserializer> {
    pub data:      Data,
    _deserializer: PhantomData<Deserializer>,
}

#[cfg(feature = "stdin")]
impl<Data, Deserializer> From<Data> for MaybeStdinDeser<Data, Deserializer> {
    fn from(data: Data) -> Self {
        MaybeStdinDeser {
            data,
            _deserializer: PhantomData,
        }
    }
}

#[cfg(feature = "stdin")]
impl<Data, Deserializer> fmt::Display for MaybeStdinDeser<Data, Deserializer>
where
    Data: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[cfg(feature = "stdin")]
impl<Data, Deserializer> FromArgMatches for MaybeStdinDeser<Data, Deserializer>
where
    Data: DeserializeOwned + Args + Clone + Send + Sync + 'static,
    Deserializer: CustomDeserializer,
{
    fn from_arg_matches(matches: &clap::ArgMatches) -> std::result::Result<Self, clap::Error> {
        if let Some(maybe_stdin) = matches.get_one::<MaybeStdin<String>>(Deserializer::NAME) {
            let data_str = maybe_stdin.as_ref();
            let data: Data = Deserializer::from_str(data_str)
                .map_err(|e: Deserializer::Error| clap::Error::raw(clap::error::ErrorKind::InvalidValue, e))?;
            Ok(Self::from(data))
        } else {
            let fields = Data::from_arg_matches(matches)?;
            Ok(Self::from(fields))
        }
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> std::result::Result<(), clap::Error> {
        if let Some(maybe_stdin) = matches.get_one::<MaybeStdin<String>>(Deserializer::NAME) {
            let data_str = maybe_stdin.as_ref();
            let data: Data = Deserializer::from_str(data_str).map_err(|e: Deserializer::Error| {
                clap::Error::raw(clap::error::ErrorKind::InvalidValue, e.to_string())
            })?;
            *self = Self::from(data);
        } else {
            *self = Self::from(Data::from_arg_matches(matches)?);
        }
        Ok(())
    }
}

#[cfg(feature = "stdin")]
impl<Data, Deserializer> Args for MaybeStdinDeser<Data, Deserializer>
where
    Data: DeserializeOwned + Args + Clone + Send + Sync + 'static,
    Deserializer: CustomDeserializer,
{
    fn augment_args(cmd: clap::Command) -> clap::Command {
        // Create a list of field names dynamically from T
        let field_names = Data::augment_args(clap::Command::new(""))
            .get_arguments()
            .map(|arg| arg.get_id().clone())
            .collect::<Vec<_>>();

        let cmd = cmd.arg(
            clap::Arg::new(Deserializer::NAME)
                .long(Deserializer::NAME)
                .num_args(1)
                .help(format!(
                    "{} input for the request. If this is provided, all other flags will be ignored.",
                    Deserializer::NAME
                ))
                .value_parser(MaybeStdin::<String>::from_str)
                .conflicts_with_all(field_names),
        );
        Data::augment_args(cmd)
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        // Create a list of field names dynamically from T
        let field_names = Data::augment_args_for_update(clap::Command::new(""))
            .get_arguments()
            .map(|arg| arg.get_id().clone())
            .collect::<Vec<_>>();

        let cmd = cmd.arg(
            clap::Arg::new(Deserializer::NAME)
                .long(Deserializer::NAME)
                .num_args(1)
                .help(format!(
                    "{} input for the request. If this is provided, all other flags will be ignored.",
                    Deserializer::NAME
                ))
                .value_parser(MaybeStdin::<String>::from_str)
                .conflicts_with_all(field_names),
        );
        Data::augment_args_for_update(cmd)
    }
}
