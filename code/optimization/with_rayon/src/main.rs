use std::time::Instant;
use rayon::prelude::*;

const MAX_NUMBER: usize = 100_000;

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2 .. n {
            if n % div == 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let candidates: Vec<usize> = (0 .. MAX_NUMBER).collect();
    let start = Instant::now(); // We're not timing the initial creation
    
    let primes: Vec<&usize> = candidates
        .par_iter()
        .filter(|n| is_prime(**n))
        .collect();
    let elapsed = start.elapsed();

    // Results
    println!("Found {} primes", primes.len());
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}
