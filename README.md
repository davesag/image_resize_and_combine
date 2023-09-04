# Image Resize and Combine

A simple couple of Wasm functions that demonstrate how to build a simple Wasm app.

It exposes a couple of functions that resize and combine a set of images.

## Set up your environment

1. Install Rust from [`www.rust-lang.org/tools/install`](https://www.rust-lang.org/tools/install)

   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Add the WebAssembly target and install wasm-pack

   ```sh
   rustup target add wasm32-unknown-unknown
   cargo install wasm-pack
   ```

## Install dependencies and build it

```sh
wasm-pack build --target web
```

## Run a web-server and see it work

```sh
npx http-server .
```

then go to [`localhost:8080`](http://localhost:8080).
