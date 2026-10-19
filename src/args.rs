use clap::Parser;
use languagetool_rust::api::check::Level;
use serde::Deserialize;

#[derive(Parser, Deserialize, Debug, Default)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None
)]
#[serde(deny_unknown_fields)]
pub struct Args {
    /// LanguageTool server url
    #[arg(short, long, value_name = "URL")]
    pub url: Option<String>,

    /// LanguageTool server port
    #[arg(short, long, value_name = "PORT")]
    pub port: Option<u16>,

    #[arg(long)]
    pub picky: Option<bool>,
}

impl Args {
    pub fn or(self, optb: Args) -> Args {
        Args {
            url: self.url.or(optb.url),
            port: self.port.or(optb.port),
            picky: self.picky.or(optb.picky),
        }
    }
}

pub struct Settings {
    pub host: String,
    pub port: u16,
    pub level: Level,
}

impl Settings {
    /// applies defaults for None values
    pub fn new(value: Args) -> Self {
        Settings {
            host: value.url.unwrap_or(String::from("http://127.0.0.1")),
            port: value.port.unwrap_or(8081),
            level: value.picky.map_or(Level::Picky, |f| match f {
                true => Level::Picky,
                false => Level::Default,
            }),
        }
    }
}
