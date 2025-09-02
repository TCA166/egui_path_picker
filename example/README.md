# examples

Here you can find a simple `eframe` app built to showcase the widget.
If you are merely interested in how to use the widget, take a look at
[app.rs](./src/app.rs).

## Native

If you wish to see the picker in action on a native device, run the following:

```sh
cargo run -p example
```

## WASM

If you wish to compile the example to run in your browser, do the following:

```sh
cd example
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
trunk serve
```

Running `trunk build --release` will create a static page ready to be statically
deployed.
