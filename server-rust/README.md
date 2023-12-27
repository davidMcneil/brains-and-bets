# Weighty Inquiry Server

## Download and Run

To run the server on an x86 linux OS grab the latest release from [here](https://github.com/davidMcneil/weighty-inquiry/releases/latest).

    > chmod +x weighty-inquiry // Make it executable
    > ./weighty-inquiry -h     // See the help message

## Development

Depends on Rust and its associated tooling. Install instructions can be found [here](https://www.rust-lang.org/tools/install).

Build and run the server for development.

    > cargo run -- -h

Run the test suite.

    > cargo test

Produce a statically linked production build (depends on [cross](https://github.com/rust-embedded/cross)). The file will be located at `target/x86_64-unknown-linux-musl/release/weighty-inquiry`.

    > cross build --release --target=x86_64-unknown-linux-musl
