use std::{
    env,
    fs::File,
    io::{self, prelude::*},
    path::PathBuf,
    process::{Command, Stdio},
    thread::spawn,
};

use clap::{Parser, ValueEnum};
use quote::quote;
use sha2::{Digest, Sha256};

use light_merkle_tree::HASH_LEN;

#[derive(Debug, Clone, ValueEnum)]
enum Hash {
    Sha256,
}

#[derive(Debug, Parser)]
pub struct Options {
    #[clap(value_enum, long, default_value_t = Hash::Sha256)]
    hash: Hash,
    #[clap(long)]
    path: Option<PathBuf>,
}

fn rustfmt(code: String) -> Result<Vec<u8>, anyhow::Error> {
    let mut cmd = match env::var_os("RUSTFMT") {
        Some(r) => Command::new(r),
        None => Command::new("rustfmt"),
    };

    let mut cmd = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = cmd.stdin.take().unwrap();
    let mut stdout = cmd.stdout.take().unwrap();

    let stdin_handle = spawn(move || {
        stdin.write_all(code.as_bytes()).unwrap();
    });

    let mut formatted_code = vec![];
    io::copy(&mut stdout, &mut formatted_code)?;

    let _ = cmd.wait();
    stdin_handle.join().unwrap();

    Ok(formatted_code)
}

pub fn generate_zero_bytes(opts: Options) -> Result<(), anyhow::Error> {
    let mut hasher = match opts.hash {
        Hash::Sha256 => Sha256::new(),
    };

    let mut zero_bytes = [[0u8; 32]; 19];
    let mut zero_bytes_tokens = vec![];

    hasher.update([1u8; 32]);
    hasher.update([1u8; 32]);

    let mut prev_hash = <[u8; HASH_LEN]>::try_from(hasher.finalize_reset().to_vec()).unwrap();

    for i in 0..19 {
        hasher.update(prev_hash);
        hasher.update(prev_hash);

        let cur_hash = <[u8; HASH_LEN]>::try_from(hasher.finalize_reset().to_vec()).unwrap();
        zero_bytes[i] = cur_hash;

        let cur_hash_iter = cur_hash.iter();
        zero_bytes_tokens.push(quote! {
            [ #(#cur_hash_iter),* ]
        });

        prev_hash = cur_hash;
    }

    // NOTE(vadorovsky): I couldn't find any way to do a double repetition
    // over a 2D array inside `quote` macro, that's why arrays are converted
    // to tokens in the loop above. But I would be grateful if there is any
    // way to make it prettier.
    //
    // Being able to do something like:
    //
    // ```rust
    // let code = quote! {
    //     const ZERO_BYTES: ZeroBytes = [ #([ #(#zero_bytes),* ]),* ];
    // };
    // ```
    //
    // would be great.
    let code = quote! {
        use super::ZeroBytes;

        pub const ZERO_BYTES: ZeroBytes = [ #(#zero_bytes_tokens),* ];
    };

    println!(
        "Zero bytes (generated with {:?} hash): {:?}",
        opts.hash, zero_bytes
    );

    if let Some(path) = opts.path {
        let mut file = File::create(&path)?;
        file.write_all(b"// This file is generated by xtask. Do not edit it manually.\n\n")?;
        file.write_all(&rustfmt(code.to_string())?)?;
        println!("Zero bytes written to {:?}", path);
    }

    Ok(())
}
