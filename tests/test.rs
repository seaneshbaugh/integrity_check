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
    static ref SMALL_TEST_FILE_CONTENTS: &'static str = {
        include_str!("small-test-file.txt")
    };

    static ref MEDIUM_TEST_FILE_CONTENTS: &'static str = {
        include_str!("medium-test-file.txt")
    };

    static ref LARGE_TEST_FILE_CONTENTS: &'static str = {
        include_str!("large-test-file.txt")
    };

    static ref CRC32_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "00000000",
            "hello" => "3610a686",
            "world" => "3a771143",
            &SMALL_TEST_FILE_CONTENTS => "3e61b90e",
            &MEDIUM_TEST_FILE_CONTENTS => "4c22cf4a",
            &LARGE_TEST_FILE_CONTENTS => "a66abf8a"
        }
    };

    static ref MD5_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "d41d8cd98f00b204e9800998ecf8427e",
            "hello" => "5d41402abc4b2a76b9719d911017c592",
            "world" => "7d793037a0760186574b0282f2f435e7",
            &SMALL_TEST_FILE_CONTENTS => "043339190b14c0a2e1e8a2583763472e",
            &MEDIUM_TEST_FILE_CONTENTS => "fd8d3f6996d52093ab70795e2c0a6788",
            &LARGE_TEST_FILE_CONTENTS => "4327870cc9e86e823cd860e2a98c1719"
        }
    };

    static ref SHA1_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "da39a3ee5e6b4b0d3255bfef95601890afd80709",
            "hello" => "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d",
            "world" => "7c211433f02071597741e6ff5a8ea34789abbf43",
            &SMALL_TEST_FILE_CONTENTS => "c4016b916668276885e5cb793e22113122c73715",
            &MEDIUM_TEST_FILE_CONTENTS => "fa33660c5264767ef9eaa027ab60b72fd3ecbe52",
            &LARGE_TEST_FILE_CONTENTS => "ad5098ca070c03532d9e97033e88237cb6f2c995"
        }
    };

    static ref SHA2_224_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "d14a028c2a3a2bc9476102bb288234c415a2b01f828ea62ac5b3e42f",
            "hello" => "ea09ae9cc6768c50fcee903ed054556e5bfc8347907f12598aa24193",
            "world" => "06d2dbdb71973e31e4f1df3d7001fa7de268aa72fcb1f6f9ea37e0e5",
            &SMALL_TEST_FILE_CONTENTS => "14959754f6908b7a7ed47735b61ca50c5abd3173cbd9d102b5f95991",
            &MEDIUM_TEST_FILE_CONTENTS => "b8282853f5ab0edda62feccefbea6951d8562d3f9ffd4b03fbe3fc22",
            &LARGE_TEST_FILE_CONTENTS => "351a511d65f0bc1b5e112c604358e28d5acec8ff96e79810fe7eae49"
        }
    };

    static ref SHA2_256_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            "hello" => "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
            "world" => "486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7",
            &SMALL_TEST_FILE_CONTENTS => "4f1a08f3202c3f01dbb6ce284bdb3d0b74a7615064f337703a8612dc71795451",
            &MEDIUM_TEST_FILE_CONTENTS => "97faf0b737a9cd5cd0ab932efd38345d9603f5ace21f2d127fd302873e148c84",
            &LARGE_TEST_FILE_CONTENTS => "d86e8fc7eb6192e27255d543ecc60a6d6cba44a0dd2394320e636639a882a5b1"
        }
    };

    static ref SHA2_384_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da274edebfe76f65fbd51ad2f14898b95b",
            "hello" => "59e1748777448c69de6b800d7a33bbfb9ff1b463e44354c3553bcdb9c666fa90125a3c79f90397bdf5f6a13de828684f",
            "world" => "a4d102bb2a39b6f1d9e481ef1a16b8948a0df2b594fd031bad6f201fbd6b0656846a6e58a30aa57ff34d912e7d3ea185",
            &SMALL_TEST_FILE_CONTENTS => "3a0cfacd27f65814b6a5e4ad73f2103ef525879c4cab162d2833dc322e025a1d3dbf95e88cf4516129631297a78cc416",
            &MEDIUM_TEST_FILE_CONTENTS => "e24896904ce3e20bc1b1a26a591f3b3859db827d69984b78f4992e91d67e16f2711ea4de4142e8a3b79892bd289b385f",
            &LARGE_TEST_FILE_CONTENTS => "d339edbcbe58250be99d7f575d5f43dd88efb737b960f9d87990947421708dc6609ff6841ed23175d80f1f028fec7ae1"
        }
    };

    static ref SHA2_512_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
            "hello" => "9b71d224bd62f3785d96d46ad3ea3d73319bfbc2890caadae2dff72519673ca72323c3d99ba5c11d7c7acc6e14b8c5da0c4663475c2e5c3adef46f73bcdec043",
            "world" => "11853df40f4b2b919d3815f64792e58d08663767a494bcbb38c0b2389d9140bbb170281b4a847be7757bde12c9cd0054ce3652d0ad3a1a0c92babb69798246ee",
            &SMALL_TEST_FILE_CONTENTS => "168edafdeaddc99224326058301898d7bcc40dfd7bf48c0d17f6478b658c3d7d6c194b28452cd7a728c5892533746218e161586a1b3920cf2939c16d19ae3020",
            &MEDIUM_TEST_FILE_CONTENTS => "7651854178baf325ce6031f79d1acd808f8952ab548bcdaef4b48e98bff1844fb8fb2b1d651a2d77433f51e2668e02d03c07358c65ec47c2ae4e49b21f114b22",
            &LARGE_TEST_FILE_CONTENTS => "c75345a5a58a4017299ab3d941618532d38dd36b6241ac720f24b884a7469f3cc315bbb3e2a1705f0337f3fb53032da8b9a3837f7c98774c540f94c1bd7d7820"
        }
    };

    static ref SHA3_224_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "6b4e03423667dbb73b6e15454f0eb1abd4597f9a1b078e3f5b5a6bc7",
            "hello" => "b87f88c72702fff1748e58b87e9141a42c0dbedc29a78cb0d4a5cd81",
            "world" => "cd7b2b8e2d55948edcc4811388ab3915f26df12e6b7a39f744efdb95",
            &SMALL_TEST_FILE_CONTENTS => "1eb7f31e4265909fc083426132d9ee1662e5d081d1672c122a6b0496",
            &MEDIUM_TEST_FILE_CONTENTS => "32a5d228892ccb33f8c00e69d18687ad4b9c8668e2d553065e5fc3c2",
            &LARGE_TEST_FILE_CONTENTS => "f02f7b7f3840ed256cf62682eecab248f27af3a5c623a294298b4b59"
        }
    };

    static ref SHA3_256_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a",
            "hello" => "3338be694f50c5f338814986cdf0686453a888b84f424d792af4b9202398f392",
            "world" => "420baf620e3fcd9b3715b42b92506e9304d56e02d3a103499a3a292560cb66b2",
            &SMALL_TEST_FILE_CONTENTS => "ce510fc983ddecbfb54d25304de06e6474651b9e0443e9ef92b07af387786416",
            &MEDIUM_TEST_FILE_CONTENTS => "ed0d7bd5ada0e0670fa6368e1172c8e278574ef6871a740ad3fd36ec280a15e7",
            &LARGE_TEST_FILE_CONTENTS => "e13cb19010e3a20c6a4429eb44bc883da9976890a6fe424e3d61ddde21b20924"
        }
    };

    static ref SHA3_384_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "0c63a75b845e4f7d01107d852e4c2485c51a50aaaa94fc61995e71bbee983a2ac3713831264adb47fb6bd1e058d5f004",
            "hello" => "720aea11019ef06440fbf05d87aa24680a2153df3907b23631e7177ce620fa1330ff07c0fddee54699a4c3ee0ee9d887",
            "world" => "693ff8ff69391116b2763613c60b560fbba523ecbad06e2e93a0511239cea39a272614535f6a4c7dc6d1ea6f477563a4",
            &SMALL_TEST_FILE_CONTENTS => "0840afd9bce4805232e804bab1e1054b20220f81d4c2bc41bab786dc1836e77edc9c2495c0eff546daec8b3854bc67ae",
            &MEDIUM_TEST_FILE_CONTENTS => "9bb7686aa798b94751bae19a8d4818320bb44142f3c72183f423eb19c196890c522713616edce63c5184aad17f1f2598",
            &LARGE_TEST_FILE_CONTENTS => "037d2ecd06990071aab20f18d7e68defcb402a75bcc62612db25381b17f1afd178c835efcc11fd069ca7b336c140bef9"
        }
    };

    static ref SHA3_512_EXPECTED: HashMap<&'static str, &'static str> = {
        map!{
            "" => "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26",
            "hello" => "75d527c368f2efe848ecf6b073a36767800805e9eef2b1857d5f984f036eb6df891d75f72d9b154518c1cd58835286d1da9a38deba3de98b5a53e5ed78a84976",
            "world" => "6ec5025ab9e3f5c74d15fb95404746c24ff11d3a4b597e2eab26f938d42aa2fd2a47e2e48e314372d129a5b6db88e63e315bb99273612641da44630d842fb6d9",
            &SMALL_TEST_FILE_CONTENTS => "3ec05d8ee493a99fba33c34e8d150ac15ceb071e82a48766fb4c8d467cd8a6cacb3fab88346f6f54b22afca6c84cbaae187c95ca130775fa971990803fd72a01",
            &MEDIUM_TEST_FILE_CONTENTS => "13ab7ffc821da6a0101727d776df47e758309700cfff3cb427ef49707b0833dc7f3cc37ef3b2264f60a868bea47b1229d9110de3949415c1ce0709072b24352b",
            &LARGE_TEST_FILE_CONTENTS => "684b36f9d0532c1736d82a1c27245ebcdd2a168e22710ca744dce5ff9fb8c75d438d345dd02b7a39186d3434118252e9155f66473ea8427bae07c448d4efe7e6"
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
fn test_sha2_224() {
    for (string, expected) in SHA2_224_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha2_224::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}

#[test]
fn test_sha2_256() {
    for (string, expected) in SHA2_256_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha2_256::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}

#[test]
fn test_sha2_384() {
    for (string, expected) in SHA2_384_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha2_384::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}

#[test]
fn test_sha2_512() {
    for (string, expected) in SHA2_512_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha2_512::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}

#[test]
fn test_sha3_224() {
    for (string, expected) in SHA3_224_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha3_224::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}

#[test]
fn test_sha3_256() {
    for (string, expected) in SHA3_256_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha3_256::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}

#[test]
fn test_sha3_384() {
    for (string, expected) in SHA3_384_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha3_384::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}

#[test]
fn test_sha3_512() {
    for (string, expected) in SHA3_512_EXPECTED.iter() {
        assert_eq!(integrity_check::digest::sha3_512::digest(&string.to_string().into_bytes()), expected.to_string());
    }
}
