extern crate integrity_check;

#[test]
fn test_crc32() {
    assert_eq!(integrity_check::digest::crc32::digest(&"hello".to_string().into_bytes()), "3610a686".to_string());

    assert_eq!(integrity_check::digest::crc32::digest(&"world".to_string().into_bytes()), "3a771143".to_string());
}
