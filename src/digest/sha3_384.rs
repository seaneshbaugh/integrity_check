use super::sha3;

pub fn digest(bytes: &Vec<u8>) -> String {
    sha3::digest(bytes, 48)
}
