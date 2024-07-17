use std::fs;
// use console::style;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::path::Path;
use serde::{Deserialize, Serialize};
use std::process::Command;
#[derive(Deserialize, Serialize)]
struct Config {
    path: String,
    editor: String,
}
fn main() {
    println!("Loading config...");
    let cfg = load_config();
    println!("Config loaded successfully");
    list_projects(cfg);
}
fn load_projects(cfg: &Config) -> Vec<String> {
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
    let vec = load_projects(&cfg);
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick")
        .default(0)
        .items(&vec[..])
        .interact()
        .unwrap();
    let open_or_browse = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("")
        .default(0)
        .items(&["editor", "browse", "file explorer"])
        .interact()
        .unwrap();
    match open_or_browse {
        0 => open_project(Config {
            path: format!("{}/{}", cfg.path, vec[selection]),
            editor: cfg.editor,
        }),
        1 => list_projects(Config {
            path: format!("{}//{}", cfg.path,vec[selection]),
            editor: cfg.editor,
        }),
        2 => open_explorer(format!("{}/{}", cfg.path, vec[selection])),
        _ => println!("Error")
    }
}
fn open_project(cfg: Config) {
    // println!("{}", &format!("{} {}", cfg.editor, cfg.path.replace("/", "\\")));
    Command::new("cmd").args(["/C",&format!("{} {}", cfg.editor, cfg.path.replace("/", "\\"))]).status().expect("err");
}
fn open_explorer(path: String){
    open::that(path).unwrap();
}
fn load_config() -> Config {
    serde_json::from_str(&fs::read_to_string("config.json").expect("Error reading config.json")).unwrap()
}