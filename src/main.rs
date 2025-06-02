use image::{GenericImageView, Pixel};
use std::env;
use terminal_size::terminal_size;

// Helper struct to get the dimensions of the console
// This struct will be used to get the dimensions of the console
// and will be used to resize the image to fit the console width.
struct Screen {
    width: u32,
    height: u32,
}
impl Screen {
    // Get the dimensions of the console
    fn get_dimensions(&self) -> (u32, u32) {
        if let Some((width, height)) = terminal_size() {
            (width.0 as u32, height.0 as u32)
        } else {
            println!("Unable to get terminal size");
            (self.width, self.height)
        }
    }
}

// Class that will render an image in the console
// using ANSI escape codes for colors.
// This class will take an image path as input and render the image in the console.
// It will resize the image to fit the console width and maintain the aspect ratio.
struct ImageRenderer {
    image: image::DynamicImage,
}
impl ImageRenderer {
    fn new(image_path: &str) -> Self {
        // Load the image from the given path
        let img = image::open(image_path).expect("Failed to open image");
                
        ImageRenderer { image: img }
    }

    fn render(&self, fixed_width: u32) {
        // Get the dimensions of the image
        let (img_width, img_height) = self.image.dimensions(); // Get Image dimensions.

        // Calculate the scaling factor
        let scale = fixed_width as f32 / img_width as f32;
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

// Function to print help message
fn print_help(program_name: &str) {
    println!(
        "Usage: {} <image-path>\n\
         Options:\n\
         \t--help\t\tShow this help message\n\
         \n\
         Environment variables:\n\
         \tCOLUMNS\t\tOverride detected terminal width\n\
         \n\
         Example:\n\
         \t{} ./my_image.png",
        program_name, program_name
    );
}

fn main() {
    // Get the image path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        print_help(&args[0]);
        std::process::exit(0);
    }
    if args.len() != 2 {
        eprintln!("Usage: {} <image-path>\nTry --help for more information.", args[0]);
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

    // Get width and height of the available console space
    let screen = Screen {
        width: 80, // Adjust this for your console width
        height: 40, // Adjust this for your console height
    };
    
    let (screen_width, screen_height) = screen.get_dimensions(); // Get the dimensions of the console.

    // Use either console_width or screen_width, whichever is smaller
    let fixed_width = if console_width < screen_width {
        console_width
    } else {
        screen_width
    };

    // Print the dimensions of the console
    println!("Console dimensions: {}x{}", screen_width, screen_height);

    // Render the image
    renderer.render(fixed_width);
}