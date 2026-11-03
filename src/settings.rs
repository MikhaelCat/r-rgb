use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const DIRECTIONS: &[&str] = &["right", "left", "down", "up", "out", "in"];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_effect")]
    pub effect: String,
    #[serde(default = "default_color")]
    pub color: String,
    #[serde(default = "default_highlight")]
    pub highlight: String,
    #[serde(default = "default_one")]
    pub brightness: f32,
    #[serde(default = "default_half")]
    pub speed: f32,
    #[serde(default = "default_direction")]
    pub direction: String,
    #[serde(default = "default_half")]
    pub wave_size: f32,
    #[serde(default)]
    pub independent_zones: bool,
    #[serde(default)]
    pub bar_color: Option<String>,
}

fn default_true() -> bool {
    true
}
fn default_effect() -> String {
    "static".to_string()
}
fn default_color() -> String {
    "#00ff00".to_string()
}
fn default_highlight() -> String {
    "#ffffff".to_string()
}
fn default_one() -> f32 {
    1.0
}
fn default_half() -> f32 {
    0.5
}
fn default_direction() -> String {
    "right".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: true,
            effect: default_effect(),
            color: default_color(),
            highlight: default_highlight(),
            brightness: 1.0,
            speed: 0.5,
            direction: default_direction(),
            wave_size: 0.5,
            independent_zones: false,
            bar_color: None,
        }
    }
}

pub fn parse_color(value: &str) -> Result<(u8, u8, u8), String> {
    let s = value.trim().trim_start_matches('#');
    if s.len() != 6 {
        return Err(format!(
            "Invalid color: '{}' (expected 6 hex chars like ff8000)",
            value
        ));
    }
    let r = u8::from_str_radix(&s[0..2], 16).map_err(|e| e.to_string())?;
    let g = u8::from_str_radix(&s[2..4], 16).map_err(|e| e.to_string())?;
    let b = u8::from_str_radix(&s[4..6], 16).map_err(|e| e.to_string())?;
    Ok((r, g, b))
}

pub fn format_color(rgb: (u8, u8, u8)) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb.0, rgb.1, rgb.2)
}

pub fn default_config_path() -> PathBuf {
    let base = match std::env::var_os("XDG_CONFIG_HOME") {
        Some(dir) => PathBuf::from(dir),
        None => {
            let home = std::env::var_os("HOME").unwrap_or_default();
            PathBuf::from(home).join(".config")
        }
    };
    base.join("rrgb").join("settings.json")
}

pub fn default_profiles_path() -> PathBuf {
    let base = match std::env::var_os("XDG_CONFIG_HOME") {
        Some(dir) => PathBuf::from(dir),
        None => {
            let home = std::env::var_os("HOME").unwrap_or_default();
            PathBuf::from(home).join(".config")
        }
    };
    base.join("rrgb").join("profiles.json")
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppData {
    #[serde(default)]
    pub profiles: std::collections::HashMap<String, Settings>,
    #[serde(default)]
    pub favorites: Vec<String>,
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "ru".to_string()
}

pub fn load_app_data<P: AsRef<Path>>(path: P) -> AppData {
    let p = path.as_ref();
    if let Ok(data) = fs::read_to_string(p) {
        if let Ok(app_data) = serde_json::from_str::<AppData>(&data) {
            return app_data;
        }
    }
    AppData::default()
}

pub fn save_app_data<P: AsRef<Path>>(app_data: &AppData, path: P) -> std::io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(app_data)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn legacy_config_path() -> PathBuf {
    let base = match std::env::var_os("XDG_CONFIG_HOME") {
        Some(dir) => PathBuf::from(dir),
        None => {
            let home = std::env::var_os("HOME").unwrap_or_default();
            PathBuf::from(home).join(".config")
        }
    };
    base.join("keyglow").join("settings.json")
}

pub fn load<P: AsRef<Path>>(path: P) -> Settings {
    let p = path.as_ref();
    if let Ok(data) = fs::read_to_string(p) {
        if let Ok(settings) = serde_json::from_str::<Settings>(&data) {
            return settings;
        }
    }
    // Fallback to legacy keyglow settings if rrgb settings don't exist yet
    let leg = legacy_config_path();
    if p != leg {
        if let Ok(data) = fs::read_to_string(&leg) {
            if let Ok(settings) = serde_json::from_str::<Settings>(&data) {
                return settings;
            }
        }
    }
    Settings::default()
}

pub fn latest_user_settings() -> Option<Settings> {
    let home_dirs = match fs::read_dir("/home") {
        Ok(entries) => entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect::<Vec<_>>(),
        Err(_) => Vec::new(),
    };

    let mut newest_time = std::time::SystemTime::UNIX_EPOCH;
    let mut newest_settings = None;

    for home in home_dirs {
        for sub in &["rrgb", "keyglow"] {
            let conf = home.join(".config").join(sub).join("settings.json");
            if let Ok(meta) = fs::metadata(&conf) {
                if let Ok(mtime) = meta.modified() {
                    if mtime > newest_time {
                        if let Ok(data) = fs::read_to_string(&conf) {
                            if let Ok(s) = serde_json::from_str::<Settings>(&data) {
                                newest_time = mtime;
                                newest_settings = Some(s);
                            }
                        }
                    }
                }
            }
        }
    }

    newest_settings
}

pub fn save<P: AsRef<Path>>(settings: &Settings, path: P) -> std::io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(settings)?;
    fs::write(path, json)?;
    Ok(())
}
