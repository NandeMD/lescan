use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::PathBuf;

use crate::app::modals::settings::SettingsMenuContents;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppSettings {
    pub settings_file_path: String,
    #[serde(with = "theme_serde")]
    pub app_theme: iced::Theme,
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
                app_theme: iced::Theme::TokyoNight,
            }
        }
    }

    pub fn apply_from_modal(&mut self, settings_modal: &SettingsMenuContents) {
        self.settings_file_path = settings_modal.general_settings_file_path.clone();
        self.app_theme = settings_modal.app_theme.clone();
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

mod theme_serde {
    use serde::{de::Visitor, Deserializer, Serializer};

    pub fn serialize<S>(theme: &iced::Theme, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let theme_name = theme.to_string();
        serializer.serialize_str(&theme_name)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<iced::Theme, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ThemeVisitor;

        impl Visitor<'_> for ThemeVisitor {
            type Value = iced::Theme;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a theme name")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match_theme_names(v)
                    .ok_or_else(|| serde::de::Error::custom(format!("Unknown theme name: {}", v)))
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match_theme_names(v.as_ref())
                    .ok_or_else(|| serde::de::Error::custom(format!("Unknown theme name: {}", v)))
            }
        }

        deserializer.deserialize_any(ThemeVisitor)
    }

    fn match_theme_names(name: &str) -> Option<iced::Theme> {
        match name {
            "Light" => Some(iced::Theme::Light),
            "Dark" => Some(iced::Theme::Dark),
            "Dracula" => Some(iced::Theme::Dracula),
            "Nord" => Some(iced::Theme::Nord),
            "Solarized Light" => Some(iced::Theme::SolarizedLight),
            "Solarized Dark" => Some(iced::Theme::SolarizedDark),
            "Gruvbox Light" => Some(iced::Theme::GruvboxLight),
            "Gruvbox Dark" => Some(iced::Theme::GruvboxDark),
            "Catppuccin Latte" => Some(iced::Theme::CatppuccinLatte),
            "Catppuccin Frappé" => Some(iced::Theme::CatppuccinFrappe),
            "Catppuccin Macchiato" => Some(iced::Theme::CatppuccinMacchiato),
            "Catppuccin Mocha" => Some(iced::Theme::CatppuccinMocha),
            "Tokyo Night" => Some(iced::Theme::TokyoNight),
            "Tokyo Night Storm" => Some(iced::Theme::TokyoNightStorm),
            "Tokyo Night Light" => Some(iced::Theme::TokyoNightLight),
            "Kanagawa Wave" => Some(iced::Theme::KanagawaWave),
            "Kanagawa Dragon" => Some(iced::Theme::KanagawaDragon),
            "Kanagawa Lotus" => Some(iced::Theme::KanagawaLotus),
            "Moonfly" => Some(iced::Theme::Moonfly),
            "Nightfly" => Some(iced::Theme::Nightfly),
            "Oxocarbon" => Some(iced::Theme::Oxocarbon),
            "Ferra" => Some(iced::Theme::Ferra),
            _ => None,
        }
    }
}
