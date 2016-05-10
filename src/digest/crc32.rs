lazy_static! {
    static ref LOOKUP_TABLE: [u32; 256] = {
        let mut table: [u32; 256] = [0; 256];

        for index in 0..256 {
            let mut crc: u32 = index as u32;

            for _ in 0..8 {
                let and1: u32 = crc & 1;

                // See https://internals.rust-lang.org/t/forbid-unsigned-integer/752/11
                let mask: u32 = match and1 {
                    0 => 0,
                    _ => !(and1) + 1
                };

                crc = (crc >> 1) ^ (0xedb88320 & mask);
            }

            table[index] = crc;
        }

        table
    };
}

pub fn digest(bytes: &Vec<u8>) -> String {
    if bytes.len() == 0 {
        return "00000000".to_string();
    }

    let mut crc: u32 = 0xffffffff;

    for byte in bytes {
        crc = (crc >> 8) ^ LOOKUP_TABLE[((crc ^ (*byte as u32)) & 0xff) as usize];
    }

    format!("{:01$x}", !crc, 8)
}
