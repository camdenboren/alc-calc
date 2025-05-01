// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// todo
// - match instead of expect
// - handle init

use gpui::{hsla, rgb, rgba, App, Global, Hsla, Rgba};
use serde::{Deserialize, Serialize};
use std::{
    fs::{write, File},
    io::Read,
    path::PathBuf,
    str::FromStr,
};
use strum::{Display, EnumCount, EnumString};
use strum_macros::EnumIter;

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
}

impl Global for Theme {}

impl Theme {
    pub fn set(cx: &mut App) {
        let username = whoami::username();
        let path = Theme::path(username);
        let config_content = Theme::load(path);
        let theme = match Theme::read(config_content) {
            ThemeVariant::Dark => Theme::dark(),
            ThemeVariant::Light => Theme::light(),
            ThemeVariant::RedDark => Theme::red_dark(),
            ThemeVariant::RosePineMoon => Theme::rose_pine_moon(),
            ThemeVariant::SolarizedDark => Theme::solarized_dark(),
        };
        cx.set_global::<Theme>(theme);
    }

    fn dark() -> Self {
        Self {
            text: hsla(0., 0., 0.9, 0.9),
            subtext: hsla(0., 0., 0., 0.5),
            background: rgb(0x3c3c3c),
            foreground: rgb(0x282828),
            field: rgb(0x1d1d1d),
            button: rgb(0x404040),
            cursor: rgb(0x3311ff),
            highlight: rgba(0x3311ff30),
        }
    }

    fn light() -> Self {
        Self {
            text: hsla(0., 0., 0., 0.5),
            subtext: hsla(0., 0., 0.1, 0.1),
            background: rgb(0xe0e0e0),
            foreground: rgb(0xc0c0c0),
            field: rgb(0xb0b0b0),
            button: rgb(0xd0d0d0),
            cursor: rgb(0x3311ff),
            highlight: rgba(0x3311ff30),
        }
    }

    fn red_dark() -> Self {
        Self {
            text: hsla(0., 0., 0.9, 0.9),
            subtext: hsla(0., 0., 0., 0.5),
            background: rgb(0x600000),
            foreground: rgb(0x490000),
            field: rgb(0x390000),
            button: rgb(0x404040),
            cursor: rgb(0xd12727),
            highlight: rgba(0xd1272730),
        }
    }

    fn rose_pine_moon() -> Self {
        Self {
            text: hsla(0., 0., 0.9, 0.9),
            subtext: hsla(0., 0., 0., 0.5),
            background: rgb(0x393552),
            foreground: rgb(0x2a273f),
            field: rgb(0x1e1c31),
            button: rgb(0x404040),
            cursor: rgb(0x9bced6),
            highlight: rgba(0x9bced630),
        }
    }

    fn solarized_dark() -> Self {
        Self {
            text: hsla(0., 0., 0.9, 0.9),
            subtext: hsla(0., 0., 0., 0.5),
            background: rgb(0x0a404c),
            foreground: rgb(0x002b36),
            field: rgb(0x00212c),
            button: rgb(0x404040),
            cursor: rgb(0x278ad1),
            highlight: rgba(0x278ad130),
        }
    }

    fn path(username: String) -> PathBuf {
        #[cfg(target_os = "macos")]
        let user_dir = PathBuf::from("/Users").join(username.clone());
        #[cfg(target_os = "linux")]
        let user_dir = PathBuf::from("/home").join(username.clone());
        user_dir
            .clone()
            .join(".config")
            .join("alc-calc")
            .join("config.toml")
    }

    fn load(path: PathBuf) -> String {
        let mut config_file = File::open(path).expect("Failed to open config file");
        let mut config_content = String::new();
        config_file
            .read_to_string(&mut config_content)
            .expect("Failed to read config file");
        config_content
    }

    fn read(config_content: String) -> ThemeVariant {
        let config: Config = toml::from_str(&config_content).expect("Failed to parse config file");
        config.theme
    }

    pub fn update(theme_str: &str, cx: &mut App) {
        let theme: ThemeVariant = ThemeVariant::from_str(theme_str).unwrap();
        let config = Config { theme };
        let config_content = toml::to_string(&config).unwrap();
        write(Theme::path(whoami::username()), config_content)
            .expect("Failed to write to config file");
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
        assert_eq!(
            path,
            PathBuf::from("/Users/abc/.config/alc-calc/config.toml")
        );
        #[cfg(target_os = "linux")]
        assert_eq!(
            path,
            PathBuf::from("/home/abc/.config/alc-calc/config.toml")
        );
    }

    #[test]
    fn test_read() {
        let config_content = String::from(
            r#"
            theme = 'SolarizedDark'
            "#,
        );
        let theme = Theme::read(config_content);
        assert!(theme == ThemeVariant::SolarizedDark);
    }
}
