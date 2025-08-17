# Tasks to build and the project

## test-core
Tests the core rover functionality
```sh
cargo test -p rover
```

## build-rust-go
> Builds the rust & go code


```sh
echo 'building rust library'
echo 'building for AMD64 linux'
rustup target add x86_64-unknown-linux-gnu
cargo zigbuild --release --target=x86_64-unknown-linux-gnu -p c_rover
echo 'building for Linux ARM'
rustup target add aarch64-unknown-linux-gnu
cargo zigbuild --release --target=aarch64-unknown-linux-gnu -p c_rover
echo 'building for macOS ARM'
rustup target add aarch64-apple-darwin 
cargo zigbuild --target=aarch64-apple-darwin -p c_rover
```

## run
> Runs the go code which calls the built go library
```sh
go get github.com/ebitengine/purego
echo 'running the go code...'
go run main.go
```

## build-run-rust-go
> Build the rust library & go crate, and run the go code which calls rust.
```sh
mask build && mask run
```

## rover-docs
Open the docs for the rover crate in-browser.
This will include the private items, which have comments.
```sh
cargo doc --open --document-private-items -p rover
```

## c-docs
Open the docs for the c-rover crate in-browser.
This will include the private items, which have comments.
```sh
cargo doc --open --document-private-items -p c_rover
```
