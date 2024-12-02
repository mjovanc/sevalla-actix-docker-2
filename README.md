# sevalla-actix-docker

This project demonstrates how to create a simple Actix Web API in Rust and containerize it using Docker. The API performs a basic computation and returns a JSON response, including a message powered by Sevalla Hosting.

## Project Structure

- `Cargo.toml`: Contains the project dependencies and metadata.
- `src/main.rs`: The main source file for the Actix Web API.
- `Dockerfile`: Instructions to build and run the Docker container.

## Dependencies

- Rust
- Actix Web
- Serde
- Serde JSON

## Endpoints

### GET /compute

Performs a simple computation and returns the result along with a message powered by Sevalla Hosting.

**Response:**
```json
{
  "message": "The result of the computation is: 42",
  "powered_by": "Sevalla Hosting"
}
```

## Building and Running

To build and run the Actix Web API locally, use the following commands:

1. Build the project:
```sh
cargo build
```

2. Run the project:
```sh
cargo run
```

The API will be accessible at `http://localhost:8080`.

## Containerization

To containerize the Actix Web API using Docker, follow these steps:

1. Build the Docker image:
```sh
docker build -t sevalla-actix-docker .
```

2. Run the Docker container:
```sh
docker run -p 8080:8080 sevalla-actix-docker
```