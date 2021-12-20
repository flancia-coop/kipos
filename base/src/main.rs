mod layout;

use std::{fs, collections::HashMap, fmt::Debug, path::PathBuf};

use layout::garden::Garden;
use serde::{self, Deserialize};
use serde_yaml;
use tera;

fn error(msg: &str, error: &dyn Debug) {
    eprintln!("{}: {:?}", msg, error);
    std::process::exit(1)
}

#[derive(Debug, Deserialize)]
struct GardenConfig {
    level: u8,
    metadata: Option<HashMap<String, String>>,
    /// The only difference between this and another [Config] garden is that reletive paths are parsed reletive to their parent
    ///
    /// Oh, and also it's easy to read.
    ///
    /// # Example
    /// ```yaml
    /// garden: {
    ///   path: ./bob # root/bob
    ///   children: [
    ///     {
    ///       path: ./joe # root/bob/joe
    ///     }
    ///   ]
    children: Option<Vec<GardenConfig>>,
    plugin: String,
    path: PathBuf
}

#[derive(Debug, Deserialize)]
struct Config {
    plugins: Option<Vec<String>>,
    metadata: Option<Vec<String>>,
    gardens: Vec<GardenConfig>,
}

fn parse_config() -> Option<String>{
    let bytes = fs::read("./config.yml");
    let mightres = match bytes {
        Ok(t) => Some(t),
        Err(e) => {
            error("Oh nos, an error!",&e);
            None
        }
    };
    if let Some(res) = mightres {
        Some(String::from_utf8_lossy(&res).into_owned())
    } else {
        None
    }
}

fn config(c: &GardenConfig) {
    if let Some(g) = &c.children {
        for garden in g.iter() {
            println!("{:?}",garden);
            config(garden);
        }
    }
}

fn main() {
    if let Some(yaml) = parse_config() {
        match serde_yaml::from_str::<Config>(&yaml) {
            Ok(t) => {
                let mut registrar: HashMap<u8, Vec<GardenConfig>> = HashMap::new();
                for garden in t.gardens.into_iter() {
                    config(&garden);
                    if !registrar.contains_key(&garden.level) {
                        registrar.insert(garden.level, vec![]);
                    }
                    let e = registrar.get(&garden.level).unwrap();
                    e.push(garden);
                }
            },
            Err(e) => eprintln!("{}",e)
        }
    }
}