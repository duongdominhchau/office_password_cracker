/// Calculate the hash based on the algorithm used by MS Office.
/// Basically the algorithm just rotate and XOR
pub fn office_hash(content: &str) -> u16 {
    let mut hash: u16 = 0;
    content.bytes().rev().for_each(|byte| {
        hash ^= byte as u16;
        let bit_14: u16 = (hash & 0b0100_0000_0000_0000) >> 14;
        hash = (hash << 1) | bit_14;
    });
    hash ^ (content.len() as u16) ^ 0xCE4B
}
