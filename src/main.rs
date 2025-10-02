#![windows_subsystem = "windows"]

use anyhow::{Context, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use std::{env, time::Duration};
use tokio::time::sleep;
use winapi::um::wincon::GetConsoleWindow;
use winapi::um::winuser::{SW_HIDE, ShowWindow};
use winreg::RegKey;
use winreg::enums::*;

use crate::background_backup::perform_backup_transaction;
use crate::config::load_settings;
use crate::log_manager::create_logs_json;
use crate::validation::check_single_instance;

mod background_backup;
mod config;
mod log_manager;
mod validation;

const STARTUP_REG_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const APP_NAME: &str = "SilentBackupApp";

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Logs {
    log_type: String,
    time_date: String,
    logs: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AppSettings {
    debug: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Settings {
    app: AppSettings,
    backup: Backup,
}

#[derive(Debug, Deserialize, Serialize)]
struct Backup {
    src_dir: String,
    dst_dir: String,
    hours: u32,
    minutes: u32,
    seconds: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    if !check_single_instance("Global\\SilentBackupAppMutex") {
        eprintln!("Another instance of the application is already running.");
        return Ok(());
    }

    let config_builder = ConfigBuilder::new()
        .set_time_offset_to_local()
        .unwrap_or_else(|cb| cb)
        .build();

    SimpleLogger::init(LevelFilter::Info, config_builder).context("Failed to initialize logger")?;
    info!("Starting Silent Backup App...");

    let settings = load_settings().context("Failed to load configuration")?;
    info!("Loaded config: {:?}", settings);

    if !settings.app.debug {
        info!("Debug mode is off. Hiding console window.");
        hide_console_window();
    } else {
        info!(
            "Debug mode is on. Console window will remain visible if app is run from a terminal."
        );
    }

    add_to_startup().context("Failed to add app to startup")?;
    info!("Added app to startup registry");

    loop {
        info!("Starting scheduled backup transaction...");

        match perform_backup_transaction(&settings.backup.src_dir, &settings.backup.dst_dir).await {
            Ok(final_path) => {
                let success_msg = format!("Backup completed successfully to: {:?}", final_path);
                info!("{}", success_msg);
                create_logs_json("SUCCESS".to_string(), success_msg).await?;
            }
            Err(e) => {
                let error_msg = format!("Backup transaction failed: {:?}", e);
                error!("{}", error_msg);
                create_logs_json("ERROR".to_string(), error_msg).await?;
            }
        }

        let interval = Duration::from_secs(
            (settings.backup.hours as u64 * 3600)
                + (settings.backup.minutes as u64 * 60)
                + settings.backup.seconds as u64,
        );

        info!("Sleeping for {:?} until next backup.", interval);
        sleep(interval).await;
    }
}

fn hide_console_window() {
    let window = unsafe { GetConsoleWindow() };
    if !window.is_null() {
        unsafe { ShowWindow(window, SW_HIDE) };
    }
}

fn add_to_startup() -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (run_key, _) = hkcu.create_subkey(STARTUP_REG_KEY)?;

    let exe_path = env::current_exe()
        .context("Failed to get current executable path")?
        .to_str()
        .context("Executable path is not valid UTF-8")?
        .to_owned();

    run_key.set_value(APP_NAME, &exe_path)?;
    Ok(())
}
