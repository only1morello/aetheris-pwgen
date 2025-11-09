---

### Исправленный README (Академический английский)

```markdown
# Aetheris: Secure Password Generator

## Abstract

Aetheris is a robust password generation utility. It employs the ChaCha12 Cryptographically Secure Pseudorandom Number Generator (CSPRNG), accessed via `rand::thread_rng`, to ensure a high degree of entropy and security in the generated output.

## Core Features

*   **Cryptographic Security**: The generator's foundation is the ChaCha12 algorithm, ensuring that the generated passwords are not susceptible to predictive analysis.
*   **Performance**: The password generation process is virtually instantaneous due to the high throughput of the underlying CSPRNG.
*   **Entropy and Strength**: A standard 16-character password possesses over 105 bits of entropy, rendering it highly resistant to brute-force attacks.
*   **Configurable Length**: The desired password length can be adjusted directly within the `main.rs` source file.

## Prerequisites and Installation

1.  A working installation of the Rust toolchain (including `rustc` and `cargo`) is required. You can obtain it from the [official Rust website](https://www.rust-lang.org/tools/install).
2.  Clone the repository to your local machine:
    ```bash
    git clone https://github.com/your-username/aetheris.git
    cd aetheris
    ```

## Usage

For optimal performance, it is recommended to compile and run the application in release mode.

**Generate a Password**

By default, the application generates a 16-character password using a predefined set of alphanumeric and special characters.
```bash
cargo run --release

## Roadmap
- [ ] Tauri GUI
- [ ] Batch generation
- [ ] Custom character sets
