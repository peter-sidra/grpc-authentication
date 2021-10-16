use serde::Deserialize;

use figment::{
    providers::{Env, Format, Json},
    Figment,
};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub key_path: String,
    pub cert_path: String,
    pub server_addr: String,
    pub use_tls: bool,
    pub database_url: String,
}

#[non_exhaustive]
pub struct Profiles;

#[allow(dead_code)]
impl Profiles {
    pub const PROD: &'static str = "prod";
    pub const DEV: &'static str = "dev";
}

pub struct ConfigLoader {}

impl ConfigLoader {
    pub fn load_config(profile: &str) -> Config {
        dotenv::dotenv().ok();

        Figment::new()
            .select(profile)
            .merge(Json::file("config.json").nested())
            .merge(Env::raw().only(&["DATABASE_URL"]))
            .extract()
            .expect("Error while loading the config file")
    }
}
