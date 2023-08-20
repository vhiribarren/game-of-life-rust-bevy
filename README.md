# Conway's Game of Life

## How to run

To run with a native window:

    $ cargo run

To run the WebAssembly version, as suggested the [Bevy
Cheatbook](https://bevy-cheatbook.github.io/platforms/wasm.html),
`wasm-server-runner` can be used:

    $ cp .cargo/config.toml.example .cargo/config.toml
    $ cargo install wasm-server-runner

Now, a local web server is launched with the compiled WASM file when the
following command is launched:

    $ cargo run --target wasm32-unknown-unknown

If you've copied the `config.toml`, this alias command can also be used:

    $ cargo serve