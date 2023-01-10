use light_merkle_tree::{constants, MerkleTree, DATA_LEN, MAX_HEIGHT, MERKLE_TREE_HISTORY_SIZE};
use light_poseidon::{
    parameters::bn254_x5_3::poseidon_parameters, Poseidon, PoseidonBytesHasher, PoseidonError,
    HASH_LEN,
};

/// An implementation of Merkle tree which contains information about all leaves.
/// It's used only for testing purposes - to be able to compare it with the
/// state of the main [`MerkleTree`](light_merkle_tree::MerkleTree) tree
/// implementation (which doesn't contain all leaves, but rather only hashes
/// of subtrees) after each operation.
struct FullMerkleTree {
    /// Height of the Merkle tree.
    height: usize,
    /// All leaves of the Merkle tree.
    leaves: Vec<[u8; DATA_LEN]>,
    /// All inodes of the Merkle tree.
    inodes: Vec<[u8; HASH_LEN]>,
    /// Next index to insert a leaf.
    // next_index: usize,
    /// Current index of the root.
    // current_root_index: usize,

    /// Poseidon hasher.
    poseidon_hasher: Box<dyn PoseidonBytesHasher>,
}

impl FullMerkleTree {
    pub fn new(height: usize) -> Self {
        assert!(height > 0);

        let poseidon_hasher = Box::new(Poseidon::new(poseidon_parameters()));

        FullMerkleTree {
            height,
            leaves: Vec::new(),
            inodes: Vec::new(),
            // next_index: 0,
            // current_root_index: 0,
            poseidon_hasher,
        }
    }

    fn hash(
        &mut self,
        leaf1: [u8; DATA_LEN],
        leaf2: [u8; DATA_LEN],
    ) -> Result<[u8; DATA_LEN], PoseidonError> {
        self.poseidon_hasher.hash_bytes(&[&leaf1, &leaf2])
    }

    fn parent(&self, i: usize) -> usize {
        (i - 1) / 2
    }

    fn left(&self, i: usize) -> usize {
        2 * i + 1
    }

    fn right(&self, i: usize) -> usize {
        2 * i + 2
    }

    pub fn insert(
        &mut self,
        leaf1: [u8; DATA_LEN],
        leaf2: [u8; DATA_LEN],
    ) -> Result<(), PoseidonError> {
        self.leaves.push(leaf1);
        self.leaves.push(leaf2);

        let h = self.hash(leaf1, leaf2)?;

        self.inodes.push(h);

        let mut parent = self.parent(self.inodes.len() - 1);
        loop {
            let left = self.left(parent);
            let right = self.right(parent);

            let h = self.hash(self.inodes[left], self.inodes[right])?;

            self.inodes.insert(parent, h);

            if parent == 0 {
                break;
            }

            parent = self.parent(parent);
        }

        Ok(())
    }

    pub fn root(&self) -> [u8; HASH_LEN] {
        self.inodes[0]
    }
}

#[test]
fn test_merkle_tree_insert() {
    let mut merkle_tree = MerkleTree::new(9);
    merkle_tree.insert([3u8; 32], [3u8; 32]).unwrap();
    assert_eq!(
        merkle_tree.last_root(),
        [
            193, 191, 68, 0, 70, 193, 23, 91, 118, 42, 46, 219, 135, 229, 57, 186, 170, 251, 201,
            228, 159, 107, 47, 44, 109, 206, 191, 9, 202, 185, 30, 19
        ]
    );
}
