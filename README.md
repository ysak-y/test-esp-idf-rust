# test-esp-idf-rust

Example to implement firmware for esp32 with Rust

## espup

This firmware is witten in Rust. You need to install `espup`. first. Please follow this document https://github.com/esp-rs/rust-build .

## Wokwi

You can simulate this firmware on [Wokwi](https://wokwi.com/). Read this tutorial https://docs.wokwi.com/vscode/getting-started .

## espflash

You can write firmware by using [espflash](https://github.com/esp-rs/espflash).

```sh
$ cargo build
$ espflash flash /target/xtensa-esp-espidf/debug/test-esp-idf-rust
```
