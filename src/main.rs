//! # Image Viewer
//!
//! A simple Rust program to render images directly in the terminal using colored background blocks.
//!
//! ## Features
//! - Detects terminal size automatically (with optional override via `COLUMNS` environment variable).
//! - Resizes images to fit within the terminal window or user-specified dimensions.
//! - Renders images using colored blocks for a visual representation in the console.
//!
//! ## Usage
//!
//! ```sh
//! image-view <image-path> [options]
//! ```
//!
//! ### Options
//! - `-w <width>`: Set the maximum width for the rendered image.
//! - `-h <height>`: Set the maximum height for the rendered image.
//! - `--help`: Show help message.
//!
//! ### Environment Variables
//! - `COLUMNS`: Override detected terminal width.
//!
//! ### Example
//!
//! ```sh
//! image-view ./my_image.png -w 100 -h 40
//! ```
//!
//! ## Modules and Structs
//! - `Screen`: Handles terminal size detection and provides dimensions for rendering.
//! - `ImageRenderer`: Loads and renders the image in the terminal.
//!
//! ## Dependencies
//! - `image`: For image loading and resizing.
//! - `terminal_size`: For terminal dimension detection.
//! - `colored`: For colored terminal output.
//!
//! ## Limitations
//! - Only supports images that can be loaded by the `image` crate.
//! - Does not upscale images beyond their original size.
//!
//! ## Author
//! - Dragana (as per file path)
//!
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use colored::*;
use crossterm::ExecutableCommand;
use crossterm::cursor;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{self, ClearType};
use image::{GenericImageView, Pixel};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use terminal_size::terminal_size;

// Struct to determine console dimensions and resize the image accordingly.
struct Screen {
    width: u32,
    height: u32,
}
impl Screen {
    fn get_dimensions(&self) -> (u32, u32) {
        if let Some((width, height)) = terminal_size() {
            (width.0 as u32, height.0 as u32)
        } else {
            (self.width, self.height)
        }
    }
}
// Struct to handle image rendering
struct ImageRenderer {
    image: image::DynamicImage,
}
// Implementation for the ImageRenderer
impl ImageRenderer {
    fn new(image_path: &str) -> Self {
        let img = image::open(image_path).expect("Failed to open image");
        ImageRenderer { image: img }
    }

    // Renders the image to the terminal
    fn render(&self, max_width: u32, max_height: u32) {
        let (img_width, img_height) = self.image.dimensions();

        // Each pixel is rendered as two spaces ("  "), so each pixel is 2 columns wide.
        // Therefore, the maximum number of pixels per row is max_width / 2.
        let pixel_width = 2;
        let available_pixels_per_row = if max_width >= pixel_width {
            (max_width / pixel_width).max(1)
        } else {
            1
        };

        // Calculate scale factors for both axes
        let scale_w = available_pixels_per_row as f32 / img_width as f32;
        let scale_h = max_height as f32 / img_height as f32;

        // Scale so that the image fits within both max_width and max_height constraints.
        // Choose the smaller scale factor to ensure both dimensions fit.
        let scale = scale_h.min(scale_w).min(1.0); // Don't upscale

        // Ensure new_width is at least 1 to avoid zero-width images
        let new_width = ((img_width as f32 * scale).round() as u32).max(1);
        let new_height = ((img_height as f32 * scale).round() as u32).max(1);

        let resized_img =
            self.image
                .resize(new_width, new_height, image::imageops::FilterType::Nearest);

        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        for y in 0..resized_img.height() {
            for x in 0..resized_img.width() {
                let pixel = resized_img.get_pixel(x, y);
                let image::Rgba([r, g, b, _]) = pixel.to_rgba();
                let _ = write!(stdout, "{}", "  ".on_truecolor(r, g, b));
            }
            let _ = write!(stdout, "\r\n");
        }
    }
}

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
    }
}

// Print help information
fn print_help(program_name: &str) {
    let copy_hint = if cfg!(target_os = "macos") {
        "Cmd+C"
    } else {
        "Ctrl+C"
    };
    println!(
        "Usage:\n\
         \t{0} <image-path> [options]\n\
         \t{0} -g [path]\n\
         \n\
         Options:\n\
         \t-w <width>     Set max width\n\
         \t-h <height>    Set max height\n\
         \t-g [path]      Gallery mode (left/right to navigate, q to quit, {1} to copy)\n\
         \t--help         Show this help message\n\
         \n\
         Gallery mode controls:\n\
         \tLeft/Right     Previous/next image\n\
         \t{1}            Copy full path of current image\n\
         \tq              Quit gallery\n\
         \n\
         Environment variables:\n\
         \tCOLUMNS        Override detected terminal width\n\
         \tLINES          Override detected terminal height\n\
         \n\
         Examples:\n\
         \t{0} ./my_image.png -w 100 -h 40\n\
         \t{0} -g\n\
         \t{0} -g ./images",
        program_name, copy_hint
    );
}

