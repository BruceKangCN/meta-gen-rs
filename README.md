# Rusty FFMPEG Meta

[中文](README-zh_CN.md)

A track generator for XSPF music playlists.

## Usage

create `rfm.toml` in `${XDG_CONFIG_HOME}` directory (usually `${HOME}/.config` on *nix operating systems, or `C:\Users\<username>\config` on Windows systems), set `indent`, `indent_level`, `base_dir`. For example:

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

Install `ffmpeg` using your system package manager or vcpkg.

Then build using cargo, link with system ffmpeg:

```sh
$ cargo build --release --features rusty_ffmpeg/link_system_ffmpeg
```

or with vcpkg ffmpeg:

```sh
$ cargo build --release --features rusty_ffmpeg/link_vcpkg_ffmpeg
```
