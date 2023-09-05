#![crate_name = "find_duplicate_files"]

use std::collections::HashMap;
use std::collections::BTreeMap;
use walkdir::WalkDir;
use std::io::Error;

// see https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Read};

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {

    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

fn main() -> Result<(), Error>{
    let mut filenames = HashMap::new();
    let mut shas = BTreeMap::new();

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file()) {
            let f_name = String::from(entry.file_name().to_string_lossy());
            let f_path = String::from(entry.path().to_string_lossy());
            let counter = filenames.entry(f_name.clone()).or_insert(0);
            *counter += 1;

            let input = File::open(f_path.clone())?;
            let reader = BufReader::new(input);
            let digest = sha256_digest(reader)?;
            let sha_val = HEXUPPER.encode(digest.as_ref());
            let sha_matches = shas.entry(sha_val).or_insert(Vec::new());
            sha_matches.push(f_path);

            //if *counter == 2 {
            //    println!("** {}", f_name);
            //}
    }

    for (sha_value, paths) in shas.iter() {
        if paths.len() > 1 {
            println!("{}:", sha_value);
            for path in paths {
                println!("  {}", path);
            }
        }
    }
    Ok(())
}
