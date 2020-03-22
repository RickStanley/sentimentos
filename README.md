<div align="center">

  <h1><code>Sentiment Analysis WebAssembly</code></h1>

  <small>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></small>
</div>

## About (by Rick Stanley)

The goal is to have an Sentiment Analysis compiled to wasm, experiment purpose 🧪.
I love Rust, but I haven't the pleasure of using it in production yeat.
I adore WebAssembly, ditto.

### Thoughts
- Should we keep and dictionary for every language, whilst compressing it?
- Should we create a separate build for:
  - Build for Browser: Compile a `.wasm` binary with all the dictionaries;
  - Build for Server (agnostic Host): Compile the core function and separate build for every dictionary
- **If we separate dictionaries from core:** How would we include the dictionaries?
  1. Compile stemm entries if not stemmed beforehand?
  2. Expect a stemmed entry?

## 🚴 Usage

This project uses [`wasm-pack`](https://github.com/RickStanley/sentimentos/blob/d6c803f33fd2e26b761a415a0edb41f0f152566d/README.md#L5)

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
