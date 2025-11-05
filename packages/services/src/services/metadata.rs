use crate::prelude::*;
use error_stack::{FutureExt, ResultExt};

pub struct MetadataStore {
    dir: PathBuf,
}

impl MetadataStore {
    #[must_use]
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    pub fn get(&self, id: &str) -> Result<Podcast, Report<GetMetadataError>> {
        let path = self.get_path(id);
        if !path.exists() {
            let report = Report::new(GetMetadataError::NotFound)
                .attach(format!("ID: {id}"))
                .attach(format!("File does not exist: {}", path.display()));
            return Err(report);
        }
        let file = File::open(&path)
            .change_context(GetMetadataError::Open)
            .attach_path(&path)?;
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader)
            .change_context(GetMetadataError::Deserialize)
            .attach_path(&path)
    }

    pub fn put(&self, podcast: &Podcast) -> Result<(), Report<PutMetadataError>> {
        let path = self.get_path(&podcast.id);
        let file = File::create(&path)
            .change_context(PutMetadataError::Create)
            .attach_path(&path)?;
        let writer = BufWriter::new(file);
        serde_yaml::to_writer(writer, podcast)
            .change_context(PutMetadataError::Serialize)
            .attach_path(&path)
    }

    fn get_path(&self, id: &str) -> PathBuf {
        self.dir.join(id).with_extension("yml")
    }
}

impl Default for MetadataStore {
    fn default() -> Self {
        Self {
            dir: PathProvider::default().get_metadata_dir(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    #[traced_test]
    pub fn put_then_get() {
        // Arrange
        let metadata = MetadataStore::default();
        let podcast = Podcast::example();

        // Act
        metadata.put(&podcast).assert_ok_debug();
        let result = metadata.get(&podcast.id);

        // Assert
        let result = result.assert_ok_debug();
        assert_eq!(podcast, result);
    }

    #[tokio::test]
    #[traced_test]
    pub async fn filter() {
        // Arrange
        let services = ServiceProvider::create()
            .await
            .expect("ServiceProvider should not fail");
        let podcast = services.metadata.get("irl").expect("podcast should exist");

        // Act
        let mut y2019 = podcast.clone();
        y2019.filter(&FilterOptions {
            year: Some(2019),
            ..FilterOptions::default()
        });
        let mut y2019_2020 = podcast.clone();
        y2019_2020.filter(&FilterOptions {
            from_year: Some(2019),
            to_year: Some(2020),
            ..FilterOptions::default()
        });
        let mut s1 = podcast.clone();
        s1.filter(&FilterOptions {
            season: Some(1),
            ..FilterOptions::default()
        });
        let mut s1_2 = podcast.clone();
        s1_2.filter(&FilterOptions {
            from_season: Some(1),
            to_season: Some(2),
            ..FilterOptions::default()
        });
        let mut s1 = podcast.clone();
        s1.filter(&FilterOptions {
            season: Some(1),
            ..FilterOptions::default()
        });
        let mut s4_2018 = podcast.clone();
        s4_2018.filter(&FilterOptions {
            season: Some(4),
            year: Some(2018),
            ..FilterOptions::default()
        });

        // Preview
        info!("Episode count: {}", podcast.episodes.len());
        trace!("Podcast:\n{}", serde_yaml::to_string(&podcast).expect("should be able to serialize podcast"));

        // Assert
        assert!(podcast.episodes.len() >= 60);
        assert_eq!(y2019.episodes.len(), 13);
        assert_eq!(y2019_2020.episodes.len(), 13);
        assert_eq!(s1.episodes.len(), 10);
        assert_eq!(s1_2.episodes.len(), 17);
        assert_eq!(s4_2018.episodes.len(), 3);
    }
}
