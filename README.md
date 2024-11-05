# SP1 Addition Project

The main goal is to prove that the sum of two given numbers, `a` and `b`, is equal to an expected result, `c`, without revealing the numbers `a`, `b`, and `c`.

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/getting-started/install.html)

## Running the Project

### Build the Program

To build the program, run the following command:

```sh
cd program
cargo prove build
```

### Execute the Program

To run the program without generating a proof:

```sh
cd script
cargo run --release -- --execute a b c
```

This will execute the program and display the output.