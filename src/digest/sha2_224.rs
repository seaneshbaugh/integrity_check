static K: [u32; 64] = [ 0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
                        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
                        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
                        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
                        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
                        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
                        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
                        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2 ];

fn rotate_right(x: u32, n: usize) -> u32 {
    (x >> n) | (x << (32 - n))
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

    let mut h0: u32 = 0xc1059ed8;

    let mut h1: u32 = 0x367cd507;

    let mut h2: u32 = 0x3070dd17;

    let mut h3: u32 = 0xf70e5939;

    let mut h4: u32 = 0xffc00b31;

    let mut h5: u32 = 0x68581511;

    let mut h6: u32 = 0x64f98fa7;

    let mut h7: u32 = 0xbefa4fa4;

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

        for i in 16..64 {
            let w1: u32 = block[i - 15];

            let s0: u32 = rotate_right(w1, 7) ^ rotate_right(w1, 18) ^ (w1 >> 3);

            let w2: u32 = block[i - 2];

            let s1: u32 = rotate_right(w2, 17) ^ rotate_right(w2, 19) ^ (w2 >> 10);

            let w3: u32 = block[i - 16];

            let w4: u32 = block[i - 7];

            block.push(w3.wrapping_add(s0).wrapping_add(w4).wrapping_add(s1));
        }

        let mut a: u32 = h0;

        let mut b: u32 = h1;

        let mut c: u32 = h2;

        let mut d: u32 = h3;

        let mut e: u32 = h4;

        let mut f: u32 = h5;

        let mut g: u32 = h6;

        let mut h: u32 = h7;

        for i in 0..64 {
            let s1: u32 = rotate_right(e, 6) ^ rotate_right(e, 11) ^ rotate_right(e, 25);

            let ch: u32 = (e & f) ^ (!e & g);

            let t1: u32 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(block[i]);

            let s0: u32 = rotate_right(a, 2) ^ rotate_right(a, 13) ^ rotate_right(a, 22);

            let maj: u32 = (a & b) ^ (a &c) ^ (b & c);

            let t2: u32 = s0.wrapping_add(maj);

            h = g;

            g = f;

            f = e;

            e = d.wrapping_add(t1);

            d = c;

            c = b;

            b = a;

            a = t1.wrapping_add(t2);
        }

        h0 = h0.wrapping_add(a);

        h1 = h1.wrapping_add(b);

        h2 = h2.wrapping_add(c);

        h3 = h3.wrapping_add(d);

        h4 = h4.wrapping_add(e);

        h5 = h5.wrapping_add(f);

        h6 = h6.wrapping_add(g);

        h7 = h7.wrapping_add(h);
    }

    let mut result: String = "".to_string();

    for word in [h0, h1, h2, h3, h4, h5, h6].iter() {
        result.push_str(&format!("{:08x}", word));
    }

    result
}
