use image::{GenericImageView, Pixel};
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

// Implement class that will render an image within the available width of the terminal.
struct ImageRenderer {
    image: image::DynamicImage,
}
struct Screen {
    width: u32,
    height: u32,
}
impl ImageRenderer {
    fn new(image_path: &str) -> Self {
        // Load the image from the given path
        let img = image::open(image_path).expect("Failed to open image");
        
        // Get width and height of the available console space
        let screen = Screen {
            width: 80, // Adjust this for your console width
            height: 40, // Adjust this for your console height
        };

        // Grab current terminal width and height.
        
        
        ImageRenderer { image: img }
    }

    fn render(&self, console_width: u32) {
        // Get the dimensions of the image
        let (img_width, img_height) = self.image.dimensions();

        // Calculate the scaling factor
        let scale = console_width as f32 / img_width as f32;
        let new_width = (img_width as f32 * scale) as u32;
        let new_height = (img_height as f32 * scale) as u32;

        // Resize the image to fit the console width
        let resized_img = self.image.resize(new_width, new_height, image::imageops::FilterType::Nearest);

        // Render the resized image in the console
        for y in 0..resized_img.height() {
            for x in 0..resized_img.width() {
                let pixel = resized_img.get_pixel(x, y);
                let image::Rgba([r, g, b, _]) = pixel.to_rgba();
                print!(
                    "\x1b[48;2;{};{};{}m  ",
                    r, g, b
                ); // ANSI escape code for background color
            }
            println!("\x1b[0m"); // Reset color at the end of the line
        }
    }
}


fn main() {
    // Get the image path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <image-path>", args[0]);
        std::process::exit(1);
    }
    let image_path = &args[1];

    // Use ImageRenderer to render the image
    let renderer = ImageRenderer::new(image_path);

    // Get the console width from the environment variable or set a default value
    let console_width = env::var("COLUMNS")
        .unwrap_or_else(|_| "80".to_string())
        .parse::<u32>()
        .unwrap_or(80);

    // Get the console width from the environment variable or set a default value
    let console_width = 80; // Adjust this for your console width
    // Render the image
    renderer.render(console_width);
}