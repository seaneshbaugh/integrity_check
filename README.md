# Integrity Check

Find various hashes/checksums of files.

Currently supports:

* CRC32
* MD5
* SHA1
* SHA2 (224/256/384/512)
* SHA3 (512)

## Building

    $ cargo build

## Testing

    $ cargo test

## Usage

    $ cargo run file1.txt file2.txt file3.txt

or

    $ ./target/debug/integrity_check file1.txt file2.txt file3.txt
