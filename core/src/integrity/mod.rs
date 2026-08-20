const INITIAL_STATE: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const ROUND_CONSTANTS: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Digest(String);

impl Digest {
    pub fn of_text(value: &str) -> Self {
        Self::of_bytes(value.as_bytes())
    }

    pub fn of_bytes(value: &[u8]) -> Self {
        Self(format!("sha256:{}", sha256_hex(value)))
    }

    pub fn parse(value: impl Into<String>) -> Result<Self, DigestError> {
        let value = value.into();
        validate_digest(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigestError {
    InvalidFormat,
}

impl DigestError {
    pub fn code(self) -> &'static str {
        "invalid_digest_format"
    }
}

pub fn sha256_hex(input: &[u8]) -> String {
    let blocks = padded_words(input);
    let state = process_blocks(&blocks);
    format_state(&state)
}

fn padded_words(input: &[u8]) -> Vec<u32> {
    let padded = pad_bytes(input);
    bytes_to_words(&padded)
}

fn process_blocks(words: &[u32]) -> [u32; 8] {
    let mut state = INITIAL_STATE;
    for block in words.chunks_exact(16) {
        compress_block(&mut state, block);
    }
    state
}

fn format_state(state: &[u32; 8]) -> String {
    state.iter().map(|word| format!("{word:08x}")).collect()
}

fn validate_digest(value: &str) -> Result<(), DigestError> {
    let hex = value
        .strip_prefix("sha256:")
        .ok_or(DigestError::InvalidFormat)?;
    if hex.len() == 64 && hex.bytes().all(is_lower_hex) {
        return Ok(());
    }
    Err(DigestError::InvalidFormat)
}

fn is_lower_hex(value: u8) -> bool {
    value.is_ascii_digit() || (b'a'..=b'f').contains(&value)
}

fn pad_bytes(input: &[u8]) -> Vec<u8> {
    let mut padded = input.to_vec();
    padded.push(0x80);
    append_zero_padding(&mut padded);
    padded.extend_from_slice(&bit_length(input).to_be_bytes());
    padded
}

fn append_zero_padding(padded: &mut Vec<u8>) {
    let zero_count = (56 + 64 - (padded.len() % 64)) % 64;
    padded.resize(padded.len() + zero_count, 0);
}

fn bit_length(input: &[u8]) -> u64 {
    (input.len() as u64).wrapping_mul(8)
}

fn bytes_to_words(bytes: &[u8]) -> Vec<u32> {
    bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect()
}

fn compress_block(state: &mut [u32; 8], block: &[u32]) {
    let schedule = expand_schedule(block);
    let working = run_rounds(*state, &schedule);
    merge_state(state, &working);
}

fn expand_schedule(block: &[u32]) -> [u32; 64] {
    let mut schedule = [0_u32; 64];
    schedule[..16].copy_from_slice(block);
    for index in 16..64 {
        schedule[index] = next_schedule_word(&schedule, index);
    }
    schedule
}

fn next_schedule_word(schedule: &[u32; 64], index: usize) -> u32 {
    small_sigma_one(schedule[index - 2])
        .wrapping_add(schedule[index - 7])
        .wrapping_add(small_sigma_zero(schedule[index - 15]))
        .wrapping_add(schedule[index - 16])
}

fn run_rounds(mut working: [u32; 8], schedule: &[u32; 64]) -> [u32; 8] {
    for index in 0..64 {
        working = run_round(working, schedule[index], ROUND_CONSTANTS[index]);
    }
    working
}

fn run_round(value: [u32; 8], word: u32, constant: u32) -> [u32; 8] {
    let first = round_first(&value, word, constant);
    let second = round_second(&value);
    [
        first.wrapping_add(second),
        value[0],
        value[1],
        value[2],
        value[3].wrapping_add(first),
        value[4],
        value[5],
        value[6],
    ]
}

fn round_first(value: &[u32; 8], word: u32, constant: u32) -> u32 {
    big_sigma_one(value[4])
        .wrapping_add(choose(value[4], value[5], value[6]))
        .wrapping_add(value[7])
        .wrapping_add(constant)
        .wrapping_add(word)
}

fn round_second(value: &[u32; 8]) -> u32 {
    big_sigma_zero(value[0]).wrapping_add(majority(value[0], value[1], value[2]))
}

fn merge_state(state: &mut [u32; 8], working: &[u32; 8]) {
    for index in 0..8 {
        state[index] = state[index].wrapping_add(working[index]);
    }
}

fn choose(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

fn majority(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

fn big_sigma_zero(value: u32) -> u32 {
    value.rotate_right(2) ^ value.rotate_right(13) ^ value.rotate_right(22)
}

fn big_sigma_one(value: u32) -> u32 {
    value.rotate_right(6) ^ value.rotate_right(11) ^ value.rotate_right(25)
}

fn small_sigma_zero(value: u32) -> u32 {
    value.rotate_right(7) ^ value.rotate_right(18) ^ (value >> 3)
}

fn small_sigma_one(value: u32) -> u32 {
    value.rotate_right(17) ^ value.rotate_right(19) ^ (value >> 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_matches_known_empty_vector() {
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn sha256_matches_known_abc_vector() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn digest_parse_rejects_noncanonical_values() {
        assert_eq!(Digest::parse("abc"), Err(DigestError::InvalidFormat));
    }
}
