use anyhow::Result;
use chrono::{Datelike, Local, Timelike};
use log::info;
use tokio::{fs, io::AsyncWriteExt};

use crate::Logs;

pub async fn create_logs_json(log_type: String, log_message: String) -> Result<()> {
    let now = Local::now();
    let time_date_log_format = format!(
        "DATE:{:04}-{:02}-{:02} TIME:{:02}:{:02}:{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );

    let log_entry = Logs {
        log_type: log_type.clone(),
        time_date: time_date_log_format,
        logs: log_message,
    };

    let json_bytes = serde_json::to_vec_pretty(&log_entry)?;

    let logs_dir_name = format!("{:04}-{:02}-{:02}-LOGS", now.year(), now.month(), now.day());

    fs::create_dir_all(&logs_dir_name).await?;

    let file_path = format!(
        "{}\\{:02}-{:02}-{:02}-{}.json",
        logs_dir_name,
        now.hour(),
        now.minute(),
        now.second(),
        log_type
    );

    let mut file = fs::File::create(&file_path).await?;
    file.write_all(&json_bytes).await?;
    info!("Log written to {}", file_path);

    Ok(())
}
