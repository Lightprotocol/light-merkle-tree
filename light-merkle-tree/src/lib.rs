use constants::ZeroBytes;
use digest::{Digest, FixedOutputReset};
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

pub struct MerkleTree<D>
where
    D: Digest + FixedOutputReset,
{
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

    /// sha256 hasher.
    hasher: D,
    /// Initial bytes of the Merkle tree (with all leaves having zero value).
    zero_bytes: ZeroBytes,
}

impl<D> MerkleTree<D>
where
    D: Digest + FixedOutputReset,
{
    pub fn new(height: usize, hasher: D, zero_bytes: ZeroBytes) -> Self {
        assert!(height > 0);
        assert!(height <= MAX_HEIGHT);

        let mut filled_subtrees = [[0; HASH_LEN]; MAX_HEIGHT];

        for i in 0..height {
            filled_subtrees[i] = zero_bytes[i];
        }

        let mut roots = [[0; HASH_LEN]; MERKLE_TREE_HISTORY_SIZE];
        roots[0] = zero_bytes[height - 1];

        MerkleTree {
            height,
            filled_subtrees,
            roots,
            next_index: 0,
            current_root_index: 0,
            hasher,
            zero_bytes,
        }
    }

    pub fn hash(
        &mut self,
        leaf1: [u8; DATA_LEN],
        leaf2: [u8; DATA_LEN],
    ) -> Result<[u8; HASH_LEN], MerkleTreeError> {
        // TODO(vadorovsky): The `digest` crate defines the `update` method both
        // in `Digest` and `Update` trait separately, therefore we need to
        // specify the trait explicitly here. That could be fixed by just
        // removing the `update` method from the `Digest` trait.
        Digest::update(&mut self.hasher, &leaf1);
        Digest::update(&mut self.hasher, &leaf2);
        Ok(
            <[u8; HASH_LEN]>::try_from(self.hasher.finalize_reset().to_vec())
                .map_err(|_| MerkleTreeError::SliceToArray)?,
        )
    }

    pub fn insert(
        &mut self,
        leaf1: [u8; DATA_LEN],
        leaf2: [u8; DATA_LEN],
    ) -> Result<(), MerkleTreeError> {
        // Check if next index doesn't exceed the Merkle tree capacity.
        assert_ne!(self.next_index, 2usize.pow(self.height as u32));

        let mut current_index = self.next_index / 2;
        let mut current_level_hash = self.hash(leaf1, leaf2)?;

        println!(
            "current level hash (hash of new leaves) {:?}",
            current_level_hash
        );
        println!("starting the loop (1..height)");

        for i in 1..self.height {
            // println!("current index: {current_index}");
            let (left, right) = if current_index % 2 == 0 {
                println!("assiging current hash to subtree {}", i);
                self.filled_subtrees[i] = current_level_hash;

                // println!("current_hash = hash(current_hash, zeros[{i}])");
                (current_level_hash, self.zero_bytes[i])
            } else {
                // println!("current_hash = hash(filled_subtrees[{i}], current_hash)");
                (self.filled_subtrees[i], current_level_hash)
            };

            current_index /= 2;
            current_level_hash = self.hash(left, right)?;
            println!("current level hash {} {:?}", i, current_level_hash);
        }

        self.current_root_index = (self.current_root_index + 1) % MERKLE_TREE_HISTORY_SIZE;
        // println!("current root index: {}", self.current_root_index);
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
