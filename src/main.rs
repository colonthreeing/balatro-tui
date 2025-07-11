use tokio::io::{AsyncBufReadExt, BufReader};
use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use log::info;
use balatro_tui::{clone_online_mod_list, download_to_tmp, get_balatro_appdata_dir, get_balatro_dir, get_repo_at, open, unzip, update_repo};
use crate::app::App;
use balatro_tui::motd::motd;
use crate::config::get_data_dir;
use crate::mods::ModList;

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

    // let mut temp_file = download_to_tmp("https://github.com/colonthreeing/SealSealBalatro/releases/download/1.1.0/SealSeal.zip").await;
    // 
    // let file = temp_file.as_file();
    // 
    // unzip(file, &get_balatro_appdata_dir().join("Mods"), "SealSeal");
    
    app.run().await?;
    
    Ok(())
}
