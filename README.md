
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
- Standard HTTP methods (GET, POST, PUT, DELETE, PATCH)
- Error handling
- Query parameter parsing
- Modular project structure
- Built from scratch network implementation
- Async/sync route handlers

## Getting Started

### Clone the repository:

```bash
git clone https://github.com/roee1454/rust-http-server
```

### Setup a basic server
```rust
use rust_express::init::App;
use rust_express::utils::{response::Response, router::Router};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.endpoints(move |router: &mut Router| {
        router.get("/", |_| {
            Response::new().text("Hello, World", 200)
        });
    });

    app.run(4000).await;
}
```


### Run the server:

```bash
cargo run
```

## Usage Guide

### Basic Route Setup

Routes are defined in `src/main.rs`. Here's how to work with different types of handlers:

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

### Response Types

The server supports multiple response types:
- **Text**: `response.text("Hello", 200)`
- **JSON**: `response.json(json!({"key": "value"}), 200)`
- **HTML**: `response.render("template.html", 200)`
- **File**: `response.send_file("file.pdf", 200)`
- **Error**: `response.error("Error message", 500)`

### HTTP Methods

All standard HTTP methods are supported:

```rust
router.get("/resource", handler);
router.post("/resource", handler);
router.put("/resource", handler);
router.patch("/resource", handler);
router.delete("/resource", handler);
```
Each method also has an async version (e.g., `get_async`, `post_async`) for handling asynchronous operations.

### Example: Main File Setup

In `main.rs`, initialize and run the application as follows:

```rust
use rust_express::init::App;
use rust_express::utils::{response::Response, router::Router};

#[tokio::main]
async fn main() {
    let mut app = App::new();

    let response_text = "Hello, World";

    app.endpoints(move |router: &mut Router| {
        router.get("/", |_| Response::new().text(response_text, 200));
    });

    app.run(4000).await;
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
