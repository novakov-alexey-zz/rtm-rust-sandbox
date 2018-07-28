#!/usr/bin/env bash
 alias rust-musl-builder='docker run --rm -it  -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder:nightly'
 rust-musl-builder cargo build --release
 docker build -t rtm:0.1.0 .