use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Db = Arc<Mutex<Vec<Todo>>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: u64,
    pub text: String,
    pub completed: bool,
}
