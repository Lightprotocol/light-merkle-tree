use anchor_lang::solana_program::hash::{hash, hashv};
use bytemuck::{Pod, Zeroable};

use crate::{Hash, Hasher};

#[derive(Clone, Copy)] // To allow using with zero copy Solana accounts.
#[repr(C)]
pub struct Sha256;

impl Hasher for Sha256 {
    fn hash(val: &[u8]) -> Hash {
        hash(val).to_bytes()
    }

    fn hashv(vals: &[&[u8]]) -> Hash {
        hashv(vals).to_bytes()
    }
}

unsafe impl Zeroable for Sha256 {}
unsafe impl Pod for Sha256 {}
