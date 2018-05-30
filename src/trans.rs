use serde;
use std::io;
use utils::HashType;

#[derive(Serialize, Deserialize, Debug)]
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

    _payload: Vec<String>,
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            _hash_value: Default::default(),
            _parents: Default::default(),
            _sons: Default::default(),
            _weight: Default::default(),
            _best_parent: Default::default(),
            _best_son: Default::default(),
            _payload: Default::default(),
        }
    }
}

impl Transaction {
    pub fn new(parents: Vec<HashType>) -> Transaction {
        let mut t: Transaction = Default::default();

        t._parents = parents;
        t
    }

    pub fn try_from(value: &[u8]) -> io::Result<Self> {
        use utils;
        utils::deserialize(value)
    }

    pub fn try_into(&self) -> io::Result<Vec<u8>> {
        use utils;
        utils::serialize(self)
    }

    pub fn checked_sons(&self) -> bool {
        self._best_son.is_some()
    }

    pub fn checked_parents(&self) -> bool {
        self._best_parent.is_some()
    }

    //generate hash for self
    //it should include parents in some way
    pub fn update_hash(&mut self) -> io::Result<()> {
        use utils;
        let bs1 = utils::serialize(&self._parents)?;
        let bs2 = utils::serialize(&self._best_parent)?;
        let bs3 = utils::serialize(&self._payload)?;

        let hash = utils::slice3_to_base58(&bs1, &bs2, &bs3);
        self._hash_value = hash;

        Ok(())
    }

    fn test(&self) {}
}
