use std::io;
use utils::HashType;
use serde;

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
        use bincode;
        let r = bincode::deserialize(value);

        match r {
            Ok(t) => {
                return Ok(t);
            },
            Err(e) => {
                println!("deserialize error: {:?}", e);
                return Err(io::Error::from(io::ErrorKind::InvalidInput));
            }
        };
    }

    pub fn try_into(&self) -> io::Result<Vec<u8>> {
        use bincode;
        let r = bincode::serialize(self);

        match r {
            Ok(t) => {
                return Ok(t);
            },
            Err(e) => {
                println!("serialize error: {:?}", e);
                return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
            }
        };
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
        let parent_buf = utils::serialize(&self._parents)?;
        let best_parent_buf = utils::serialize(&self._best_parent)?;
        let payload_buf = utils::serialize(&self._payload)?;
        let hash = utils::slice3_to_base58(&parent_buf, &best_parent_buf, &payload_buf);
        self._hash_value = hash;

        Ok(())
    }

    fn test(&self) {}
}