// Parses command line arguments
fn parse_arg(args: &[String], flag: &str) -> Option<u32> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1).and_then(|v| v.parse::<u32>().ok()))
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

fn parse_gallery_path(args: &[String]) -> Option<String> {
    let pos = args.iter().position(|a| a == "-g")?;
    match args.get(pos + 1) {
        Some(next) if !next.starts_with('-') => return Some(next.clone()),
        _ => {}
    }
    match args.get(1) {
        Some(first) if !first.starts_with('-') && first != "-g" => return Some(first.clone()),
        _ => {}
    }
    None
}

fn is_image_file(path: &Path) -> bool {
    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => ext.to_ascii_lowercase(),
        None => return false,
    };
    matches!(
        ext.as_str(),
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "tiff" | "tif" | "webp" | "tga" | "ico" | "avif"
    )
}

fn list_images(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut images = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && is_image_file(&path) {
            images.push(path);
        }
    }
    images.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    Ok(images)
}

fn render_gallery_image(
    image_path: &Path,
    max_width: u32,
    max_height: u32,
    status: Option<&str>,
) -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    let full_path = fs::canonicalize(image_path).unwrap_or_else(|_| image_path.to_path_buf());
    let footer = format!("Path: {}", full_path.display());
    let mut footer_lines = wrap_footer(&footer, max_width);
    if let Some(status_text) = status.filter(|text| !text.is_empty()) {
        footer_lines.extend(wrap_footer(status_text, max_width));
    }
    let footer_rows = footer_lines.len() as u32;
    let image_height = max_height.saturating_sub(footer_rows).max(1);

    let renderer = ImageRenderer::new(&image_path.to_string_lossy());
    renderer.render(max_width, image_height);

    let mut stdout = stdout.lock();
    for line in footer_lines {
        let _ = write!(stdout, "{}\r\n", line);
    }
    stdout.flush()?;
    Ok(())
}

fn wrap_footer(text: &str, columns: u32) -> Vec<String> {
    if columns == 0 {
        return vec![text.to_string()];
    }
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut count = 0usize;
    let max_columns = columns as usize;
    for ch in text.chars() {
        if count == max_columns {
            lines.push(current);
            current = String::new();
            count = 0;
        }
        current.push(ch);
        count += 1;
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn copy_to_clipboard(text: &str, owner: &mut Option<Child>) -> Result<(), String> {
    if is_x11_session() && copy_with_xclip_owner(text, owner).is_ok() {
        return Ok(());
    }
    if arboard::Clipboard::new()
        .and_then(|mut clipboard| clipboard.set_text(text.to_string()))
        .is_ok()
    {
        cleanup_clipboard_owner(owner);
        return Ok(());
    }
    if copy_to_clipboard_command(text).is_ok() {
        cleanup_clipboard_owner(owner);
        return Ok(());
    }
    copy_to_clipboard_osc52(text)
}

fn is_x11_session() -> bool {
    if env::var("XDG_SESSION_TYPE")
        .map(|value| value.eq_ignore_ascii_case("x11"))
        .unwrap_or(false)
    {
        return true;
    }
    env::var("DISPLAY").map(|v| !v.is_empty()).unwrap_or(false)
}

fn cleanup_clipboard_owner(owner: &mut Option<Child>) {
    if let Some(mut child) = owner.take() {
        let _ = child.kill();
        let _ = child.wait();
    }
}

fn copy_with_xclip_owner(text: &str, owner: &mut Option<Child>) -> Result<(), String> {
    let mut child = Command::new("xclip")
        .args(["-selection", "clipboard", "-i", "-loops", "100"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;
    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| e.to_string())?;
    }
    let _ = child.stdin.take();
    cleanup_clipboard_owner(owner);
    *owner = Some(child);
    Ok(())
}

fn copy_to_clipboard_command(text: &str) -> Result<(), String> {
    if is_wsl() {
        if try_copy_with_command("clip.exe", &[], text).is_ok() {
            return Ok(());
        }
        if try_copy_with_command("cmd.exe", &["/c", "clip"], text).is_ok() {
            return Ok(());
        }
    }
    let candidates: &[(&str, &[&str])] = &[
        ("wl-copy", &[]),
        ("xsel", &["--clipboard", "--input"]),
        ("pbcopy", &[]),
    ];

    for (cmd, args) in candidates {
        if try_copy_with_command(cmd, args, text).is_ok() {
            return Ok(());
        }
    }
    Err("no clipboard command available".to_string())
}

fn try_copy_with_command(cmd: &str, args: &[&str], text: &str) -> Result<(), String> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;
    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| e.to_string())?;
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("command {} failed", cmd))
    }
}

