pub mod util;

use ffmpeg_next::Rational;
use ffmpeg_next::util::mathematics::rescale::TIME_BASE;
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
    let bitrate = ctx.bit_rate() / 1000; // in kbps

    // get duration in seconds (in rational representation), then convert to
    // milliseconds in `i64` representation.
    let duration: f64 = (Rational::new(ctx.duration() as i32, 1) * TIME_BASE).into();
    let duration = (duration * 1000.0) as i64;

    let track = Track {
        location,
        title,
        creator,
        album,
        bitrate,
        duration,
    };

    let info = format!("{}", serializer.to_owned().serialize(&track));

    Ok(info)
}
