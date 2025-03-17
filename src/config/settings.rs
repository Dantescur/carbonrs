use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use syntect::highlighting::{Theme, ThemeSet};

use crate::syntax::theme::load_theme;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub theme: ThemeConfig,
    pub layout: LayoutConfig,
    pub typography: TypographyConfig,
    pub decorations: DecorationsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub name: String,
    pub theme: Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    pub padding_vertical: u32,
    pub padding_horizontal: u32,
    pub window_controls: WindowControlsConfig,
    pub shadow: ShadowConfig,
    pub watermark: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyConfig {
    pub font_family: Option<String>,
    pub font_size: f32,
    pub line_height: Option<f32>,
    pub line_numbers: LineNumberConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecorationsConfig {
    pub language: Option<String>, // Force language override
    pub line_numbers: bool,
    pub line_numbers_start: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowControlsConfig {
    pub visible: bool,
    pub button_colors: Vec<[u8; 4]>,
    pub style: String, // "macos", "windows", "linux"
    pub size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShadowConfig {
    pub enabled: bool,
    pub offset_y: u32,
    pub blur_radius: u32,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineNumberConfig {
    pub enabled: bool,
    pub color: String,
}

impl Config {
    // pub fn load() -> Result<Self> {
    //     let mut config = load_config().unwrap_or_else(|_| Config::default());
    //     config.theme.theme = load_theme(&config.theme.name, &config)?;
    //     Ok(config)
    // }

    pub fn with_theme(mut self, theme_name: &str) -> Result<Self> {
        self.theme.name = theme_name.to_string();
        self.theme.theme = load_theme(theme_name)?;
        Ok(self)
    }
}

// Implement defaults matching carbon.now.sh's initial state

impl Default for ThemeConfig {
    fn default() -> Self {
        let theme_set = ThemeSet::load_defaults();
        Self {
            name: "base16-ocean.dark".into(),
            theme: theme_set.themes["base16-ocean.dark"].clone(),
        }
    }
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            padding_vertical: 40,
            padding_horizontal: 40,
            window_controls: WindowControlsConfig::default(),
            shadow: ShadowConfig::default(),
            watermark: false,
        }
    }
}

impl Default for TypographyConfig {
    fn default() -> Self {
        Self {
            font_family: Some("Fira Code".into()),
            font_size: 24.0,
            line_height: None, // Let the renderer auto-calculate if needed
            line_numbers: LineNumberConfig::default(),
        }
    }
}

impl Default for DecorationsConfig {
    fn default() -> Self {
        Self {
            language: None,
            line_numbers: false,
            line_numbers_start: 1,
        }
    }
}

impl Default for WindowControlsConfig {
    fn default() -> Self {
        Self {
            visible: true,
            style: "macos".into(),
            size: 60,
            button_colors: vec![[255, 0, 0, 1], [0, 255, 0, 1], [255, 255, 0, 1]],
        }
    }
}

impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            offset_y: 10,
            blur_radius: 5,
            color: "#000000".into(),
        }
    }
}

impl Default for LineNumberConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            color: "#aaaaaa".into(),
        }
    }
}
