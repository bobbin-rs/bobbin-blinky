# Bobbin Blinky

This repository contains a set of minimal Rust crates to blink LEDs on popular development boards. 

These crates have no dependencies, and use hard-coded constants for register addresses. They are also compiled with opt-level="s" to simplify the compiled code and to eliminate the dependency on compiler_builtins.

## Bobbin CLI

See [Bobbin CLI](https://github.com/bobbin-rs/bobbin-cli/) for a Rust-based command line utility for
building and deploying these applications.

## Build and Deploy

In most cases, once you have Bobbin CLI and the required prerequisites for your board installed, you should be
able to `cd` into the directory for your board and `bobbin load` to build and load the binary.

```
$ bobbin load
   Compiling nucleo-f429zi v0.1.0 (file:///home/bobbin/bobbin-blinky/nucleo-f429zi)
    Finished dev [optimized + debuginfo] target(s) in 0.12 secs
   text	   data	    bss	    dec	    hex	filename
    148	      0	      4	    152	     98	target/thumbv7em-none-eabihf/debug/nucleo-f429zi
     Loading target/thumbv7em-none-eabihf/debug/nucleo-f429zi
    Complete Successfully flashed device
      Loader Load Complete
$
```

In some cases, you may need to use `bobbin run` to start your application after loading it.

## Troubleshooting

If you get errors or the board simply doesn't blink, here's a good list of things to check:

- Make sure you have the most recent version of Bobbin CLI by running `cargo install bobbin-cli --force`.
- Run `bobbin check` to see the versions of the prerequisites that Bobbin CLI can find. You may need
to install newer versions of these prerequisites.
- The next most likely cause is the firmware for the on-board debugger if your board has one. You
may wish to review [Bobbin CLI - Development Board Firmware](https://github.com/bobbin-rs/bobbin-cli/blob/master/FIRMWARE.md) for an overview of the situation and links
to updated versions.