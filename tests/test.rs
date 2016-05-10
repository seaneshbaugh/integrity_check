#[macro_use]
extern crate lazy_static;

extern crate integrity_check;

use std::collections::HashMap;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();

            $(m.insert($key, $value);)+

            m
        }
     };
);

lazy_static! {
    static ref CRC32_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "00000000",
            "hello" => "3610a686",
            "world" => "3a771143"
        }
    };

    static ref MD5_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "d41d8cd98f00b204e9800998ecf8427e",
            "hello" => "5d41402abc4b2a76b9719d911017c592",
            "world" => "7d793037a0760186574b0282f2f435e7"
        }
    };

    static ref SHA1_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "da39a3ee5e6b4b0d3255bfef95601890afd80709",
            "hello" => "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d",
            "world" => "7c211433f02071597741e6ff5a8ea34789abbf43"
        }
    };

    static ref SHA224_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f",
            "hello" => "ea09ae9cc6768c50fcee903ed054556e5bfc8347907f12598aa24193",
            "world" => "06d2dbdb71973e31e4f1df3d7001fa7de268aa72fcb1f6f9ea37e0e5"
        }
    };

    static ref SHA256_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "hello" => "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
            "world" => "486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7"
        }
    };
}

#[test]
fn test_crc32() {
    for (string, expected) in CRC32_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::crc32::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}

#[test]
fn test_md5() {
    for (string, expected) in MD5_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::md5::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}

#[test]
fn test_sha1() {
    for (string, expected) in SHA1_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha1::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}

#[test]
fn test_sha224() {
    for (string, expected) in SHA224_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha224::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}

#[test]
fn test_sha256() {
    for (string, expected) in SHA256_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha256::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}
