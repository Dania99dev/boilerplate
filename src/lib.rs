use std::env;
use std::path::Path;
use std::process::{self, Command};

pub enum CommandType {
    New,
}
pub struct Config {
    pub command: CommandType,
    name: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next() {
            Some(c) => {
                if c.trim() == "new" {
                    CommandType::New
                } else {
                    return Err("This command is not available");
                }
            }
            None => return Err("Please pass a command"),
        };
        let name = match args.next() {
            Some(name) => name,
            None => return Err("Please specify a name for the project"),
        };
        Ok(Config { command, name })
    }
}

pub fn initialize(config: Config) {
    if Path::new(&config.name).exists() {
        eprintln!("`{}` directory already exists", config.name);
        process::exit(1);
    } else {
        let output = Command::new("cmd")
            .args(&[
                "/C",
                &format!("npm init vite@latest {} -- --template vue-ts", config.name),
            ])
            .output()
            .expect("Failed to Initialize");
        if output.status.success() {
            println!("Initialized successfully");
        } else {
            println!("Problem initializing project");
        }
    }
}
