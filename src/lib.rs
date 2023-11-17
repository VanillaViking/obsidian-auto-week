use std::io::Error;
use std::{fs, path::PathBuf };
use chrono::prelude::*;
use chrono::Duration;

pub struct Config {
    pub command: String,
    pub vault_dir: PathBuf,
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

    fs::create_dir_all(&archive_dir)?;
    
    // TODO: copy all the tickets in backlog
    
    fs::rename(&current_file, archive_dir.join("WEEK.canvas"))?;
    fs::copy(template, current_file)?;

    Ok(())
}

