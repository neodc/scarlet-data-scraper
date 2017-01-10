use std::path::Path;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, RustcDecodable)]
pub struct Config {
    username: String,
    password: String,
    database_url: String,
}

pub const DEFAULT_PATH: &'static str = "config.toml";

impl Config {
    pub fn load(path: Option<&Path>) -> Self {
        let path = path.unwrap_or_else(|| Path::new(DEFAULT_PATH));

        let mut f = File::open(path).expect("Could not open config file");
        let mut buffer = String::new();

        // load content
        f.read_to_string(&mut buffer).expect("Could not read config file");

        toml::decode_str(buffer.as_ref()).expect("Unable to parse config file")
    }

    pub fn username(&self) -> &str {
        self.username.as_ref()
    }

    pub fn password(&self) -> &str {
        self.password.as_ref()
    }

    pub fn database_url(&self) -> &str {
        self.database_url.as_ref()
    }
}