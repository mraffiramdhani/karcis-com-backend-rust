use rand::{thread_rng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_otp() -> u32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 30;
    let mut rng = thread_rng();
    let random_number: u32 = rng.gen_range(0..1000000);
    let otp = (now as u32 ^ random_number) % 1000000;

    otp
}