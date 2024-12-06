# sevalla-actix-docker-2

This second project demonstrates how to create a simple Actix Web API in Rust and containerize it using Docker.
The API performs a basic computation, stores and retrieves data from a PostgreSQL database, and returns JSON responses, including a message powered by Sevalla Hosting.

## Project Structure

- `Cargo.toml`: Contains the project dependencies and metadata.
- `src/main.rs`: The main source file for the Actix Web API.
- `Dockerfile`: Instructions to build and run the Docker container.
- `migrations`: Directory containing SQL migration files.

## Dependencies

- Rust
- Actix Web
- Serde
- Serde JSON
- SQLx
- dotenv

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

### POST /items

Stores data in the PostgreSQL database and returns a success message.

**Request:**
```json
{
  "name": "Item name"
}
```

**Response:**
```json
{
  "id": 1,
  "name": "Item name"
}
```

### GET /items/{id}

Retrieves data from the PostgreSQL database based on the provided ID.

**Response:**
```json
{
  "id": 1,
  "name": "Item name"
}
```

## Environment Variables

The application uses environment variables to configure the database connection. Create a `.env` file in the project root directory with the following variables:

```
DATABASE_URL=postgres://username:password@host:port/database
```

## SQLx Migrations

The application uses SQLx to manage database migrations. To create a new migration, run the following command:

```bash
sqlx migrate add migration_name
```

To apply the migrations, run:

```bash
sqlx migrate run
```

## Building and Running

1. Clone the repository: `git clone
2. Change into the project directory: `cd sevalla-actix-docker-2`
3. Build the application: `cargo build`
4. Run the application: `cargo run`
5. Access the API at `http://localhost:8080`

## Running the Application with Docker

1. Clone the repository: `git clone
2. Change into the project directory: `cd sevalla-actix-docker-2`
3. Build the Docker image: `docker build -t sevalla-actix-docker-2 .`
4. Run the Docker container: `docker run -p 8080:8080 sevalla-actix-docker-2`
5. Access the API at `http://localhost:8080`

## License

This project is open source and available under the [MIT License](LICENSE).