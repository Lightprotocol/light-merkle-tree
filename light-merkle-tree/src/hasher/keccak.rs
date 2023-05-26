use anchor_lang::solana_program::keccak::{hash, hashv};
use bytemuck::{Pod, Zeroable};

use crate::{Hash, Hasher};

#[derive(Clone, Copy)] // To allow using with zero copy Solana accounts.
#[repr(C)]
pub struct Keccak;

impl Hasher for Keccak {
    fn hash(val: &[u8]) -> Hash {
        hash(val).to_bytes()
    }

    fn hashv(vals: &[&[u8]]) -> Hash {
        hashv(vals).to_bytes()
    }
}

unsafe impl Zeroable for Keccak {}
unsafe impl Pod for Keccak {}
