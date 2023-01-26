// TODO: check john the ripper and use it to improve this crate
// TODO: benchmark and profile it.

use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_LENGTH: usize = 40;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sharipper: <wordlist.txt> <SHA1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();

    if hash_to_crack.len() != SHA1_LENGTH {
        return Err("SHA1 hash is not valid".into());
    }

    let wordlist = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();

        if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!(" Secret found: {}", &common_password);
            return Ok(());
        }
    }

    println!("Secret of the provided SHA1 hash is not found in the wordlist :(");

    Ok(())
}
