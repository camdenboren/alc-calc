// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// ActiveTheme adapted from https://github.com/zed-industries/zed/blob/main/crates/theme/src/theme.rs

use gpui::{App, Global, Hsla, Rgba, TestAppContext, hsla, rgb, rgba};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, write},
    io::Read,
    path::PathBuf,
    str::FromStr,
};
use strum_macros::{Display, EnumCount, EnumIter, EnumString};

const DEFAULT_THEME: &str = "theme = \"Dark\"\n";

#[derive(Serialize, Deserialize)]
struct Config {
    theme: ThemeVariant,
}

#[derive(
    Serialize, Clone, Deserialize, PartialEq, EnumCount, EnumString, EnumIter, Debug, Display,
)]
pub enum ThemeVariant {
    Dark,
    Light,
    RedDark,
    RosePineMoon,
    SolarizedDark,
}

pub struct Theme {
    pub variant: ThemeVariant,
    pub text: Hsla,
    pub subtext: Hsla,
    pub background: Rgba,
    pub foreground: Rgba,
    pub foreground_inactive: Rgba,
    pub field: Rgba,
    pub cursor: Rgba,
    pub highlight: Rgba,
    pub border: Rgba,
    pub separator: Rgba,
    #[cfg(target_os = "linux")]
    pub close_button: Rgba,
    #[cfg(target_os = "linux")]
    pub close_button_hover: Rgba,
    #[cfg(target_os = "linux")]
    pub close_button_click: Rgba,
    #[cfg(target_os = "linux")]
    pub close_button_inactive: Rgba,
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
        let path = dirs::config_dir().unwrap_or_default().join("alc-calc");
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

    pub fn preview(cx: &mut App, val: &str) {
        let theme = match ThemeVariant::from_str(val).unwrap_or(ThemeVariant::Dark) {
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
            variant: ThemeVariant::Dark,
            text: hsla(0., 0., 0.9, 0.9),
            subtext: hsla(0., 0., 0.5, 0.2),
            background: rgb(0x3c3c3c),
            foreground: rgb(0x282828),
            foreground_inactive: rgb(0x232323),
            field: rgb(0x1d1d1d),
            cursor: rgb(0x3311ff),
            highlight: rgba(0x3311ff30),
            border: rgba(0x646464ff),
            separator: rgba(0x000000ff),
            #[cfg(target_os = "linux")]
            close_button: rgb(0x404040),
            #[cfg(target_os = "linux")]
            close_button_hover: rgb(0x464646),
            #[cfg(target_os = "linux")]
            close_button_click: rgb(0x505050),
            #[cfg(target_os = "linux")]
            close_button_inactive: rgb(0x3b3b3b),
        }
    }

    fn light() -> Self {
        Self {
            variant: ThemeVariant::Light,
            text: hsla(0., 0., 0.1, 0.9),
            subtext: hsla(0., 0., 0.5, 0.2),
            background: rgb(0xe0e0e0),
            foreground: rgb(0xc0c0c0),
            foreground_inactive: rgb(0xd0d0d0),
            field: rgb(0xb0b0b0),
            cursor: rgb(0x3311ff),
            highlight: rgba(0x3311ff30),
            border: rgba(0x969696ff),
            separator: rgba(0x969696ff),
            #[cfg(target_os = "linux")]
            close_button: rgb(0xd0d0d0),
            #[cfg(target_os = "linux")]
            close_button_hover: rgb(0xc8c8c8),
            #[cfg(target_os = "linux")]
            close_button_click: rgb(0xc0c0c0),
            #[cfg(target_os = "linux")]
            close_button_inactive: rgb(0xe0e0e0),
        }
    }

