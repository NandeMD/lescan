use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs::read_to_string;
use rust_i18n::t;

#[cfg(unix)]
const SETTINGS_FILE_PATH: &str = "~/.config/lescan/settings.toml";

#[cfg(windows)]
const SETTINGS_FILE_PATH: &str = "~\\AppData\\Roaming\\lescan\\settings.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct AppSettings {
    pub settings_file_path: String,
}

impl std::default::Default for AppSettings {
    fn default() -> Self {
        let pb = PathBuf::from(SETTINGS_FILE_PATH);

        if pb.is_file() {
            let file = read_to_string(&pb).unwrap_or_else(|_| panic!("{} {:?}", t!("could_not_open_config"), std::path::absolute(&pb)));
            toml::from_str::<Self>(&file).unwrap_or_else(|_| panic!("{} {:?}", t!("could_not_parse_config"), std::path::absolute(&pb)))
        } else {
            AppSettings {
                settings_file_path: SETTINGS_FILE_PATH.to_string(),
            }
        }
    }
}

impl AppSettings {
    pub fn save(&self) {
        let pb = PathBuf::from(&self.settings_file_path);
        let file = toml::to_string_pretty(&self).unwrap_or_else(|_| panic!("{} {:?}", t!("could_not_serialize_config"), std::path::absolute(&pb)));
        std::fs::write(&pb, file).unwrap_or_else(|_| panic!("{} {:?}", t!("could_not_write_config"), std::path::absolute(&pb)));
    }
}

impl std::fmt::Display for AppSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

