pub mod util;

use rusty_ffmpeg::ffi::{
    av_dict_get, avformat_find_stream_info, avformat_open_input, AVFormatContext,
};
use std::{
    ffi::CString,
    ptr::{null, null_mut},
};
use util::{Serializer, Track};

pub fn generate_info(location: &str, serializer: &Serializer) -> Result<String, String> {
    let mut ctx = null_mut::<AVFormatContext>();
    unsafe {
        let ret = avformat_open_input(
            &mut ctx,
            CString::new(location.to_owned()).unwrap().into_raw(),
            null(),
            null_mut(),
        );
        if ret < 0 {
            return Err("Cannot open input file".into());
        }

        let ret = avformat_find_stream_info(ctx, null_mut());
        if ret < 0 {
            return Err("Cannot find stream information".into());
        }

        if (*ctx).metadata.is_null() {
            return Err("No metadata".into());
        }
    };

    let location = location.to_owned();
    let creator = dict_tag_value(ctx, "artist");
    let title = dict_tag_value(ctx, "title");
    let album = dict_tag_value(ctx, "album");

    let track = Track {
        location,
        title,
        creator,
        album,
    };

    let info = format!("{}", serializer.to_owned().serialize(&track));

    Ok(info)
}

pub fn dict_tag_value(ctx: *const AVFormatContext, tag: &str) -> Option<String> {
    unsafe {
        let tag = av_dict_get(
            (*ctx).metadata,
            CString::new(tag).unwrap().into_raw(),
            null(),
            0,
        );

        if tag.is_null() {
            None
        } else {
            Some(
                CString::from_raw((*tag).value)
                    .to_str()
                    .unwrap()
                    .to_string(),
            )
        }
    }
}
