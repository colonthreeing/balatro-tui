pub mod motd;

use tokio::process::{Child, Command};

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

pub fn xdg_open(path: &str) -> Result<Child, std::io::Error> {
    Command::new("xdg-open")
        .arg(path)
        .spawn()
}