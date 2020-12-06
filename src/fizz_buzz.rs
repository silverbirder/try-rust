use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct FizzBuzz {
    pub counter: i32
}

#[wasm_bindgen]
impl FizzBuzz {
    pub fn call(&self) {
        if self.counter % 2 == 0 && self.counter % 3 == 0 {
            console::log_1(&JsValue::from("FizzBuzz"));
        } else if self.counter % 2 == 0 {
            console::log_1(&JsValue::from("Fizz"));
        } else if self.counter % 3 == 0 {
            console::log_1(&JsValue::from("Buzz"));
        } else {
            console::log_1(&JsValue::from("Nothing"));
        }
    }
    pub fn increment(&mut self) {
        self.counter += 1 ;
    }

    pub fn new() -> FizzBuzz {
        return FizzBuzz {counter: 0}
    }
}