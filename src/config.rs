use anyhow::{Context, Result};
use config::{Config, File};
use log::{info, warn};
use std::{env, path::Path};

use crate::Settings;

pub fn create_default_config(path: &Path) -> Result<()> {
    info!("Config file not found. Creating a default config.ini...");

    let content = r#"
[app]
# Set to true to prevent the console window from being hidden.
# To see the output, you must run this executable from an existing command prompt (cmd.exe or PowerShell).
# The '#![windows_subsystem = "windows"]' directive in the code means a new console window
# will NOT be created if you just double-click the .exe.
debug = true

[backup]
# Path to the directory you want to back up.
# IMPORTANT: You must change this path.
# Example: src_dir = "C:\\Users\\YourUser\\Documents"
src_dir = "C:\\replace\\with\\your\\source\\folder"

# Path to the directory where backups will be stored.
# IMPORTANT: You must change this path.
# Example: dst_dir = "D:\\Backups"
dst_dir = "D:\\replace\\with\\your\\backup\\destination"

# Backup interval. The app will back up the folder every X hours, Y minutes, Z seconds.
# Default is every 24 hours.
hours = 24
minutes = 0
seconds = 0
"#;

    std::fs::write(path, content).context("Failed to write default config file")?;
    warn!("Default config.ini created at {:?}. Please edit it with your backup paths, otherwise backups will fail.", path);
    Ok(())
}

pub fn load_settings() -> Result<Settings> {
    let exe_path = env::current_exe().context("Failed to get executable path")?;
    let exe_dir = exe_path
        .parent()
        .context("Failed to get executable directory")?;

    let config_path = exe_dir.join("config.ini");
    info!("Loading config from {:?}", config_path);

    if !config_path.exists() {
        create_default_config(&config_path)?;
    }

    let settings_loader = Config::builder()
        .add_source(File::from(config_path))
        .build()
        .context("Failed to build config")?;

    let settings: Settings = settings_loader
        .try_deserialize()
        .context("Failed to deserialize config. Check if all required sections ([app], [backup]) and their fields are set correctly.")?;
    Ok(settings)
}
