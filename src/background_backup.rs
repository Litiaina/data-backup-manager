use anyhow::{Context, Result};
use chrono::{Datelike, Local, Timelike};
use fs_extra::dir::{CopyOptions, copy};
use log::{error, info};
use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn perform_backup_transaction(src: &str, base_dst: &str) -> Result<PathBuf> {
    let src_path = Path::new(src);
    if !src_path.exists() || !src_path.is_dir() {
        return Err(anyhow::anyhow!(
            "Source folder '{}' does not exist or is not a directory",
            src
        ));
    }

    let now = Local::now();
    let folder_name = format!(
        "{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
    );

    let final_dst_path = Path::new(base_dst).join(&folder_name);
    let tmp_dst_path = Path::new(base_dst).join(format!("{}.tmp", folder_name));

    if tmp_dst_path.exists() {
        info!("Removing stale temporary directory: {:?}", tmp_dst_path);
        fs::remove_dir_all(&tmp_dst_path).await.with_context(|| {
            format!(
                "Failed to remove stale temporary directory: {:?}",
                tmp_dst_path
            )
        })?;
    }
    fs::create_dir_all(&tmp_dst_path)
        .await
        .with_context(|| format!("Failed to create temporary directory: {:?}", tmp_dst_path))?;

    info!(
        "Copying from {:?} to temporary location {:?}",
        src, tmp_dst_path
    );
    let copy_options = CopyOptions {
        overwrite: true,
        copy_inside: true,
        ..Default::default()
    };

    let src_owned = src.to_owned();
    let tmp_dst_path_owned = tmp_dst_path.clone();
    let copy_result =
        tokio::task::spawn_blocking(move || copy(&src_owned, &tmp_dst_path_owned, &copy_options))
            .await?;

    if let Err(e) = copy_result {
        error!("Copy operation failed: {}", e);
        fs::remove_dir_all(&tmp_dst_path).await.with_context(|| {
            format!(
                "Failed to cleanup failed temporary directory: {:?}",
                tmp_dst_path
            )
        })?;
        return Err(anyhow::anyhow!("Copy operation failed: {}", e));
    }
    info!("Copy to temporary location completed.");

    info!("Renaming {:?} to {:?}", tmp_dst_path, final_dst_path);
    fs::rename(&tmp_dst_path, &final_dst_path)
        .await
        .with_context(|| {
            format!("Failed to atomically rename backup folder to its final destination")
        })?;

    info!("Transaction committed successfully.");
    Ok(final_dst_path)
}
