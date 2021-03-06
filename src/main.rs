use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

extern crate integrity_check;

fn main() {
    let arguments: Vec<_> = env::args_os().collect();

    for argument in arguments.iter().skip(1) {
        match fs::metadata(&argument) {
            Ok(_) => (),
            Err(e) => {
                writeln!(&mut std::io::stderr(), "Error opening file {:?}. {}.", argument, e.to_string()).unwrap();

                std::process::exit(1);
            }
        }

        let mut file = File::open(argument).unwrap();

        let mut bytes = Vec::new();

        match file.read_to_end(&mut bytes) {
            Ok(_) => (),
            Err(e) => {
                writeln!(&mut std::io::stderr(), "Error reading file {:?}. {}.", argument, e.to_string()).unwrap();

                std::process::exit(1);
            }
        }

        let file_name: std::borrow::Cow<str> = argument.to_string_lossy();

        println!("{}", file_name);

        for _ in 0..(file_name.len()) {
            print!("-");
        }

        println!("");

        println!("   CRC32 = {}", integrity_check::digest::crc32::digest(&bytes));

        println!("     MD5 = {}", integrity_check::digest::md5::digest(&bytes));

        println!("    SHA1 = {}", integrity_check::digest::sha1::digest(&bytes));

        println!("SHA2-224 = {}", integrity_check::digest::sha2_224::digest(&bytes));

        println!("SHA2-256 = {}", integrity_check::digest::sha2_256::digest(&bytes));

        println!("SHA2-384 = {}", integrity_check::digest::sha2_384::digest(&bytes));

        println!("SHA2-512 = {}", integrity_check::digest::sha2_512::digest(&bytes));

        println!("SHA3-224 = {}", integrity_check::digest::sha3_224::digest(&bytes));

        println!("SHA3-256 = {}", integrity_check::digest::sha3_256::digest(&bytes));

        println!("SHA3-384 = {}", integrity_check::digest::sha3_384::digest(&bytes));

        println!("SHA3-512 = {}", integrity_check::digest::sha3_512::digest(&bytes));

        println!("");
    }
}
