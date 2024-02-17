#[derive(serde::Serialize, serde::Deserialize)]
pub struct TsConfig {
    extends: Option<String>,
    files: Option<Vec<String>>,
    include: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
}

impl Default for TsConfig {
    fn default() -> Self {
        Self {
            extends: None,
            files: None,
            include: Some(vec!["**/*".to_string()]),
            exclude: Some(vec![]),
        }
    }
}

fn read_ts_config(path: &std::path::Path) -> crate::core::Result<TsConfig> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let ts_config: TsConfig = serde_json::from_reader(reader)?;
    Ok(ts_config)
}
