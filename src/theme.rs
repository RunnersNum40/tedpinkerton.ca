use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use dioxus_sdk_storage::StorageBacking;
use dioxus_sdk_storage::{LocalStorage, use_storage};
#[cfg(target_arch = "wasm32")]
use dioxus_sdk_window::theme::{Theme as SystemTheme, get_theme};
use serde::{Deserialize, Serialize};
use std::fmt;

pub fn use_theme() -> Signal<Theme> {
    #[cfg(target_arch = "wasm32")]
    let initialize_from_system =
        use_hook(|| LocalStorage::get::<Theme>(&"theme".to_string()).is_none());
    let theme = use_storage::<LocalStorage, Theme>("theme".to_string(), Theme::default);

    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        if initialize_from_system {
            let mut theme = theme;
            theme.set(get_theme().map(Theme::from).unwrap_or_default());
        }
    });

    theme
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

#[cfg(target_arch = "wasm32")]
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

impl Theme {
    pub fn toggle(&self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}
