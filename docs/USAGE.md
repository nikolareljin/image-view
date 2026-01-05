# Usage

## Basic

```bash
image-view <image-path> [options]
```

Render an image directly in the terminal with colored blocks.

## Options

- `-w <width>`: Set max width in terminal columns.
- `-h <height>`: Set max height in terminal rows.
- `-a`: ASCII art (grayscale).
- `-c`: ASCII art with color.
- `-g [path]`: Gallery mode for a directory (left/right to navigate, `q` to quit, copy shortcut to copy current path).
- `--help`: Show help.

## Gallery mode

```bash
image-view -g
image-view -g ./images
```

If `-g` is provided without a path, the current directory is used. If `-g` is provided with a file path, the gallery opens that file's directory and starts on the file when possible.

Controls:
- Left/Right arrows: previous/next image.
- Ctrl+C (Cmd+C on macOS): copy full path of current image.
- `q`: quit.

## Environment variables

- `COLUMNS`: override detected terminal width.
- `LINES`: override detected terminal height.

## Examples

```bash
image-view ./test.jpeg
image-view ./test.jpeg -w 120 -h 40
image-view ./test.jpeg -a
image-view ./test.jpeg -c
image-view -g ~/Pictures
```
