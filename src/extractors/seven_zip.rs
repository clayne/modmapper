use anyhow::Result;
use std::io::{Seek, SeekFrom};
use std::process::Command;
use tempfile::tempdir;
use tracing::{info, info_span};
use walkdir::WalkDir;

use crate::models::file::File;
use crate::models::game_mod::Mod;
use crate::plugin_processor::process_plugin;

pub async fn extract_with_7zip(
    file: &mut std::fs::File,
    pool: &sqlx::Pool<sqlx::Postgres>,
    db_file: &File,
    db_mod: &Mod,
) -> Result<()> {
    file.seek(SeekFrom::Start(0))?;
    let temp_dir = tempdir()?;
    let temp_file_path = temp_dir.path().join("download.zip");
    let mut temp_file = std::fs::File::create(&temp_file_path)?;
    std::io::copy(file, &mut temp_file)?;
    drop(temp_file); // close handle to temp file so 7zip process can open it
    let extracted_path = temp_dir.path().join("extracted");

    Command::new("7z")
        .args(&[
            "x",
            &format!("-o{}", &extracted_path.to_string_lossy()),
            &temp_file_path.to_string_lossy().to_string(),
        ])
        .status()?;

    for entry in WalkDir::new(&extracted_path)
        .contents_first(true)
        .into_iter()
        .filter_entry(|e| {
            if let Some(extension) = e.path().extension() {
                extension == "esp" || extension == "esm" || extension == "esl"
            } else {
                false
            }
        })
    {
        let entry = entry?;
        let file_path = entry.path();
        let plugin_span = info_span!("plugin", name = ?file_path);
        let _plugin_span = plugin_span.enter();
        info!("processing uncompressed file from downloaded archive");
        let mut plugin_buf = std::fs::read(extracted_path.join(file_path))?;
        process_plugin(
            &mut plugin_buf,
            &pool,
            &db_file,
            &db_mod,
            &file_path.to_string_lossy(),
        )
        .await?;
    }
    Ok(())
}
