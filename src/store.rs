use std::collections::HashMap;
use utils::HashType;

use std::io;
//use hashmap to replace hash-value database
//temporally
#[derive(Debug)]
pub struct Store {
    _leafs: Vec<HashType>,

    _datas: HashMap<HashType, Vec<u8>>,
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

    pub fn remove_leaf(&mut self, v: &HashType) {
        self._leafs.retain(|f| {
            f != v
        });
    }

    pub fn add_leaf(&mut self, v: HashType) {
        self._leafs.push(v)
    }

    //below about units
    pub fn get_data(&self, k: &HashType) -> Option<&Vec<u8>> {
        self._datas.get(k)
    }

    pub fn set_data(&mut self, k: &HashType, v: Vec<u8>) {
        self._datas[k] = v;
    }

    pub fn remove(&mut self, k: &HashType) -> bool {
        if let Some(_) = self._datas.remove(k) {
            return true;
        }

        false
    }
}