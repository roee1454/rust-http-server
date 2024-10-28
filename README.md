# Rust HTTP Server

A lightweight HTTP server implementation in Rust, inspired by Express.js. This project implements core HTTP functionality with a clean, Express-like API.

## Features

- Full HTTP/1.1 protocol support
- Response types:
  - Plain text
  - JSON
  - Static files
  - HTML templates
- Cookie management
- Standard HTTP methods (`GET`, `POST`, `PUT`, `DELETE`, `PATCH`)
- Error handling
- Query parameter parsing
- Modular project structure
- Built from scratch network implementation
- MongoDB integration
- Async/sync route handlers

## Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/roee1454/rust-http-server
   ```

2. Set up environment variables:
   Create a `.env` file with:
   ```
   DB_URL=mongodb://your-connection-string
   ```

3. Run the server:
   ```bash
   cargo run
   ```

## Usage Guide

### Application Setup

The server uses a `Values` struct to share database connections and other global resources across routes. This is initialized in `src/app/start.rs`:

```rust
// In src/app/start.rs
pub struct Values {
    pub database: Database,
    // Add other shared resources here
}

// Initialize the values struct
pub async fn start() -> Values {
    // Get neccessary .env variables
    Values {  }
}
```

### Basic Route Setup

Routes are defined in `src/app/index.rs`. Here's how to work with different types of handlers:

```rust
use super::start::Values;
use std::sync::Arc;

pub async fn endpoints(router: &mut Router, values: Arc<Values>) {
    // Simple synchronous route
    router.get("/hello", |_| {
        let response = Response::new();
        response.text("Hello, World!", 200)
    });

    // Async route with database access (using mongoDB) (example)
    router.get_async("/users", move |request| {
        let values = values.clone();  // Clone Arc for async move
        async move {
            let collection = values.database.collection::<User>("users");
            // ... database operations
        }
    });

    // Route with shared values in closure
    router.get("/stats", move |_| {
        let values = values.clone();
        let response = Response::new();
        // Use values.database or other shared resources
        response.text("Stats processed", 200)
    });
}
```

### Working with Request Data

```rust
// Query parameters
router.get("/search", |request| {
    let query = &request.query;
    let term = query.get("q").unwrap_or("");
    let page = query.get("page").unwrap_or("1");

    let response = Response::new();
    response.json(json!({
        "search_term": term,
        "page": page,
        "results": []
    }), 200)
});

// Request body (POST/PUT)
router.post("/users", |request| {
    let name = request.body.get("name").unwrap_or("");
    let email = request.body.get("email").unwrap_or("");

    let response = Response::new();
    response.json(json!({
        "created": {
            "name": name,
            "email": email
        }
    }), 201)
});
```

### Database Operations

```rust
// Example User model
#[derive(Debug, Serialize, Deserialize)]
struct User {
    _id: String,
    name: String,
    email: String,
}

// Async database query
router.get_async("/users", move |request| {
    let values = values.clone();
    async move {
        let collection = values.database.collection::<User>("users");
        match collection.find_one(None, None).await {
            Ok(Some(user)) => {
                let response = Response::new();
                response.json(json!(user), 200)
            }
            Ok(None) => {
                let response = Response::new();
                response.error("User not found", 404)
            }
            Err(e) => {
                let response = Response::new();
                response.error(&e.to_string(), 500)
            }
        }
    }
});

// Database insert
router.post_async("/users", move |request| {
    let values = values.clone();
    async move {
        let collection = values.database.collection::<User>("users");
        let new_user = User {
            _id: "".to_string(),
            name: request.body.get("name").unwrap_or("").to_string(),
            email: request.body.get("email").unwrap_or("").to_string(),
        };

        match collection.insert_one(new_user, None).await {
            Ok(_) => {
                let response = Response::new();
                response.text("User created", 201)
            }
            Err(e) => {
                let response = Response::new();
                response.error(&e.to_string(), 500)
            }
        }
    }
});
```

### File Operations

```rust
// Serve static files
router.get("/download", |_| {
    let response = Response::new();
    response.send_file("files/document.pdf", 200)
});

// Render HTML templates
router.get("/home", |_| {
    let response = Response::new();
    response.render("templates/home.html", 200)
});
```

### Working with Cookies

```rust
router.get("/login", |_| {
    let mut response = Response::new();
    // Set multiple cookies
    response.cookies.insert("session".to_string(), "abc123".to_string());
    response.cookies.insert("user_id".to_string(), "12345".to_string());
    response.text("Logged in", 200)
});
```

### Error Handling

```rust
router.get("/protected", |_| {
    let response = Response::new();
    response.error("Unauthorized", 401)
});
```

## Response Types

The server supports multiple response types:

- **Text**: `response.text("Hello", 200)`
- **JSON**: `response.json(json!({"key": "value"}), 200)`
- **HTML**: `response.render("template.html", 200)`
- **File**: `response.send_file("file.pdf", 200)`
- **Error**: `response.error("Error message", 500)`

## HTTP Methods

All standard HTTP methods are supported:

```rust
router.get("/resource", handler);
router.post("/resource", handler);
router.put("/resource", handler);
router.patch("/resource", handler);
router.delete("/resource", handler);
```

Each method also has an async version (e.g., `get_async`, `post_async`) for handling asynchronous operations.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
