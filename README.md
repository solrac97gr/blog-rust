# Blog Rust

A modern blog application built with Rust using clean architecture principles. This project demonstrates the implementation of a web API with Actix-Web, Diesel ORM, and SQLite database.

## 🚀 Features

- **Complete CRUD API** for blog post management
  - Create new posts
  - Read all posts or get specific post by ID
  - Update existing posts
  - Delete posts
- **Clean Architecture** implementation (Domain, Application, Infrastructure)
- **SQLite database** with Diesel ORM
- **Connection pooling** with R2D2
- **Database migrations** support
- **JSON API** with proper HTTP status codes
- **Error handling** with meaningful error messages
- **Web server** with Actix-Web framework

## 🏗️ Architecture

This project follows Clean Architecture principles with the following layers:

```
src/
├── domain/           # Business logic and entities
│   ├── entities/     # Domain entities
│   └── ports/        # Repository interfaces
├── application/      # Application use cases
│   └── use_cases/    # Business use cases
├── infrastructure/   # External concerns
│   ├── adapters/     # Adapters for external services
│   └── drivers/      # Database drivers and external tools
├── models.rs         # Database models
├── schema.rs         # Database schema
└── main.rs          # Application entry point
```

## 📋 Prerequisites

- Rust (latest stable version)
- SQLite
- Diesel CLI (for database migrations)

## 🛠️ Installation

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd blog-rust
   ```

2. **Install dependencies:**
   ```bash
   cargo build
   ```

3. **Install Diesel CLI:**
   ```bash
   cargo install diesel_cli --no-default-features --features sqlite
   ```

4. **Set up environment variables:**
   Create a `.env` file in the project root:
   ```env
   DATABASE_URL=sqlite:blog.db
   ```

5. **Run database migrations:**
   ```bash
   diesel migration run
   ```

## 🚀 Usage

### Running the Application

```bash
cargo run
```

The server will start on `http://localhost:8080` (or the configured port).

### Available Endpoints

#### Posts API (CRUD Operations)

- **GET /posts** - Get all blog posts
  ```bash
  curl -X GET http://localhost:8080/posts
  ```

- **GET /posts/{id}** - Get a specific post by ID
  ```bash
  curl -X GET http://localhost:8080/posts/1
  ```

- **POST /posts** - Create a new blog post
  ```bash
  curl -X POST http://localhost:8080/posts \
    -H "Content-Type: application/json" \
    -d '{
      "title": "My Blog Post",
      "body": "This is the content of my blog post.",
      "slug": "my-blog-post"
    }'
  ```

- **PUT /posts/{id}** - Update an existing post
  ```bash
  curl -X PUT http://localhost:8080/posts/1 \
    -H "Content-Type: application/json" \
    -d '{
      "title": "Updated Title",
      "body": "Updated content."
    }'
  ```

- **DELETE /posts/{id}** - Delete a post
  ```bash
  curl -X DELETE http://localhost:8080/posts/1
  ```

#### Other Endpoints

- `GET /` - Hello world endpoint
- `POST /echo` - Echo endpoint for testing

### Database Operations

The application includes:
- Post creation and management
- Connection pooling for efficient database access
- Automated schema generation with Diesel

## 🗄️ Database Schema

### Posts Table

```sql
CREATE TABLE posts (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title VARCHAR NOT NULL,
  slug VARCHAR NOT NULL,
  body TEXT NOT NULL
)
```

## 🧪 Testing the API

Once the server is running, you can test all CRUD operations:

### Quick Demo

Run the provided demo script to test all endpoints:
```bash
./examples/api_demo.sh
```

### Manual Testing

### 1. Get all posts
```bash
curl -X GET http://localhost:8080/posts
```

### 2. Create a new post
```bash
curl -X POST http://localhost:8080/posts \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My First Post",
    "body": "This is my first blog post content!",
    "slug": "my-first-post"
  }'
```

### 3. Get a specific post
```bash
curl -X GET http://localhost:8080/posts/1
```

### 4. Update a post
```bash
curl -X PUT http://localhost:8080/posts/1 \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Post Title",
    "body": "This is the updated content."
  }'
```

### 5. Delete a post
```bash
curl -X DELETE http://localhost:8080/posts/1
```

## 🔧 Development

### Running Tests

```bash
cargo test
```

### Database Migrations

Create a new migration:
```bash
diesel migration generate <migration_name>
```

Run migrations:
```bash
diesel migration run
```

Rollback migrations:
```bash
diesel migration revert
```

### Code Structure

- **Models**: Database models are defined in `src/models.rs`
- **Schema**: Auto-generated database schema in `src/schema.rs`
- **Posts**: Post-related operations in `src/posts.rs`
- **Main**: Application entry point and server configuration

## 📦 Dependencies

- **actix-web**: Web framework for building HTTP services
- **diesel**: ORM and query builder for Rust
- **dotenvy**: Environment variable loading
- **libsqlite3-sys**: SQLite bindings
- **r2d2**: Connection pooling

## 🏃‍♂️ Getting Started (Quick Start)

1. Make sure you have Rust installed
2. Clone the project and navigate to the directory
3. Run `cargo build` to install dependencies
4. Create a `.env` file with `DATABASE_URL=sqlite:blog.db`
5. Install Diesel CLI: `cargo install diesel_cli --no-default-features --features sqlite`
6. Run migrations: `diesel migration run`
7. Start the application: `cargo run`
8. Visit `http://localhost:8080` to see the hello world message
9. Test the CRUD API endpoints at `http://localhost:8080/posts`
10. Run the demo script: `./examples/api_demo.sh`

## 📝 Project Status

This project is part of a Platzi Rust course and serves as a learning exercise for:
- Clean Architecture in Rust
- Web API development with Actix-Web
- Database integration with Diesel ORM
- Modern Rust development practices

## 🤝 Contributing

This is an educational project. Feel free to fork and experiment with the code.

## 📄 License

This project is for educational purposes.

---

*Built with ❤️ using Rust*
