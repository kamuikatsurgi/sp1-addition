//! A simple program that takes three numbers as inputs, and proves addition of
//! first two numbers equal to the third number.

#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let a = sp1_zkvm::io::read::<u32>();
    let b = sp1_zkvm::io::read::<u32>();
    let c = sp1_zkvm::io::read::<u32>();

    let is_valid_addition = is_valid_addition(a, b, c);

    sp1_zkvm::io::commit(&is_valid_addition);
}

fn is_valid_addition(a: u32, b: u32, c: u32) -> bool {
    if a + b == c {
        return true;
    }
    false
}
