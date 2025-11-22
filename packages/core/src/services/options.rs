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
}

impl AppOptions {
    pub fn get() -> Result<Self, Report<ServiceError>> {
        envy::from_env().change_context(ServiceError::EnvConfig)
    }
}
