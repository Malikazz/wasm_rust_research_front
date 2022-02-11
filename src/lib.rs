mod utils;

use wasm_bindgen::prelude::*;
use primes::{Sieve, PrimeSet};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct PrimeReturn{
    ix : usize,
    n : u64
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-rust-front!");
}
#[wasm_bindgen]
pub fn find_prime_after(start: u64) -> usize{
    let mut pset = Sieve::new();
    let (ix, n) = pset.find(start);
    ix
}