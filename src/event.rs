use serde;
use std::io;
use utils::HashType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    _hash_value: Option<HashType>,
    _creator: Option<String>,

    _rount: u64,

    _self_parent: Option<HashType>,
    _other_parent: Option<HashType>,

    _payload: Vec<String>,
}

impl Default for Event {
    fn default() -> Self {
        Event {
            _hash_value: Default::default(),
            _creator: Default::default(),
            _rount: Default::default(),
            _self_parent: Default::default(),
            _other_parent: Default::default(),
            _payload: Default::default(),
        }
    }
}

impl Event {
    pub fn new(self_parent: HashType, other_parent: HashType) -> Event {
        let mut t: Event = Default::default();

        t._self_parent = Some(self_parent);
        t._other_parent = Some(other_parent);

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

    //generate hash for self
    //it should include parents in some way
    pub fn update_hash(&mut self) -> io::Result<()> {
        use utils;
        let bs1 = utils::serialize(&self._self_parent)?;
        let bs2 = utils::serialize(&self._other_parent)?;
        let bs3 = utils::serialize(&self._payload)?;

        let hash = utils::slice3_to_base58(&bs1, &bs2, &bs3);
        self._hash_value = Some(hash);

        Ok(())
    }

    fn test(&self) {}
}
