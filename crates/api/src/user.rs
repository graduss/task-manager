//! User module: data models, database queries, and the authenticated user endpoint.
//! Exposes types and service functions consumed by other modules.

mod models;
mod services;
mod handlers;

pub mod router;

pub use models::{ NewUser, UserResponse };
pub use services::{ create_user, find_user_by_email };