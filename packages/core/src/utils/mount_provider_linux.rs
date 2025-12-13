use crate::prelude::*;
use procfs::process::{MountInfo, Process};

pub struct MountProvider {
    mounts: Vec<MountInfo>,
}

impl MountProvider {
    #[must_use]
    pub fn new() -> Self {
        let process = Process::myself().expect("should be able to get process");
        let mounts = process
            .mountinfo()
            .expect("should be able to get mount info");
        Self { mounts: mounts.0 }
    }

    pub fn get_mount_id(&self, path: &Path) -> Result<i32, Report<MountIdError>> {
        let path = path
            .canonicalize()
            .change_context(MountIdError::Canonicalize)?;
        let path = path.to_str().ok_or(MountIdError::PathNotUtf8)?;
        let mut best_match = None;
        let mut best_len = 0;
        for mount in self.mounts.iter() {
            let Some(mount_point) = mount.mount_point.to_str() else {
                warn!(mount_point = %mount.mount_point.display(), "Skipping mount point as it is not valid utf-8");
                continue;
            };
            let is_match = path.starts_with(mount_point);
            if is_match && mount_point.len() > best_len {
                best_match = Some(mount.mnt_id);
                best_len = mount_point.len();
            }
        }
        best_match.ok_or_else(|| Report::new(MountIdError::NoMatch))
    }
}

impl Service for MountProvider {
    type Error = Infallible;

    async fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new())
    }
}

#[derive(Debug, Error)]
pub enum MountIdError {
    #[error("Unable to canonicalize path")]
    Canonicalize,
    #[error("Path is not valid utf-8")]
    PathNotUtf8,
    #[error("Path does not match any mount points")]
    NoMatch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _get_mount_id() {
        // Arrange
        let _logger = init_test_logger();
        let mounts = MountProvider::new();

        // Act
        // Assert
        assert_ne!(
            mounts.get_mount_id(&PathBuf::from("/boot")).assert_ok(),
            0,
            "/boot"
        );
        assert_ne!(
            mounts.get_mount_id(&PathBuf::from("/var")).assert_ok(),
            0,
            "/var"
        );
        assert_ne!(mounts.get_mount_id(&PathBuf::from("/")).assert_ok(), 0, "/");
    }
}
