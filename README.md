<div align="center">
  <h1><code>tantara</code></h1>
  <strong>Full-text search built with Rust and WebAssembly🦀🕸</strong>
</div>

## Quick Start

**Step 1: Install Rustup**

Install [Rustup](https://rustup.rs). We'll use it to install dependencies. If you already installed Rustup, you can update to the latest version.

```bash
rustup update
```

**Step 2: Install Development Tools**

```bash
# Install Clippy
rustup component add clippy

# Install rustfmt
rustup component add rustfmt
```

## Available Scripts

### 🛠️ Build with `yarn build`

It runs [`wasm-pack build`](https://rustwasm.github.io/wasm-pack/book/commands/build.html) to create necessary files for publishing a npm package.

### 🧪 Test in Headless Browsers with `yarn test`

It runs the [end-to-end test](https://rustwasm.github.io/wasm-pack/book/commands/test.html) with headless browsers and Node.js. The browsers are:

- Chrome
- Firefox
- Safari

You'll need to have the browsers installed in your machine to run the test script.

### 🎁 Publish to npm with `yarn publish`

It [creates a tarball and publishes on npm](https://rustwasm.github.io/wasm-pack/book/commands/pack-and-publish.html).

### 🔬 Lint with `yarn lint`

It runs [Clippy](https://github.com/rust-lang/rust-clippy) to lint and fix the source code.

### 📝 Format with `yarn format`

It runs [rustfmt](https://github.com/rust-lang/rustfmt) to format Rust code according to community style guidelines.

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
- `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you

> [📚 Read this template tutorial][template-docs] & [other `wasm-pack` tutorials online][tutorials] to learn more about `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
