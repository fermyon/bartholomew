use anyhow::Result;
use commands::{calendar::CalendarCommand, check::CheckCommand, new::NewCommand};
use structopt::{clap::AppSettings, StructOpt};

mod commands;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    BartApp::from_args().run().await
}

/// The Bartholomew CLI
#[derive(Debug, StructOpt)]
#[structopt(
    name = "bart",
    version = env!("CARGO_PKG_VERSION"),
    global_settings = &[
        AppSettings::VersionlessSubcommands,
        AppSettings::ColoredHelp
    ])]
enum BartApp {
    Calendar(CalendarCommand),
    Check(CheckCommand),
    New(NewCommand),
}

impl BartApp {
    /// The main entry point to Bart.
    pub async fn run(self) -> Result<()> {
        match self {
            BartApp::Calendar(cmd) => cmd.run().await,
            BartApp::Check(cmd) => cmd.run().await,
            BartApp::New(cmd) => cmd.run().await,
        }
    }
}
