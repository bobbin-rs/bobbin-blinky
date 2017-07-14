# Bobbin Blinky

This repository contains a set of minimal Rust crates to blink LEDs on popular development boards. They exist to make it possible to produce a known-good binary 
for validating a development toolchain.

These crates have no dependencies, and use hard-coded constants for register addresses. They are also compiled with opt-level="s" to simplify the compiled code  and to eliminate the dependency on compiler_builtins.