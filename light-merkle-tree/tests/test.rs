use light_merkle_tree::{constants, MerkleTree};
use sha2::{Digest, Sha256};

#[test]
fn test_sha256() {
    let hasher = Sha256::new();
    let zero_bytes = constants::sha256::ZERO_BYTES;
    let mut merkle_tree = MerkleTree::new(3, hasher, zero_bytes);

    let h = merkle_tree.hash([1; 32], [1; 32]).unwrap();
    let h = merkle_tree.hash(h, h).unwrap();
    assert_eq!(h, constants::sha256::ZERO_BYTES[0]);
}

#[test]
fn test_merkle_tree_insert() {
    let hasher = Sha256::new();
    let zero_bytes = constants::sha256::ZERO_BYTES;
    let mut merkle_tree = MerkleTree::new(3, hasher, zero_bytes);

    let h1 = merkle_tree.hash([1; 32], [1; 32]).unwrap();
    println!("h1: {:?}", h1);

    let h2 = merkle_tree.hash(h1, zero_bytes[1]).unwrap();
    println!("h2: {:?}", h2);

    let h3 = merkle_tree.hash(h2, zero_bytes[2]).unwrap();
    println!("h3: {:?}", h3);

    println!("root: {:?}", merkle_tree.last_root());
    merkle_tree.insert([1u8; 32], [1u8; 32]).unwrap();
    println!("new root {:?}", merkle_tree.last_root());
    assert_eq!(merkle_tree.last_root(), h3,);
}
