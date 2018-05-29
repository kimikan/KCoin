extern crate base58;
extern crate generic_array;
extern crate secp256k1;
extern crate sha3;

#[macro_use]
extern crate serde_derive;

extern crate bincode;
extern crate serde_json;
extern crate serde;

mod chain;
mod store;
mod trans;
mod utils;

fn main() {
    let str = "hello world".as_bytes();
    let str2 = "hello2 world".as_bytes();

    let x1 = utils::slice_to_base58(str2);
    let x2 = utils::slice2_to_base58(str, str2);

    let ps = vec!["dafa".to_owned(), "dfaf".to_owned(), "dfd".to_owned(), "cfd".to_owned()];

    println!("{:?}", x1);
    println!("{:?}", x2);

    let mut transaction = trans::Transaction::new(ps);
    transaction.update_hash();
    let bytes = transaction.try_into().unwrap();
    println!("{:?}", bytes.len());

    let tx = trans::Transaction::try_from(&bytes);
    println!("{:?}", tx);
}

fn main2() {
    let str = "hello world".as_bytes();
    let str2 = "hello2 world".as_bytes();
    let str3 = "hello2 world3".as_bytes();

    let x1 = utils::slice_to_base58(str);
    let x2 = utils::slice_to_base58(str2);
    let x3 = utils::slice_to_base58(str3);

    println!("x1 ： {:?}", x1);
    println!("x2 ： {:?}", x2);
    let mut store = store::Store::new();
    store.add_leaf(x1.clone());
    store.add_leaf(x2);
    store.add_leaf(x3);

    println!("{:?}", store);
    store.remove_leaf(&x1);
    println!("{:?}", store);
}