fn copy_to_clipboard_osc52(text: &str) -> Result<(), String> {
    let encoded = BASE64_STANDARD.encode(text.as_bytes());
    let mut stdout = io::stdout();
    stdout
        .write_all(format!("\x1b]52;c;{}\x07", encoded).as_bytes())
        .map_err(|e| e.to_string())?;
    stdout.flush().map_err(|e| e.to_string())?;
    Ok(())
}

fn is_wsl() -> bool {
    if let Ok(value) = env::var("WSL_DISTRO_NAME") {
        return !value.is_empty();
    }
    if let Ok(version) = fs::read_to_string("/proc/version") {
        return version.to_lowercase().contains("microsoft");
    }
    false
}

fn run_gallery(input_path: &Path, max_width: u32, max_height: u32) -> Result<(), String> {
    let dir = if input_path.is_dir() {
        input_path
    } else {
        input_path.parent().unwrap_or(input_path)
    };
    let images = list_images(dir).map_err(|e| e.to_string())?;
    if images.is_empty() {
        return Err(format!("No supported images found in {}", dir.display()));
    }

    let mut index = 0;
    if let Some(found) = input_path
        .is_file()
        .then(|| images.iter().position(|p| p == input_path))
        .flatten()
    {
        index = found;
    }

    let _raw = RawModeGuard::new().map_err(|e| e.to_string())?;
    let render_width = max_width.saturating_sub(1).max(1);
    let mut status: Option<String> = None;
    let mut clipboard_owner: Option<Child> = None;
    loop {
        render_gallery_image(&images[index], render_width, max_height, status.as_deref())
            .map_err(|e| e.to_string())?;
        status = None;

        if let Event::Key(key_event) = event::read().map_err(|e| e.to_string())? {
            match key_event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('c') if is_copy_shortcut(&key_event) => {
                    let full_path =
                        fs::canonicalize(&images[index]).unwrap_or_else(|_| images[index].clone());
                    match copy_to_clipboard(&full_path.to_string_lossy(), &mut clipboard_owner) {
                        Ok(()) => status = Some("Copied".to_string()),
                        Err(err) => status = Some(format!("Copy failed: {}", err)),
                    }
                }
                KeyCode::Left => {
                    if index == 0 {
                        index = images.len() - 1;
                    } else {
                        index -= 1;
                    }
                }
                KeyCode::Right => {
                    index = (index + 1) % images.len();
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn is_copy_shortcut(key_event: &crossterm::event::KeyEvent) -> bool {
    if cfg!(target_os = "macos") {
        key_event.modifiers.contains(KeyModifiers::SUPER)
    } else {
        key_event.modifiers.contains(KeyModifiers::CONTROL)
    }
}

// Main function
fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() >= 2 && args[1] == "--help") || (args.len() == 2 && args[1] == "-h") {
        print_help(&args[0]);
        std::process::exit(0);
    }
    let gallery_mode = has_flag(&args, "-g");
    if !gallery_mode && args.len() < 2 {
        eprintln!(
            "Usage: {} <image-path> [options]\nTry --help for more information.",
            args[0]
        );
        std::process::exit(1);
    }

    // Parse optional width and height
    let max_width = parse_arg(&args, "-w");
    let max_height = parse_arg(&args, "-h");

    // Get terminal size as fallback
    let screen = Screen {
        width: 80,
        height: 40,
    };
    let (screen_width, screen_height) = screen.get_dimensions();

    // Use COLUMNS env var if set, else terminal width
    let env_width = env::var("COLUMNS")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(screen_width);

    let env_height = env::var("LINES")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(screen_height);

    // Final constraints
    let final_width = max_width.unwrap_or(env_width);
    let final_height = max_height.unwrap_or(env_height);

    if gallery_mode {
        let gallery_path = parse_gallery_path(&args).unwrap_or_else(|| ".".to_string());
        if let Err(err) = run_gallery(Path::new(&gallery_path), final_width, final_height) {
            eprintln!("Gallery mode error: {}", err);
            std::process::exit(1);
        }
    } else {
        let image_path = &args[1];
        let renderer = ImageRenderer::new(image_path);
        renderer.render(final_width, final_height);
    }
}
