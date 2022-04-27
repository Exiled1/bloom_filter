# Bloom Filter

A simple bloom filter implementation in Rust. With some fun addendums if I have the time :3

How to compile:

```
Usage: cargo run <dictionary_name> <input_file_name> <3_hash_output_file_name> <5_hash_output_file_name>
```

If you want to get a raw binary to use instead, run these commands

```
cargo build --release
./target/release/bloom_filter <dictionary_name> <input_file_name> <3_hash_output_file_name> <5_hash_output_file_name>
```

example
```
cargo build --release
./target/release/bloom_filter src/dictionary2.txt src/sample_input.txt hash3_file.txt hash5_file.txt
```

