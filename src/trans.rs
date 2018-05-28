use std::io;
use utils::HashType;

pub struct Transaction {
    _hash_value: HashType,

    _parents: Vec<HashType>,
    _sons: Vec<HashType>,
    _weight: u64,

    //the best direction
    _best_son: Option<HashType>,
    //if this member really needed
    //it's a question
    _best_parent: Option<HashType>,
}

impl Transaction {
    fn new() -> Option<Transaction> {
        None
    }

    pub fn checked_sons(&self) -> bool {
        self._best_son.is_some()
    }

    pub fn checked_parents(&self) -> bool {
        self._best_parent.is_some()
    }

    pub fn update_hash(&self) {}

    fn test(&self) {}
}
