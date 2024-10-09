use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Track {
    pub location: String,
    pub title: Option<String>,
    pub creator: Option<String>,
    pub album: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SerializationConfig {
    pub indent: String,
    pub indent_level: usize,
    // TODO: base_dir: String
}

#[derive(Clone, Debug)]
pub struct Serializer {
    pub config: SerializationConfig,
}

impl Serializer {
    /// serialize track information to XSPF track entity
    pub fn serialize(self, track: &Track) -> String {
        let config = self.config;
        let indent = config.indent;
        let pre_indent = indent.repeat(config.indent_level);

        let Track {
            location,
            title,
            creator,
            album,
        } = track.to_owned();
        let location = urlencoding::encode(&location).into_owned();

        let title = title.unwrap_or_default();
        let title = html_escape::encode_text(&title).into_owned();
        let creator = creator.unwrap_or_default();
        let creator = html_escape::encode_text(&creator).into_owned();
        let album = album.unwrap_or_default();
        let album = html_escape::encode_text(&album).into_owned();

        format!(
            "\
{pre_indent}<track>
{pre_indent}{indent}<location>{location}</location>
{pre_indent}{indent}<title>{title}</title>
{pre_indent}{indent}<creator>{creator}</creator>
{pre_indent}{indent}<album>{album}</album>
{pre_indent}</track>
"
        )
    }
}
