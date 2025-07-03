use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use git2::Repository;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use balatro_tui::get_balatro_appdata_dir;
use crate::config::get_data_dir;

#[derive(Default)]
pub struct ModList {
    repo: Option<Repository>,
}

impl ModList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open_mod_list(&mut self) {
        if self.repo.is_none() {
            self.clone_online_mod_list();
        } else {
            let repo = match Repository::open(get_data_dir().display().to_string() + "/mods/") {
                Ok(repo) => {
                    self.repo = Option::from(repo);
                },
                Err(e) => panic!("failed to open: {}", e),
            };
        }
    }

    pub fn clone_online_mod_list(&mut self) {
        let url = "https://github.com/skyline69/balatro-mod-index.git";
        let repo = match Repository::clone(url, get_data_dir().display().to_string() + "/mods/")
        {
            Ok(repo) => {
                self.repo = Option::from(repo);
            },
            Err(e) => panic!("failed to clone: {}", e),
        };
    }

    pub fn get_local_mods(&mut self) -> Vec<Mod> {
        let mod_path = get_balatro_appdata_dir().join("Mods");
        
        let mut mods = vec![];

        for entry in std::fs::read_dir(mod_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if !path.is_dir() { continue; }

            // let mut mod_obj = Mod::new();
            //
            // // mod_obj.name = path.file_name().unwrap().to_str().unwrap().to_string();
            // mod_obj.folder = path;
            //
            // mods.push(mod_obj);

            for file in std::fs::read_dir(&path).unwrap() {
                let file = file.unwrap();
                let path = file.path();
                if !path.is_file() { continue; }
                if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                    if extension != "json" { continue; }
                    if let Some(mut mod_obj) = Mod::from_file(&file.path()) {
                        if mod_obj.id.is_empty() { continue; }
                        mod_obj.enabled = Some(mod_obj.get_enabled());
                        mods.push(mod_obj);
                    }
                }
            }
        }

        mods
    }
}

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct Mod {
    pub id: String,
    pub name: String,
    #[serde(default)] // Not in JSON
    pub folder: PathBuf,
    pub description: String,
    pub version: String,

    #[serde(default)]
    pub author: Vec<String>,

    pub dependencies: Vec<String>,

    #[serde(default)]
    pub url: Option<String>,
    
    #[serde(default)]
    pub enabled: Option<bool>,
}

impl Mod {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_file(path: &Path) -> Option<Self> {
        let file = File::open(path).ok()?;
        let reader = BufReader::new(file);

        let mut loaded_mod: Mod = serde_json::from_reader(reader).ok()?;
        loaded_mod.folder = path.parent()?.to_path_buf();

        Some(loaded_mod)
    }
    
    pub fn get_enabled(&mut self) -> bool {
        let mut enabled = true;
        for entry in std::fs::read_dir(&self.folder).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            
            if let Some(name) = path.file_name().and_then(|e| e.to_str()) {
                if name == ".lovelyignore" {
                    enabled = false;
                    break;
                }
            }
        }

        enabled
    }

    /*
    pub fn populate(&mut self) -> () {
        for entry in std::fs::read_dir(&self.folder).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                let file = File::open(&path).unwrap();
                let reader = BufReader::new(file);

                if extension == "json" {
                    let json: Value = serde_json::from_reader(reader).unwrap();

                    // println!("{:?}", json);

                    if let Some(id) = json.get("id") {
                        self.id = id.as_str().unwrap().parse().unwrap();
                        println!("id: {}", self.id);
                    }
                }
            }
        }
    }
    */
}