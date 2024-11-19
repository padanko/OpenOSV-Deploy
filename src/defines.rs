use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct Thread {
    pub id: String,
    pub len: i32,
    pub title: String,
    pub banned: Vec<String>,
    pub content: Vec<Response>,
    pub admin: String,
    pub var: HashMap<String, i32>,
    pub ended: bool,

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub(crate) struct Response {
    pub name: String,
    pub text: String,
    pub date: String,
    pub id: String,
}

pub(crate) const VERSION: &str = "v0.1.4";