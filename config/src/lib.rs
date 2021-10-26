use schemars::JsonSchema;
use serde::Deserialize;

use figment::{
    providers::{Env, Format, Json},
    Figment,
};

#[derive(JsonSchema, Deserialize, Debug)]
pub struct Config {
    pub key_path: String,
    pub cert_path: String,
    pub server_addr: String,
    pub use_tls: bool,
    #[schemars(skip)]
    pub database_url: String,
    pub jwt_settings: JwtSettings,
    pub password_work_factor: u8,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct JwtSettings {
    pub issuer: String,
    pub access_token_key: String,
    pub access_token_expiration_minutes: u32,
    pub refresh_token_key: String,
    pub refresh_token_expiration_minutes: u32,
}

pub struct ConfigLoader {}

impl ConfigLoader {
    pub fn load_config() -> Config {
        dotenv::dotenv().ok();

        let profile = std::env::var("PROFILE").unwrap_or_else(|_| "dev".to_owned());

        Figment::new()
            .merge(Json::file(format!("config.{}.json", profile)))
            .merge(Env::raw().only(&["DATABASE_URL"]))
            .extract()
            .expect("Error while loading the config file")
    }
}

pub fn write_schema_to_disk(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let schema = schemars::schema_for!(Config);
    let serialized_schema = serde_json::to_string_pretty(&schema)?;
    std::fs::write(path, serialized_schema)?;

    Ok(())
}
