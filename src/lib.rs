use light_poseidon::{
    parameters::bn254_x5_3::poseidon_parameters, Poseidon, PoseidonBytesHasher, PoseidonError,
    HASH_LEN,
};

pub mod constants;

pub const DATA_LEN: usize = 32;
pub const MAX_HEIGHT: usize = 9;
pub const MERKLE_TREE_HISTORY_SIZE: usize = 256;

pub trait Hasher {}

pub struct MerkleTree {
    /// Height of the Merkle tree.
    pub height: usize,
    /// Subtree hashes.
    pub filled_subtrees: [[u8; HASH_LEN]; MAX_HEIGHT],
    /// Full history of roots of the Merkle tree (the last one is the current
    /// one).
    pub roots: [[u8; HASH_LEN]; MERKLE_TREE_HISTORY_SIZE],
    /// Next index to insert a leaf.
    pub next_index: usize,
    /// Current index of the root.
    pub current_root_index: usize,

    /// Poseidon hasher.
    poseidon_hasher: Box<dyn PoseidonBytesHasher>,
}

impl MerkleTree {
    pub fn new(height: usize) -> Self {
        assert!(height > 0);
        assert!(height <= MAX_HEIGHT);

        let mut filled_subtrees = [[0; HASH_LEN]; MAX_HEIGHT];

        for i in 0..height {
            filled_subtrees[i] = constants::ZERO_BYTES_MERKLE_TREE[i];
        }

        let mut roots = [[0; HASH_LEN]; MERKLE_TREE_HISTORY_SIZE];
        roots[0] = constants::ZERO_BYTES_MERKLE_TREE[height];

        let poseidon_hasher = Box::new(Poseidon::new(poseidon_parameters()));

        MerkleTree {
            height,
            filled_subtrees,
            roots,
            next_index: 0,
            current_root_index: 0,
            poseidon_hasher,
        }
    }

    fn hash(
        &mut self,
        leaf1: [u8; DATA_LEN],
        leaf2: [u8; DATA_LEN],
    ) -> Result<[u8; HASH_LEN], PoseidonError> {
        self.poseidon_hasher.hash_bytes(&[&leaf1, &leaf2])
    }

    pub fn insert(
        &mut self,
        leaf1: [u8; DATA_LEN],
        leaf2: [u8; DATA_LEN],
    ) -> Result<(), PoseidonError> {
        // Check if next index doesn't exceed the Merkle tree capacity.
        assert_ne!(self.next_index, 2usize.pow(self.height as u32));

        let mut current_index = self.next_index / 2;
        let mut current_level_hash = self.hash(leaf1, leaf2)?;

        for i in 1..self.height {
            println!("current index: {current_index}");
            let (left, right) = if current_index % 2 == 0 {
                println!("assiging current hash to subtree {}", i);
                self.filled_subtrees[i] = current_level_hash;

                println!("current_hash = hash(current_hash, zeros[{i}])");
                (current_level_hash, constants::ZERO_BYTES_MERKLE_TREE[i])
            } else {
                println!("current_hash = hash(filled_subtrees[{i}], current_hash)");
                (self.filled_subtrees[i], current_level_hash)
            };

            current_index /= 2;
            current_level_hash = self.hash(left, right)?;
        }

        self.current_root_index = (self.current_root_index + 1) % MERKLE_TREE_HISTORY_SIZE;
        println!("current root index: {}", self.current_root_index);
        self.roots[self.current_root_index] = current_level_hash;
        self.next_index += 2;

        Ok(())
    }

    pub fn is_known_root(&self, root: [u8; HASH_LEN]) -> bool {
        for i in (0..(self.current_root_index + 1)).rev() {
            if self.roots[i] == root {
                return true;
            }
        }
        return false;
    }

    pub fn last_root(&self) -> [u8; HASH_LEN] {
        self.roots[self.current_root_index]
    }
}
