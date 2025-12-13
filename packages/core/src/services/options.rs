use crate::prelude::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AppOptions {
    /// Directory to store cache files.
    ///
    /// Default: `$HOME/.cache/alnwick` (or equivalent)
    pub cache_dir: Option<PathBuf>,
    /// Directory to store app data.
    ///
    /// Default: `$HOME/.local/share/alnwick` (or equivalent)
    pub data_dir: Option<PathBuf>,
    /// Base URL to use for server.
    ///
    /// Default: None
    pub server_base: Option<Url>,
    /// Expected external IP address.
    ///
    /// Execution will stop if different.
    ///
    /// Default: None
    pub expect_ip: Option<String>,
    /// Expected country geolocation.
    ///
    /// Execution will stop if different.
    ///
    /// Default: None
    pub expect_country: Option<String>,
    /// Should hardlinks be used when copying between cache and data directory?
    ///
    /// Defaults to automatically determining
    pub hard_link_from_cache: Option<bool>,
}

impl Service for AppOptions {
    type Error = AppOptionsError;

    async fn from_services(_services: &ServiceProvider) -> Result<Self, Report<AppOptionsError>> {
        envy::from_env().change_context(AppOptionsError::EnvConfig)
    }
}

#[derive(Clone, Debug, Error)]
pub enum AppOptionsError {
    #[error("Unable to read config from environment variables")]
    EnvConfig,
}
