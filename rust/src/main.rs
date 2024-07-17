use std::{fs, path::PathBuf};
use console::style;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::path::Path;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
struct Config {
    path: String,
}
fn main() {
    println!("Loading config...");
    let cfg = load_config();
    println!("Config loaded successfully");
    list_projects(cfg);
}
fn load_projects(cfg: Config) -> Vec<String> {
    let a = fs::read_dir(cfg.path.clone()).unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Vec<_>>();
    let mut paths: Vec<String> = vec![];
    for i in a {
        let p = i.unwrap();
        if Path::new(&p).is_dir() {
            paths.push(p.to_str().unwrap().to_string().replace(&cfg.path, "").replace("\\", ""));
        }
        
    }
    paths
}
fn list_projects(cfg: Config) {
    let vec = load_projects(cfg);
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick")
        .default(0)
        .items(&vec[..])
        .interact()
        .unwrap();
    println!("Enjoy your {}!", vec[selection]);
}
fn load_config() -> Config {
    serde_json::from_str(&fs::read_to_string("config.json").expect("Error reading config.json")).unwrap()
}