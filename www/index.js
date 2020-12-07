import { FizzBuzz } from "wasm-game-of-life";

const fb = FizzBuzz.new();
fb.call();
fb.increment();
fb.call();
console.log(fb);