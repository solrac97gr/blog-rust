# Blog Rust - Hexagonal Architecture

A modern blog application built with Rust implementing **Hexagonal Architecture** (Ports and Adapters pattern). This project demonstrates clean architecture principles, dependency inversion, and separation of concerns in a production-ready web API.

## 🚀 Features

- **Complete CRUD API** for blog post management
  - Create new posts with validation
  - Read all posts or get specific post by ID
  - Update existing posts
  - Delete posts with proper error handling
- **Hexagonal Architecture** implementation (Domain, Application, Infrastructure)
- **Domain-Driven Design** with rich domain entities
- **Dependency Injection** with trait-based repositories
- **Async/Await** support throughout the application
- **SQLite database** with Diesel ORM
- **Connection pooling** with R2D2
- **Database migrations** support
- **JSON API** with proper HTTP status codes
- **Comprehensive error handling** with meaningful error messages
- **Web server** with Actix-Web framework

## 🏗️ Hexagonal Architecture

This project implements **Hexagonal Architecture** (also known as Ports and Adapters) which provides:

- **Domain isolation**: Core business logic independent of external concerns
- **Dependency inversion**: Infrastructure depends on domain, not vice versa
- **Testability**: Easy to mock and test each layer independently
- **Flexibility**: Easy to swap implementations (e.g., different databases)

### Architecture Layers

```
src/
├── domain/              # 🔵 CORE: Business logic (innermost layer)
│   ├── entities/        # Domain entities (Post)
│   │   ├── mod.rs
│   │   └── post.rs     # Post entity with business rules
│   └── ports/          # Interfaces/Contracts (Repository traits)
│       ├── mod.rs
│       └── post_repository.rs  # PostRepository trait
├── application/         # 🟡 APPLICATION: Use cases
│   ├── mod.rs
│   └── use_cases/
│       ├── mod.rs
│       └── post_service.rs    # Business use cases orchestration
├── infrastructure/      # 🔴 EXTERNAL: Adapters (outermost layer)
│   ├── mod.rs
│   ├── database/        # Database connection utilities
│   │   ├── mod.rs
│   │   └── connection.rs
│   ├── persistence/     # Database adapters
│   │   ├── mod.rs
│   │   ├── models.rs    # ORM models (Diesel)
│   │   └── sqlite_post_repository.rs  # Repository implementation
│   └── web/            # HTTP adapters
│       ├── mod.rs
│       ├── handlers/    # HTTP request handlers
│       │   ├── mod.rs
│       │   └── post_handler.rs
│       └── models/      # DTOs for HTTP
│           ├── mod.rs
│           └── post_dto.rs
├── schema.rs           # Database schema (Diesel generated)
├── lib.rs             # Module exports
└── main.rs            # Application bootstrap and DI container
```

### Dependency Flow

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   HTTP Client   │───▶│   Web Handler   │───▶│  Post Service   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                                        │
                                                        ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│    Database     │◀───│  SQLite Repo    │◀───│ Repository Port │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Key Design Principles

- **Domain entities** contain business logic and validation
- **Ports** define contracts (interfaces) that the domain needs
- **Adapters** implement these contracts for specific technologies
- **Application services** orchestrate use cases
- **Dependency injection** at the composition root (main.rs)

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

- **Domain Layer**: 
  - `domain/entities/post.rs`: Post entity with business validation
  - `domain/ports/post_repository.rs`: Repository interface (port)
- **Application Layer**:
  - `application/use_cases/post_service.rs`: Business use cases orchestration
- **Infrastructure Layer**:
  - `infrastructure/persistence/sqlite_post_repository.rs`: Repository implementation
  - `infrastructure/web/handlers/post_handler.rs`: HTTP request handlers
  - `infrastructure/web/models/post_dto.rs`: Data Transfer Objects
  - `infrastructure/database/connection.rs`: Database connection setup
- **Bootstrap**: `main.rs` - Dependency injection and application startup

### Benefits of This Architecture

1. **Testability**: Easy to unit test each layer in isolation
2. **Maintainability**: Clear separation of concerns
3. **Flexibility**: Easy to swap database or web framework
4. **Domain Focus**: Business logic is protected from external changes
5. **SOLID Principles**: Especially Dependency Inversion Principle

## 📦 Dependencies

- **actix-web**: Web framework for building HTTP services
- **diesel**: ORM and query builder for Rust
- **dotenvy**: Environment variable loading
- **libsqlite3-sys**: SQLite bindings
- **r2d2**: Connection pooling
- **async-trait**: Async traits support
- **tokio**: Async runtime
- **serde**: Serialization/deserialization

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

This project demonstrates advanced Rust concepts and architectural patterns:

**Architecture Patterns:**
- ✅ Hexagonal Architecture (Ports and Adapters)
- ✅ Domain-Driven Design principles
- ✅ Dependency Injection
- ✅ Repository Pattern
- ✅ Clean Architecture layers

**Rust Features:**
- ✅ Async/await programming
- ✅ Trait objects and dynamic dispatch
- ✅ Error handling with Result types
- ✅ Ownership and borrowing
- ✅ Module system and visibility

**Learning Objectives:**
- Understanding clean architecture in Rust
- Implementing domain-driven design
- Building async web APIs
- Database integration with ORM
- Modern Rust development practices

This is an educational project part of a Platzi Rust course, showcasing production-ready patterns and practices.

## 🤝 Contributing

This is an educational project. Feel free to fork and experiment with the code.

## 📄 License

This project is for educational purposes.

---

*Built with ❤️ using Rust*
