use std::str::FromStr;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Default, Debug, Clone)]
pub(crate) enum BodyFormat {
    #[default]
    Text,
    Base64,
    Hex,
}

impl FromStr for BodyFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "base64" => Ok(BodyFormat::Base64),
            "hex" => Ok(BodyFormat::Hex),
            "text" | _ => Ok(BodyFormat::Text),
        }
    }
}

#[derive(Parser, Clone, Debug)]
#[command(about, version, author)]
pub(crate) struct Args {
    #[arg(short, long, env = "PORT", default_value = "8092")]
    pub port: u16,
    #[arg(short = 'o', long, env = "HOST", default_value = "0.0.0.0")]
    pub host: String,
    #[arg(short, long, env = "USE_COLOR", default_value = "true")]
    pub use_color: bool,
    #[arg(short, long, env = "MAX_SIZE", default_value = "262144")]
    pub max_size: usize,
    #[arg(short, long, env = "BODY_FORMAT", default_value = "text")]
    pub body_format: BodyFormat,
}