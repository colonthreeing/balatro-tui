pub mod motd;

use std::process::Stdio;
use log::error;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command};
use std::thread;
use home::home_dir;
use platform_dirs::AppDirs;

pub fn launch_balatro(disable_console: bool) -> Result<Child, std::io::Error> {
    if disable_console {
        Command::new("steam")
            .arg("-applaunch")
            .arg("2379780")
            .arg("--disable-console")
            .spawn()
    } else {
        Command::new("steam")
            .arg("-applaunch")
            .arg("2379780")
            .spawn()
    }
}

pub fn xdg_open(path: &str) {
    let mut child = Command::new("xdg-open")
        .arg(path)
        .stderr(Stdio::piped())
        .spawn().expect("failed to execute process xdg-open");
    
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        thread::spawn(move || {
            for line in reader.lines() {
                if let Ok(line) = line {
                    error!("xdg-open error: {}", line);
                }
            }
        });
    }
}

pub fn locate_steam_appdata() -> Option<AppDirs> {
    AppDirs::new(Some("Steam"), false)
}

pub fn get_balatro_dir() -> PathBuf {
    let mut path = locate_steam_appdata().expect("failed to locate steam").data_dir;
    
    path.extend([
        "steamapps", "common", "Balatro"
    ]);
    
    path
}

pub fn get_balatro_appdata_dir() -> PathBuf {
    
    #[cfg(target_os = "linux")]
    {
        let steam = locate_steam_appdata().expect("failed to locate steam");
        let mut path = steam.data_dir;
        path.extend([
            "steamapps", "compatdata", "2379780", "pfx", "drive_c",
            "users", "steamuser", "AppData", "Roaming", "Balatro"
        ]);
        
        return path;
    }
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        //! UNTESTED
        let balatro = AppDirs::new(Some("Balatro"), false).expect("failed to locate balatro");
        balatro.config_dir
    }
}