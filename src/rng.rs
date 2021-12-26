use std::sync::Mutex;

use lazy_static::lazy_static;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use rand_pcg::Pcg32;

lazy_static! {
    static ref RNG: Mutex<Pcg32> = {
        use rand::SeedableRng;
        let rng = Pcg32::seed_from_u64(123);
        Mutex::new(rng)
    };
}

// Convenience method
pub fn seed(seed: u64) {
    use rand::SeedableRng;
    *RNG.lock().unwrap() = Pcg32::seed_from_u64(seed);
}
pub fn random<T>(seed: u64) -> T
where
    Standard: Distribution<T>,
{
    self::seed(seed);
    RNG.lock().unwrap().gen()
}
