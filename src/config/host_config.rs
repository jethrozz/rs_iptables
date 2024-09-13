use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub host: Host,
}

#[derive(Deserialize)]
pub struct Host {
    pub port: u16,
    pub host: String,
    pub user: String,
    pub password: String,
}