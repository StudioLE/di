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

pub async fn copy_with_logging(
    source: PathBuf,
    destination: PathBuf,
) -> Result<(), Report<HttpError>> {
    create_parent_dir_if_not_exist(&destination)
        .await
        .change_context(HttpError::CreateDestinationDirectory)?;
    if destination.exists() {
        remove_file(&destination)
            .await
            .change_context(HttpError::RemoveExisting)?;
    }
    let result = if use_hardlink(&source, &destination).await {
        trace!(
            source = %source.display(),
            destination = %destination.display(),
            "Hard linking file"
        );
        hard_link(&source, &destination).await
    } else {
        trace!(
            source = %source.display(),
            destination = %destination.display(),
            "Copying file"
        );
        copy(&source, &destination).await.map(|_| ())
    };
    result.change_context(HttpError::Copy).attach_with(|| {
        format!(
            "Source: {}\nDestination: {}",
            source.display(),
            destination.display()
        )
    })
}

#[cfg(unix)]
async fn use_hardlink(source: &Path, destination: &Path) -> bool {
    use std::os::unix::fs::MetadataExt;
    use tokio::fs::metadata;
    let source = source
        .parent()
        .expect("source should have a parent directory");
    let source = metadata(source)
        .await
        .expect("should be able to get source metadata");
    let destination = destination
        .parent()
        .expect("destination should have a parent directory");
    let destination = metadata(destination)
        .await
        .expect("should be able to get destination metadata");
    source.dev() == destination.dev()
}

#[cfg(not(unix))]
async fn use_hardlink(_source: &Path, _dest: &Path) -> bool {
    false
}
