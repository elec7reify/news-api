use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;
use serde::{Deserialize, Serialize};
use toml::to_string;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    database: Database
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Database {
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database: Database {
                host: "localhost".to_string(),
                port: 5432,
                database: "database".to_string(),
                username: "username".to_string(),
                password: "".to_string(),
            }
        }
    }
}

pub fn write() -> Result<(), Box<dyn Error>> {
    let configuration = to_string(&Config::default())?;
    let mut file = File::create(Path::new("config.toml"))?;
    file.write_all(configuration.as_bytes())?;

    Ok(())
}

impl Config {
    pub fn new(filename: &str) -> Self {
        let mut file = match File::open(filename) {
            Ok(c) => c,
            Err(error) => {
                eprintln!("Could not load file: {filename}\n{error}");
                exit(1);
            }
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Could not read file: {filename}\n{error}");
                exit(1);
            }
        }

        let config: Config = toml::from_str(&contents)
            .map_err(|e| {
                eprintln!("Could not parse TOML config: {filename}\n{e}");
                exit(1);
            })
            .unwrap();

        config
    }

    pub fn database(&self) -> Database {
        self.database.clone()
    }
}

impl Database {
    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn database(&self) -> String {
        self.database.clone()
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn password(&self) -> String {
        self.password.clone()
    }
}
