use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    host: Host,
}

#[derive(Deserialize)]
pub struct Host {
    port: u16,
    host: String,
    user: String,
    password: String,
}