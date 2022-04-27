mod filter;
use filter::BloomFilter;
use openssl::hash::MessageDigest;
use std::error::Error;
use std::rc::Rc;
use std::{fs, io::BufRead};
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let usage = "Usage: cargo run <dictionary_name> <file_name>";
    let file_name = args.next().expect(usage);
    let other_name = args.next().expect(usage);
    let file = fs::File::open(file_name)?;
    let other_file = fs::File::open(other_name)?;
    let buf_reader = std::io::BufReader::new(&file);
    let checker_buf_reader = std::io::BufReader::new(&other_file);
    let mut bloom_filter1 = BloomFilter::new(
        Rc::new(vec![
            MessageDigest::shake_256(),
            MessageDigest::sha512(),
            MessageDigest::sha1(),
        ]),
        (*&file.metadata()?.len().to_owned()) as usize,
    );

    let mut bloom_filter2 = BloomFilter::new(
        Rc::new(vec![
            MessageDigest::sha3_256(),
            MessageDigest::shake_256(),
            MessageDigest::sha512(),
            MessageDigest::sha1(),
            MessageDigest::sha384(),
        ]),
        (*&file.metadata()?.len().to_owned()) as usize,
    );

    for line in buf_reader.lines().skip(1) {
        // skip first line
        let line = line?;
        bloom_filter1.insert(&line)?;
        bloom_filter2.insert(&line)?;
    }
    for line in checker_buf_reader.lines().skip(1) {
        let line = line?;
        println!("{line} {}", bloom_filter1.lookup(&line)?);
        println!("{line} {}", bloom_filter2.lookup(&line)?);
    }

    Ok(())
}
