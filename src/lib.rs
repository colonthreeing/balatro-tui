pub mod motd;

use std::process::Stdio;
use log::error;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command};
use std::thread;
use home::home_dir;

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

pub fn get_balatro_dir() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.extend([
        ".local", "share", "Steam", "steamapps", "common", "Balatro"
    ]);
    path
}

pub fn get_balatro_appdata_dir() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.extend([
        ".local", "share", "Steam", "steamapps", "compatdata", "2379780",
        "pfx", "drive_c", "users", "steamuser", "AppData", "Roaming", "Balatro"
    ]);
    path
}