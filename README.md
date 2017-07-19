# Bobbin Blinky

This repository contains a set of minimal Rust crates to blink LEDs on popular development boards. 

These crates have no dependencies, and use hard-coded constants for register addresses. They are also compiled with opt-level="s" to simplify the compiled code and to eliminate the dependency on compiler_builtins.

## bobbin-cli

See [bobbin-cli](https://github.com/bobbin-rs/bobbin-cli/) for a Rust-based command line utility for
building and deploying these applications.