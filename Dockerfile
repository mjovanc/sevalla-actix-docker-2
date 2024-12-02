# Use the official Rust image as the base image
FROM rust:1.83 as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a minimal base image for the final container
FROM debian:buster-slim

# Set the working directory
WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/sevalla-actix-docker .

# Expose the port the app runs on
EXPOSE 8080

# Run the application
CMD ["./sevalla-actix-docker"]