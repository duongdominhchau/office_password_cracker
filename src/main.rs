use office_password_cracker::{
    bruteforce_generator::AsciiStringBruteForceGenerator, office_hash::office_hash,
};
use std::time::{Duration, Instant};

const TARGET_HASH: u16 = 0xC092;

fn main() {
    let charset: Vec<u8> = (32..=126).collect();
    let bruteforcer = AsciiStringBruteForceGenerator::new(&charset);
    let mut start = Instant::now();
    let mut count = 0;
    let mut count_start = Instant::now();

    for s in bruteforcer {
        if count_start.elapsed() >= Duration::from_secs(1) {
            println!("Processed {} hashes", count);
            count = 0;
            count_start = Instant::now();
        }
        // Stop if can't find new password within 5s
        if start.elapsed() > Duration::from_secs(5) {
            break;
        }
        let hash = office_hash(&s);
        if hash == TARGET_HASH {
            println!("{}", s);
            start = Instant::now();
        }
        count += 1;
    }
}
