# Control RGB LED displays from Rust

This repository contains my fork of the [Rust crate `rpi-led-matrix`](https://crates.io/crates/rpi-led-matrix)
providing Rust bindings for [my fork](https://github.com/compensis/rpi-rgb-led-matrix/tree/feature-textwrap)
of Hzeller's [`rpi-rgb-led-matrix`](https://github.com/hzeller/rpi-rgb-led-matrix) C++ library.

Hzeller's rpi-rgb-led-matrix C++ library allows the Raspberry Pi to drive commonly available RGB LED panels.

My fork of the library adds an optimal-fit text wrapping feature.

This repository includes both raw bindings to the library in [`rpi-led-matrix-sys`](./rpi-led-matrix-sys/)
as well as higher-level, safe Rust bindings in [`rpi-led-matrix`](./rpi-led-matrix/).

[`rpi-led-matrix` README](./rpi-led-matrix/README.md)

[`rpi-led-matrix-sys` README](./rpi-led-matrix-sys/README.md)

There is also a pure Rust rewrite in the crate [`rpi-led-panel`](https://crates.io/crates/rpi_led_panel)!

## Run tests
```
sudo -E $(which cargo) test -- --test-threads=1
```
