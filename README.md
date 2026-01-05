# image-view

[![CI](https://github.com/nikolareljin/image-view/actions/workflows/rust.yml/badge.svg)](https://github.com/nikolareljin/image-view/actions/workflows/rust.yml)
[![Release](https://github.com/nikolareljin/image-view/actions/workflows/release.yml/badge.svg)](https://github.com/nikolareljin/image-view/actions/workflows/release.yml)
[![Rust Scan](https://github.com/nikolareljin/image-view/actions/workflows/rust-scan.yml/badge.svg)](https://github.com/nikolareljin/image-view/actions/workflows/rust-scan.yml)

Render images directly in your terminal.

## Overview

**image-view** is a Rust-based command-line tool that enables you to display image files (such as JPEG, PNG, etc.) right in your terminal window. It renders images using colored background blocks, making it useful for quick previews without leaving the command line.

Key features:
- **Terminal Image Rendering:** Supports common image formats and displays them using colored background blocks.
- **Cross-Platform:** Runs on Linux, macOS, and Windows (with supported terminals).
- **Docker Support:** Includes a Dockerfile and build script for containerized usage and deployment.
- **CI/CD Integration:** GitHub Actions workflow automates building and packaging for multiple platforms.
- **Gallery Mode:** Browse a directory of images with left/right navigation and copy the current image path.

## Why It’s Useful

When you are working on a remote server or running long terminal workflows, opening a GUI image viewer is often not an option. **image-view** lets you see images inline, make quick visual decisions, and keep moving.

Common scenarios:
- **Server browsing:** Preview a directory of images in the terminal, find the one you want, and copy its full path from Gallery mode for use in scripts or logs.
- **Project cues:** Show project logos in the terminal while running batch tasks so you can visually confirm which project is being processed.

Typical usage:

```bash
image-view ./test.jpeg
```

This command will "render" `test.jpeg` directly in your terminal as a pixelated content.

Example: 

![image](https://github.com/user-attachments/assets/1c98aaac-9ba7-44db-97f6-c2cd713ec813)

Gallery Mode:

```bash
image-view -g ~/Pictures
```

Allows browsing through the images in the given directory (`~/Pictures`) with pressed Arrow Left/Right keys

Grab a full path of the displayed image (if you need to paste it elsewhere) - with `Ctrl+c` / `Cmd+C`.

Exit the gallery mode: press `q` .


## Install (Cargo required)

From Git (recommended):

```bash
cargo install --git https://github.com/nikolareljin/image-view --bin image-view
```

From the repository root:

```bash
cargo install --path ./image-view
```

This installs `image-view` into `~/.cargo/bin`. Ensure it is on your `PATH`.


## Build

- edit `./src/main.rs`
- build: `cargo run ./src/test.jpeg`
- local run (with lint checks): `./run`

## Scripts

Run from repo root unless noted.

`./run`
- `-h`: show help
- `-g [<dir>]`: gallery mode (optional directory)

`./scripts/lint.sh`
- `-h`: show help
- `-f`: auto-fix formatting and clippy where possible, then re-check

`./scripts/build.sh`
- No flags

`./scripts/test.sh`
- No flags

`./setup`
- No flags (uses env vars `INSTALL_DIR`, `PROFILE_FILE`)

`./update`
- No flags

## Local Development

- `./run` runs linting (`cargo fmt --check` and `cargo clippy -D warnings`) before building and running.
- `./setup` configures a pre-commit hook to run `./scripts/lint.sh`.

## Usage

Render a single image:

```bash
image-view <image-path> [-w <width>] [-h <height>] [-a | -c]
```

Gallery mode (browse a directory):

```bash
image-view -g [path]
```

Controls in gallery mode:
- Left/Right arrows: previous/next image
- Ctrl+C (Cmd+C on macOS): copy full path of current image (shows "Copied" under the path)
- q: quit

ASCII modes:
- `-a`: grayscale ASCII art
- `-c`: colorized ASCII art
- ASCII rendering uses doubled monospace characters to preserve horizontal aspect ratio.

### ASCII-art display

ASCII-art (grayscale) mode:

```bash
image-view ./src/test.jpeg -a
```

<img width="1530" height="931" alt="image" src="https://github.com/user-attachments/assets/ef732829-fe61-4bba-8201-93ab11731352" />


ASCII-art in color mode: 

```bash
image-view ./src/test.jpeg -c
```

<img width="1530" height="931" alt="image" src="https://github.com/user-attachments/assets/7c7c9626-1d1c-4afd-8918-40d37f5daedd" />


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
