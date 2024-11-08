# Rusty FFMPEG Meta

[中文](README-zh_CN.md)

A track generator for XSPF music playlists, mainly used for Audacious player.

## Usage

create `rfm.toml` in `${XDG_CONFIG_HOME}` directory (usually `${HOME}/.config` on *nix operating systems, or `C:\Users\<username>\AppData\Roaming` on Windows systems), set `indent`, `indent_level`, `base_dir`. For example:

```toml
indent = "    "
indent_level = 3
base_dir = "/home/jack/music"
```

Then you can use the cli program like:

```sh
$ rusty-ffmpeg-meta [OPTIONS] [FILES]...
```

See more help with the `--help` option.

## Build

Install `ffmpeg` using your system package manager.

Then build using cargo, link with system ffmpeg:

```sh
$ cargo build --release
```

Note: You may need to download FFmpeg DLL files using vcpkg on Windows systems, and put them in the same directory of the executable.
