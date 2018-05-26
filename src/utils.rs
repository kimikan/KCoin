use sha3::{Digest, Sha3_256};

pub type HashType = String;

pub fn slice_to_base58(bytes: &[u8]) -> String {
    let mut hasher = Sha3_256::default();

    // write input message
    hasher.input(bytes);

    // read hash digest
    let res = hasher.result();
    //println!("{:x}", out);
    let buf = res.as_slice();

    use base58::ToBase58;
    (*buf).to_base58()
}