# wasmi sample usage

This sample is dedicated to do act like Parity's [interpret](https://github.com/paritytech/wasmi/blob/master/examples/interpret.rs) sample.

# Generate the sample wasm file

First, please add the rustup target

```
rustup target add wasm32-unknown-unknown
```

Then compile the wasm-fib code sample as follows

```
cd wasm-fib
cargo build --target wasm32-unknown-unknown --release
```

The generated wasm file is located at `wasm-fib/target/wasm32-unknown-unknown/release/wasm_fib.wasm`. Copy this file to `bin/xx.wasm`.

Then `make` to build this code sample, and then

```
cd bin
./app
```

# Acknowledgement

The interpreter driver comes from Parity's interpreter.rs.

The fib code comes from Tony. Thanks to Tony!
