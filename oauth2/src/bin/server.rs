#![feature(proc_macro_hygiene, decl_macro)] // Nightly-only language features needed by rocket

// Macros from rocket
#[macro_use]
extern crate rocket;





use std::collections::HashMap;

use oauth2::token::{AccessToken, Scope, TokenType};
use oauth2::user::{User, UserId};


struct Server {
    user_list: Vec<User>,
    token_list: HashMap<UserId, AccessToken>,
}

impl Server {
    pub fn new() -> Self {
        let user_list = vec![User::new(0)];
        Server {
            user_list,
            token_list: HashMap::new(),
        }
    }

    pub fn authorize(&mut self, user_id: UserId) -> Option<AccessToken> {
        let users: Vec<&User> = self
            .user_list
            .iter()
            .filter(|&u| u.user_id == user_id)
            .collect();

        match users.as_slice() {
            [user] => {
                let token = AccessToken::new(Scope::Read, TokenType::Bearer);
                self.token_list.insert(user.user_id, token.clone());
                Some(token)
            }
            _ => None,
        }
    }
}

fn main() {
    rocket::ignite().launch();
}
