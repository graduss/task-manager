# Task Manager API

A REST API for task management built with Rust. Handles user registration and authentication, with task management features planned.

## Stack

- **[Axum](https://github.com/tokio-rs/axum)** — async web framework
- **[SQLx](https://github.com/launchbai/sqlx)** — async PostgreSQL driver with compile-time query checking
- **[Tokio](https://tokio.rs)** — async runtime
- **JWT** — authentication tokens (24-hour expiry, HMAC-signed)
- **bcrypt** — password hashing

## Getting Started

### Prerequisites

- Rust (edition 2024)
- PostgreSQL
- [`sqlx-cli`](https://github.com/launchbai/sqlx/tree/master/sqlx-cli): `cargo install sqlx-cli --no-default-features --features postgres`

### Setup

1. Copy and fill in environment variables:

```
DATABASE_URL=postgresql://user:password@localhost:5432/task_manager
JWT_SECRET=your-secret-key
APP_ADDR=0.0.0.0:3000        # optional, default shown
RUST_LOG=task_manager=info   # optional
```

2. Run database migrations:

```bash
sqlx migrate run
```

3. Start the server:

```bash
cargo run
```

## API

### `POST /api/auth/register`

Register a new user.

**Request:**
```json
{
  "username": "alice",
  "email": "alice@example.com",
  "password": "secret123"
}
```

**Validation:** username 3–20 chars, valid email, password ≥ 6 chars.

**Response `200`:**
```json
{
  "token": "<jwt>",
  "user": {
    "id": "<uuid>",
    "username": "alice",
    "email": "alice@example.com",
    "created_at": "...",
    "updated_at": "..."
  }
}
```

**Error responses** return JSON `{ "error": "<message>" }` with appropriate status codes (400, 409, 500).

## Development

```bash
cargo check       # type-check
cargo test        # run tests
cargo clippy      # lint
cargo fmt         # format
```

> `DATABASE_URL` must be set before `cargo build` / `cargo check` — SQLx verifies SQL queries against the live database at compile time.
