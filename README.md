# Conway's GoL

## What?

Conway's Game of Life written in Rust using the crate MacroQuad with WASM.

## Why?

This is a simple project that is used as a Rust+WASM playground.
Collaborative workflow using git is also put into practice.

## How?

Instructions:

### Locally:

```sh
cargo run 
```

#### Test WASM locally:

Deps:

```sh
rustup target add wasm32-unknown-unknown 
cargo install basic-http-server
```

Now the `Makefile` can be used as follows. It starts automatically the server at `localhost:4000`


```sh
make wasm
```

### Web:

TODO! -> add link to github pages












