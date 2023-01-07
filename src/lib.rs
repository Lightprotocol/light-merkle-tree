use ark_bn254::Fq;
use ark_ff::{BigInteger, PrimeField};
use light_poseidon::{parameters::bn254_x5_3::poseidon_parameters, PoseidonError, PoseidonHasher};
use thiserror::Error;

pub(crate) mod constants;

pub const HASH_LEN: usize = 32;
pub const FILLED_SUBTREES: usize = 9;
pub const MERKLE_TREE_HISTORY_SIZE: usize = 256;

#[derive(Error, Debug)]
pub enum MerkleTreeError {
    #[error("Poseidon error: {0}")]
    Poseidon(#[from] PoseidonError),
    #[error("Could not convert vector to array")]
    ArrayToVec,
}

pub struct MerkleTree {
    pub height: usize,
    // Can this be a slice or vec? Or does it need to be a fixed size array?
    pub filled_subtrees: [[u8; HASH_LEN]; FILLED_SUBTREES],
    pub roots: [[u8; HASH_LEN]; MERKLE_TREE_HISTORY_SIZE],
    pub next_index: usize,
    pub current_root_index: usize,

    poseidon_hasher: PoseidonHasher<Fq>,
}

impl MerkleTree {
    pub fn new(height: usize) -> Self {
        assert!(height > 0);
        assert!(height <= FILLED_SUBTREES);

        let mut filled_subtrees = [[0; HASH_LEN]; FILLED_SUBTREES];

        for i in 0..height {
            filled_subtrees[i] = constants::ZERO_BYTES_MERKLE_TREE[i];
        }

        let mut roots = [[0; HASH_LEN]; MERKLE_TREE_HISTORY_SIZE];
        roots[0] = constants::ZERO_BYTES_MERKLE_TREE[height];

        let poseidon_hasher = PoseidonHasher::new(poseidon_parameters());

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
        leaf1: [u8; HASH_LEN],
        leaf2: [u8; HASH_LEN],
    ) -> Result<[u8; HASH_LEN], MerkleTreeError> {
        let leaf1 = Fq::from_be_bytes_mod_order(&leaf1);
        let leaf2 = Fq::from_be_bytes_mod_order(&leaf2);

        let hash = self.poseidon_hasher.hash(&[leaf1, leaf2])?;

        Ok(hash
            .into_repr()
            .to_bytes_be()
            .try_into()
            .map_err(|_| MerkleTreeError::ArrayToVec)?)
    }

    pub fn insert(
        &mut self,
        leaf1: [u8; HASH_LEN],
        leaf2: [u8; HASH_LEN],
    ) -> Result<(), MerkleTreeError> {
        assert_ne!(self.next_index, 2usize.pow(self.height as u32));

        let mut current_index = self.next_index / 2;
        let mut current_level_hash = self.hash(leaf1, leaf2)?;

        for i in 1..self.height {
            let (left, right) = if current_index % 2 == 0 {
                self.filled_subtrees[i] = current_level_hash;

                (current_level_hash, constants::ZERO_BYTES_MERKLE_TREE[i])
            } else {
                (self.filled_subtrees[i], current_level_hash)
            };

            current_index /= 2;
            current_level_hash = self.hash(left, right)?;
        }

        self.current_root_index = (self.current_root_index + 1) % MERKLE_TREE_HISTORY_SIZE;
        self.roots[self.current_root_index] = current_level_hash;
        self.next_index += 2;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merkle_tree() {
        let mut merkle_tree = MerkleTree::new(9);

        assert_eq!(merkle_tree.height, 9);
        assert_eq!(merkle_tree.next_index, 0);
        assert_eq!(merkle_tree.current_root_index, 0);

        assert_eq!(
            merkle_tree.filled_subtrees[0],
            constants::ZERO_BYTES_MERKLE_TREE[0]
        );
        assert_eq!(
            merkle_tree.filled_subtrees[1],
            constants::ZERO_BYTES_MERKLE_TREE[1]
        );

        let leaf1_1 = [1u8; HASH_LEN];
        let leaf1_2 = [2u8; HASH_LEN];

        merkle_tree.insert(leaf1_1, leaf1_2).unwrap();

        let hash1 = merkle_tree.hash(leaf1_1, leaf1_2).unwrap();

        assert_eq!(merkle_tree.next_index, 2);
        assert_eq!(merkle_tree.current_root_index, 1);

        assert_eq!(
            merkle_tree.filled_subtrees[0],
            constants::ZERO_BYTES_MERKLE_TREE[0]
        );
        assert_eq!(merkle_tree.filled_subtrees[1], hash1);

        let leaf2_1 = [3u8; HASH_LEN];
        let leaf2_2 = [4u8; HASH_LEN];

        merkle_tree.insert(leaf2_1, leaf2_2).unwrap();

        let hash2 = merkle_tree.hash(leaf2_1, leaf2_2).unwrap();

        assert_eq!(merkle_tree.next_index, 4);
        assert_eq!(merkle_tree.current_root_index, 2);

        // assert_eq!(
        //     merkle_tree.filled_subtrees[0],
        //     constants::ZERO_BYTES_MERKLE_TREE[0]
        // );
        // assert_eq!(merkle_tree.filled_subtrees[1], hash1);
        // assert_eq!(merkle_tree.filled_subtrees[2], ZERO_BYTES_MERKLE_TREE[3]);
    }
}
