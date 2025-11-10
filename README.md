# Aetheris: Secure Password Generator

## Abstract

Aetheris is a security focused password generation utility. It employs the ChaCha12 Cryptographically Secure Pseudorandom Number Generator (CSPRNG) to ensure a high degree of entropy and security in the generated output.

## Core Features

*   **Cryptographic Security**: The generator's foundation is the ChaCha12 algorithm, ensuring the output is not susceptible to predictive analysis.
*   **Performance**: The password generation process is virtually instantaneous.
*   **Entropy and Strength**: A standard 16-character password possesses over 105 bits of entropy, rendering it highly resistant to brute-force attacks.
*   **Configurable Length**: The desired password length can be adjusted directly within the source code.

## Prerequisites and Installation

1.  A working installation of the Rust toolchain (including `rustc` and `cargo`) is required. You can obtain it from the [official Rust website](https://www.rust-lang.org/tools/install).
2.  Clone the repository to your local machine:
    ```bash
    git clone https://github.com/your-username/aetheris.git
    cd aetheris
    ```

## Usage

For optimal performance, it is recommended to compile and run the application in release mode.

### 1. Generate a Password

By default, the application generates a 16-character password.

```bash
cargo run --release
```
Example output:
```
Your random password: qG)7!p#R&zK$vB*3
```
### 2. Customize Password Length
To modify the length of the generated password, edit the src/main.rs file. Change the value assigned to the n variable within the main function.
code

```rust
// src/main.rs

fn main() {
    let n: usize = 16; // <-- Modify this value for a different length
    println!("Your random password: {}", gen_pass(n));
}
```
## Roadmap

- [ ] Implement command-line argument parsing for dynamic configuration.
- [ ] Develop a Graphical User Interface (GUI) using Tauri.
- [ ] Add functionality for batch password generation.

## License

This project is licensed under the MIT License.
