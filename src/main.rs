use tokio::io::{AsyncBufReadExt, BufReader};
use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use log::info;
use balatro_tui::{clone_online_mod_list, get_repo_at, update_repo};
use crate::app::App;
use balatro_tui::motd::motd;
use crate::config::get_data_dir;

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

    if let Some(repo) = get_repo_at(&get_data_dir().join("mods")) {
        println!("Balatro-tui is updating the mod list, please wait...");
        update_repo(&repo).expect("Failed to update repository.");
    } else {
        println!("Balatro-tui is downloading the mod list, please wait...");
        clone_online_mod_list(get_data_dir().join("mods")).expect("Failed to download mod list.");
    }

    app.run().await?;

    Ok(())
}
