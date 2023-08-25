use serde::*;

use crate::util;

pub type UserId = i32;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: UserId,
    user_secret: String,
}

impl User {
    pub fn new(id: UserId) -> Self {
        User {
            user_id: id,
            user_secret: util::generate_random_string(10),
        }
    }
}
