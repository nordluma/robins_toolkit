use serde_derive::{Deserialize, Serialize};
use toml::{self, de::Error};

#[derive(Deserialize)]
pub struct Config {
    pub custom_config: Option<CustomConfig>,
    pub custom_error_messages: Option<CustomErrorMessages>,
}

#[derive(Deserialize)]
pub struct CustomConfig {
    pub use_custom_config: Option<bool>,
    pub custom_settings_path: Option<String>,
}

#[derive(Deserialize)]
pub struct CustomErrorMessages {
    pub use_custom_error_messages: Option<bool>,
    pub custom_error_messages_path: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        let config_file =
            std::fs::read_to_string("settings.toml").expect("Could not open settings.toml");
        let config: Result<(), Error> =
            toml::from_str(&config_file).expect("Could not parse config file");

        Config {
            custom_config: todo!("Read user config"),
            custom_error_messages: todo!("Read custom error messages"),
        }
    }
}
