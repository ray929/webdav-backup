use crate::config::{parse_db_table, PgSqlConfig};
use anyhow::Result;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command;
use zip::write::ZipWriter;
use zip::{AesMode, CompressionMethod};

pub async fn backup(config: &PgSqlConfig, dump_path: &str, zip_path: &Path, password: Option<&str>) -> Result<()> {
    // 按库名分组，收集每个库需要导出的表
    let mut db_tables: BTreeMap<&str, Vec<Option<&str>>> = BTreeMap::new();
    for entry in &config.database {
        let (db_name, table) = parse_db_table(entry);
        db_tables
            .entry(db_name)
            .or_default()
            .push(table);
    }

    let zip_file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(zip_file);

    for (db_name, tables) in &db_tables {
        let mut cmd = Command::new(dump_path);
        cmd.arg(format!("--host={}", config.host))
            .arg(format!("--port={}", config.port))
            .arg(format!("--username={}", config.username))
            .arg(format!("--dbname={}", db_name))
            .arg("--no-password")
            .arg("--clean")
            .arg("--if-exists");

        // 如果任一配置项不指定表（即全库备份），则忽略所有 --table
        let dump_all = tables.iter().any(|t| t.is_none());
        if !dump_all {
            for table in tables.iter().flatten() {
                cmd.arg("--table").arg(table);
            }
        }

        if let Some(ref ssl_mode) = config.ssl_mode {
            cmd.env("PGSSLMODE", ssl_mode);
        }

        cmd.env("PGPASSWORD", &config.password);

        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("pg_dump failed for '{}': {}", db_name, stderr);
        }

        let filename = format!("{}.sql", db_name);
        if let Some(password) = password {
            let options = zip::write::FileOptions::<'_, ()>::default()
                .compression_method(CompressionMethod::Deflated)
                .with_aes_encryption(AesMode::Aes256, password);
            zip.start_file(filename, options)?;
        } else {
            let options = zip::write::FileOptions::<'_, ()>::default()
                .compression_method(CompressionMethod::Deflated);
            zip.start_file(filename, options)?;
        }
        zip.write_all(&output.stdout)?;
    }

    zip.finish()?;

    Ok(())
}
