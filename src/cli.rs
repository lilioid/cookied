use clap::{Parser, ValueEnum};

/// An RFC865 quote-of-the-day server
///
/// Note that the server can only be started using socket activation.
/// This could be achieved using e.g. systemd socket activation or a wrapper program like systemfd.
#[derive(Clone, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// Which algorithm to use for response generation
    #[arg(long, value_enum, default_value = "time-and-place")]
    pub alg: ResponseAlgorithm,

    /// Use this text as a response for --alg=text
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
