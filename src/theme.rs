use dioxus::prelude::*;
use dioxus_sdk_storage::*;
use dioxus_sdk_window::theme::{Theme as SystemTheme, use_system_theme};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl From<SystemTheme> for Theme {
    fn from(value: SystemTheme) -> Self {
        match value {
            SystemTheme::Light => Theme::Light,
            SystemTheme::Dark => Theme::Dark,
        }
    }
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Theme::Light => write!(f, "light"),
            Theme::Dark => write!(f, "dark"),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}

impl Theme {
    pub fn toggle(&self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}
