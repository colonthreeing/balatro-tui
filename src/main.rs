use std::fs::DirEntry;
use std::{fs, io};
use std::path::Path;
use clap::Parser;
use cli::Cli;
use color_eyre::Result;

use crate::app::App;
use crate::config::get_data_dir;
use crate::mods::{ModList};

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
    crate::logging::init()?;

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;

    Ok(())
}
