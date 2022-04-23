use bitvec::prelude::*;
use hex::{self, ToHex};
use openssl::hash::{self, MessageDigest};
use std::{error::Error, io::Read};

pub struct BloomFilter {
    pub bit_vector: BitVec,
    digest_list: Vec<MessageDigest>,
    size: usize,
}

impl BloomFilter {
    pub fn new(digest_list: Vec<MessageDigest>, size: usize) -> BloomFilter {
        let mut bit_vector = BitVec::<usize, LocalBits>::with_capacity(size);
        bit_vector.resize(size, false);
        // dbg!(&bit_vector);
        BloomFilter {
            size,
            bit_vector,
            digest_list: digest_list.to_owned(),
        }
    }

    pub fn insert(&mut self, item: &str) -> Result<(), Box<dyn Error>> {
        let mut hash_values = Vec::new();
        // println!("{}", self.size);
        for index in self.hash_index(item)? {
            hash_values.push(index);
        }
        // println!("Size of the bit vector: {}", self.bit_vector.len());
        for hash_value in hash_values {
            // println!("place to set {hash_value}");
            self.bit_vector.set(hash_value, true);
        }
        // dbg!(&self.bit_vector);
        Ok(())
    }

    pub fn lookup(&self, item: &str) -> Result<String, Box<dyn Error>> {
        let mut hash_values = Vec::new();
        let mut exists = true;
        for index in self.hash_index(item)? {
            hash_values.push(index);
        }
        // println!("{:?} ", hash_values);
        for hash_value in hash_values {
            // println!(" retrieve {:?}", self.bit_vector.get(hash_value));

            if let Some(item) = self.bit_vector.get(hash_value) {
                // If any bit isn't set, then the item is not in the set
                if *item == false {
                    exists = false;
                    break;
                }
            }
        }

        if exists {
            Ok(String::from("maybe"))
        } else {
            Ok("no".to_owned())
        }
    }

    fn hash_index(&self, item: &str) -> Result<Vec<usize>, Box<dyn Error>> {
        let mut final_indices = Vec::new();
        for cur_hash in &self.digest_list {
            let mut hasher = hash::Hasher::new(*cur_hash)?;
            hasher.update(item.as_bytes())?;
            let hash_value = hasher.finish()?;
            let hash_value = hash_value
                .iter()
                .enumerate()
                .fold(0, |acc, (i, x)| acc + (*x as usize) * (i * 16 + 1));

            // dbg!(hash_value);
            final_indices.push(hash_value as usize % self.size);
        }
        Ok(final_indices)
    }
}
