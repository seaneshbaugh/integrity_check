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

    static ref SHA384_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b",
            "hello" => "59e1748777448c69de6b800d7a33bbfb9ff1b463e44354c3553bcdb9c666fa90125a3c79f90397bdf5f6a13de828684f",
            "world" => "a4d102bb2a39b6f1d9e481ef1a16b8948a0df2b594fd031bad6f201fbd6b0656846a6e58a30aa57ff34d912e7d3ea185"
        }
    };

    static ref SHA512_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
            "hello" => "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043",
            "world" => "11853df40f4b2b919d3815f64792e58d08663767a494bcbb38c0b2389d9140bbb170281b4a847be7757bde12c9cd0054ce3652d0ad3a1a0c92babb69798246ee"
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

#[test]
fn test_sha384() {
    for (string, expected) in SHA384_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha384::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}

#[test]
fn test_sha512() {
    for (string, expected) in SHA512_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha512::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}
