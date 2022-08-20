use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
  pub name: String,
  pub location: String,
  pub title: String,
}