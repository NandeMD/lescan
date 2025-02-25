use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppSettings {
    pub settings_file_path: String,
}

impl AppSettings {
    pub fn new(settings_file_path: String) -> Self {
        let pb = PathBuf::from(&settings_file_path);

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
            AppSettings {
                settings_file_path: settings_file_path.to_string(),
            }
        }
    }

    pub fn save(&self) {
        let pb = PathBuf::from(&self.settings_file_path);
        let file = toml::to_string_pretty(&self).unwrap_or_else(|_| {
            panic!(
                "{} {:?}",
                t!("errors.could_not_serialize_config"),
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

impl std::fmt::Display for AppSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
