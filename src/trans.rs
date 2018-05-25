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


    fn test(&self) {}
}