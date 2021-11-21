#!/bin/sh

targets="aarch64-apple-darwin i686-pc-windows-gnu i686-unknown-linux-gnu x86_64-apple-darwin x86_64-pc-windows-gnu x86_64-unknown-linux-gnu"

if [ "$1" = "install" ]; then
    if type "rustup" > /dev/null; then
        for target in $targets; do
            rustup target add $target
        done
    else
        echo "Error: rustup doesn't exist in this system, please install Rust" >&2
        exit 1
    fi
elif [ "$1" = "remove" ]; then
    if type "rustup" > /dev/null; then
        for target in $targets; do
            rustup target remove $target
        done
    else
        echo "Error: rustup doesn't exist in this system, please install Rust" >&2
        exit 1
    fi
else
    if type "cargo" > /dev/null; then
        for target in $targets; do
            cargo build --target $target
        done
    else
        echo "Error: cargo doesn't exist in this system, please install Rust" >&2
        exit 1
    fi
fi
