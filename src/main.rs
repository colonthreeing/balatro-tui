use tokio::io::{AsyncBufReadExt, BufReader};
use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use log::info;
use crate::app::App;
use balatro_tui::motd::motd;

mod action;
mod app;
mod cli;
mod components;
mod config;
mod errors;
mod logging;
mod tui;
mod mods;

#[tokio::main]
async fn main() -> Result<()> {
    crate::errors::init()?;

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;

    // Set max_log_level to Info
    tui_logger::init_logger(log::LevelFilter::Trace)?;

    // Set default level for unknown targets to Info
    tui_logger::set_default_level(log::LevelFilter::Info);
    let config = config::Config::new()?;

    info!("{}", motd());

    app.run().await?;

    Ok(())
}
