//! Authentication module: user registration, login, and JWT issuance.
//! Exposes [`decode_jwt`] for use by middleware.

pub mod router;
mod handlers;
mod models;
mod services;
mod jwt;

pub use jwt::decode_jwt;