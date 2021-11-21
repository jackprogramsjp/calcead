# calcead
Calculator that compiles to LLVM

## Binaries

We only have one binary for `aarch64-apple-darwin`. If you have a different OS Architecture, you need to build from source, which is very easy to do.

If you do have `aarch64-apple-darwin` as your OS, you can just execute this script to automatically install the binary into your system:
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/jackprogramsjp/calcead/main/install.sh)"
```

For `aarch64-apple-darwin`, if you installed using the `install.sh` script and you want to uninstall the binary, just execute this:
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/jackprogramsjp/calcead/main/uninstall.sh)"
```

## To build from source

You must install Rust; you must have `rustup` and `cargo`.

To build, all you have to do is execute `cargo build` on the terminal (make sure on root directory).

## Cross-compilation

If you want to cross-compile, you can add the targets yourself and cross-compile by that, or use our default targets that we use.

To install our default targets, use `cross.bat` if you're on Windows, `cross.sh` if you're on Unix, and add `install` as your first argument: `./cross.sh install` or `.\cross.bat install`.

After installing the targets, you can finally cross compile by just running the script: `./cross.sh` or `.\cross.bat`.

If you want to remove our default targets, just add `remove` as your first argument: `./cross.sh remove` or `.\cross.bat remove`.

**Note:** I've had some linking errors trying to cross-compile on MacOS AArch64.
