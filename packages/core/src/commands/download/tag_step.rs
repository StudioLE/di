use crate::prelude::*;
use lofty::config::WriteOptions;
use lofty::error::LoftyError;
use lofty::id3::v2::Id3v2Tag;
use lofty::picture::{Picture, PictureType};
use lofty::prelude::{Accessor, TagExt, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::{Tag, TagType};

impl DownloadHandler {
    #[allow(clippy::unused_self)]
    pub(super) fn tag_step(&self, context: &DownloadContext) -> Result<(), Report<DownloadError>> {
        let content_type = context.episode.source_content_type.as_str();
        if content_type != "audio/mpeg" {
            warn!(%context.episode, content_type, "Skipping file as it's not an MP3");
            return Ok(());
        }
        let cover = if let Some(image_path) = &context.image_path {
            let file = File::open(image_path).change_context(DownloadError::OpenPicture)?;
            let mut reader = BufReader::new(file);
            let mut picture =
                Picture::from_reader(&mut reader).change_context(DownloadError::ReadPicture)?;
            picture.set_pic_type(PictureType::CoverFront);
            Some(picture)
        } else {
            None
        };
        let tag = create_tag(&context.podcast, &context.episode, cover);
        write_tag(&context.file_path, tag).change_context(DownloadError::TagEpisode)?;
        Ok(())
    }
}

#[allow(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
fn create_tag(
    podcast: &DownloadPodcastPartial,
    episode: &DownloadEpisodePartial,
    cover: Option<Picture>,
) -> Id3v2Tag {
    let mut tag = Id3v2Tag::default();
    tag.set_title(episode.title.trim().to_owned());
    tag.set_artist(podcast.title.clone());
    if let Some(season) = episode.season {
        tag.set_album(format!("Season {season}"));
    }
    tag.set_disk(episode.season.unwrap_or_default());
    let year = episode.published_at.year() as u32;
    tag.set_year(year);
    if let Some(number) = episode.episode {
        tag.set_track(number);
    }
    if let Some(cover) = cover {
        tag.insert_picture(cover);
    }
    tag
}

fn write_tag(path: &PathBuf, tag: Id3v2Tag) -> Result<(), LoftyError> {
    for tag_type in get_tag_types(path)? {
        trace!("Removing tag: {:?}", tag_type);
        tag.remove_from_path(path)?;
    }
    tag.save_to_path(path, WriteOptions::default())?;
    Ok(())
}

fn get_tag_types(path: &Path) -> Result<Vec<TagType>, LoftyError> {
    let tagged_file = Probe::open(path)?.read()?;
    let tag_types = tagged_file.tags().iter().map(Tag::tag_type).collect();
    Ok(tag_types)
}
