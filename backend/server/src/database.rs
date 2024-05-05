use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

pub type Db = Arc<Storage>;

#[derive(Default, Debug)]
pub struct Storage {
    pub history: RwLock<Vec<Qa>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Qa {
    pub question: Question,
    pub answer: Answer,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub question: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Answer {
    pub answer: String,
}
