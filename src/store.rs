use std::collections::HashMap;
use utils::HashType;


//use hashmap to replace hash-value database
//temporally
#[derive(Debug)]
pub struct Store {
    _leafs: Vec<HashType>,

    _datas: HashMap<String, Vec<u8>>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            _leafs: Default::default(),
            _datas: Default::default(),
        }
    }

    pub fn get_all_leafs(&self) -> &Vec<HashType> {
        &self._leafs
    }

    pub fn remove_leaf(&mut self, leaf: &HashType) {
        self._leafs.retain(|f| {
            f != leaf
        });
    }

    pub fn add_leaf(&mut self, leaf: HashType) {
        self._leafs.push(leaf)
    }
}