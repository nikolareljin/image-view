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
        let screen: Screen = Screen {
            width: 80, // Adjust this for your console width
            height: 40, // Adjust this for your console height
        };
        // Get screen dimensions
        
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
        let resized_img = self.image.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);

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

    // Load the image
    let img = image::open(image_path).expect("Failed to open image");

    // Get the width of the console
    let console_width = 80; // Adjust this for your console width
    let console_height = 40; // Adjust this for your console height
    let (img_width, img_height) = img.dimensions();
    
    if img_width > console_width || img_height > console_height {
        // Scale down the image to fit the console
        let scale_x = console_width as f32 / img_width as f32;
        let scale_y = console_height as f32 / img_height as f32;
        let scale = scale_x.min(scale_y);
        let new_width = (img_width as f32 * scale) as u32;
        let new_height = (img_height as f32 * scale) as u32;
        let img = img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);
        // Print the new dimensions
        println!("Resized image dimensions: {}x{}", new_width, new_height);
    }

    if img_width > console_width {
        // Scale down the image to fit the console width
        let scale = console_width as f32 / img_width as f32;
        let new_width = (img_width as f32 * scale) as u32;
        let new_height = (img_height as f32 * scale) as u32;
        let img = img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);
        // Print the new dimensions
        println!("Resized image dimensions: {}x{}", new_width, new_height);
    }
    if img_height > console_height {
        // Scale down the image to fit the console height
        let scale = console_height as f32 / img_height as f32;
        let new_width = (img_width as f32 * scale) as u32;
        let new_height = (img_height as f32 * scale) as u32;
        let img = img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);
        // Print the new dimensions
        println!("Resized image dimensions: {}x{}", new_width, new_height);
    }
    // Make the new (displayed) image dimensions in the same aspect ratio as the original image.
    // Use the Width of the available console space and the height of the image to calculate the new height.
    // This is a simple way to maintain the aspect ratio.
    // The console width is 80 characters, and each character is approximately twice as tall as it is wide.
    // The new height is calculated by multiplying the original height by the ratio of the console width to the original width.
    // This is a simple way to maintain the aspect ratio.
    let new_height = (img_height as f32 * (console_width as f32 / img_width as f32)) as u32;
    let new_width = console_width as u32;
    let img = img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);
    // Print the new dimensions
    println!("Resized image dimensions: {}x{}", new_width, new_height);
    
    // Convert the image to RGBA format
    let img = img.to_rgba8();
    // Get the pixel data
    let pixel_data = img.pixels().collect::<Vec<_>>();
    // Print the pixel data
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let image::Rgba([r, g, b, _]) = pixel.to_rgba();
            print!(
                "\x1b[48;2;{};{};{}m  ",
                r, g, b
            ); // ANSI escape code for background color
        }
        println!("\x1b[0m"); // Reset color at the end of the line
    }

    // Resize the image to fit the console
    let (width, height) = img.dimensions();
    let new_width = 80; // Adjust this for your console width
    let new_height = (height * new_width) / (2 * width); // Maintain aspect ratio
    let resized_img = img.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest);

    // Render the image in the console
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