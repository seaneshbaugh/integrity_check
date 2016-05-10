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

        let mut buffer = String::new();

        match file.read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(e) => {
                writeln!(&mut std::io::stderr(), "Error reading file {:?}. {}.", argument, e.to_string()).unwrap();

                std::process::exit(1);
            }
        }

        let bytes: Vec<u8> = buffer.into_bytes();

        let file_name: std::borrow::Cow<str> = argument.to_string_lossy();

        println!("{}", file_name);

        for _ in 0..(file_name.len()) {
            print!("-");
        }

        println!("");

        println!(" crc32 = {}", integrity_check::digest::crc32::digest(&bytes));

        println!("   md5 = {}", integrity_check::digest::md5::digest(&bytes));

        println!("  sha1 = {}", integrity_check::digest::sha1::digest(&bytes));

        println!("sha224 = {}", integrity_check::digest::sha224::digest(&bytes));

        println!("sha256 = {}", integrity_check::digest::sha256::digest(&bytes));

        println!("sha512 = {}", integrity_check::digest::sha512::digest(&bytes));

        println!("");
    }
}
