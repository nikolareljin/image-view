# image-view

Render images directly in your terminal.

## Overview

**image-view** is a Rust-based command-line tool that enables you to display image files (such as JPEG, PNG, etc.) right in your terminal window. It leverages terminal graphics protocols (like Sixel or Kitty graphics) to render images inline, making it useful for quick previews without leaving the command line.

Key features:
- **Terminal Image Rendering:** Supports common image formats and displays them using compatible terminal graphics protocols.
- **Cross-Platform:** Runs on Linux, macOS, and Windows (with supported terminals).
- **Docker Support:** Includes a Dockerfile and build script for containerized usage and deployment.
- **CI/CD Integration:** GitHub Actions workflow automates building and packaging for multiple platforms.

Typical usage:

```bash
./image-view ./test.jpeg
```

This command will render `test.jpeg` directly in your terminal, provided your terminal emulator supports inline image display.

Example: 

```
./image-view ./test.jpeg
```

![image](https://github.com/user-attachments/assets/1c98aaac-9ba7-44db-97f6-c2cd713ec813)


## Build

- edit `./src/main.rs`
- build: `cargo run ./src/test.jpeg`


## Project Structure
- **.github/workflows/release.yml**: Defines the GitHub Actions workflow for building the project and creating binaries for various platforms.
- **src/main.rs**: The main entry point of the Rust application containing the core logic.
- **Cargo.toml**: Configuration file for the Rust project, specifying package details and dependencies.
- **Cargo.lock**: Automatically generated file that locks the versions of dependencies for reproducible builds.
- **Dockerfile**: Instructions for building a Docker image of the application.
- **build.sh**: Shell script that automates the build process, including Docker image creation and container management.

## Building the Project
To build the project locally, ensure you have Rust and Cargo installed. Then run the following command:

```bash
cargo build --release
```

## Running the Application
You can run the application using Docker. First, build the Docker image:

```bash
./build.sh <image_name> <tag>
```

Replace `<image_name>` and `<tag>` with your desired values.

## GitHub Actions
The project includes a GitHub Actions workflow located in `.github/workflows/release.yml`. This workflow automates the process of building the project and creating binaries for multiple platforms, including:
- Windows
- DEB
- Pacman
- Yum
- RedHat
- Mac

## Contributing
Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.
