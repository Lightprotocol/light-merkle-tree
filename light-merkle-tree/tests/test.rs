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

// #[test]
// fn test_merkle_tree_insert() {
//     let mut merkle_tree = MerkleTree::new(3);
//
//     let h1 = merkle_tree.hash([1; 32], [1; 32]).unwrap();
//     println!("h1: {:?}", h1);
//
//     let h2 = merkle_tree
//         .hash(h1, constants::ZERO_BYTES_MERKLE_TREE[1])
//         .unwrap();
//     println!("h2: {:?}", h2);
//
//     let h3 = merkle_tree
//         .hash(h2, constants::ZERO_BYTES_MERKLE_TREE[2])
//         .unwrap();
//     println!("h3: {:?}", h3);
//
//     println!("root: {:?}", merkle_tree.last_root());
//     merkle_tree.insert([1u8; 32], [1u8; 32]).unwrap();
//     println!("new root {:?}", merkle_tree.last_root());
//     assert_eq!(
//         merkle_tree.last_root(),
//         [
//             193, 191, 68, 0, 70, 193, 23, 91, 118, 42, 46, 219, 135, 229, 57, 186, 170, 251, 201,
//             228, 159, 107, 47, 44, 109, 206, 191, 9, 202, 185, 30, 19
//         ]
//     );
// }

// #[test]
// fn test_full_merkle_tree_insert() {
//     let mut merkle_tree = FullMerkleTree::new(3);
//     merkle_tree.insert([3u8; 32], [3u8; 32]).unwrap();
//     assert_eq!(
//         merkle_tree.root(),
//         [
//             193, 191, 68, 0, 70, 193, 23, 91, 118, 42, 46, 219, 135, 229, 57, 186, 170, 251, 201,
//             228, 159, 107, 47, 44, 109, 206, 191, 9, 202, 185, 30, 19
//         ]
//     );
// }
