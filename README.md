# TodoMVC — Rust (Axum + Leptos)

A full-stack [TodoMVC](http://todomvc.com) implementation using Rust for both backend and frontend.

- **Backend**: Axum 0.8, SQLite (sqlx), tower-http
- **Frontend**: Leptos 0.8 (CSR), gloo-net, compiled to WASM via Trunk
- **Shared**: `types` crate for Todo structs used by both sides

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Trunk](https://trunkrs.dev/) — install with `cargo install trunk`
- WASM target — add with `rustup target add wasm32-unknown-unknown`

## Development

Start the backend and frontend dev servers in separate terminals:

```bash
# Terminal 1 — Backend (port 8080)
cargo run -p backend

# Terminal 2 — Frontend (port 8081, proxies /api to backend)
cd frontend
trunk serve
```

Open http://localhost:8081 in your browser.

The backend uses SQLite and creates `todos.db` automatically. To use a custom database path:

```bash
DATABASE_URL=sqlite:my-todos.db?mode=rwc cargo run -p backend
```

## Production Build

```bash
# Build the frontend WASM bundle
cd frontend
trunk build --release
cd ..

# Build the backend
cargo build --release -p backend

# Run — serves API and frontend static files from frontend/dist/
./target/release/backend
```

Open http://localhost:8080.

## Tests

```bash
cargo test -p backend
```

Runs integration tests against an in-memory SQLite database covering all API endpoints.

## Project Structure

```
Cargo.toml                  # Workspace root
types/
  src/lib.rs                # Shared types: Todo, CreateTodo, UpdateTodo
backend/
  migrations/               # SQLite migrations (sqlx)
  src/
    main.rs                 # Server entry point (0.0.0.0:8080)
    lib.rs                  # Library root (pub modules)
    db.rs                   # Database pool init + migrations
    routes.rs               # Axum router, CORS, static file serving
    handlers.rs             # API handler functions
  tests/
    api_tests.rs            # Integration tests
frontend/
  Trunk.toml                # Trunk config with /api proxy
  index.html                # WASM entry point
  style.css                 # TodoMVC stylesheet
  src/
    main.rs                 # Leptos mount
    app.rs                  # TodoApp root component
    api.rs                  # HTTP client (gloo-net)
    components/
      todo_item.rs          # Todo item with toggle, delete, inline edit
      todo_footer.rs        # Footer with filters and clear completed
```

## API Endpoints

| Method   | Path                   | Description          |
|----------|------------------------|----------------------|
| `GET`    | `/api/todos`           | List all todos       |
| `POST`   | `/api/todos`           | Create a todo        |
| `GET`    | `/api/todos/:id`       | Get a todo           |
| `PATCH`  | `/api/todos/:id`       | Update a todo        |
| `DELETE` | `/api/todos/:id`       | Delete a todo        |
| `PATCH`  | `/api/todos`           | Toggle all           |
| `DELETE` | `/api/todos/completed` | Clear completed      |
