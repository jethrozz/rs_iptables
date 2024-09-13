use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::config::host_config::Config;
pub fn read_config_file(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}