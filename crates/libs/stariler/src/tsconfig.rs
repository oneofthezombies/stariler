use tracing::debug;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct CompilerOptions {
    #[serde(rename = "outDir")]
    pub(crate) out_dir: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct TsConfig {
    pub(crate) files: Option<Vec<String>>,
    pub(crate) include: Option<Vec<String>>,
    pub(crate) exclude: Option<Vec<String>>,

    #[serde(rename = "compilerOptions")]
    pub(crate) compiler_options: Option<CompilerOptions>,
}

impl Default for TsConfig {
    fn default() -> Self {
        Self {
            files: None,
            include: Some(vec!["**/*".to_string()]),
            exclude: None,
            compiler_options: None,
        }
    }
}

static DEFAULT_EXCLUDE: &[&str] = &["node_modules", "bower_components", "jspm_packages"];

pub(crate) fn update_exclude(ts_config: TsConfig) -> TsConfig {
    if ts_config.exclude.is_some() {
        debug!("exclude already set, not updating");
        return ts_config;
    }
    let mut exclude: Vec<String> = DEFAULT_EXCLUDE.iter().map(|s| (*s).to_string()).collect();
    if let Some(compiler_options) = &ts_config.compiler_options {
        if let Some(out_dir) = &compiler_options.out_dir {
            debug!(
                out_dir = out_dir.as_str(),
                "add compilerOptions.outDir to exclude"
            );
            exclude.push(out_dir.clone());
        }
    }
    debug!(exclude = ?exclude, "updated exclude");
    TsConfig {
        exclude: Some(exclude),
        ..ts_config
    }
}
