# image-view

Render image from the terminal.

## Overview
This Rust application allows rendering images directly in the terminal. It includes a Docker setup for easy deployment and a GitHub Actions workflow for continuous integration and deployment.

Example: 

```
./image-view ./test.jpeg
```

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