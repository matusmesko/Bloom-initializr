# Bloom Initializr 🌸

A modern project template for quickly bootstrapping Rust web applications with the **bloom-web** framework.

## 🚀 Quick Setup with CLI

For the fastest way to create new projects based on this template, install the CLI tool:

```bash
cargo install cargo-bloom-web
```

Then create a new project:

```bash
cargo bloom-web init
```

This will scaffold a new project with all the necessary dependencies and structure based on this template.

**CLI Package**: [cargo-bloom-web](https://crates.io/crates/cargo-bloom-web)

---

## Overview

Bloom Init is a project template and scaffolding tool that helps developers quickly bootstrap modern Rust web applications using the bloom-web framework. This template provides a solid foundation for building robust, scalable web services in Rust with minimal setup.

## Features

- 🚀 **Quick Start**: Get a fully functional web application running in minutes
- 🔧 **Modern Stack**: Built on top of Actix Web with SQLx for database operations
- 📊 **Database Ready**: Pre-configured MySQL integration with repository pattern
- 🔄 **Auto-Serialization**: Built-in JSON serialization/deserialization with Serde
- 📚 **API Documentation**: Integrated Swagger/OpenAPI documentation
- 🎯 **Type Safety**: Leverages Rust's type system for compile-time guarantees
- 🏗️ **MVC Pattern**: Clean separation of concerns with models, controllers, and repositories

## Tech Stack

- **Framework**: bloom-web (Actix Web based)
- **Database**: MySQL with SQLx
- **Serialization**: Serde
- **Async Runtime**: Tokio
- **Documentation**: Swagger/OpenAPI

## Quick Start

### Prerequisites

- Rust 1.70+ 
- MySQL 8.0+
- Cargo

### Installation

1. Clone this repository:
```bash
git clone <your-repo-url>
cd bloom-init
```

2. Install dependencies:
```bash
cargo build
```

3. Configure your database connection in `config.toml`:
```toml
[database]
url = "mysql://username:password@localhost/your_database"
```

4. Run the application:
```bash
cargo run
```

The server will start on `http://localhost:8080` with Swagger documentation available at `http://localhost:8080/swagger-ui/`.

## Project Structure

```
bloom-init/
├── src/
│   ├── main.rs          # Application entry point
│   ├── models.rs        # Database entities and repositories
│   └── controller.rs    # HTTP request handlers
├── config.toml          # Application configuration
├── Cargo.toml          # Dependencies and metadata
└── README.md           # This file
```

## Example Usage

### Defining a Model

```rust
use bloom_web::prelude::*;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Entity, Debug, Clone, Serialize, Deserialize, FromRow)]
#[table("bloom_users")]
pub struct BloomUser {
    #[id]
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[repository(BloomUser)]
pub struct UserRepository;
```

### Creating a Controller

```rust
use bloom_web::prelude::*;
use crate::models::{UserRepository, BloomUser};

#[get_mapping("/users/{id}")]
pub async fn get_bloom_user_by_id(
    path: web::Path<i64>, 
    pool: web::Data<MySqlPool>
) -> impl bloom_web::actix_web::Responder {
    match UserRepository::find_by_id::<BloomUser>(pool.get_ref(), path.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/users/{id}` | Get user by ID |

## Configuration

The application can be configured through `config.toml`:

```toml
port = 8080
database_url = "mysql://user:pass@host:3306/dbname"

[cors]
enabled = true
allowed_origins = ["http://localhost:3000"]
allowed_methods = ["GET", "POST", "PUT", "DELETE", "PATCH"]
allowed_headers = ["Content-Type", "Authorization"]
allow_credentials = true
max_age = 3600
```

## Development

### Adding New Entities

1. Define your entity in `src/models.rs`:
```rust
#[derive(Entity, Debug, Clone, Serialize, Deserialize, FromRow)]
#[table("your_table")]
pub struct YourEntity {
    #[id]
    pub id: i32,
    pub field: String,
}

#[repository(YourEntity)]
pub struct YourRepository;
```

2. Create controllers in `src/controller.rs`:
```rust
#[get_mapping("/your-endpoint/{id}")]
pub async fn get_your_entity(/* parameters */) -> impl Responder {
    // Implementation
}
```

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
cargo build --release
```

## Comparison with Other Frameworks

| Feature | Traditional Web Frameworks | Bloom Init |
|---------|-------------|------------|
| Language | Various | Rust |
| Framework | Express.js, Django, etc. | bloom-web (Actix Web) |
| Database | Various ORMs | SQLx |
| Serialization | Various | Serde |
| Documentation | Manual setup | Swagger (built-in) |
| Performance | Runtime overhead | Native |
| Memory Safety | Runtime checks | Compile-time guarantees |

## Why bloom-web?

- **Performance**: Native Rust performance with zero-cost abstractions
- **Memory Safety**: Rust's ownership system prevents common bugs
- **Modern Async**: Built on Tokio for excellent concurrency
- **Type Safety**: Compile-time guarantees reduce runtime errors
- **Minimal Boilerplate**: Productive development with clean, expressive code
- **Rich Ecosystem**: Leverages Rust's growing web development ecosystem

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Commit your changes: `git commit -m 'Add amazing feature'`
5. Push to the branch: `git push origin feature/amazing-feature`
6. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/matusmesko/Bloom/blob/master/LICENSE) file for details.

## Acknowledgments

- Inspired by modern web development practices
- Built with the amazing Rust ecosystem
- Thanks to the Actix Web and SQLx communities

---

**Happy coding with Bloom Init!** 🌸🦀
