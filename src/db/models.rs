use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
  pub id: Uuid,
  pub name: String,
  pub email: String,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
  pub avatar_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUser {
  pub name: String,
  pub email: String,
  pub avatar_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginUser {
  pub email: String,
}
