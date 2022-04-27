mod filter;
use filter::BloomFilter;
use openssl::hash::MessageDigest;
use std::error::Error;
use std::io::Write;
use std::{fs, io::BufRead};
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let usage = "Usage: cargo run <dictionary_name> <input_file_name> <3_hash_output_file_name> <5_hash_output_file_name>";
    let dictionary = args.next().expect(usage);
    let input_file_name = args.next().expect(usage);
    let output_file_name_3 = args.next().expect(usage);
    let output_file_name_5 = args.next().expect(usage);

    let dictionary_file = fs::File::open(dictionary)?;
    let input_file = fs::File::open(input_file_name)?;

    let buf_reader = std::io::BufReader::new(&dictionary_file);
    let input_buf_reader = std::io::BufReader::new(&input_file);

    let mut bloom_filter_1 = BloomFilter::new(
        vec![
            MessageDigest::shake_256(),
            MessageDigest::sha512(),
            MessageDigest::sha1(),
        ],
        (*&dictionary_file.metadata()?.len().to_owned()) as usize,
    );

    let mut bloom_filter_2 = BloomFilter::new(
        vec![
            MessageDigest::sha3_256(),
            MessageDigest::shake_256(),
            MessageDigest::sha512(),
            MessageDigest::sha1(),
            MessageDigest::sha384(),
        ],
        (*&dictionary_file.metadata()?.len().to_owned()) as usize,
    );

    for line in buf_reader.lines() {
        let line = line?;
        bloom_filter_1.insert(&line)?;
        bloom_filter_2.insert(&line)?;
    }

    let mut out_file_1 = fs::File::create(output_file_name_3)?;
    let mut out_file_2 = fs::File::create(output_file_name_5)?;
    for line in input_buf_reader.lines().skip(1) {
        let line = line?;
        let file_1_buf = format!("{line} {}\n",bloom_filter_1.lookup(&line)?);
        let file_2_buf = format!("{line} {}\n",bloom_filter_2.lookup(&line)?);
        
        out_file_1.write_all(file_1_buf.as_bytes())?;
        out_file_2.write_all(file_2_buf.as_bytes())?;
        
    }

    Ok(())
}
