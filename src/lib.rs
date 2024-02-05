// #![crate_name = "find_duplicate_files"]

use std::collections::HashMap;
use std::collections::BTreeMap;
use walkdir::WalkDir;
use std::io::Error;

use clap::{Parser, ArgAction};
// use std::error::Error;

#[derive(Parser,Default,Debug)]
#[clap(name="ZTask", author="Tom Zakrajsek", version, about)]
/// A very simple Task Manager
struct Arguments {
    /// Declare one or more paths to search for duplicate files
    #[clap(short, long, num_args(0..), action=ArgAction::Append)]
    path: Option<Vec<String>>,

    /// Increase logging verbosity
    #[clap(short, long, action=ArgAction::Count)]
    verbose: u8,
}

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

pub fn run() -> Result<(), Error>{
    let args: Arguments = Arguments::parse();

    // Print arguments for debugging
    println!("{:?}", args);

    let mut filenames = HashMap::new();
    let mut shas = BTreeMap::new();

    if let Some(paths) = args.path {
        if paths.is_empty() {
            // --path was provided without paths
            // println!("No paths provided with --path option");
            return Err(Error::new(std::io::ErrorKind::Other, "No paths provided with --path option"));
        } else {
            // --path was provided with path(s)
            for path in paths {
                println!("Searching for duplicate files in {}", path);
                for entry in WalkDir::new(path)
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
                }
            }
        }
    }


    println!("Found {} files with duplicate names", filenames.values().filter(|&count| *count > 1).count());
    println!("Found {} files with distinct contents", shas.len());
    println!("Found {} files replicated to multiple locations", shas.values().filter(|&v| v.len() >1).count());

    if args.verbose > 0 {
        for (name, count) in filenames.iter() {
            if *count > 1 {
                println!("{}: {}", name, count);
            }
        }

        for (sha_value, paths) in shas.iter() {
            if paths.len() > 1 {
                println!("{}:", sha_value);
                for path in paths {
                    println!("  {}", path);
                }
            }
        }
    }

    Ok(())
}
