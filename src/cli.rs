use clap::{Parser, ValueEnum};

#[derive(Clone, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(long, value_enum, default_value = "pattern")]
    pub alg: ResponseAlgorithm,
    #[arg(long, default_value = "Hello World")]
    pub text: String,
}

#[derive(Clone, PartialEq, Eq, Debug, ValueEnum)]
pub enum ResponseAlgorithm {
    /// Respond with the hex value 0x55 to be easily recognizable
    Pattern,
    /// Respond with the current time and the remote address of the client
    TimeAndPlace,
    /// Respond with the text given via --text
    Text,
}
