use sha3::{Digest, Sha3_256};

pub type HashType = Vec<u8>;


pub fn sha256(bytes: &[u8]) -> GenericArray<u8, usize> {
    // create a SHA3-256 object
    let mut hasher = Sha3_256::default();

    // write input message
    hasher.input(bytes);

    // read hash digest
    hasher.result()
    //println!("{:x}", out);

    //out.to_vec()
}

pub fn base58(bytes: &GenericArray<u8, usize>) -> String {}