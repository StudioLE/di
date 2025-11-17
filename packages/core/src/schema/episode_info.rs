use crate::prelude::*;
use std::fmt::Write as _;

/// Information about a podcast episode
///
/// - <https://help.apple.com/itc/podcasts_connect/#/itcb54353390>
/// - <https://github.com/Podcastindex-org/podcast-namespace>
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[allow(clippy::struct_field_names)]
pub struct EpisodeInfo {
    // Required
    /// Title
    pub title: String,
    /// URL of source media file including a file extension
    /// - Supported file formats include M4A, MP3, MOV, MP4, M4V, and PDF
    pub source_url: String,
    /// Size of source media file in bytes
    pub source_file_size: u64,
    /// Mime type of source media file
    pub source_content_type: String,
    /// GUID
    ///
    /// Matches `source_id` if it's a GUID otherwise it's derived deterministically.
    pub id: Uuid,
    /// GUID or Apple Podcasts Episode ID
    pub source_id: String,

    // Recommended
    /// Date and time episode was released
    pub published_at: DateTime<FixedOffset>,
    /// HTML formatted description
    pub description: Option<String>,
    /// Duration in seconds
    pub source_duration: Option<u64>,
    /// URL of JPEG or PNG artwork
    /// - Min: 1400 x 1400 px
    /// - Max: 3000 x 3000 px
    pub image: Option<String>,
    /// Parental advisory information
    pub explicit: Option<bool>,

    // Situationial
    /// Apple Podcasts specific title
    pub itunes_title: Option<String>,
    /// Episode number
    pub episode: Option<usize>,
    /// Season number
    pub season: Option<usize>,
    /// Episode type
    pub kind: Option<EpisodeKind>,
}

impl EpisodeInfo {}

impl EpisodeInfo {
    #[must_use]
    pub fn get_source_url(&self) -> Url {
        Url::parse(&self.source_url)
            .map_err(|error| {
                warn!(episode = %self, url = %self.source_url, %error, "Failed to parse episode source URL");
            })
            .expect("Should be able to parse episode source URL")
    }

    #[must_use]
    pub fn get_image_url(&self) -> Option<Url> {
        self.image.clone().and_then(|url| {
            Url::parse(&url)
                .map_err(|error| {
                    warn!(episode = %self, %url, %error, "Failed to parse episode image URL");
                })
                .ok()
        })
    }
    #[must_use]
    pub fn get_filename(&self) -> String {
        let file_stem = self.get_file_stem();
        let extension = self.get_extension().unwrap_or(MP3_EXTENSION.to_owned());
        format!("{file_stem}.{extension}")
    }

    #[must_use]
    pub fn get_file_stem(&self) -> String {
        let mut output = self.get_formatted_date();
        if let Some(number) = self.episode {
            let _ = write!(output, " {number:03}");
        }
        if let Some(kind) = self.kind
            && kind != EpisodeKind::Full
        {
            output.push(' ');
            output.push_str(&kind.to_string().to_uppercase());
        }
        if self.episode.is_none() && self.kind == Some(EpisodeKind::Full) {
            warn!(
                "Episode has no number and is not a trailer or bonus: {}",
                self.title
            );
        }
        output.push(' ');
        output.push_str(&self.get_sanitized_title());
        output
    }

    #[must_use]
    pub fn get_extension(&self) -> Option<String> {
        self.get_source_url().get_extension()
    }

    #[must_use]
    pub fn get_formatted_season(&self) -> String {
        Self::format_season(self.season)
    }

    #[must_use]
    pub fn format_season(season: Option<usize>) -> String {
        format!("S{:02}", season.unwrap_or(0))
    }

    fn get_formatted_date(&self) -> String {
        self.published_at.format("%Y-%m-%d").to_string()
    }

    fn get_sanitized_title(&self) -> String {
        Sanitizer::execute(&self.title).trim().to_owned()
    }

    /// Determine a UUID from `source_id`.
    ///
    /// Matches `source_id` if it's a GUID otherwise it's derived deterministically.
    #[must_use]
    pub fn determine_uuid(source_id: &str) -> Uuid {
        const UUID_NAMESPACE: Uuid = uuid!("a2c7f853-abb1-4ac5-86fa-10b6de5a8386");
        Uuid::try_parse(source_id)
            .unwrap_or_else(|_| Uuid::new_v5(&UUID_NAMESPACE, source_id.as_bytes()))
    }

    #[must_use]
    pub fn example() -> Self {
        Self {
            title: "Lorem ipsum dolor sit amet".to_owned(),
            source_url: "https://example.com/season-1/episode-1.mp3".to_owned(),
            source_file_size: 1024,
            source_content_type: "audio/mpeg".to_owned(),
            id: uuid!("550e8400-e29b-41d4-a716-446655440000"),
            source_id: "550e8400-e29b-41d4-a716-446655440000".to_owned(),
            published_at: DateTime::default(),
            description: Some("Aenean sit amet sem quis velit viverra vestibulum. Vivamus aliquam mattis ipsum, a dignissim elit pulvinar vitae. Aliquam neque risus, tincidunt sit amet elit quis, malesuada ultrices urna.".to_owned()),
            source_duration: None,
            image: Some("https://example.com/image.jpg".to_owned()),
            explicit: None,
            itunes_title: None,
            episode: Some(3),
            season: Some(2),
            kind: Some(EpisodeKind::default()),
        }
    }
}

impl Display for EpisodeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.get_file_stem())
    }
}
