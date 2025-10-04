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

use crate::ui::comp::toast::toast;

const DEFAULT_THEME: &str = "theme = \"Dark\"\n";
#[cfg(target_os = "linux")]
const DEFAULT_CUSTOM_THEME: &str = "variant = \"Custom\"
text = \"#e6e6e6e6\"
subtext = \"#cccccc99\"
inactivetext = \"#80808033\"
background = \"#3c3c3cff\"
foreground = \"#282828ff\"
foreground_inactive = \"#232323ff\"
field = \"#1d1d1dff\"
cursor = \"#3311ffff\"
highlight = \"#3311ff30\"
border = \"#646464ff\"
separator = \"#000000ff\"
close_button = \"#404040ff\"
close_button_hover = \"#464646ff\"
close_button_click = \"#505050ff\"
close_button_inactive = \"#3b3b3bff\"
";
#[cfg(not(target_os = "linux"))]
const DEFAULT_CUSTOM_THEME: &str = "variant = \"Custom\"
text = \"#e6e6e6e6\"
subtext = \"#cccccc99\"
inactivetext = \"#80808033\"
background = \"#3c3c3cff\"
foreground = \"#282828ff\"
foreground_inactive = \"#232323ff\"
field = \"#1d1d1dff\"
cursor = \"#3311ffff\"
highlight = \"#3311ff30\"
border = \"#646464ff\"
separator = \"#000000ff\"
";

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
    Custom,
}

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct Theme {
    pub variant: ThemeVariant,
    pub text: Hsla,
    pub subtext: Hsla,
    pub inactivetext: Hsla,
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
        let config_content = Theme::read(cx, path.clone()).unwrap_or(String::from(DEFAULT_THEME));
        let theme = match Theme::deserialize(cx, config_content) {
            ThemeVariant::Dark => Theme::dark(),
            ThemeVariant::Light => Theme::light(),
            ThemeVariant::RedDark => Theme::red_dark(),
            ThemeVariant::RosePineMoon => Theme::rose_pine_moon(),
            ThemeVariant::SolarizedDark => Theme::solarized_dark(),
            ThemeVariant::Custom => Theme::read_theme(cx, path).unwrap_or(Theme::custom()),
        };
        cx.set_global(theme);
    }

    pub fn preview(cx: &mut App, val: &str) {
        let path = dirs::config_dir().unwrap_or_default().join("alc-calc");
        let theme = match ThemeVariant::from_str(val).unwrap_or(ThemeVariant::Dark) {
            ThemeVariant::Dark => Theme::dark(),
            ThemeVariant::Light => Theme::light(),
            ThemeVariant::RedDark => Theme::red_dark(),
            ThemeVariant::RosePineMoon => Theme::rose_pine_moon(),
            ThemeVariant::SolarizedDark => Theme::solarized_dark(),
            ThemeVariant::Custom => Theme::read_theme(cx, path).unwrap_or(Theme::custom()),
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
            subtext: hsla(0., 0., 0.8, 0.6),
            inactivetext: hsla(0., 0., 0.5, 0.2),
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
            subtext: hsla(0., 0., 0.3, 0.6),
            inactivetext: hsla(0., 0., 0.5, 0.2),
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
            subtext: hsla(0., 0.3, 0.8, 0.6),
            inactivetext: hsla(0., 0.1, 0.5, 0.2),
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
            subtext: hsla(0.7, 0.3, 0.8, 0.6),
            inactivetext: hsla(0.7, 0.1, 0.5, 0.2),
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
            subtext: hsla(0.5, 0.3, 0.8, 0.6),
            inactivetext: hsla(0.5, 0.1, 0.5, 0.2),
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

    fn custom() -> Self {
        let mut theme = Theme::dark();
        theme.variant = ThemeVariant::Custom;
        theme
    }

    fn deserialize(cx: &mut App, config_content: String) -> ThemeVariant {
        match toml::from_str(&config_content) {
            Ok(config) => config,
            Err(_) => {
                toast(cx, "Failed to deserialize config. Defaulting to Dark theme");
                Config {
                    theme: ThemeVariant::Dark,
                }
            }
        }
        .theme
    }

    fn serialize(cx: &mut App, theme_str: &str) -> String {
        let theme: ThemeVariant = ThemeVariant::from_str(theme_str).unwrap_or(ThemeVariant::Dark);
        let config = Config { theme };
        match toml::to_string(&config) {
            Ok(config_content) => config_content,
            Err(_) => {
                toast(cx, "Failed to serialize config. Defaulting to Dark theme");
                String::from(DEFAULT_THEME)
            }
        }
    }

    fn read(cx: &mut App, path: PathBuf) -> Result<String, anyhow::Error> {
        let file_path = path.join("config.toml");
        if std::fs::metadata(&file_path).is_err() {
            Theme::write(cx, "Dark");
        }

        let mut config_file = File::open(file_path)?;
        let mut config_content = String::new();
        match config_file.read_to_string(&mut config_content) {
            Ok(_) => (),
            Err(_) => {
                config_content = String::from(DEFAULT_THEME);
                toast(cx, "Failed to read config file. Defaulting to Dark theme");
            }
        }

        Ok(config_content)
    }

    fn write(cx: &mut App, theme_str: &str) {
        let config_content = Theme::serialize(cx, theme_str);
        let path = dirs::config_dir().unwrap_or_default().join("alc-calc");
        if std::fs::metadata(&path).is_err() {
            match std::fs::create_dir(&path) {
                Ok(_) => (),
                Err(_) => toast(cx, "Failed to create config directory"),
            }
        }
        match write(path.join("config.toml"), &config_content) {
            Ok(_) => (),
            Err(_) => toast(cx, "Failed to write to config file"),
        }
    }

    fn deserialize_theme(cx: &mut App, theme_content: &str) -> Result<Theme, anyhow::Error> {
        match toml::from_str(theme_content) {
            Ok(theme) => Ok(theme),
            Err(_) => {
                toast(cx, "Failed to deserialize theme. Defaulting to Dark theme");
                Ok(Theme::dark())
            }
        }
    }

    fn serialize_theme(cx: &mut App, theme: Theme) -> String {
        match toml::to_string(&theme) {
            Ok(custom_theme) => custom_theme,
            Err(_) => {
                toast(
                    cx,
                    "Failed to serialize default custom theme. Defaulting to hardcoded default custom theme",
                );
                String::from(DEFAULT_CUSTOM_THEME)
            }
        }
    }

    fn read_theme(cx: &mut App, path: PathBuf) -> Result<Theme, anyhow::Error> {
        let file_path = path.join("theme.toml");

        // prevents fs access on tests (namely, when Custom is selected in Theme::preview())
        #[cfg(not(test))]
        if std::fs::metadata(&file_path).is_err() {
            Theme::write_theme(cx, path);
        }

        let mut theme_file = File::open(file_path)?;
        let mut theme_content = String::new();
        match theme_file.read_to_string(&mut theme_content) {
            Ok(_) => (),
            Err(_) => {
                theme_content = String::from(DEFAULT_CUSTOM_THEME);
                toast(
                    cx,
                    "Failed to read theme file. Defaulting to default custom theme",
                );
            }
        }
        Theme::deserialize_theme(cx, &theme_content)
    }

    // RA thinks this is dead code even though it is used
    #[allow(dead_code)]
    fn write_theme(cx: &mut App, path: PathBuf) {
        let mut default_custom_theme = Theme::dark();
        default_custom_theme.variant = ThemeVariant::Custom;
        let custom_theme = Theme::serialize_theme(cx, default_custom_theme);
        match write(path.join("theme.toml"), &custom_theme) {
            Ok(_) => (),
            Err(_) => toast(cx, "Failed to write to theme file"),
        }
    }

    // RA thinks this is dead code even though it is used
    #[allow(dead_code)]
    pub fn update(theme_str: &str, cx: &mut App) {
        Theme::write(cx, theme_str);
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
    use gpui::TestAppContext;

    #[gpui::test]
    fn test_deserialize(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        let mut theme = ThemeVariant::Dark;
        let config_content = String::from("theme = \"SolarizedDark\"\n");

        cx.update(|_, cx| {
            theme = Theme::deserialize(cx, config_content);
        });

        assert!(theme == ThemeVariant::SolarizedDark);
    }

    #[gpui::test]
    fn test_serialize(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        let theme_str = "RosePineMoon";
        let mut config_content = String::new();
        let expected = String::from("theme = \"RosePineMoon\"\n");

        cx.update(|_, cx| {
            config_content = Theme::serialize(cx, theme_str);
        });

        assert_eq!(config_content, expected);
    }

    #[gpui::test]
    fn test_deserialize_theme(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        let mut theme = Theme::dark();
        let expected = Theme::custom();

        cx.update(|_, cx| {
            theme = Theme::deserialize_theme(cx, DEFAULT_CUSTOM_THEME).unwrap();

            // round these particular values due to lossy conversions from hsla -> rgb -> hsla
            theme.text.l = crate::calc::round_to_place(theme.text.l, 1.0).unwrap();
            theme.text.a = crate::calc::round_to_place(theme.text.a, 1.0).unwrap();
            theme.subtext.l = crate::calc::round_to_place(theme.subtext.l, 1.0).unwrap();
            theme.subtext.a = crate::calc::round_to_place(theme.subtext.a, 1.0).unwrap();
            theme.inactivetext.l = crate::calc::round_to_place(theme.inactivetext.l, 1.0).unwrap();
            theme.inactivetext.a = crate::calc::round_to_place(theme.inactivetext.a, 1.0).unwrap();
        });

        assert_eq!(theme, expected);
    }

    #[gpui::test]
    fn test_serialize_theme(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        let theme = Theme::custom();
        let mut theme_content = String::new();

        cx.update(|_, cx| {
            theme_content = Theme::serialize_theme(cx, theme);
        });

        assert_eq!(theme_content, DEFAULT_CUSTOM_THEME);
    }
}
