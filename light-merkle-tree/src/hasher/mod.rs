pub mod solana;

pub const HASH_BYTES: usize = 32;

pub type Hash = [u8; HASH_BYTES];

pub trait Hasher {
    fn hash(val: &[u8]) -> Hash;
    fn hashv(vals: &[&[u8]]) -> Hash;
}
