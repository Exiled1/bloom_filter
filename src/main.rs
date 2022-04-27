mod filter;
use filter::BloomFilter;
use openssl::hash::MessageDigest;
use std::error::Error;
use std::{fs, io::BufRead};
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let usage = "Usage: cargo run <dictionary_name> <file_name> <3 or 5 for the hash functions>";
    let file_name = args.next().expect(usage);
    let other_name = args.next().expect(usage);
    let hash_num = args.next().expect(usage);

    let file = fs::File::open(file_name)?;
    let other_file = fs::File::open(other_name)?;

    let buf_reader = std::io::BufReader::new(&file);
    let checker_buf_reader = std::io::BufReader::new(&other_file);

    let mut bloom_filter = match hash_num.parse::<u8>()? {
        3 => BloomFilter::new(
            vec![
                MessageDigest::shake_256(),
                MessageDigest::sha512(),
                MessageDigest::sha1(),
            ],
            (*&file.metadata()?.len().to_owned()) as usize,
        ),

        5 => BloomFilter::new(
            vec![
                MessageDigest::sha3_256(),
                MessageDigest::shake_256(),
                MessageDigest::sha512(),
                MessageDigest::sha1(),
                MessageDigest::sha384(),
            ],
            (*&file.metadata()?.len().to_owned()) as usize,
        ),
        _ => {
            panic!("Invalid hash number, please input 3 or 5")
        }
    };

    for line in buf_reader.lines() {
        let line = line?;
        bloom_filter.insert(&line)?;
    }
    for line in checker_buf_reader.lines().skip(1) {
        let line = line?;
        println!("{line} {}", bloom_filter.lookup(&line)?);
    }

    Ok(())
}
