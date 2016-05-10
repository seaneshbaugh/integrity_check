static K: [u64; 80] = [ 0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc, 0x3956c25bf348b538,
                        0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118, 0xd807aa98a3030242, 0x12835b0145706fbe,
                        0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2, 0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235,
                        0xc19bf174cf692694, 0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
                        0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5, 0x983e5152ee66dfab,
                        0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4, 0xc6e00bf33da88fc2, 0xd5a79147930aa725,
                        0x06ca6351e003826f, 0x142929670a0e6e70, 0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed,
                        0x53380d139d95b3df, 0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
                        0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30, 0xd192e819d6ef5218,
                        0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8, 0x19a4c116b8d2d0c8, 0x1e376c085141ab53,
                        0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8, 0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373,
                        0x682e6ff3d6b2b8a3, 0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
                        0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b, 0xca273eceea26619c,
                        0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178, 0x06f067aa72176fba, 0x0a637dc5a2c898a6,
                        0x113f9804bef90dae, 0x1b710b35131c471b, 0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc,
                        0x431d67c49c100d4c, 0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817 ];

fn rotate_right(x: u64, n: usize) -> u64 {
    (x >> n) | (x << (64 - n))
}

fn pad(bytes: &Vec<u8>) -> Vec<u8> {
    let mut padded_bytes: Vec<u8> = bytes.clone();

    padded_bytes.push(0x80);

    let zero_padding_length: usize = ((((bytes.len() + 17) / 128) * 128) + 111) - bytes.len();

    let mut zero_padding: Vec<u8> = vec![0; zero_padding_length];

    padded_bytes.append(&mut zero_padding);

    let original_length_hi: usize = bytes.len() >> 61;

    for offset in (0..8).rev() {
        padded_bytes.push((original_length_hi >> (offset * 8)) as u8);
    }

    let original_length_lo: usize = bytes.len() << 3;

    for offset in (0..8).rev() {
        padded_bytes.push((original_length_lo >> (offset * 8)) as u8);
    }

    padded_bytes
}

pub fn digest(bytes: &Vec<u8>) -> String {
    let padded_bytes: Vec<u8> = pad(bytes);

    let mut h0: u64 = 0xcbbb9d5dc1059ed8;

    let mut h1: u64 = 0x629a292a367cd507;

    let mut h2: u64 = 0x9159015a3070dd17;

    let mut h3: u64 = 0x152fecd8f70e5939;

    let mut h4: u64 = 0x67332667ffc00b31;

    let mut h5: u64 = 0x8eb44a8768581511;

    let mut h6: u64 = 0xdb0c2e0d64f98fa7;

    let mut h7: u64 = 0x47b5481dbefa4fa4;

    for offset in 0..(padded_bytes.len() / 128) {
        let block_start: usize = offset * 128;

        let block_end: usize = (offset * 128) + 128;

        let slice = &padded_bytes[block_start..block_end];

        let mut block: Vec<u64> = vec![];

        for i in 0..16 {
            let w1: u64 = (slice[i * 8] as u64) << 56;

            let w2: u64 = (slice[(i * 8) + 1] as u64) << 48;

            let w3: u64 = (slice[(i * 8) + 2] as u64) << 40;

            let w4: u64 = (slice[(i * 8) + 3] as u64) << 32;

            let w5: u64 = (slice[(i * 8) + 4] as u64) << 24;

            let w6: u64 = (slice[(i * 8) + 5] as u64) << 16;

            let w7: u64 = (slice[(i * 8) + 6] as u64) << 8;

            let w8: u64 = slice[(i * 8) + 7] as u64;

            let word: u64 = w1 | w2 | w3 | w4 | w5 | w6 | w7 | w8;

            block.push(word);
        }

        for i in 16..80 {
            let w1: u64 = block[i - 15];

            let s0: u64 = rotate_right(w1, 1) ^ rotate_right(w1, 8) ^ (w1 >> 7);

            let w2: u64 = block[i - 2];

            let s1: u64 = rotate_right(w2, 19) ^ rotate_right(w2, 61) ^ (w2 >> 6);

            let w3: u64 = block[i - 16];

            let w4: u64 = block[i - 7];

            block.push(w3.wrapping_add(s0).wrapping_add(w4).wrapping_add(s1));
        }

        let mut a: u64 = h0;

        let mut b: u64 = h1;

        let mut c: u64 = h2;

        let mut d: u64 = h3;

        let mut e: u64 = h4;

        let mut f: u64 = h5;

        let mut g: u64 = h6;

        let mut h: u64 = h7;

        for i in 0..80 {
            let s1: u64 = rotate_right(e, 14) ^ rotate_right(e, 18) ^ rotate_right(e, 41);

            let ch: u64 = (e & f) ^ (!e & g);

            let t1: u64 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(block[i]);

            let s0: u64 = rotate_right(a, 28) ^ rotate_right(a, 34) ^ rotate_right(a, 39);

            let maj: u64 = (a & b) ^ (a &c) ^ (b & c);

            let t2: u64 = s0.wrapping_add(maj);

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

    for word in [h0, h1, h2, h3, h4, h5].iter() {
        result.push_str(&format!("{:016x}", word));
    }

    result
}
