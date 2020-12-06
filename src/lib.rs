mod fizz_buzz;

use fizz_buzz::FizzBuzz;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let mut fb: FizzBuzz = FizzBuzz {counter: 0};
    for _i in 1..100 {
        console::log_1(&fb.call().into());
        fb.increment();
    }
}