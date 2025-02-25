use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[cfg(unix)]
const CACHE_FILE_PATH: &str = ".config/lescan/app_cache.toml";

#[cfg(windows)]
const CACHE_FILE_PATH: &str = "AppData\\Roaming\\lescan\\app_cache.toml";

#[cfg(unix)]
const SETTINGS_FILE_PATH: &str = ".config/lescan/settings.toml";

#[cfg(windows)]
const SETTINGS_FILE_PATH: &str = "AppData\\Roaming\\lescan\\settings.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct AppCache {
    pub last_document: Option<String>,
    pub settings_file_path: String,
}

impl Default for AppCache {
    fn default() -> Self {
        let home_dir = home::home_dir().unwrap();
        let pb = home_dir.join(CACHE_FILE_PATH);

        if pb.is_file() {
            let file = read_to_string(&pb).unwrap_or_else(|_| {
                panic!(
                    "{} {:?}",
                    t!("errors.could_not_open_config"),
                    std::path::absolute(&pb)
                )
            });
            toml::from_str::<Self>(&file).unwrap_or_else(|_| {
                panic!(
                    "{} {:?}",
                    t!("errors.could_not_parse_config"),
                    std::path::absolute(&pb)
                )
            })
        } else {
            let default_cache_file = AppCache {
                last_document: None,
                settings_file_path: home_dir.join(SETTINGS_FILE_PATH).display().to_string(),
            };
            default_cache_file.save();

            default_cache_file
        }
    }
}

impl AppCache {
    pub fn save(&self) {
        let home_dir = home::home_dir().unwrap();
        let pb = home_dir.join(CACHE_FILE_PATH);
        let file = toml::to_string_pretty(&self).unwrap_or_else(|_| {
            panic!(
                "{} {:?}",
                t!("errors.could_not_serialize_config"),
                std::path::absolute(&pb)
            )
        });

        std::fs::create_dir_all(pb.parent().unwrap()).unwrap_or_else(|_| {
            panic!(
                "{} {:?}",
                t!("errors.could_not_create_config_dir"),
                std::path::absolute(&pb)
            )
        });

        std::fs::write(&pb, file).unwrap_or_else(|_| {
            panic!(
                "{} {:?}",
                t!("errors.could_not_write_config"),
                std::path::absolute(&pb)
            )
        });
    }
}
