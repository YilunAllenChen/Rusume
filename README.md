# [A blazingly-fast resume builder](https://yilunallenchen.github.io/Rusume/#/)
- Opinionated layout, so you don't have to worry about it.
- Fill-in-the-blanks, but real-time updates.
- Supports markdown in experience description.
- Blazingly-fast, because it's powered by Rust/Yew and WebAssembly. 🦀🦀🦀.

# Known Issues
- "-" in markdown doesn't work.

## Features to Add
- The `Skills` section
- Adjutable layout


### Developer setup
1.  Install Rust
    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```
2. Install trunk & add wasm target
    ```bash
    cargo install trunk wasm-bindgen-cli
    rustup target add wasm32-unknown-unknown
    ```
3. Run under root
    ```bash
    trunk serve
    ```

4. If run into issues, try unintall yew and recomile everything
    ```bash
    cargo remove yew
    cargo clean
    cargo build
    ```
