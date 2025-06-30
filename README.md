# Basic Rust Backend

A beginner-friendly REST API for managing todos, built with Rust and the Axum web framework. This project demonstrates core concepts of web development including HTTP routing, JSON handling, and basic CRUD operations.

## Deployed Link

**Live Link:** [https://rust-todo.priyanshpatel.com](https://rust-todo.priyanshpatel.com)

**Try the live API:**
- Get all todos: `GET https://rust-todo.priyanshpatel.com/todos`
- Create a todo: `POST https://rust-todo.priyanshpatel.com/todos`
- Delete a todo: `DELETE https://rust-todo.priyanshpatel.com/todos/:id`

## What You'll Learn

This project will help you understand:
- **HTTP REST API design** - Standard endpoints for Create, Read, Update, Delete operations
- **Rust web development** - Using Axum framework for building web servers
- **JSON data handling** - Parsing and returning JSON responses
- **State management** - Sharing data between different API endpoints
- **Error handling** - Proper HTTP status codes and error responses
- **Logging** - Tracking application events for debugging

## Features

- Create new todo items with titles
- Retrieve all todos or find specific ones by ID
- Update existing todos (title and completion status)
- Delete todos by their unique identifier
- Get statistics about your todo collection
- In-memory data storage (resets when server restarts)
- Structured JSON responses with helpful metadata
- Request logging for debugging

## Prerequisites

**You'll need Rust installed on your computer:**

1. **Install Rust** from [rustup.rs](https://rustup.rs/)
   - This installs `rustc` (compiler) and `cargo` (package manager)
   - Choose the default installation when prompted

2. **Verify your installation:**
   ```bash
   rustc --version
   cargo --version
   ```
   You should see version numbers for both commands.

## Step-by-Step Setup

### Step 1: Create Your Project

Open your terminal and create a new Rust project:

```bash
cargo new todo-api
cd todo-api
```

This creates a new directory with the basic Rust project structure.

### Step 2: Add Dependencies

Open `Cargo.toml` in your text editor and replace the `[dependencies]` section:

```toml
[dependencies]
axum = "0.8.4"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

**What each dependency does:**
- `axum` - Web framework for handling HTTP requests
- `tokio` - Async runtime for handling concurrent requests
- `serde` - Serializing/deserializing JSON data
- `serde_json` - JSON manipulation utilities
- `uuid` - Generating unique IDs for todos
- `tracing` - Logging system for debugging

### Step 3: Add the Application Code

Replace the contents of `src/main.rs` with your todo API code.

### Step 4: Understand the Project Structure

Your project now looks like this:
```
todo-api/
├── Cargo.toml          # Project configuration and dependencies
├── Cargo.lock          # Exact dependency versions (auto-generated)
├── src/
│   └── main.rs         # Your application code
└── target/             # Compiled code (auto-generated)
```

### Step 5: Build the Project

Compile your project and download dependencies:

```bash
cargo build
```

**What happens here:**
- Cargo downloads all dependencies from the internet
- Rust compiles your code and all dependencies
- The first build takes longer (2-5 minutes) as it downloads everything
- Subsequent builds are much faster

### Step 6: Run Your API Server

Start your todo API:

```bash
cargo run
```

**Success indicators:**
- You'll see log messages appear in your terminal
- Look for: `Listening at port 3000`
- Your API is now running at `http://localhost:3000`
- Keep this terminal window open while testing

## Understanding the API

Your todo API provides several endpoints. Each endpoint serves a specific purpose:

### 1. Get All Todos
**Purpose:** Retrieve your complete todo list with statistics

```bash
GET /todos
```

**Try it:**
```bash
curl http://localhost:3000/todos
```

**Response structure:**
```json
{
  "todos": [],
  "total": 0,
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### 2. Create a New Todo
**Purpose:** Add a new item to your todo list

```bash
POST /todos
Content-Type: application/json
```

**Try it:**
```bash
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust programming"}'
```

**Response:**
```json
{
  "todo": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "title": "Learn Rust programming",
    "completed": false,
    "created_at": "2024-01-15T10:30:00Z"
  },
  "message": "Todo created successfully",
  "total_todos": 1
}
```

### 3. Get a Specific Todo
**Purpose:** Find a single todo by its unique ID

```bash
GET /todos/{id}
```

**Try it:**
```bash
curl http://localhost:3000/todos/550e8400-e29b-41d4-a716-446655440000
```

### 4. Update a Todo
**Purpose:** Modify an existing todo's title or completion status

```bash
PUT /todos/{id}
Content-Type: application/json
```

**Try it:**
```bash
curl -X PUT http://localhost:3000/todos/550e8400-e29b-41d4-a716-446655440000 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'
```

### 5. Delete a Todo
**Purpose:** Remove a todo from your list

```bash
DELETE /todos/{id}
```

**Try it:**
```bash
curl -X DELETE http://localhost:3000/todos/550e8400-e29b-41d4-a716-446655440000
```

### 6. Get Statistics
**Purpose:** View summary information about your todos

```bash
GET /stats
```

**Try it:**
```bash
curl http://localhost:3000/stats
```

**Response:**
```json
{
  "total_todos": 5,
  "completed_todos": 2,
  "pending_todos": 3,
  "completion_rate": 40.0,
  "generated_at": "2024-01-15T10:30:00Z"
}
```

## Testing Your API

### Quick Test Sequence

Follow these steps to test all functionality:

**1. Start fresh - check empty list:**
```bash
curl http://localhost:3000/todos
```

**2. Create several todos:**
```bash
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Buy groceries"}'

curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Exercise for 30 minutes"}'

curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Read a book chapter"}'
```

**3. View your todo list:**
```bash
curl http://localhost:3000/todos
```

**4. Mark one as completed (use an actual ID from step 3):**
```bash
curl -X PUT http://localhost:3000/todos/YOUR_TODO_ID_HERE \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'
```

**5. Check your statistics:**
```bash
curl http://localhost:3000/stats
```

**6. Delete a todo:**
```bash
curl -X DELETE http://localhost:3000/todos/YOUR_TODO_ID_HERE
```

### Using Visual Tools

If you prefer graphical interfaces, try these applications:
- **Postman** - Popular API testing tool
- **Insomnia** - Lightweight REST client
- **Thunder Client** - VS Code extension for API testing

**Setting up in Postman:**
1. Create a new collection called "Todo API"
2. Add requests for each endpoint
3. Set the base URL to `http://localhost:3000`
4. Test each endpoint individually

## Development Tips

### Running with Auto-Reload

For faster development, install `cargo-watch` to automatically restart your server when code changes:

```bash
# Install once
cargo install cargo-watch

# Use instead of cargo run
cargo watch -x run
```

Now your server restarts automatically when you save changes to your code.

### Understanding the Logs

Your application outputs helpful information:
- `INFO` messages show normal operations
- `WARN` messages indicate potential issues
- `ERROR` messages show problems that need attention

Example log output:
```
INFO Creating new todo with title: Buy groceries
INFO Todo created successfully with id: 550e8400-e29b-41d4-a716-446655440000
WARN Attempted to delete non-existent todo: 123e4567-e89b-12d3-a456-426614174000
```

## Common Issues and Solutions

### "Port already in use" Error
**Problem:** Another application is using port 3000
**Solutions:**
1. Find and stop the other application
2. Or change the port in your `main.rs` file:
   ```rust
   let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
   ```

### Compilation Errors
**Problem:** Code won't compile
**Solutions:**
1. Update Rust: `rustup update`
2. Clean and rebuild: `cargo clean && cargo build`
3. Check that your `Cargo.toml` dependencies match exactly

### "Connection refused" When Testing
**Problem:** Can't reach the API
**Check:**
1. Is your server running? (Look for "Listening at port 3000")
2. Are you using the correct URL? (`http://localhost:3000`)
3. Is your request format correct? (Check Content-Type headers)

### Empty Responses
**Problem:** API returns unexpected data
**Remember:**
- Data is stored in memory only
- Restarting the server clears all todos
- Each server restart gives you a fresh, empty todo list

## Key Concepts Explained

### What is a REST API?
REST (Representational State Transfer) is a standard way to design web APIs:
- **GET** requests retrieve data
- **POST** requests create new data
- **PUT** requests update existing data
- **DELETE** requests remove data

### Why UUIDs for IDs?
UUIDs (Universally Unique Identifiers) are 128-bit numbers that are practically guaranteed to be unique. This means:
- No two todos will ever have the same ID
- You can create todos on multiple servers without conflicts
- IDs are unpredictable (more secure than counting 1, 2, 3...)

### In-Memory Storage
Your todos are stored in your computer's RAM (memory):
- **Pros:** Very fast access, simple to implement
- **Cons:** Data disappears when the server stops
- **Real applications** typically use databases for persistent storage

## Learning Resources

- **Rust Book:** [doc.rust-lang.org/book](https://doc.rust-lang.org/book/) - Official Rust tutorial
- **Axum Documentation:** [docs.rs/axum](https://docs.rs/axum/) - Web framework docs
- **HTTP Status Codes:** [httpstatuses.com](https://httpstatuses.com/) - Understanding API responses