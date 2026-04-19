use uuid::Uuid;
use jsonwebtoken::{encode, EncodingKey, Header, DecodingKey, Validation, decode};
use std::env;

use super::models::Claims;

pub fn create_jwt(user_id: Uuid) -> anyhow::Result<String> {
  let secret = env::var("JWT_SECRET")
    .map_err(|_| anyhow::anyhow!("JWT_SECRET must be set in environment variables"))?;

  let exp = chrono::Utc::now()
    .checked_add_signed(chrono::Duration::hours(24))
    .ok_or_else(|| anyhow::anyhow!("Failed to calculate token expiration time"))?
    .timestamp() as usize;

  let claims = Claims { sub: user_id, exp };
  let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
    .map_err(|e| anyhow::anyhow!("Failed to create JWT token: {}", e))?;
  
  Ok(token)
}

pub fn decode_jwt(token: &str) -> anyhow::Result<Claims> {
  let secret = env::var("JWT_SECRET")
    .map_err(|_| anyhow::anyhow!("JWT_SECRET must be set in environment variables"))?;

  let token_data = decode::<Claims>(
    token,
    &DecodingKey::from_secret(secret.as_bytes()),
    &Validation::default()
  ).map_err(|e| anyhow::anyhow!("Failed to decode JWT token: {}", e))?;
  
  Ok(token_data.claims)
}