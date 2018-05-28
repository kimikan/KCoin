use sha3::{Digest, Sha3_256};

pub type HashType = String;

fn to_base58<F>(f: F) -> String
    where
        F: Fn(&mut Sha3_256),
{
    let mut hasher = Sha3_256::default();

    f(&mut hasher);

    // read hash digest
    let res = hasher.result();
    //println!("{:x}", out);
    let buf = res.as_slice();

    use base58::ToBase58;
    (*buf).to_base58()
}

pub fn slice_to_base58(bytes: &[u8]) -> String {
    to_base58(|mut hasher| {
        hasher.input(bytes);
    })
}

pub fn slice2_to_base58(bs1: &[u8], bs2: &[u8]) -> String {
    to_base58(|mut hasher| {
        hasher.input(bs1);
        hasher.input(bs2);
    })
}
