mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn fibonacci(num: u32) -> u32 {
    match num {
        1 | 2 => 1,
        _ => fibonacci(num - 1) + fibonacci(num - 2),
    }
}
