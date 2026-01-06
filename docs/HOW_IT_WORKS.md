# How It Works

image-view loads an image, scales it to the terminal, and renders it as either colored blocks or ASCII characters.

## Rendering flow

1. Detect terminal size (or use `COLUMNS`/`LINES` overrides).
2. Compute a scale factor to fit the image within the max width and height.
3. Resize the image using a high-quality filter.
4. Render each pixel:
   - Blocks mode: two spaces with a colored background.
   - ASCII mode: map pixel luminance to a shade character.
   - ASCII color: shade character tinted by the pixel color.

## Gallery mode

Gallery mode scans the target directory for supported image extensions, sorts by filename, and draws a single image plus footer text. Input is read in raw mode so arrow keys and copy shortcuts work without extra prompts.

## Clipboard support

Copy in gallery mode tries these options in order:

1. X11 clipboard with `xclip` (Linux, when available).
2. Native clipboard via the Rust `arboard` crate.
3. OS commands like `pbcopy` (macOS), `wl-copy` (Wayland), `xsel` (X11).
4. OSC52 escape sequence as a fallback for compatible terminals.
