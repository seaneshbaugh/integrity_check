static S: [u32; 64] = [ 7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
                        5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
                        4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
                        6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21 ];

static K: [u32; 64] = [ 0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
                        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
                        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
                        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
                        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
                        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
                        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
                        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
                        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
                        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
                        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
                        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
                        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
                        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
                        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
                        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391 ];

fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !z)
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    (x ^ y) ^ z
}

fn i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}

macro_rules! step {
    ($f:ident, $a:ident, $b:ident, $c:ident, $d:ident, $x:expr, $k:expr, $s:expr) => {{
        $a = $a.wrapping_add($f($b, $c, $d)).wrapping_add($x).wrapping_add(K[$k]);

        $a = ($a << S[$s]) | (($a & 0xffffffff) >> (32 - S[$s]));

        $a = $a.wrapping_add($b);
    }};
}

fn pad(bytes: &Vec<u8>) -> Vec<u8> {
    let mut padded_bytes: Vec<u8> = bytes.clone();

    padded_bytes.push(0x80);

    let zero_padding_length: usize = ((((bytes.len() + 9) / 64) * 64) + 55) - bytes.len();

    let mut zero_padding: Vec<u8> = vec![0; zero_padding_length];

    padded_bytes.append(&mut zero_padding);

    let original_length = bytes.len() * 8;

    for offset in 0..8 {
        padded_bytes.push((original_length >> (offset * 8)) as u8);
    }

    padded_bytes
}

pub fn digest(bytes: &Vec<u8>) -> String {
    let padded_bytes: Vec<u8> = pad(bytes);

    let mut a: u32 = 0x67452301;

    let mut b: u32 = 0xefcdab89;

    let mut c: u32 = 0x98badcfe;

    let mut d: u32 = 0x10325476;

    for offset in 0..(padded_bytes.len() / 64) {
        let aa: u32 = a;

        let bb: u32 = b;

        let cc: u32 = c;

        let dd: u32 = d;

        let block_start: usize = offset * 64;

        let block_end: usize = (offset * 64) + 64;

        let slice = &padded_bytes[block_start..block_end];

        let mut block: Vec<u32> = vec![];

        for i in 0..16 {
            let w1: u32 = slice[i * 4] as u32;

            let w2: u32 = (slice[(i * 4) + 1] as u32) << 8;

            let w3: u32 = (slice[(i * 4) + 2] as u32) << 16;

            let w4: u32 = (slice[(i * 4) + 3] as u32) << 24;

            let word: u32 = w1 | w2 | w3 | w4;

            block.push(word);
        }

        step!(f, a, b, c, d, block[0], 0, 0);
        step!(f, d, a, b, c, block[1], 1, 1);
        step!(f, c, d, a, b, block[2], 2, 2);
        step!(f, b, c, d, a, block[3], 3, 3);

        step!(f, a, b, c, d, block[4], 4, 4);
        step!(f, d, a, b, c, block[5], 5, 5);
        step!(f, c, d, a, b, block[6], 6, 6);
        step!(f, b, c, d, a, block[7], 7, 7);

        step!(f, a, b, c, d, block[8], 8, 8);
        step!(f, d, a, b, c, block[9], 9, 9);
        step!(f, c, d, a, b, block[10], 10, 10);
        step!(f, b, c, d, a, block[11], 11, 11);

        step!(f, a, b, c, d, block[12], 12, 12);
        step!(f, d, a, b, c, block[13], 13, 13);
        step!(f, c, d, a, b, block[14], 14, 14);
        step!(f, b, c, d, a, block[15], 15, 15);


        step!(g, a, b, c, d, block[1], 16, 16);
        step!(g, d, a, b, c, block[6], 17, 17);
        step!(g, c, d, a, b, block[11], 18, 18);
        step!(g, b, c, d, a, block[0], 19, 19);

        step!(g, a, b, c, d, block[5], 20, 20);
        step!(g, d, a, b, c, block[10], 21, 21);
        step!(g, c, d, a, b, block[15], 22, 22);
        step!(g, b, c, d, a, block[4], 23, 23);

        step!(g, a, b, c, d, block[9], 24, 24);
        step!(g, d, a, b, c, block[14], 25, 25);
        step!(g, c, d, a, b, block[3], 26, 26);
        step!(g, b, c, d, a, block[8], 27, 27);

        step!(g, a, b, c, d, block[13], 28, 28);
        step!(g, d, a, b, c, block[2], 29, 29);
        step!(g, c, d, a, b, block[7], 30, 30);
        step!(g, b, c, d, a, block[12], 31, 31);


        step!(h, a, b, c, d, block[5], 32, 32);
        step!(h, d, a, b, c, block[8], 33, 33);
        step!(h, c, d, a, b, block[11], 34, 34);
        step!(h, b, c, d, a, block[14], 35, 35);

        step!(h, a, b, c, d, block[1], 36, 36);
        step!(h, d, a, b, c, block[4], 37, 37);
        step!(h, c, d, a, b, block[7], 38, 38);
        step!(h, b, c, d, a, block[10], 39, 39);

        step!(h, a, b, c, d, block[13], 40, 40);
        step!(h, d, a, b, c, block[0], 41, 41);
        step!(h, c, d, a, b, block[3], 42, 42);
        step!(h, b, c, d, a, block[6], 43, 43);

        step!(h, a, b, c, d, block[9], 44, 44);
        step!(h, d, a, b, c, block[12], 45, 45);
        step!(h, c, d, a, b, block[15], 46, 46);
        step!(h, b, c, d, a, block[2], 47, 47);


        step!(i, a, b, c, d, block[0], 48, 48);
        step!(i, d, a, b, c, block[7], 49, 49);
        step!(i, c, d, a, b, block[14], 50, 50);
        step!(i, b, c, d, a, block[5], 51, 51);

        step!(i, a, b, c, d, block[12], 52, 52);
        step!(i, d, a, b, c, block[3], 53, 53);
        step!(i, c, d, a, b, block[10], 54, 54);
        step!(i, b, c, d, a, block[1], 55, 55);

        step!(i, a, b, c, d, block[8], 56, 56);
        step!(i, d, a, b, c, block[15], 57, 57);
        step!(i, c, d, a, b, block[6], 58, 58);
        step!(i, b, c, d, a, block[13], 59, 59);

        step!(i, a, b, c, d, block[4], 60, 60);
        step!(i, d, a, b, c, block[11], 61, 61);
        step!(i, c, d, a, b, block[2], 62, 62);
        step!(i, b, c, d, a, block[9], 63, 63);

        a = aa.wrapping_add(a);

        b = bb.wrapping_add(b);

        c = cc.wrapping_add(c);

        d = dd.wrapping_add(d);
    }

    let mut result = "".to_string();

    for word in [a, b, c, d].iter() {
        for i in 0..4 {
            result.push_str(&format!("{:02x}", (word >> (i * 8)) as u8));
        }
    }

    result
}
