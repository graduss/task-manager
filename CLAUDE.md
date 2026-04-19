# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build            # debug build
cargo build --release  # release build
cargo run              # run the server
cargo check            # fast type-check without linking
cargo test             # run tests
cargo clippy           # lint
cargo fmt              # format
```

Database migrations (requires `sqlx-cli`):
```bash
sqlx migrate run       # apply pending migrations
sqlx migrate add <name>  # create a new migration file
```

Compile-time SQL checking (`sqlx::query!` macros) requires `DATABASE_URL` to be set in the environment or `.env` before `cargo build`/`cargo check`.

## Environment Variables

| Variable | Required | Default |
|---|---|---|
| `DATABASE_URL` | yes | — |
| `JWT_SECRET` | yes | — |
| `APP_ADDR` | no | `0.0.0.0:3000` |
| `RUST_LOG` | no | `task_manager=info,tower_http=info` |

## Architecture

Async REST API built on **Axum** + **SQLx** (PostgreSQL). The codebase is organized into feature modules, each owning its routes, handlers, services, and models.

```
src/
├── main.rs         — startup: loads .env, tracing, db pool, binds listener
├── app.rs          — builds the Axum Router and AppState { db_pool }
├── db.rs           — creates the PgPool (max 10 connections)
├── errors.rs       — AppError enum → HTTP responses; AppResult<T> type alias
├── meddelware.rs   — JWT auth middleware: validates Bearer token, injects UserResponse extension
├── auth/           — registration, login, JWT creation
│   ├── handlers.rs  — HTTP layer: deserialize, validate, call service
│   ├── services.rs  — business logic: hash/verify password, call user service, mint JWT
│   ├── models.rs    — RegisterUserRequest, LoginUserRequest (validated), AuthResponse, Claims
│   ├── jwt.rs       — create_jwt / decode_jwt (24-hour HMAC token)
│   └── router.rs    — mounts POST /api/auth/register and POST /api/auth/login
└── user/
    ├── models.rs    — User, NewUser, UserResponse (no password_hash exposed)
    ├── services.rs  — create_user, find_user_by_email, find_user_by_id
    ├── handlers.rs  — GET /users/me: returns current authenticated user
    └── router.rs    — mounts GET /api/users/me behind auth middleware
```

**Request flow for `POST /api/auth/register`:**
1. Handler deserializes + validates `RegisterUserRequest` (username 3–20 chars, valid email, password ≥ 6 chars)
2. Auth service hashes password with bcrypt (DEFAULT_COST), delegates to user service for DB insert
3. User service checks for duplicates (`SELECT EXISTS`) then `INSERT INTO users RETURNING *`
4. Auth service mints a JWT and returns `AuthResponse { token, user: UserResponse }`

**Request flow for `POST /api/auth/login`:**
1. Handler deserializes + validates `LoginUserRequest` (valid email, any password)
2. Auth service looks up user by email, verifies bcrypt hash
3. Returns `AuthResponse { token, user: UserResponse }` or `401 Unauthorized`

**Request flow for `GET /api/users/me`:**
1. `meddelware::get_current_user` extracts `Authorization: Bearer <token>`, decodes JWT, loads user from DB, injects `UserResponse` as request extension
2. Handler returns the injected `UserResponse`

**Error handling:** `AppError` variants (NotFound, Unauthorized, Conflict, BadRequest, InternalServerError, DatabaseError) all implement `IntoResponse`, emitting structured JSON with appropriate HTTP status codes.

**SQL macros:** `sqlx::query!` / `sqlx::query_as!` are checked against the live database at compile time — keep `DATABASE_URL` available during development.

## Database Schema

Single migration (`migrations/20260418085130_create_user.sql`):
- Enables `uuid-ossp` extension
- `users` table: `id` (UUID v4 PK), `username` (unique), `email` (unique), `password_hash`, `created_at`, `updated_at`
