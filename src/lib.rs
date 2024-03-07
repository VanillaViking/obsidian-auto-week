use std::io::Error;
use std::{fs, path::PathBuf };
use chrono::prelude::*;
use chrono::Duration;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Add;

pub struct Config {
    pub command: String,
    pub vault_dir: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Canvas {
    pub nodes: Vec<Node>
}

impl Add for Canvas {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.nodes.clone();
        result.append(&mut rhs.nodes.to_owned());
        Self {
            nodes: result
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Node {
    id: String,
    #[serde(rename = "type")]
    node_type: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    #[serde(default)]
    label: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    color: String,
}

impl Default for Config {
    fn default() -> Config {
        Config { command: String::from("new"), vault_dir: PathBuf::from("/home/vanilla/Documents/vault") }
    }
}


impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        
        let command = match args.next() {
            Some(value) => value,
            None => return Err("no command provided"),
        };
        
        Ok(Config{ command, ..Default::default() })
    }
}

pub fn new_week(config: Config) -> Result<(), Error> {

    let end_of_week = Local::now();
    let start_of_week = end_of_week - Duration::days(6);

    let current_file = config.vault_dir.join("WEEK.canvas");
    let current_list_file = config.vault_dir.join("weekly.md");

    let archive_dir = config.vault_dir.join(format!(".archive/{} - {}/", start_of_week.format("%Y.%m.%d"), end_of_week.format("%Y.%m.%d")));
    fs::create_dir_all(&archive_dir)?;
    
    // CANVAS
    let template = config.vault_dir.join(".template/WEEK.canvas");
    let mut backlog = get_canvas(config.vault_dir.join("WEEK.canvas"))?;
    backlog.nodes.retain(|n| {
        n.node_type == "text" && n.x < -1300
    });

    fs::rename(&current_file, archive_dir.join("WEEK.canvas"))?;
    fs::copy(template, current_file)?;

    let mut new_canvas = get_canvas(config.vault_dir.join("WEEK.canvas"))?;
    new_canvas = new_canvas + backlog;
    let canvas_str = serde_json::to_string(&new_canvas)?;
    fs::write(config.vault_dir.join("WEEK.canvas"), canvas_str).expect("Unable to write new canvas");
    
    // LIST
    let unchecked_items: String = fs::read_to_string(&current_list_file)?.lines()
                                    .filter(|line| line.contains("- [ ]"))
                                    .map(|line| line.to_owned() + "\n").collect();
    let list_template: String = fs::read_to_string(config.vault_dir.join(".template/template.md"))?;
    fs::rename(&current_list_file, archive_dir.join("weekly.md"))?;
    fs::write(current_list_file, list_template + &unchecked_items)?;

    Ok(())
}

fn get_canvas(path: PathBuf) -> Result<Canvas, Error> {
    let canvas_str = fs::read_to_string(path)?;

    let canvas: Canvas = serde_json::from_str(&canvas_str)?;

    Ok(canvas)
}

pub fn sort_checklist(config: Config) -> Result<(), Error> {
    let checklist_file = config.vault_dir.join("weekly.md");

    let unchecked_items: String = fs::read_to_string(&checklist_file)?.lines()
                                    .filter(|line| line.contains("- [ ]"))
                                    .map(|line| line.to_owned() + "\n").collect();

    let checked_items: String = fs::read_to_string(&checklist_file)?.lines()
                                    .filter(|line| line.contains("- [x]"))
                                    .map(|line| line.to_owned() + "\n").collect();

    fs::write(checklist_file, unchecked_items + &checked_items)?;

    Ok(())

}

