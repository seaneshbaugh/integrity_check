const STATE_SIZE: usize = 200;

const NUMBER_OF_ROUNDS: usize = 24;

static K: [u64; 24] = [ 0x0000000000000001, 0x0000000000008082, 0x800000000000808a, 0x8000000080008000,
                        0x000000000000808b, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
                        0x000000000000008a, 0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
                        0x000000008000808b, 0x800000000000008b, 0x8000000000008089, 0x8000000000008003,
                        0x8000000000008002, 0x8000000000000080, 0x000000000000800a, 0x800000008000000a,
                        0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008 ];

static ROTC: [usize; 24] = [  1,  3,  6, 10, 15, 21, 28, 36,
                             45, 55,  2, 14, 27, 41, 56,  8,
                             25, 43, 62, 18, 39, 61, 20, 44 ];

static PILN: [usize; 24] = [ 10,  7, 11, 17, 18,  3,  5, 16,
                              8, 21, 24,  4, 15, 23, 19, 13,
                             12,  2, 20, 14, 22,  9,  6,  1 ];

fn rotate_left(x: u64, n: usize) -> u64 {
    (x << n) | (x >> (64 - n))
}

fn sha3_keccak(state: &mut [u8]) {
    let mut block: Vec<u64> = vec![];

    for i in 0..25 {
        let w1: u64 = state[i * 8] as u64;

        let w2: u64 = (state[(i * 8) + 1] as u64) << 8;

        let w3: u64 = (state[(i * 8) + 2] as u64) << 16;

        let w4: u64 = (state[(i * 8) + 3] as u64) << 24;

        let w5: u64 = (state[(i * 8) + 4] as u64) << 32;

        let w6: u64 = (state[(i * 8) + 5] as u64) << 40;

        let w7: u64 = (state[(i * 8) + 6] as u64) << 48;

        let w8: u64 = (state[(i * 8) + 7] as u64) << 56;

        let word: u64 = w1 | w2 | w3 | w4 | w5 | w6 | w7 | w8;

        block.push(word);
    }

    let mut bc: [u64; 5] = [0; 5];

    for round in 0..NUMBER_OF_ROUNDS {
        for i in 0..5 {
            bc[i] = block[i] ^ block[i + 5] ^ block[i + 10] ^ block[i + 15] ^ block[i + 20];
        }

        for i in 0..5 {
            let t: u64 = bc[(i + 4) % 5] ^ rotate_left(bc[(i + 1) % 5], 1);

            for j in 0..5 {
                block[(j * 5) + i] ^= t;
            }
        }

        let mut t: u64 = block[1];

        for i in 0..NUMBER_OF_ROUNDS {
            bc[0] = block[PILN[i]];

            block[PILN[i]] = rotate_left(t, ROTC[i]);

            t = bc[0];
        }

        for j in 0..5 {
            for i in 0..5 {
                bc[i] = block[(j * 5) + i];
            }

            for i in 0..5 {
                let n: u64 = bc[(i + 1) % 5];

                let o: u64 = !n;

                block[(j * 5) + i] ^= o & bc[(i + 2) % 5];
            }
        }

        block[0] ^= K[round];
    }

    for i in 0..25 {
        for j in 0..8 {
            state[(i * 8) + j] = (block[i] >> (j * 8)) as u8;
        }
    }
}

pub fn digest(bytes: &Vec<u8>, digest_length: usize) -> String {
    let mut state: [u8; STATE_SIZE] = [0; STATE_SIZE];

    let rate: usize = STATE_SIZE - (2 * digest_length);

    let mut j: usize = 0;

    for i in 0..bytes.len() {
        state[j] ^= bytes[i];

        j = j + 1;

        if j >= rate {
            sha3_keccak(&mut state);

            j = 0;
        }
    }

    state[j] ^= 0x06;

    state[rate - 1] ^= 0x80;

    sha3_keccak(&mut state);

    let mut result: String = "".to_string();

    for i in 0..digest_length {
        result.push_str(&format!("{:02x}", state[i]));
    }

    result
}
