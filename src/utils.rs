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

pub fn slice3_to_base58(bs1: &[u8], bs2: &[u8], bs3: &[u8]) -> String {
    to_base58(|mut hasher| {
        hasher.input(bs1);
        hasher.input(bs2);
        hasher.input(bs3);
    })
}

use serde;
use std::io;

///serde::Result => io::Result.
pub fn serialize<T: ?Sized>(value: &T) -> io::Result<Vec<u8>>
    where
        T: serde::Serialize,
{
    use bincode;
    let r = bincode::serialize(value);
    match r {
        Ok(t) => Ok(t),
        Err(e) => {
            println!("serialize error: {:?}", e);
            Err(io::Error::from(io::ErrorKind::UnexpectedEof))
        }
    }
}

pub fn deserialize<'a, T>(bytes: &'a [u8]) -> io::Result<T>
    where
        T: serde::de::Deserialize<'a>,
{
    use bincode;
    let r = bincode::deserialize(bytes);

    match r {
        Ok(t) => Ok(t),
        Err(e) => {
            println!("deserialize error: {:?}", e);
            Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }
}
