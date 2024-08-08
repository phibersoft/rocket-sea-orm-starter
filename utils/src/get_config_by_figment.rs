use serde::{Deserialize, Serialize};

use rocket::figment::Figment;

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Databases {
    pub sea_orm: DatabaseConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub databases: Databases,
}

#[allow(unused)]
pub fn get_config_by_figment(figment: &Figment) -> Config {
    let config: Config = figment.extract().expect("config");
    config
}