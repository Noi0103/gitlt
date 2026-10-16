use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None
)]
pub struct Cli {
    /// LanguageTool server hostname
    #[arg(short, long, value_name = "HOSTNAME", default_value_t = String::from("127.0.0.1"))]
    pub address: String,

    /// LanguageTool server port
    #[arg(short, long, value_name = "PORT", default_value_t = String::from("8081"))]
    pub port: String,

    #[arg(long, default_value_t = true)]
    pub picky: bool,
}
