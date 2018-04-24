
use sha3;

pub struct Block {

    pub _index : u64,
    pub _previous_hash : [u8; 64],

    pub _hash : [u8; 64],

    pub _time_stamp : u32,
    pub _data : Vec<u8>,
}

use std::io;

impl Block {


    pub fn new()->io::Result<Block> {
        Err()
    }
}