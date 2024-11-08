use std::path::Path;

use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Track {
    pub location: String,
    pub title: Option<String>,
    pub creator: Option<String>,
    pub album: Option<String>,
    pub bitrate: i64,
    pub duration: i64,
}

/// Configurations to control output style.
///
/// - indent: controls which string is used to indent tags.
///
///       - `indent = "  "`:
///
///           ```sh
///           <track>
///             <location>...</location>
///             ...
///           </track>
///           ```
///
///       `indent = "    "`:
///
///           ```sh
///           <track>
///               <location>...</location>
///               ...
///           </track>
///           ```
///
/// - indent_level: controls how many indent level is added at the start of each
/// line. for example, when `indent = "  "`:
///
///     - `indent_level = 0`:
///
///         ```sh
///         <track>
///           ...
///         </track>
///         ```
///
///     - `indent_level = 3`:
///
///         ```sh
///               <track>
///                 ...
///               </track>
///         ```
/// - base_dir: base directory for music library. locations will try to use
/// relative path according to it. for example, when `base_dir = "/foo/music"`,
/// file "/foo/music/bar.flac" will generate `<location>bar.flac</location>`,
/// while "/baz/blah.flac" will generate `<location>/baz/blah.flac</location>`.
#[derive(Deserialize, Clone, Debug)]
pub struct SerializationConfig {
    pub indent: String,
    pub indent_level: usize,
    pub base_dir: String,
}

#[derive(Clone, Debug)]
pub struct Serializer {
    pub config: SerializationConfig,
}

impl Serializer {
    /// serialize track information to XSPF track entity
    ///
    /// if field is `None`, empty string is used.
    pub fn serialize(self, track: &Track) -> String {
        let config = self.config;
        let indent = config.indent;
        let pre_indent = indent.repeat(config.indent_level);

        let Track {
            location,
            title,
            creator,
            album,
            bitrate,
            duration,
        } = track.to_owned();
        let location = match Path::new(&location).strip_prefix(&config.base_dir) {
            Ok(path) => path,
            Err(_) => Path::new(&location),
        };
        let location = location
            .components()
            .map(|s| s.as_os_str())
            .map(|s| s.to_os_string())
            .map(|s| s.into_string().unwrap())
            .map(|s| urlencoding::encode(&s).into_owned())
            .collect::<Vec<String>>()
            .join("/");

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
{pre_indent}{indent}<duration>{duration}</duration>
{pre_indent}{indent}<meta rel=\"bitrate\">{bitrate}</meta>
{pre_indent}</track>
"
        )
    }
}