    fn red_dark() -> Self {
        Self {
            variant: ThemeVariant::RedDark,
            text: hsla(0., 0.5, 0.9, 0.9),
            subtext: hsla(0., 0.1, 0.5, 0.2),
            background: rgb(0x600000),
            foreground: rgb(0x490000),
            foreground_inactive: rgb(0x410000),
            field: rgb(0x390000),
            cursor: rgb(0xd12727),
            highlight: rgba(0xd1272730),
            border: rgba(0x6e2c2fff),
            separator: rgba(0x000000ff),
            #[cfg(target_os = "linux")]
            close_button: rgb(0x6a0000),
            #[cfg(target_os = "linux")]
            close_button_hover: rgb(0x740000),
            #[cfg(target_os = "linux")]
            close_button_click: rgb(0x7e0000),
            #[cfg(target_os = "linux")]
            close_button_inactive: rgb(0x5f0000),
        }
    }

    fn rose_pine_moon() -> Self {
        Self {
            variant: ThemeVariant::RosePineMoon,
            text: hsla(0.7, 0.5, 0.9, 0.9),
            subtext: hsla(0.7, 0.1, 0.5, 0.2),
            background: rgb(0x393552),
            foreground: rgb(0x2a273f),
            foreground_inactive: rgb(0x252038),
            field: rgb(0x1e1c31),
            cursor: rgb(0x9bced6),
            highlight: rgba(0x9bced630),
            border: rgba(0x504c68ff),
            separator: rgba(0x000000ff),
            #[cfg(target_os = "linux")]
            close_button: rgb(0x3b3754),
            #[cfg(target_os = "linux")]
            close_button_hover: rgb(0x413d5e),
            #[cfg(target_os = "linux")]
            close_button_click: rgb(0x464166),
            #[cfg(target_os = "linux")]
            close_button_inactive: rgb(0x36324c),
        }
    }

    fn solarized_dark() -> Self {
        Self {
            variant: ThemeVariant::SolarizedDark,
            text: hsla(0.5, 0.5, 0.9, 0.9),
            subtext: hsla(0.5, 0.1, 0.5, 0.2),
            background: rgb(0x0a404c),
            foreground: rgb(0x002b36),
            foreground_inactive: rgb(0x002631),
            field: rgb(0x00212c),
            cursor: rgb(0x278ad1),
            highlight: rgba(0x278ad130),
            border: rgba(0x2b4e58ff),
            separator: rgba(0x000000ff),
            #[cfg(target_os = "linux")]
            close_button: rgb(0x0b434f),
            #[cfg(target_os = "linux")]
            close_button_hover: rgb(0x0c4a58),
            #[cfg(target_os = "linux")]
            close_button_click: rgb(0x0c5262),
            #[cfg(target_os = "linux")]
            close_button_inactive: rgb(0x093e48),
        }
    }

    fn deserialize(config_content: String) -> ThemeVariant {
        match toml::from_str(&config_content) {
            Ok(config) => config,
            Err(_) => {
                eprintln!("Failed to deserialize config. Defaulting to Dark theme");
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
                eprintln!("Failed to serialize config. Defaulting to Dark theme");
                String::from(DEFAULT_THEME)
            }
        }
    }

    fn read(path: PathBuf) -> Result<String, anyhow::Error> {
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
                eprintln!("Failed to read config file. Defaulting to Dark theme");
            }
        }

        Ok(config_content)
    }

    fn write(theme_str: &str) {
        let config_content = Theme::serialize(theme_str);
        let path = dirs::config_dir().unwrap_or_default().join("alc-calc");
        if std::fs::metadata(&path).is_err() {
            match std::fs::create_dir(&path) {
                Ok(_) => (),
                Err(_) => eprintln!("Failed to create config directory"),
            }
        }
        match write(path.join("config.toml"), &config_content) {
            Ok(_) => (),
            Err(_) => eprintln!("Failed to write to config file"),
        }
    }

    // RA thinks this is dead code even though it is used
    #[allow(dead_code)]
    pub fn update(theme_str: &str, cx: &mut App) {
        Theme::write(theme_str);
        Theme::set(cx);
    }

    // RA thinks this is dead code even though it is used in tests
    #[allow(dead_code)]
    pub fn test(cx: &mut TestAppContext) {
        cx.set_global(Theme::light());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
