# Dockerfile for image-view
FROM rust:latest
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .
CMD ["image-view"]
