use crate::prelude::*;
use std::io::Error;

pub async fn create_parent_dir_if_not_exist(path: &Path) -> Result<(), Report<Error>> {
    let Some(dir) = path.parent() else {
        trace!(path = %path.display(), "No parent directory to create");
        return Ok(());
    };
    if !dir.exists() {
        trace!(dir = %dir.display(), "Creating directory");
        create_dir_all(&dir).await.attach_path(dir)?;
    }
    Ok(())
}
