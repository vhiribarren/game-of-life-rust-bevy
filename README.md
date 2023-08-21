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


## License

MIT License

Copyright (c) 2023 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.