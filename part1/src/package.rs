
use rand::{Rng, rng};  // Use `rng` instead of `thread_rng`

pub fn generate_random_number() {
    let mut rng = rng(); // Updated to `rng()` instead of `thread_rng()`
    let n: i32 = rng.random_range(1..101); // Updated to `random_range`
    println!("Random number: {}", n);
}

