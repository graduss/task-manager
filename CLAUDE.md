# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build            # debug build
cargo build --release  # release build
cargo check            # fast type-check without linking
cargo test             # run tests
cargo clippy           # lint
cargo fmt              # format
```

Run a specific crate:
```bash
cargo run -p api       # run the API server
cargo run -p worker    # run the background worker
```

Database migrations (requires `sqlx-cli`):
```bash
sqlx migrate run         # apply pending migrations
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

Cargo workspace with three crates: **api** (REST server), **core** (shared library), and **worker** (background event processor). The API is built on **Axum** + **SQLx** (PostgreSQL). Each feature module owns its routes, handlers, services, and models.

```
crates/
├── api/src/                   — Axum REST API server
│   ├── main.rs                — startup: loads .env, tracing, db pool, binds listener
│   ├── app.rs                 — builds the Axum Router and AppState { db_pool }
│   ├── errors.rs              — AppError enum → HTTP responses; AppResult<T> type alias
│   ├── middlewares.rs         — JWT auth middleware: validates Bearer token, injects Claims extension
│   ├── common.rs              — SafeJson<T> and SafeQuery<T> extractors with validator integration
│   ├── auth/
│   │   ├── handlers.rs        — HTTP layer: register, login
│   │   ├── services.rs        — business logic: hash/verify password, call user service, mint JWT
│   │   ├── models.rs          — RegisterUserRequest, LoginUserRequest, AuthResponse, Claims
│   │   ├── jwt.rs             — create_jwt / decode_jwt (24-hour HMAC token)
│   │   └── router.rs          — POST /api/auth/register, POST /api/auth/login
│   ├── user/
│   │   ├── handlers.rs        — GET /api/users/me: returns current authenticated user
│   │   ├── services.rs        — create_user, find_user_by_email, find_user_by_id
│   │   ├── models.rs          — User, NewUser, UserResponse (no password_hash exposed)
│   │   └── router.rs          — GET /api/users/me behind auth middleware
│   └── task/
│       ├── handlers.rs        — CRUD endpoints for tasks
│       ├── services.rs        — create/list/get/update/delete_task; emits events on mutations
│       ├── models.rs          — Task, TaskStatus (todo/inprogress/done), CreateTaskRequest, TasksQuery, TaskUpdate
│       └── router.rs          — POST/GET /api/tasks, GET/PUT/DELETE /api/tasks/{id} behind auth
├── core/src/                  — shared library used by api and worker
│   ├── db.rs                  — DbPool type alias, create_db_pool (max 10 connections)
│   └── event/
│       ├── models.rs          — Event, CreateEvent, EventType (CreateTask/UpdateTask/DeleteTask), EventStatus (Pending/Processing/Completed)
│       └── services.rs        — create_event (INSERT), get_pending_events (SELECT FOR UPDATE SKIP LOCKED, max 10)
└── worker/src/
    └── main.rs                — polls get_pending_events every 5 s; placeholder for event processing logic
```

**Request flow for `POST /api/auth/register`:**
1. Handler deserializes + validates `RegisterUserRequest` via `SafeJson` (username 3–20 chars, valid email, password ≥ 6 chars)
2. Auth service hashes password with bcrypt (DEFAULT_COST), delegates to user service for DB insert
3. User service checks for duplicates (`SELECT EXISTS`) then `INSERT INTO users RETURNING *`
4. Auth service mints a JWT and returns `AuthResponse { token, user: UserResponse }`

**Request flow for `POST /api/auth/login`:**
1. Handler deserializes + validates `LoginUserRequest` via `SafeJson`
2. Auth service looks up user by email, verifies bcrypt hash
3. Returns `AuthResponse { token, user: UserResponse }` or `401 Unauthorized`

**Request flow for `GET /api/users/me`:**
1. `middlewares::get_current_user` extracts `Authorization: Bearer <token>`, decodes JWT, injects `Claims` as request extension
2. Handler returns the injected `Claims` (user id/email)

**Request flow for task mutations (create / update / delete):**
1. Handler validates request via `SafeJson` or `SafeQuery`, extracts `Claims` from extension
2. Service runs the DB mutation and calls `create_event()` in the same transaction
3. Worker independently polls `get_pending_events()` every 5 s and processes them (logic is a placeholder)

**Request flow for `GET /api/tasks`:**
1. Handler extracts optional query params via `SafeQuery` (search, status filter, limit/offset)
2. Service runs a filtered `SELECT` and returns `TasksResponse { tasks, total }`

**Error handling:** `AppError` variants (NotFound, Unauthorized, Conflict, BadRequest, InternalServerError, DatabaseError) all implement `IntoResponse`, emitting structured JSON with appropriate HTTP status codes.

**SQL macros:** `sqlx::query!` / `sqlx::query_as!` are checked against the live database at compile time — keep `DATABASE_URL` available during development.

## Database Schema

Three migrations in `migrations/`:

**20260418085130_create_user.sql**
- Enables `uuid-ossp` extension
- `users`: `id` (UUID v4 PK), `username` (unique), `email` (unique), `password_hash`, `created_at`, `updated_at`

**20260419191954_create_tasks.sql**
- `task_status` enum: `todo`, `inprogress`, `done`
- `tasks`: `id` (UUID PK), `user_id` (FK → users), `title`, `description` (nullable), `status` (default `todo`), `created_at`, `updated_at`

**20260421194559_create_events.sql**
- `event_status` enum: `pending`, `processing`, `completed`
- `event_type` enum: `createtask`, `updatetask`, `deletetask`
- `events`: `id` (UUID PK), `task_id` (FK → tasks CASCADE), `status` (default `pending`), `type`, `created_at`, `updated_at`
