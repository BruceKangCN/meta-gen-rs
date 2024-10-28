pub mod util;

use util::{Serializer, Track};

pub fn init() -> Result<(), ffmpeg_next::Error> {
    ffmpeg_next::init()
}

/// get `artist`, `title`, `album` from file, returns a `Track` object.
pub fn generate_info(
    location: &str,
    serializer: &Serializer,
) -> Result<String, ffmpeg_next::Error> {
    let ctx = ffmpeg_next::format::input(location)?;
    let dict = ctx.metadata();

    let location = location.to_owned();
    let title = dict.get("title").map(str::to_owned);
    let creator = dict.get("artist").map(str::to_owned);
    let album = dict.get("album").map(str::to_owned);

    let track = Track {
        location,
        title,
        creator,
        album,
    };

    let info = format!("{}", serializer.to_owned().serialize(&track));

    Ok(info)
}
