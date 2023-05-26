use anchor_lang::solana_program::blake3::{hash, hashv};
use bytemuck::{Pod, Zeroable};

use crate::{Hash, Hasher};

#[derive(Clone, Copy)] // To allow using with zero copy Solana accounts.
#[repr(C)]
pub struct Blake3;

impl Hasher for Blake3 {
    fn hash(val: &[u8]) -> Hash {
        hash(val).to_bytes()
    }

    fn hashv(vals: &[&[u8]]) -> Hash {
        hashv(vals).to_bytes()
    }
}

unsafe impl Zeroable for Blake3 {}
unsafe impl Pod for Blake3 {}
