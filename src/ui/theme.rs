// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// ActiveTheme adapted from https://github.com/zed-industries/zed/blob/main/crates/theme/src/theme.rs

use gpui::{hsla, rgb, rgba, App, Global, Hsla, Rgba};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{write, File},
    io::Read,
    path::PathBuf,
    str::FromStr,
};
use strum::{Display, EnumCount, EnumString};
use strum_macros::EnumIter;

const DEFAULT_THEME: &str = "theme = \"Dark\"\n";

#[derive(Serialize, Deserialize)]
struct Config {
    theme: ThemeVariant,
}

#[derive(Serialize, Deserialize, PartialEq, EnumCount, EnumString, EnumIter, Display)]
pub enum ThemeVariant {
    Dark,
    Light,
    RedDark,
    RosePineMoon,
    SolarizedDark,
}

pub struct Theme {
    pub text: Hsla,
    pub subtext: Hsla,
    pub background: Rgba,
    pub foreground: Rgba,
    pub field: Rgba,
    pub button: Rgba,
    pub cursor: Rgba,
    pub highlight: Rgba,
    pub border: Rgba,
    pub separator: Rgba,
}

impl Global for Theme {}

pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}

impl Theme {
    pub fn set(cx: &mut App) {
        let username = whoami::username();
        let path = Theme::path(username);
        let config_content = Theme::read(path).unwrap_or(String::from(DEFAULT_THEME));
        let theme = match Theme::deserialize(config_content) {
            ThemeVariant::Dark => Theme::dark(),
            ThemeVariant::Light => Theme::light(),
            ThemeVariant::RedDark => Theme::red_dark(),
            ThemeVariant::RosePineMoon => Theme::rose_pine_moon(),
            ThemeVariant::SolarizedDark => Theme::solarized_dark(),
        };
        cx.set_global(theme);
    }

    pub fn global(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    fn dark() -> Self {
        Self {
            text: hsla(0., 0., 0.9, 0.9),
            subtext: hsla(0., 0., 0.5, 0.2),
            background: rgb(0x3c3c3c),
            foreground: rgb(0x282828),
            field: rgb(0x1d1d1d),
            button: rgb(0x404040),
            cursor: rgb(0x3311ff),
            highlight: rgba(0x3311ff30),
            border: rgba(0x646464ff),
            separator: rgba(0x000000ff),
        }
    }

    fn light() -> Self {
        Self {
            text: hsla(0., 0., 0.1, 0.9),
            subtext: hsla(0., 0., 0.5, 0.2),
            background: rgb(0xe0e0e0),
            foreground: rgb(0xc0c0c0),
            field: rgb(0xb0b0b0),
            button: rgb(0xd0d0d0),
            cursor: rgb(0x3311ff),
            highlight: rgba(0x3311ff30),
            border: rgba(0x969696ff),
            separator: rgba(0x969696ff),
        }
    }

    fn red_dark() -> Self {
        Self {
            text: hsla(0., 0.5, 0.9, 0.9),
            subtext: hsla(0., 0.1, 0.5, 0.2),
            background: rgb(0x600000),
            foreground: rgb(0x490000),
            field: rgb(0x390000),
            button: rgb(0x6a0000),
            cursor: rgb(0xd12727),
            highlight: rgba(0xd1272730),
            border: rgba(0x6e2c2fff),
            separator: rgba(0x000000ff),
        }
    }

    fn rose_pine_moon() -> Self {
        Self {
            text: hsla(0.7, 0.5, 0.9, 0.9),
            subtext: hsla(0.7, 0.1, 0.5, 0.2),
            background: rgb(0x393552),
            foreground: rgb(0x2a273f),
            field: rgb(0x1e1c31),
            button: rgb(0x3b3754),
            cursor: rgb(0x9bced6),
            highlight: rgba(0x9bced630),
            border: rgba(0x504c68ff),
            separator: rgba(0x000000ff),
        }
    }

    fn solarized_dark() -> Self {
        Self {
            text: hsla(0.5, 0.5, 0.9, 0.9),
            subtext: hsla(0.5, 0.1, 0.5, 0.2),
            background: rgb(0x0a404c),
            foreground: rgb(0x002b36),
            field: rgb(0x00212c),
            button: rgb(0x0b434f),
            cursor: rgb(0x278ad1),
            highlight: rgba(0x278ad130),
            border: rgba(0x2b4e58ff),
            separator: rgba(0x000000ff),
        }
    }

    fn path(username: String) -> PathBuf {
        #[cfg(target_os = "macos")]
        let user_dir = PathBuf::from("/Users").join(username.clone());
        #[cfg(target_os = "linux")]
        let user_dir = PathBuf::from("/home").join(username.clone());
        user_dir.clone().join(".config").join("alc-calc")
    }

    fn deserialize(config_content: String) -> ThemeVariant {
        match toml::from_str(&config_content) {
            Ok(config) => config,
            Err(_) => {
                println!("Failed to deserialize config. Defaulting to Dark theme");
                Config {
                    theme: ThemeVariant::Dark,
                }
            }
        }
        .theme
    }

    fn serialize(theme_str: &str) -> String {
        let theme: ThemeVariant = ThemeVariant::from_str(theme_str).unwrap_or(ThemeVariant::Dark);
        let config = Config { theme };
        match toml::to_string(&config) {
            Ok(config_content) => config_content,
            Err(_) => {
                println!("Failed to serialize config. Defaulting to Dark theme");
                String::from(DEFAULT_THEME)
            }
        }
    }

    fn read(path: PathBuf) -> Result<String, Box<dyn Error>> {
        let file_path = path.join("config.toml");
        if std::fs::metadata(&file_path).is_err() {
            Theme::write("Dark");
        }

        let mut config_file = File::open(file_path)?;
        let mut config_content = String::new();
        match config_file.read_to_string(&mut config_content) {
            Ok(_) => (),
            Err(_) => {
                config_content = String::from(DEFAULT_THEME);
                println!("Failed to read config file. Defaulting to Dark theme");
            }
        }

        Ok(config_content)
    }

    fn write(theme_str: &str) {
        let config_content = Theme::serialize(theme_str);
        let path = Theme::path(whoami::username());
        if std::fs::metadata(&path).is_err() {
            match std::fs::create_dir(&path) {
                Ok(_) => (),
                Err(_) => println!("Failed to create config directory"),
            }
        }
        match write(path.join("config.toml"), &config_content) {
            Ok(_) => (),
            Err(_) => println!("Failed to write to config file"),
        }
    }

    pub fn update(theme_str: &str, cx: &mut App) {
        Theme::write(theme_str);
        Theme::set(cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path() {
        let username = String::from("abc");
        let path = Theme::path(username);
        #[cfg(target_os = "macos")]
        assert_eq!(path, PathBuf::from("/Users/abc/.config/alc-calc"));
        #[cfg(target_os = "linux")]
        assert_eq!(path, PathBuf::from("/home/abc/.config/alc-calc"));
    }

    #[test]
    fn test_deserialize() {
        let config_content = String::from("theme = \"SolarizedDark\"\n");
        let theme = Theme::deserialize(config_content);
        assert!(theme == ThemeVariant::SolarizedDark);
    }

    #[test]
    fn test_serialize() {
        let theme_str = "RosePineMoon";
        let expected = String::from("theme = \"RosePineMoon\"\n");
        let config_content = Theme::serialize(theme_str);
        assert_eq!(config_content, expected);
    }
}
