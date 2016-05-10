static K: [u32; 4] = [0x5a827999, 0x6ed9eba1, 0x8f1bbcdc, 0xca62c1d6];

fn f(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (!b & d)
}

fn g(b: u32, c: u32, d: u32) -> u32 {
    b ^ c ^ d
}

fn h(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (b & d) | (c & d)
}

fn rotate_left(x: u32, n: usize) -> u32 {
    (x << n) | (x >> (32 - n))
}

fn pad(bytes: &Vec<u8>) -> Vec<u8> {
    let mut padded_bytes: Vec<u8> = bytes.clone();

    padded_bytes.push(0x80);

    let zero_padding_length: usize = ((((bytes.len() + 9) / 64) * 64) + 55) - bytes.len();

    let mut zero_padding: Vec<u8> = vec![0; zero_padding_length];

    padded_bytes.append(&mut zero_padding);

    let original_length = bytes.len() * 8;

    for offset in (0..8).rev() {
        padded_bytes.push((original_length >> (offset * 8)) as u8);
    }

    padded_bytes
}

pub fn digest(bytes: &Vec<u8>) -> String {
    let padded_bytes: Vec<u8> = pad(bytes);

    let mut h0: u32 = 0x67452301;

    let mut h1: u32 = 0xefcdab89;

    let mut h2: u32 = 0x98badcfe;

    let mut h3: u32 = 0x10325476;

    let mut h4: u32 = 0xc3d2e1f0;

    for offset in 0..(padded_bytes.len() / 64) {
        let block_start: usize = offset * 64;

        let block_end: usize = (offset * 64) + 64;

        let slice = &padded_bytes[block_start..block_end];

        let mut block: Vec<u32> = vec![];

        for i in 0..16 {
            let w1: u32 = (slice[i * 4] as u32) << 24;

            let w2: u32 = (slice[(i * 4) + 1] as u32) << 16;

            let w3: u32 = (slice[(i * 4) + 2] as u32) << 8;

            let w4: u32 = slice[(i * 4) + 3] as u32;

            let word: u32 = w1 | w2 | w3 | w4;

            block.push(word);
        }

        for i in 16..80 {
            let word: u32 = block[i - 3] ^ block[i - 8] ^ block[i - 14] ^ block[i - 16];

            block.push(rotate_left(word, 1));
        }

        let mut a: u32 = h0;

        let mut b: u32 = h1;

        let mut c: u32 = h2;

        let mut d: u32 = h3;

        let mut e: u32 = h4;

        let mut r: u32 = 0;

        let mut k: u32 = 0;

        for i in 0..80 {
            if i <= 19 {
                r = f(b, c, d);

                k = K[0];
            }

            if i >= 20 && i <= 39 {
                r = g(b, c, d);

                k = K[1];
            }

            if i >= 40 && i <= 59 {
                r = h(b, c, d);

                k = K[2];
            }

            if i >= 60 && i <= 79 {
                r = g(b, c, d);

                k = K[3];
            }

            let t: u32 = rotate_left(a, 5).wrapping_add(r).wrapping_add(e).wrapping_add(k).wrapping_add(block[i]);

            e = d;

            d = c;

            c = rotate_left(b, 30);

            b = a;

            a = t;
        }

        h0 = h0.wrapping_add(a);

        h1 = h1.wrapping_add(b);

        h2 = h2.wrapping_add(c);

        h3 = h3.wrapping_add(d);

        h4 = h4.wrapping_add(e);
    }

    let mut result = "".to_string();

    for word in [h0, h1, h2, h3, h4].iter() {
        result.push_str(&format!("{:08x}", word));
    }

    result
}
