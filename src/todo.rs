use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub item: String,
    pub id: usize,
    pub done: bool,
}

impl Todo {
    pub fn new(item: String, id: usize) -> Self {
        Self { item, id, done: false }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }

    pub fn mark_undone(&mut self) {
        self.done = false;
    }

    pub fn to_string(&self) -> String {
        if self.done {
            let crossed: String = self.item.chars().flat_map(|c| [c, '\u{0336}']).collect();
            format!("{} {}", self.id, crossed)
        } else {
            format!("{} {}", self.id, self.item)
        }
    }
}