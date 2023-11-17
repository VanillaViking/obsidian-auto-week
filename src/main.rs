use std::{env, process};

use auto_week::Config;


fn main() {
    let config = match Config::build(env::args()) {
        Ok(config) => config,
        Err(msg) => {
            eprintln!("{msg}");
            process::exit(1);
        }
    };

    let result = match config.command.as_str() {
        "new" => auto_week::new_week(config),
        _ => Ok(())
    };

    result.unwrap_or_else(|err| eprintln!("{}", err.to_string()))
}
