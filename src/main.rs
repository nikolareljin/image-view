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
use image::{GenericImageView, Pixel};
use std::env;
use terminal_size::terminal_size;
use colored::*;

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

        let resized_img = self.image.resize(new_width, new_height, image::imageops::FilterType::Nearest);

        for y in 0..resized_img.height() {
            for x in 0..resized_img.width() {
                let pixel = resized_img.get_pixel(x, y);
                let image::Rgba([r, g, b, _]) = pixel.to_rgba();
                print!("{}", "  ".on_truecolor(r, g, b));
            }
            println!("{}", "".clear());
        }
    }
}

// Print help information
fn print_help(program_name: &str) {
    println!(
        "Usage: {} <image-path> [options]\n\
         Options:\n\
         \t-w <width>\tSet max width\n\
         \t-h <height>\tSet max height\n\
         \t--help\t\tShow this help message\n\
         \n\
         Environment variables:\n\
         \tCOLUMNS\t\tOverride detected terminal width\n\
         \n\
         Example:\n\
         \t{} ./my_image.png -w 100 -h 40",
        program_name, program_name
    );
}

// Parses command line arguments
fn parse_arg(args: &[String], flag: &str) -> Option<u32> {
    args.iter().position(|a| a == flag).and_then(|i| {
        args.get(i + 1)
            .and_then(|v| v.parse::<u32>().ok())
    })
}

// Main function
fn main() {
    let args: Vec<String> = env::args().collect();
    if (args.len() >= 2 && args[1] == "--help") || (args.len() == 2 && args[1] == "-h") {
        print_help(&args[0]);
        std::process::exit(0);
    }
    if args.len() < 2 {
        eprintln!("Usage: {} <image-path> [options]\nTry --help for more information.", args[0]);
        std::process::exit(1);
    }
    let image_path = &args[1];

    // Parse optional width and height
    let max_width = parse_arg(&args, "-w");
    let max_height = parse_arg(&args, "-h");

    // Get terminal size as fallback
    let screen = Screen { width: 80, height: 40 };
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

    let renderer = ImageRenderer::new(image_path);
    renderer.render(final_width, final_height);
}
