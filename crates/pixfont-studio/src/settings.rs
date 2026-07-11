use std::{
    fmt::Display,
    fs::{self},
    io,
    path::PathBuf,
};

use app_data::AppData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub appearance: Appearance,
}

impl Settings {
    pub fn load() -> Result<Settings, Error> {
        let str = match fs::read_to_string(Self::path()) {
            Ok(toml) => toml,
            Err(error) => match error.kind() {
                io::ErrorKind::NotFound => return Ok(Settings::default()),
                _ => return Err(error.into()),
            },
        };
        let settings = toml::from_str(&str)?;

        Ok(settings)
    }

    pub fn save(&self) -> Result<(), Error> {
        let toml = toml::to_string_pretty(self)?;
        fs::write(Self::path(), toml)?;

        Ok(())
    }

    fn path() -> PathBuf {
        // TODO: Cleanup

        let data = AppData::new("PixFont Studio");
        _ = data.ensure_data_dir().expect("Can't create data folder");

        data.get_file_path("config.toml").unwrap()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Appearance {
    pub theme: Theme,
    pub editor: EditorColors,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    /// Infer the colour theme from the system preferred scheme.
    #[default]
    Auto,

    /// Force a light theme.
    Light,

    /// Force a dark theme.
    Dark,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Theme::Auto => "System",
                Theme::Dark => "Dark",
                Theme::Light => "Light",
            }
        )
    }
}

impl Theme {
    pub const ALL: &[Theme] = &[Theme::Auto, Theme::Dark, Theme::Light];
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct EditorColors {
    pub background: Option<Color>,
    pub gridlines: Option<Color>,
    pub glyph: Option<Color>,
    pub origin: Option<Color>,
    pub metrics: Option<Color>,
    pub guidelines: Option<Color>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub b: f32,
    pub g: f32,
    pub a: f32,
}

impl From<iced::Color> for Color {
    fn from(iced::Color { r, g, b, a }: iced::Color) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Color> for iced::Color {
    fn from(Color { r, g, b, a }: Color) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Encode(#[from] toml::ser::Error),

    #[error("{0}")]
    Parse(#[from] toml::de::Error),
}
