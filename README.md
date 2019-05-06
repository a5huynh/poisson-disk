# 🦀🐟 `poisson-disk`

![example viz](docs/example.gif)

This is a Rust + WASM implementation of the algorithm presented in
"[Fast Poisson Disk Sampling in Arbitrary Dimensions][paper-link]" aka the
Bridson Algorithm.

The Rust code was meant to be extracted into it's own library to be used
in a game that utilizes procedural generation. The WASM was used to
visualize the sampling and make sure things were being generated correctly.

[paper-link]: https://www.cs.ubc.ca/~rbridson/docs/bridson-siggraph07-poissondisk.pdf


## Building the rust binary + WASM bindings.

Make sure you have [wasm-pack](https://github.com/rustwasm/wasm-pack) installed.

```
cargo install
make build
```

To see the visualization, go into the `www` directory and run the following:

```
npm run start
```

Navigate to `http://localhost:8080` and the algorithm will kick off
immediately!