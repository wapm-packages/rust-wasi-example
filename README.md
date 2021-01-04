# Rust WASI Example

A toy program implementing [HQ9+][3] to demo compiling Rust to [WASI][1] and running it with [Wasmer][2].

## Setting up

First, let's clone the repo:

```shell
git clone https://github.com/wasmerio/rust-wasi-example.git
cd rust-wasi-example
```

Ensure you have an up to date version of Rust nightly and run:

```shell
rustup target add wasm32-wasi --toolchain nightly
```

## Building

```shell
cargo +nightly build --target=wasm32-wasi --release
cp target/wasm32-wasi/release/wasi-example.wasm .
```

## Running

```shell
# Install wasmer
curl https://get.wasmer.io -sSfL | sh

# Run the WebAssembly file with Wasmer!
wasmer run wasi-example.wasm -- -e "HQ9+"

# Run again, giving access to the current directory and passing a file to `hq9+`
wasmer run wasi-example.wasm --dir=. -- -f examples/test.hq9+
```

## Uploading to WAPM

### Setting up `wapm` cli

To upload to [WAPM][4] we must first make an account:

1. Go to [wapm.io][4]
2. Create an account (sign up)
3. Log in with the `wapm` cli client

```shell
# Log in to WAPM
wapm login
# Enter username and password

# Success!
```

### Setting up the project for `wapm`

To run the project with `wapm` we must create a [`wapm.toml`][5] manifest file to instruct `wapm` how to run the wasm binary.

Our `wapm.toml` contains 3 sections:

- `[package]`
  This is where the metadata about the package lives
- `[[module]]`
  This is where metadata about the module (wasm binary) lives
- `[[command]]`
  This is where we tell `wapm` how to run the module and what to call it

For more information about these sections see the [documentation][6].

**IMPORTANT**:
To publish the project to the wapm registry, you must change the namespace (the `package.name` field before the `/`) to match the username you made while signing up and that you are now logged in as (`wapm whoami`).

For your convenience, you may run the following snippet to edit it for you.
```shell
# Update the example `wapm.toml` to be namespaced under your name
sed -i '' "s/YOUR-USERNAME/$(wapm whoami)/" wapm.toml
```

When making a new `wapm` project, this step is not necessary; the `wapm init` command creates a `wapm.toml` skeleton, with your name as the namespace, for you.

### Running the project with `wapm` locally

Now that we're all set up, we can use `wapm` to run our wasm binary!

```shell
# Run our module using `--` as a divider to avoid the ambiguity of who should process the argument
wapm run hq9+ -- --help

# Run our module without a divider as a convenience, `-e`valuating some HQ9+ source code
wapm run hq9+ -e "+9QH"
```

### Uploading to WAPM

Now that we've set up `wapm` and verified that it works locally, we're ready to upload to the WAPM registry:

```shell
# Upload to the wapm registry!
wapm publish

# Success!
```

### Running from WAPM

```shell
# Make a clean directory to test installing and running our wapm module
mkdir ../wapm-test
cd ../wapm-test

# Install from the registry, this will create a lockfile for us
wapm install YOUR-USERNAME/rust-example

# Run the command we made in our package
wapm run hq9+ -e "H"

# Run a source file, by pre-opening the current directory and passing the source file as an argument to hq9+
wapm run hq9+ --dir=. -f examples/test.hq9+
```

[1]: https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/
[2]: https://github.com/wasmerio/wasmer
[3]: https://esolangs.org/wiki/HQ9%2B
[4]: https://wapm.io
[5]: https://github.com/wasmerio/rust-wasi-example/blob/master/wapm.toml
[6]: https://wapm.io/help/reference
