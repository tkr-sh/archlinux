use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CategoryWord {
    pub name: String,
    pub links_to: String,
}
