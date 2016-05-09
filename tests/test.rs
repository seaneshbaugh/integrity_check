extern crate integrity_check;

#[test]
fn test_crc32() {
    assert_eq!(integrity_check::digest::crc32::digest(&"hello".to_string().into_bytes()), "3610a686".to_string());

    assert_eq!(integrity_check::digest::crc32::digest(&"world".to_string().into_bytes()), "3a771143".to_string());
}

#[test]
fn test_md5() {
    assert_eq!(integrity_check::digest::md5::digest(&"hello".to_string().into_bytes()), "5d41402abc4b2a76b9719d911017c592".to_string());

    assert_eq!(integrity_check::digest::md5::digest(&"world".to_string().into_bytes()), "7d793037a0760186574b0282f2f435e7".to_string());
}
