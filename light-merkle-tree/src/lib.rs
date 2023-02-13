use constants::ZeroBytes;
use hasher::{Hash, Hasher};
use thiserror::Error;

pub mod constants;

pub const DATA_LEN: usize = 32;
pub const HASH_LEN: usize = 32;
pub const MAX_HEIGHT: usize = 18;
pub const MERKLE_TREE_HISTORY_SIZE: usize = 256;

#[derive(Error, Debug)]
pub enum MerkleTreeError {
    #[error("Could not convert slice to array")]
    SliceToArray,
}

#[derive(PartialEq, Eq, Debug)]
pub struct MerkleTreeData {
    /// Height of the Merkle tree.
    pub height: usize,
    // TODO(vadorovsky): Check if Solana is OK with having a const generic
    // instead of MAX_HEIGHT.
    /// Subtree hashes.
    pub filled_subtrees: [[u8; HASH_LEN]; MAX_HEIGHT],
    /// Full history of roots of the Merkle tree (the last one is the current
    /// one).
    pub roots: [[u8; HASH_LEN]; MERKLE_TREE_HISTORY_SIZE],
    /// Next index to insert a leaf.
    pub next_index: usize,
    /// Current index of the root.
    pub current_root_index: usize,
}

pub struct MerkleTree<H>
where
    H: Hasher,
{
    /// State of the Merkle tree stored as byte arrays.
    pub data: MerkleTreeData,
    /// sha256 hasher.
    hasher: H,
    /// Initial bytes of the Merkle tree (with all leaves having zero value).
    // TODO: We don't want it as a field.
    // We want this struct to be used directly as a Solana account.
    // The main problem: we don't want to save zero bytes in a Solana account.
    zero_bytes: ZeroBytes,
}

impl<H> MerkleTree<H>
where
    H: Hasher,
{
    pub fn new(height: usize, hasher: H, zero_bytes: ZeroBytes) -> Self {
        assert!(height > 0);
        assert!(height <= MAX_HEIGHT);

        let mut filled_subtrees = [[0; HASH_LEN]; MAX_HEIGHT];

        for i in 0..height {
            filled_subtrees[i] = zero_bytes[i];
        }

        let mut roots = [[0; HASH_LEN]; MERKLE_TREE_HISTORY_SIZE];
        roots[0] = zero_bytes[height - 1];

        MerkleTree {
            data: MerkleTreeData {
                height,
                filled_subtrees,
                roots,
                next_index: 0,
                current_root_index: 0,
            },
            hasher,
            zero_bytes,
        }
    }

    pub fn from_data(data: MerkleTreeData, hasher: H, zero_bytes: ZeroBytes) -> Self {
        MerkleTree {
            data,
            hasher,
            zero_bytes,
        }
    }

    pub fn hash(&mut self, leaf1: [u8; DATA_LEN], leaf2: [u8; DATA_LEN]) -> Hash {
        // TODO(vadorovsky): The `digest` crate defines the `update` method both
        // in `Digest` and `Update` trait separately, therefore we need to
        // specify the trait explicitly here. That could be fixed by just
        // removing the `update` method from the `Digest` trait.
        self.hasher.hashv(&[&leaf1, &leaf2])
    }

    pub fn insert(&mut self, leaf1: [u8; DATA_LEN], leaf2: [u8; DATA_LEN]) {
        // Check if next index doesn't exceed the Merkle tree capacity.
        assert_ne!(self.data.next_index, 2usize.pow(self.data.height as u32));

        let mut current_index = self.data.next_index / 2;
        let mut current_level_hash = self.hash(leaf1, leaf2);

        for i in 1..self.data.height {
            let (left, right) = if current_index % 2 == 0 {
                self.data.filled_subtrees[i] = current_level_hash;
                (current_level_hash, self.zero_bytes[i])
            } else {
                (self.data.filled_subtrees[i], current_level_hash)
            };

            current_index /= 2;
            current_level_hash = self.hash(left, right);
        }

        self.data.current_root_index =
            (self.data.current_root_index + 1) % MERKLE_TREE_HISTORY_SIZE;
        self.data.roots[self.data.current_root_index] = current_level_hash;
        self.data.next_index += 2;
    }

    pub fn is_known_root(&self, root: [u8; HASH_LEN]) -> bool {
        for i in (0..(self.data.current_root_index + 1)).rev() {
            if self.data.roots[i] == root {
                return true;
            }
        }
        return false;
    }

    pub fn last_root(&self) -> [u8; HASH_LEN] {
        self.data.roots[self.data.current_root_index]
    }
}
