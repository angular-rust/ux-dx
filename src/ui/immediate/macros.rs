pub(crate) const fn hash(s: &str) -> u32 {
    let s = s.as_bytes();
    let mut hash = 3581u32;
    let mut i = 0usize;
    while i < s.len() {
        hash = hash.wrapping_mul(33).wrapping_add(s[i] as u32);
        i += 1;
    }
    return hash;
}

pub(crate) const fn splitmix(seed: u64) -> u64 {
    let next = seed.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = next;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    return z ^ (z >> 31);
}

pub(crate) const SEED: u64 = splitmix(hash(env!("CARGO")) as u64); // Pick some other env var...

pub const fn entropy(file: &str, line: u32, column: u32) -> u64 {
    splitmix(
        SEED ^ (hash(file) as u64
            ^ (line as u64).rotate_left(32)
            ^ (column as u64).rotate_left(48)),
    )
}

#[macro_export]
macro_rules! uiid {
    () => {{
        const ENTROPY: u64 = $crate::ui::immediate::entropy(file!(), line!(), column!());
        ENTROPY
    }};
}
