use tracing::instrument;

#[instrument]
pub(crate) async fn read_file(path: std::path::PathBuf) -> crate::Result<String> {
    let content = tokio::fs::read_to_string(&path).await?;
    Ok(content)
}

#[instrument]
pub(crate) async fn parse_ts_config_path(
    path: std::path::PathBuf,
) -> crate::Result<std::path::PathBuf> {
    let ts_config_path = path.join(crate::core::TS_CONFIG_FILE_NAME);
    let metadata = tokio::fs::metadata(&ts_config_path).await?;
    if !metadata.is_file() {
        return Err(crate::Error::TsConfigNotFound {
            path: ts_config_path,
        });
    }
    Ok(ts_config_path)
}
