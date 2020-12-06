pub struct FizzBuzz {
    pub counter: i32
}

impl FizzBuzz {
    pub fn call(&self) -> &str {
        if self.counter % 2 == 0 && self.counter % 3 == 0 {
            return "FizzBuzz";
        } else if self.counter % 2 == 0 {
            return "Fizz"
        } else if self.counter % 3 == 0 {
            return "Buzz"
        } else {
            return ""
        }
    }
}