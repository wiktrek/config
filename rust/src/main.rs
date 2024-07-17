use std::{fs, str::FromStr};
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
    let mut paths: Vec<String> = vec!["../".to_string()];
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

    println!("{}{}", selection, vec.len());
    if selection == 0{
        let replaced_path = cfg.path.clone().replace("//", "/");
        let mut p = replaced_path.split("/").collect::<Vec<&str>>();
        p.pop();
        if p.len() == 1 {
            return list_projects(load_config())
        }
        return list_projects(Config {
            path: p.join("/"),
            editor: cfg.editor,
        })
    } else {
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
}
fn open_project(cfg: Config) {
    Command::new("cmd").args(["/C",&format!("{} {}", cfg.editor, cfg.path.replace("/", "\\"))]).status().expect("err");
}
fn open_explorer(path: String){
    open::that(path).unwrap();
}
fn load_config() -> Config {
    serde_json::from_str(&fs::read_to_string("config.json").expect("Error reading config.json")).unwrap()
}