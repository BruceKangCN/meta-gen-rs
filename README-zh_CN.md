# Rusty FFMPEG Meta

适用于 XSPF 音乐播放列表的 track 生成器。

## 使用方法

在 `${XDG_CONFIG_HOME}` 目录下（在 *nix 系统通常位于 `${HOME}/.config`，或在 Windows 系统通常位于 `C:\Users\<username>\config`）创建 `rfm.toml` 文件，设置 `indent`, `indent_level`, `base_dir`，例如：

```toml
indent = "    "
indent_level = 3
base_dir = "/home/jack/music"
```

之后可以按照如下方式使用命令行界面：

```sh
$ rusty-ffmpeg-meta [OPTIONS] [FILES]...
```

使用 `--help` 选项以获取更多帮助。

## 构建

使用系统包管理器或 vcpkg 安装 `ffmpeg`。

之后使用 cargo 构建，链接系统 ffmpeg：

```sh
$ cargo build --release --features rusty_ffmpeg/link_system_ffmpeg
```

或链接通过 vcpkg 安装的 ffmpeg：

```sh
$ cargo build --release --features rusty_ffmpeg/link_vcpkg_ffmpeg
```
