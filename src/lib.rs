use std::io::Error;
use std::{fs, path::PathBuf };
use chrono::prelude::*;
use chrono::Duration;
use serde::Deserialize;

pub struct Config {
    pub command: String,
    pub vault_dir: PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Canvas {
    pub nodes: Vec<Node>
}


#[derive(Deserialize, Debug)]
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
    let archive_dir = config.vault_dir.join(format!("archive/{} - {}/", start_of_week.format("%Y.%m.%d"), end_of_week.format("%Y.%m.%d")));
    let template = config.vault_dir.join("template/WEEK.canvas");

    // let board = serde_json::to_string().unwrap();

    fs::create_dir_all(&archive_dir)?;
    
    // TODO: copy all the tickets in backlog
    
    fs::rename(&current_file, archive_dir.join("WEEK.canvas"))?;
    fs::copy(template, current_file)?;

    Ok(())
}

fn get_canvas(config: Config) -> Result<Canvas, Error> {
    let canvas_str = fs::read_to_string(config.vault_dir.join("WEEK.canvas"))?;

    let canvas: Canvas = serde_json::from_str(&canvas_str).unwrap();

    Ok(canvas)
}

