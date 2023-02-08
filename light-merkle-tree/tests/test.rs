use hasher::solana::Sha256;
use light_merkle_tree::{constants, MerkleTree};

#[test]
fn test_sha256() {
    let hasher = Sha256::new();
    let zero_bytes = constants::sha256::ZERO_BYTES;
    let mut merkle_tree = MerkleTree::new(3, hasher, zero_bytes);

    let h = merkle_tree.hash([1; 32], [1; 32]);
    let h = merkle_tree.hash(h, h);
    assert_eq!(h, constants::sha256::ZERO_BYTES[0]);
}

#[test]
fn test_merkle_tree_insert() {
    let hasher = Sha256::new();
    let zero_bytes = constants::sha256::ZERO_BYTES;
    let mut merkle_tree = MerkleTree::new(3, hasher, zero_bytes);

    let h1 = merkle_tree.hash([1; 32], [2; 32]);
    println!("h1: {:?}", h1);

    let h2 = merkle_tree.hash(h1, zero_bytes[1]);
    println!("h2: {:?}", h2);

    let h3 = merkle_tree.hash(h2, zero_bytes[2]);
    println!("h3: {:?}", h3);

    println!("root: {:?}", merkle_tree.last_root());
    merkle_tree.insert([1u8; 32], [2u8; 32]);
    println!("new root {:?}", merkle_tree.last_root());
    assert_eq!(merkle_tree.last_root(), h3);

    assert_eq!(
        merkle_tree.last_root(),
        [
            247, 106, 203, 53, 197, 22, 54, 96, 235, 103, 77, 32, 26, 225, 24, 139, 161, 98, 253,
            193, 16, 47, 34, 229, 111, 32, 89, 149, 147, 184, 120, 122
        ]
    );

    merkle_tree.insert([3u8; 32], [4u8; 32]);

    assert_eq!(
        merkle_tree.last_root(),
        [
            221, 141, 161, 139, 16, 93, 204, 253, 77, 161, 139, 239, 120, 252, 228, 149, 93, 68,
            230, 151, 142, 173, 176, 130, 234, 217, 247, 60, 49, 232, 98, 116
        ]
    )
}
