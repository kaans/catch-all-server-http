use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, version, author)]
pub(crate) struct Args {
    #[arg(short, long, env = "PORT", default_value = "8092")]
    pub port: u16,
    #[arg(short = 'o', long, env = "HOST", default_value = "0.0.0.0")]
    pub host: String,
    #[arg(short, long, env = "USE_COLOR", default_value = "true")]
    pub use_color: bool,
}